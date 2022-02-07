pub trait IAnnotationProvider_Impl: Sized {
    fn AnnotationTypeId(&self) -> ::windows::core::Result<i32>;
    fn AnnotationTypeName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DateTime(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Target(&self) -> ::windows::core::Result<IRawElementProviderSimple>;
}
impl ::windows::core::RuntimeName for IAnnotationProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IAnnotationProvider";
}
impl IAnnotationProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProvider_Impl, const OFFSET: isize>() -> IAnnotationProvider_Vtbl {
        unsafe extern "system" fn AnnotationTypeId<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AnnotationTypeId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnnotationTypeName<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AnnotationTypeName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Author() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateTime<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Target<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Target() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAnnotationProvider, OFFSET>(),
            AnnotationTypeId: AnnotationTypeId::<Identity, Impl, OFFSET>,
            AnnotationTypeName: AnnotationTypeName::<Identity, Impl, OFFSET>,
            Author: Author::<Identity, Impl, OFFSET>,
            DateTime: DateTime::<Identity, Impl, OFFSET>,
            Target: Target::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnnotationProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Automation_Peers")]
pub trait ICustomNavigationProvider_Impl: Sized {
    fn NavigateCustom(&self, direction: super::Peers::AutomationNavigationDirection) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "UI_Xaml_Automation_Peers")]
impl ::windows::core::RuntimeName for ICustomNavigationProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ICustomNavigationProvider";
}
#[cfg(feature = "UI_Xaml_Automation_Peers")]
impl ICustomNavigationProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomNavigationProvider_Impl, const OFFSET: isize>() -> ICustomNavigationProvider_Vtbl {
        unsafe extern "system" fn NavigateCustom<Identity: ::windows::core::IUnknownImpl, Impl: ICustomNavigationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: super::Peers::AutomationNavigationDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NavigateCustom(direction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomNavigationProvider, OFFSET>(),
            NavigateCustom: NavigateCustom::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomNavigationProvider as ::windows::core::Interface>::IID
    }
}
pub trait IDockProvider_Impl: Sized {
    fn DockPosition(&self) -> ::windows::core::Result<super::DockPosition>;
    fn SetDockPosition(&self, dockposition: super::DockPosition) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDockProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IDockProvider";
}
impl IDockProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDockProvider_Impl, const OFFSET: isize>() -> IDockProvider_Vtbl {
        unsafe extern "system" fn DockPosition<Identity: ::windows::core::IUnknownImpl, Impl: IDockProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DockPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDockPosition<Identity: ::windows::core::IUnknownImpl, Impl: IDockProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dockposition: super::DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDockPosition(dockposition).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDockProvider, OFFSET>(),
            DockPosition: DockPosition::<Identity, Impl, OFFSET>,
            SetDockPosition: SetDockPosition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDockProvider as ::windows::core::Interface>::IID
    }
}
pub trait IDragProvider_Impl: Sized {
    fn IsGrabbed(&self) -> ::windows::core::Result<bool>;
    fn DropEffect(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DropEffects(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>>;
    fn GetGrabbedItems(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
}
impl ::windows::core::RuntimeName for IDragProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IDragProvider";
}
impl IDragProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragProvider_Impl, const OFFSET: isize>() -> IDragProvider_Vtbl {
        unsafe extern "system" fn IsGrabbed<Identity: ::windows::core::IUnknownImpl, Impl: IDragProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsGrabbed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDragProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DropEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEffects<Identity: ::windows::core::IUnknownImpl, Impl: IDragProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn GetGrabbedItems<Identity: ::windows::core::IUnknownImpl, Impl: IDragProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragProvider, OFFSET>(),
            IsGrabbed: IsGrabbed::<Identity, Impl, OFFSET>,
            DropEffect: DropEffect::<Identity, Impl, OFFSET>,
            DropEffects: DropEffects::<Identity, Impl, OFFSET>,
            GetGrabbedItems: GetGrabbedItems::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragProvider as ::windows::core::Interface>::IID
    }
}
pub trait IDropTargetProvider_Impl: Sized {
    fn DropEffect(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DropEffects(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>>;
}
impl ::windows::core::RuntimeName for IDropTargetProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IDropTargetProvider";
}
impl IDropTargetProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetProvider_Impl, const OFFSET: isize>() -> IDropTargetProvider_Vtbl {
        unsafe extern "system" fn DropEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DropEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEffects<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDropTargetProvider, OFFSET>(),
            DropEffect: DropEffect::<Identity, Impl, OFFSET>,
            DropEffects: DropEffects::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropTargetProvider as ::windows::core::Interface>::IID
    }
}
pub trait IExpandCollapseProvider_Impl: Sized {
    fn ExpandCollapseState(&self) -> ::windows::core::Result<super::ExpandCollapseState>;
    fn Collapse(&self) -> ::windows::core::Result<()>;
    fn Expand(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IExpandCollapseProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IExpandCollapseProvider";
}
impl IExpandCollapseProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapseProvider_Impl, const OFFSET: isize>() -> IExpandCollapseProvider_Vtbl {
        unsafe extern "system" fn ExpandCollapseState<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapseProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ExpandCollapseState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExpandCollapseState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collapse<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapseProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Collapse().into()
        }
        unsafe extern "system" fn Expand<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapseProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Expand().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IExpandCollapseProvider, OFFSET>(),
            ExpandCollapseState: ExpandCollapseState::<Identity, Impl, OFFSET>,
            Collapse: Collapse::<Identity, Impl, OFFSET>,
            Expand: Expand::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExpandCollapseProvider as ::windows::core::Interface>::IID
    }
}
pub trait IGridItemProvider_Impl: Sized {
    fn Column(&self) -> ::windows::core::Result<i32>;
    fn ColumnSpan(&self) -> ::windows::core::Result<i32>;
    fn ContainingGrid(&self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn Row(&self) -> ::windows::core::Result<i32>;
    fn RowSpan(&self) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for IGridItemProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IGridItemProvider";
}
impl IGridItemProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProvider_Impl, const OFFSET: isize>() -> IGridItemProvider_Vtbl {
        unsafe extern "system" fn Column<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Column() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColumnSpan<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ColumnSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainingGrid<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContainingGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Row<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Row() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowSpan<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RowSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGridItemProvider, OFFSET>(),
            Column: Column::<Identity, Impl, OFFSET>,
            ColumnSpan: ColumnSpan::<Identity, Impl, OFFSET>,
            ContainingGrid: ContainingGrid::<Identity, Impl, OFFSET>,
            Row: Row::<Identity, Impl, OFFSET>,
            RowSpan: RowSpan::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridItemProvider as ::windows::core::Interface>::IID
    }
}
pub trait IGridProvider_Impl: Sized {
    fn ColumnCount(&self) -> ::windows::core::Result<i32>;
    fn RowCount(&self) -> ::windows::core::Result<i32>;
    fn GetItem(&self, row: i32, column: i32) -> ::windows::core::Result<IRawElementProviderSimple>;
}
impl ::windows::core::RuntimeName for IGridProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IGridProvider";
}
impl IGridProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridProvider_Impl, const OFFSET: isize>() -> IGridProvider_Vtbl {
        unsafe extern "system" fn ColumnCount<Identity: ::windows::core::IUnknownImpl, Impl: IGridProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ColumnCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowCount<Identity: ::windows::core::IUnknownImpl, Impl: IGridProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RowCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Identity: ::windows::core::IUnknownImpl, Impl: IGridProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItem(row, column) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGridProvider, OFFSET>(),
            ColumnCount: ColumnCount::<Identity, Impl, OFFSET>,
            RowCount: RowCount::<Identity, Impl, OFFSET>,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridProvider as ::windows::core::Interface>::IID
    }
}
pub trait IInvokeProvider_Impl: Sized {
    fn Invoke(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInvokeProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IInvokeProvider";
}
impl IInvokeProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInvokeProvider_Impl, const OFFSET: isize>() -> IInvokeProvider_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl, Impl: IInvokeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Invoke().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInvokeProvider, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInvokeProvider as ::windows::core::Interface>::IID
    }
}
pub trait IItemContainerProvider_Impl: Sized {
    fn FindItemByProperty(&self, startafter: &::core::option::Option<IRawElementProviderSimple>, automationproperty: &::core::option::Option<super::AutomationProperty>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<IRawElementProviderSimple>;
}
impl ::windows::core::RuntimeName for IItemContainerProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IItemContainerProvider";
}
impl IItemContainerProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemContainerProvider_Impl, const OFFSET: isize>() -> IItemContainerProvider_Vtbl {
        unsafe extern "system" fn FindItemByProperty<Identity: ::windows::core::IUnknownImpl, Impl: IItemContainerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startafter: ::windows::core::RawPtr, automationproperty: ::windows::core::RawPtr, value: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindItemByProperty(::core::mem::transmute(&startafter), ::core::mem::transmute(&automationproperty), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IItemContainerProvider, OFFSET>(),
            FindItemByProperty: FindItemByProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemContainerProvider as ::windows::core::Interface>::IID
    }
}
pub trait IMultipleViewProvider_Impl: Sized {
    fn CurrentView(&self) -> ::windows::core::Result<i32>;
    fn GetSupportedViews(&self) -> ::windows::core::Result<::windows::core::Array<i32>>;
    fn GetViewName(&self, viewid: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCurrentView(&self, viewid: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMultipleViewProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IMultipleViewProvider";
}
impl IMultipleViewProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewProvider_Impl, const OFFSET: isize>() -> IMultipleViewProvider_Vtbl {
        unsafe extern "system" fn CurrentView<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedViews<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn GetViewName<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetViewName(viewid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentView<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCurrentView(viewid).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMultipleViewProvider, OFFSET>(),
            CurrentView: CurrentView::<Identity, Impl, OFFSET>,
            GetSupportedViews: GetSupportedViews::<Identity, Impl, OFFSET>,
            GetViewName: GetViewName::<Identity, Impl, OFFSET>,
            SetCurrentView: SetCurrentView::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultipleViewProvider as ::windows::core::Interface>::IID
    }
}
pub trait IObjectModelProvider_Impl: Sized {
    fn GetUnderlyingObjectModel(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IObjectModelProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IObjectModelProvider";
}
impl IObjectModelProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectModelProvider_Impl, const OFFSET: isize>() -> IObjectModelProvider_Vtbl {
        unsafe extern "system" fn GetUnderlyingObjectModel<Identity: ::windows::core::IUnknownImpl, Impl: IObjectModelProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUnderlyingObjectModel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IObjectModelProvider, OFFSET>(),
            GetUnderlyingObjectModel: GetUnderlyingObjectModel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectModelProvider as ::windows::core::Interface>::IID
    }
}
pub trait IRangeValueProvider_Impl: Sized {
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
impl IRangeValueProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const OFFSET: isize>() -> IRangeValueProvider_Vtbl {
        unsafe extern "system" fn IsReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LargeChange<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LargeChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Maximum<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Maximum() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Minimum<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Minimum() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmallChange<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SmallChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRangeValueProvider, OFFSET>(),
            IsReadOnly: IsReadOnly::<Identity, Impl, OFFSET>,
            LargeChange: LargeChange::<Identity, Impl, OFFSET>,
            Maximum: Maximum::<Identity, Impl, OFFSET>,
            Minimum: Minimum::<Identity, Impl, OFFSET>,
            SmallChange: SmallChange::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeValueProvider as ::windows::core::Interface>::IID
    }
}
pub trait IScrollItemProvider_Impl: Sized {
    fn ScrollIntoView(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IScrollItemProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IScrollItemProvider";
}
impl IScrollItemProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollItemProvider_Impl, const OFFSET: isize>() -> IScrollItemProvider_Vtbl {
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows::core::IUnknownImpl, Impl: IScrollItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScrollIntoView().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollItemProvider, OFFSET>(),
            ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollItemProvider as ::windows::core::Interface>::IID
    }
}
pub trait IScrollProvider_Impl: Sized {
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
impl IScrollProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>() -> IScrollProvider_Vtbl {
        unsafe extern "system" fn HorizontallyScrollable<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HorizontallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalScrollPercent<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HorizontalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalViewSize<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HorizontalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticallyScrollable<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VerticallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalScrollPercent<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VerticalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalViewSize<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VerticalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scroll<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalamount: super::ScrollAmount, verticalamount: super::ScrollAmount) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Scroll(horizontalamount, verticalamount).into()
        }
        unsafe extern "system" fn SetScrollPercent<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScrollPercent(horizontalpercent, verticalpercent).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollProvider, OFFSET>(),
            HorizontallyScrollable: HorizontallyScrollable::<Identity, Impl, OFFSET>,
            HorizontalScrollPercent: HorizontalScrollPercent::<Identity, Impl, OFFSET>,
            HorizontalViewSize: HorizontalViewSize::<Identity, Impl, OFFSET>,
            VerticallyScrollable: VerticallyScrollable::<Identity, Impl, OFFSET>,
            VerticalScrollPercent: VerticalScrollPercent::<Identity, Impl, OFFSET>,
            VerticalViewSize: VerticalViewSize::<Identity, Impl, OFFSET>,
            Scroll: Scroll::<Identity, Impl, OFFSET>,
            SetScrollPercent: SetScrollPercent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollProvider as ::windows::core::Interface>::IID
    }
}
pub trait ISelectionItemProvider_Impl: Sized {
    fn IsSelected(&self) -> ::windows::core::Result<bool>;
    fn SelectionContainer(&self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn AddToSelection(&self) -> ::windows::core::Result<()>;
    fn RemoveFromSelection(&self) -> ::windows::core::Result<()>;
    fn Select(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISelectionItemProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ISelectionItemProvider";
}
impl ISelectionItemProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>() -> ISelectionItemProvider_Vtbl {
        unsafe extern "system" fn IsSelected<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionContainer<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelectionContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddToSelection<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddToSelection().into()
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveFromSelection().into()
        }
        unsafe extern "system" fn Select<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Select().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectionItemProvider, OFFSET>(),
            IsSelected: IsSelected::<Identity, Impl, OFFSET>,
            SelectionContainer: SelectionContainer::<Identity, Impl, OFFSET>,
            AddToSelection: AddToSelection::<Identity, Impl, OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Identity, Impl, OFFSET>,
            Select: Select::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectionItemProvider as ::windows::core::Interface>::IID
    }
}
pub trait ISelectionProvider_Impl: Sized {
    fn CanSelectMultiple(&self) -> ::windows::core::Result<bool>;
    fn IsSelectionRequired(&self) -> ::windows::core::Result<bool>;
    fn GetSelection(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
}
impl ::windows::core::RuntimeName for ISelectionProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ISelectionProvider";
}
impl ISelectionProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider_Impl, const OFFSET: isize>() -> ISelectionProvider_Vtbl {
        unsafe extern "system" fn CanSelectMultiple<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanSelectMultiple() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSelectionRequired<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsSelectionRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectionProvider, OFFSET>(),
            CanSelectMultiple: CanSelectMultiple::<Identity, Impl, OFFSET>,
            IsSelectionRequired: IsSelectionRequired::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectionProvider as ::windows::core::Interface>::IID
    }
}
pub trait ISpreadsheetItemProvider_Impl: Sized {
    fn Formula(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAnnotationObjects(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
    fn GetAnnotationTypes(&self) -> ::windows::core::Result<::windows::core::Array<super::AnnotationType>>;
}
impl ::windows::core::RuntimeName for ISpreadsheetItemProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ISpreadsheetItemProvider";
}
impl ISpreadsheetItemProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemProvider_Impl, const OFFSET: isize>() -> ISpreadsheetItemProvider_Vtbl {
        unsafe extern "system" fn Formula<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Formula() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationObjects<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn GetAnnotationTypes<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::AnnotationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpreadsheetItemProvider, OFFSET>(),
            Formula: Formula::<Identity, Impl, OFFSET>,
            GetAnnotationObjects: GetAnnotationObjects::<Identity, Impl, OFFSET>,
            GetAnnotationTypes: GetAnnotationTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpreadsheetItemProvider as ::windows::core::Interface>::IID
    }
}
pub trait ISpreadsheetProvider_Impl: Sized {
    fn GetItemByName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IRawElementProviderSimple>;
}
impl ::windows::core::RuntimeName for ISpreadsheetProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ISpreadsheetProvider";
}
impl ISpreadsheetProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetProvider_Impl, const OFFSET: isize>() -> ISpreadsheetProvider_Vtbl {
        unsafe extern "system" fn GetItemByName<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItemByName(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpreadsheetProvider, OFFSET>(), GetItemByName: GetItemByName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpreadsheetProvider as ::windows::core::Interface>::IID
    }
}
pub trait IStylesProvider_Impl: Sized {
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
impl IStylesProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const OFFSET: isize>() -> IStylesProvider_Vtbl {
        unsafe extern "system" fn ExtendedProperties<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillColor<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FillColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillPatternColor<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FillPatternColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillPatternStyle<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FillPatternStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shape<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Shape() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleId<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StyleId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleName<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StyleName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStylesProvider, OFFSET>(),
            ExtendedProperties: ExtendedProperties::<Identity, Impl, OFFSET>,
            FillColor: FillColor::<Identity, Impl, OFFSET>,
            FillPatternColor: FillPatternColor::<Identity, Impl, OFFSET>,
            FillPatternStyle: FillPatternStyle::<Identity, Impl, OFFSET>,
            Shape: Shape::<Identity, Impl, OFFSET>,
            StyleId: StyleId::<Identity, Impl, OFFSET>,
            StyleName: StyleName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStylesProvider as ::windows::core::Interface>::IID
    }
}
pub trait ISynchronizedInputProvider_Impl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn StartListening(&self, inputtype: super::SynchronizedInputType) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISynchronizedInputProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ISynchronizedInputProvider";
}
impl ISynchronizedInputProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizedInputProvider_Impl, const OFFSET: isize>() -> ISynchronizedInputProvider_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizedInputProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn StartListening<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizedInputProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputtype: super::SynchronizedInputType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartListening(inputtype).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISynchronizedInputProvider, OFFSET>(),
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            StartListening: StartListening::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISynchronizedInputProvider as ::windows::core::Interface>::IID
    }
}
pub trait ITableItemProvider_Impl: Sized {
    fn GetColumnHeaderItems(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
    fn GetRowHeaderItems(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
}
impl ::windows::core::RuntimeName for ITableItemProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITableItemProvider";
}
impl ITableItemProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableItemProvider_Impl, const OFFSET: isize>() -> ITableItemProvider_Vtbl {
        unsafe extern "system" fn GetColumnHeaderItems<Identity: ::windows::core::IUnknownImpl, Impl: ITableItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn GetRowHeaderItems<Identity: ::windows::core::IUnknownImpl, Impl: ITableItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITableItemProvider, OFFSET>(),
            GetColumnHeaderItems: GetColumnHeaderItems::<Identity, Impl, OFFSET>,
            GetRowHeaderItems: GetRowHeaderItems::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableItemProvider as ::windows::core::Interface>::IID
    }
}
pub trait ITableProvider_Impl: Sized {
    fn RowOrColumnMajor(&self) -> ::windows::core::Result<super::RowOrColumnMajor>;
    fn GetColumnHeaders(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
    fn GetRowHeaders(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
}
impl ::windows::core::RuntimeName for ITableProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITableProvider";
}
impl ITableProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableProvider_Impl, const OFFSET: isize>() -> ITableProvider_Vtbl {
        unsafe extern "system" fn RowOrColumnMajor<Identity: ::windows::core::IUnknownImpl, Impl: ITableProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::RowOrColumnMajor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RowOrColumnMajor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnHeaders<Identity: ::windows::core::IUnknownImpl, Impl: ITableProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn GetRowHeaders<Identity: ::windows::core::IUnknownImpl, Impl: ITableProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITableProvider, OFFSET>(),
            RowOrColumnMajor: RowOrColumnMajor::<Identity, Impl, OFFSET>,
            GetColumnHeaders: GetColumnHeaders::<Identity, Impl, OFFSET>,
            GetRowHeaders: GetRowHeaders::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableProvider as ::windows::core::Interface>::IID
    }
}
pub trait ITextChildProvider_Impl: Sized {
    fn TextContainer(&self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn TextRange(&self) -> ::windows::core::Result<ITextRangeProvider>;
}
impl ::windows::core::RuntimeName for ITextChildProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITextChildProvider";
}
impl ITextChildProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextChildProvider_Impl, const OFFSET: isize>() -> ITextChildProvider_Vtbl {
        unsafe extern "system" fn TextContainer<Identity: ::windows::core::IUnknownImpl, Impl: ITextChildProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TextContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextRange<Identity: ::windows::core::IUnknownImpl, Impl: ITextChildProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TextRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextChildProvider, OFFSET>(),
            TextContainer: TextContainer::<Identity, Impl, OFFSET>,
            TextRange: TextRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextChildProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ITextEditProvider_Impl: Sized + ITextProvider_Impl {
    fn GetActiveComposition(&self) -> ::windows::core::Result<ITextRangeProvider>;
    fn GetConversionTarget(&self) -> ::windows::core::Result<ITextRangeProvider>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ITextEditProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITextEditProvider";
}
#[cfg(feature = "Foundation")]
impl ITextEditProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextEditProvider_Impl, const OFFSET: isize>() -> ITextEditProvider_Vtbl {
        unsafe extern "system" fn GetActiveComposition<Identity: ::windows::core::IUnknownImpl, Impl: ITextEditProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetActiveComposition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionTarget<Identity: ::windows::core::IUnknownImpl, Impl: ITextEditProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConversionTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextEditProvider, OFFSET>(),
            GetActiveComposition: GetActiveComposition::<Identity, Impl, OFFSET>,
            GetConversionTarget: GetConversionTarget::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextEditProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ITextProvider_Impl: Sized {
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
impl ITextProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider_Impl, const OFFSET: isize>() -> ITextProvider_Vtbl {
        unsafe extern "system" fn DocumentRange<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DocumentRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTextSelection<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::SupportedTextSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedTextSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn GetVisibleRanges<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn RangeFromChild<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childelement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RangeFromChild(::core::mem::transmute(&childelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromPoint<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, screenlocation: super::super::super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RangeFromPoint(::core::mem::transmute(&screenlocation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextProvider, OFFSET>(),
            DocumentRange: DocumentRange::<Identity, Impl, OFFSET>,
            SupportedTextSelection: SupportedTextSelection::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            GetVisibleRanges: GetVisibleRanges::<Identity, Impl, OFFSET>,
            RangeFromChild: RangeFromChild::<Identity, Impl, OFFSET>,
            RangeFromPoint: RangeFromPoint::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ITextProvider2_Impl: Sized + ITextProvider_Impl {
    fn RangeFromAnnotation(&self, annotationelement: &::core::option::Option<IRawElementProviderSimple>) -> ::windows::core::Result<ITextRangeProvider>;
    fn GetCaretRange(&self, isactive: &mut bool) -> ::windows::core::Result<ITextRangeProvider>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ITextProvider2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITextProvider2";
}
#[cfg(feature = "Foundation")]
impl ITextProvider2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider2_Impl, const OFFSET: isize>() -> ITextProvider2_Vtbl {
        unsafe extern "system" fn RangeFromAnnotation<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotationelement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RangeFromAnnotation(::core::mem::transmute(&annotationelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaretRange<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isactive: *mut bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCaretRange(::core::mem::transmute_copy(&isactive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextProvider2, OFFSET>(),
            RangeFromAnnotation: RangeFromAnnotation::<Identity, Impl, OFFSET>,
            GetCaretRange: GetCaretRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextProvider2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Automation_Text")]
pub trait ITextRangeProvider_Impl: Sized {
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
impl ITextRangeProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>() -> ITextRangeProvider_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textrangeprovider: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Compare(::core::mem::transmute(&textrangeprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEndpoints<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: super::Text::TextPatternRangeEndpoint, textrangeprovider: ::windows::core::RawPtr, targetendpoint: super::Text::TextPatternRangeEndpoint, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CompareEndpoints(endpoint, ::core::mem::transmute(&textrangeprovider), targetendpoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandToEnclosingUnit<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: super::Text::TextUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExpandToEnclosingUnit(unit).into()
        }
        unsafe extern "system" fn FindAttribute<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeid: i32, value: *mut ::core::ffi::c_void, backward: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindAttribute(attributeid, ::core::mem::transmute(&value), backward) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, backward: bool, ignorecase: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindText(::core::mem::transmute(&text), backward, ignorecase) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValue<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAttributeValue(attributeid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRectangles<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, returnValue_array_size: *mut u32, returnvalue: *mut *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBoundingRectangles(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&returnvalue), returnValue_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetEnclosingElement<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnclosingElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetText(maxlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: super::Text::TextUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Move(unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByUnit<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: super::Text::TextPatternRangeEndpoint, unit: super::Text::TextUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveEndpointByUnit(endpoint, unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByRange<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: super::Text::TextPatternRangeEndpoint, textrangeprovider: ::windows::core::RawPtr, targetendpoint: super::Text::TextPatternRangeEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MoveEndpointByRange(endpoint, ::core::mem::transmute(&textrangeprovider), targetendpoint).into()
        }
        unsafe extern "system" fn Select<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Select().into()
        }
        unsafe extern "system" fn AddToSelection<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddToSelection().into()
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveFromSelection().into()
        }
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aligntotop: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScrollIntoView(aligntotop).into()
        }
        unsafe extern "system" fn GetChildren<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextRangeProvider, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Compare: Compare::<Identity, Impl, OFFSET>,
            CompareEndpoints: CompareEndpoints::<Identity, Impl, OFFSET>,
            ExpandToEnclosingUnit: ExpandToEnclosingUnit::<Identity, Impl, OFFSET>,
            FindAttribute: FindAttribute::<Identity, Impl, OFFSET>,
            FindText: FindText::<Identity, Impl, OFFSET>,
            GetAttributeValue: GetAttributeValue::<Identity, Impl, OFFSET>,
            GetBoundingRectangles: GetBoundingRectangles::<Identity, Impl, OFFSET>,
            GetEnclosingElement: GetEnclosingElement::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            MoveEndpointByUnit: MoveEndpointByUnit::<Identity, Impl, OFFSET>,
            MoveEndpointByRange: MoveEndpointByRange::<Identity, Impl, OFFSET>,
            Select: Select::<Identity, Impl, OFFSET>,
            AddToSelection: AddToSelection::<Identity, Impl, OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Identity, Impl, OFFSET>,
            ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET>,
            GetChildren: GetChildren::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRangeProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Automation_Text")]
pub trait ITextRangeProvider2_Impl: Sized + ITextRangeProvider_Impl {
    fn ShowContextMenu(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "UI_Xaml_Automation_Text")]
impl ::windows::core::RuntimeName for ITextRangeProvider2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITextRangeProvider2";
}
#[cfg(feature = "UI_Xaml_Automation_Text")]
impl ITextRangeProvider2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider2_Impl, const OFFSET: isize>() -> ITextRangeProvider2_Vtbl {
        unsafe extern "system" fn ShowContextMenu<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowContextMenu().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextRangeProvider2, OFFSET>(),
            ShowContextMenu: ShowContextMenu::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRangeProvider2 as ::windows::core::Interface>::IID
    }
}
pub trait IToggleProvider_Impl: Sized {
    fn ToggleState(&self) -> ::windows::core::Result<super::ToggleState>;
    fn Toggle(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IToggleProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IToggleProvider";
}
impl IToggleProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleProvider_Impl, const OFFSET: isize>() -> IToggleProvider_Vtbl {
        unsafe extern "system" fn ToggleState<Identity: ::windows::core::IUnknownImpl, Impl: IToggleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ToggleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ToggleState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Toggle<Identity: ::windows::core::IUnknownImpl, Impl: IToggleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Toggle().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleProvider, OFFSET>(),
            ToggleState: ToggleState::<Identity, Impl, OFFSET>,
            Toggle: Toggle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleProvider as ::windows::core::Interface>::IID
    }
}
pub trait ITransformProvider_Impl: Sized {
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
impl ITransformProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider_Impl, const OFFSET: isize>() -> ITransformProvider_Vtbl {
        unsafe extern "system" fn CanMove<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanMove() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanResize<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanResize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRotate<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanRotate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Move(x, y).into()
        }
        unsafe extern "system" fn Resize<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resize(width, height).into()
        }
        unsafe extern "system" fn Rotate<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Rotate(degrees).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITransformProvider, OFFSET>(),
            CanMove: CanMove::<Identity, Impl, OFFSET>,
            CanResize: CanResize::<Identity, Impl, OFFSET>,
            CanRotate: CanRotate::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
            Rotate: Rotate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransformProvider as ::windows::core::Interface>::IID
    }
}
pub trait ITransformProvider2_Impl: Sized + ITransformProvider_Impl {
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
impl ITransformProvider2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2_Impl, const OFFSET: isize>() -> ITransformProvider2_Vtbl {
        unsafe extern "system" fn CanZoom<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanZoom() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomLevel<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxZoom<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxZoom() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinZoom<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinZoom() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Zoom<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoom: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Zoom(zoom).into()
        }
        unsafe extern "system" fn ZoomByUnit<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomunit: super::ZoomUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ZoomByUnit(zoomunit).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITransformProvider2, OFFSET>(),
            CanZoom: CanZoom::<Identity, Impl, OFFSET>,
            ZoomLevel: ZoomLevel::<Identity, Impl, OFFSET>,
            MaxZoom: MaxZoom::<Identity, Impl, OFFSET>,
            MinZoom: MinZoom::<Identity, Impl, OFFSET>,
            Zoom: Zoom::<Identity, Impl, OFFSET>,
            ZoomByUnit: ZoomByUnit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransformProvider2 as ::windows::core::Interface>::IID
    }
}
pub trait IValueProvider_Impl: Sized {
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IValueProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IValueProvider";
}
impl IValueProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValueProvider_Impl, const OFFSET: isize>() -> IValueProvider_Vtbl {
        unsafe extern "system" fn IsReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: IValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl, Impl: IValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: IValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute(&value)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IValueProvider, OFFSET>(),
            IsReadOnly: IsReadOnly::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IValueProvider as ::windows::core::Interface>::IID
    }
}
pub trait IVirtualizedItemProvider_Impl: Sized {
    fn Realize(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVirtualizedItemProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IVirtualizedItemProvider";
}
impl IVirtualizedItemProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizedItemProvider_Impl, const OFFSET: isize>() -> IVirtualizedItemProvider_Vtbl {
        unsafe extern "system" fn Realize<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizedItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Realize().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVirtualizedItemProvider, OFFSET>(), Realize: Realize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualizedItemProvider as ::windows::core::Interface>::IID
    }
}
pub trait IWindowProvider_Impl: Sized {
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
impl IWindowProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>() -> IWindowProvider_Vtbl {
        unsafe extern "system" fn IsModal<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsModal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTopmost<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsTopmost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Maximizable<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Maximizable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Minimizable<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Minimizable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InteractionState<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::WindowInteractionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InteractionState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisualState<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VisualState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn SetVisualState<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: super::WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVisualState(state).into()
        }
        unsafe extern "system" fn WaitForInputIdle<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: i32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WaitForInputIdle(milliseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowProvider, OFFSET>(),
            IsModal: IsModal::<Identity, Impl, OFFSET>,
            IsTopmost: IsTopmost::<Identity, Impl, OFFSET>,
            Maximizable: Maximizable::<Identity, Impl, OFFSET>,
            Minimizable: Minimizable::<Identity, Impl, OFFSET>,
            InteractionState: InteractionState::<Identity, Impl, OFFSET>,
            VisualState: VisualState::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            SetVisualState: SetVisualState::<Identity, Impl, OFFSET>,
            WaitForInputIdle: WaitForInputIdle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowProvider as ::windows::core::Interface>::IID
    }
}
