#[cfg(feature = "Win32_System_Com")]
pub trait AppEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for AppEvents {
    const NAME: &'static str = "Windows.Win32.System.Mmc.AppEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl AppEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AppEventsImpl, const OFFSET: isize>() -> AppEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<AppEvents>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ColumnImpl: Sized + IDispatchImpl {
    fn Name();
    fn Width();
    fn SetWidth();
    fn DisplayPosition();
    fn SetDisplayPosition();
    fn Hidden();
    fn SetHidden();
    fn SetAsSortColumn();
    fn IsSortColumn();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for Column {
    const NAME: &'static str = "Windows.Win32.System.Mmc.Column";
}
#[cfg(feature = "Win32_System_Com")]
impl ColumnVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ColumnImpl, const OFFSET: isize>() -> ColumnVtbl {
        unsafe extern "system" fn Name<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width(::core::mem::transmute_copy(&width)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWidth(width) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayPosition<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayPosition(::core::mem::transmute_copy(&displayposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayPosition<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDisplayPosition(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hidden<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hidden: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hidden(::core::mem::transmute_copy(&hidden)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHidden<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hidden: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHidden(&*(&hidden as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAsSortColumn<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sortorder: _ColumnSortOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAsSortColumn(sortorder) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSortColumn<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issortcolumn: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSortColumn(::core::mem::transmute_copy(&issortcolumn)) {
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
            ::windows::core::GetRuntimeClassName::<Column>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            Width::<Impl, OFFSET>,
            SetWidth::<Impl, OFFSET>,
            DisplayPosition::<Impl, OFFSET>,
            SetDisplayPosition::<Impl, OFFSET>,
            Hidden::<Impl, OFFSET>,
            SetHidden::<Impl, OFFSET>,
            SetAsSortColumn::<Impl, OFFSET>,
            IsSortColumn::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ColumnsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for Columns {
    const NAME: &'static str = "Windows.Win32.System.Mmc.Columns";
}
#[cfg(feature = "Win32_System_Com")]
impl ColumnsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ColumnsImpl, const OFFSET: isize>() -> ColumnsVtbl {
        unsafe extern "system" fn Item<Impl: ColumnsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, column: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&column)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ColumnsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ColumnsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<Columns>, ::windows::core::GetTrustLevel, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ContextMenuImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ContextMenu {
    const NAME: &'static str = "Windows.Win32.System.Mmc.ContextMenu";
}
#[cfg(feature = "Win32_System_Com")]
impl ContextMenuVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ContextMenuImpl, const OFFSET: isize>() -> ContextMenuVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ContextMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ContextMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexorpath: ::core::mem::ManuallyDrop<super::Com::VARIANT>, menuitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&indexorpath as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&menuitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ContextMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ContextMenu>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait DocumentImpl: Sized + IDispatchImpl {
    fn Save();
    fn SaveAs();
    fn Close();
    fn Views();
    fn SnapIns();
    fn ActiveView();
    fn Name();
    fn SetName();
    fn Location();
    fn IsSaved();
    fn Mode();
    fn SetMode();
    fn RootNode();
    fn ScopeNamespace();
    fn CreateProperties();
    fn Application();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for Document {
    const NAME: &'static str = "Windows.Win32.System.Mmc.Document";
}
#[cfg(feature = "Win32_System_Com")]
impl DocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DocumentImpl, const OFFSET: isize>() -> DocumentVtbl {
        unsafe extern "system" fn Save<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAs<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAs(&*(&filename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, savechanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close(&*(&savechanges as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Views<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, views: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Views(::core::mem::transmute_copy(&views)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnapIns<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapins: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SnapIns(::core::mem::transmute_copy(&snapins)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveView<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveView(::core::mem::transmute_copy(&view)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location(::core::mem::transmute_copy(&location)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSaved<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issaved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSaved(::core::mem::transmute_copy(&issaved)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut _DocumentMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode(::core::mem::transmute_copy(&mode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: _DocumentMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootNode<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RootNode(::core::mem::transmute_copy(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScopeNamespace<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScopeNamespace(::core::mem::transmute_copy(&scopenamespace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProperties<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProperties(::core::mem::transmute_copy(&properties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Application<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Application(::core::mem::transmute_copy(&application)) {
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
            ::windows::core::GetRuntimeClassName::<Document>,
            ::windows::core::GetTrustLevel,
            Save::<Impl, OFFSET>,
            SaveAs::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            Views::<Impl, OFFSET>,
            SnapIns::<Impl, OFFSET>,
            ActiveView::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            Location::<Impl, OFFSET>,
            IsSaved::<Impl, OFFSET>,
            Mode::<Impl, OFFSET>,
            SetMode::<Impl, OFFSET>,
            RootNode::<Impl, OFFSET>,
            ScopeNamespace::<Impl, OFFSET>,
            CreateProperties::<Impl, OFFSET>,
            Application::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ExtensionImpl: Sized + IDispatchImpl {
    fn Name();
    fn Vendor();
    fn Version();
    fn Extensions();
    fn SnapinCLSID();
    fn EnableAllExtensions();
    fn Enable();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for Extension {
    const NAME: &'static str = "Windows.Win32.System.Mmc.Extension";
}
#[cfg(feature = "Win32_System_Com")]
impl ExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ExtensionImpl, const OFFSET: isize>() -> ExtensionVtbl {
        unsafe extern "system" fn Name<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Vendor<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendor: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Vendor(::core::mem::transmute_copy(&vendor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version(::core::mem::transmute_copy(&version)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extensions<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extensions(::core::mem::transmute_copy(&extensions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnapinCLSID<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapinclsid: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SnapinCLSID(::core::mem::transmute_copy(&snapinclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAllExtensions<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableAllExtensions(&*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enable(&*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<Extension>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, Vendor::<Impl, OFFSET>, Version::<Impl, OFFSET>, Extensions::<Impl, OFFSET>, SnapinCLSID::<Impl, OFFSET>, EnableAllExtensions::<Impl, OFFSET>, Enable::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ExtensionsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for Extensions {
    const NAME: &'static str = "Windows.Win32.System.Mmc.Extensions";
}
#[cfg(feature = "Win32_System_Com")]
impl ExtensionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ExtensionsImpl, const OFFSET: isize>() -> ExtensionsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, extension: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&extension)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<Extensions>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait FrameImpl: Sized + IDispatchImpl {
    fn Maximize();
    fn Minimize();
    fn Restore();
    fn Top();
    fn SetTop();
    fn Bottom();
    fn SetBottom();
    fn Left();
    fn SetLeft();
    fn Right();
    fn SetRight();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for Frame {
    const NAME: &'static str = "Windows.Win32.System.Mmc.Frame";
}
#[cfg(feature = "Win32_System_Com")]
impl FrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: FrameImpl, const OFFSET: isize>() -> FrameVtbl {
        unsafe extern "system" fn Maximize<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Maximize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Minimize<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Minimize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Restore<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Restore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Top<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Top(::core::mem::transmute_copy(&top)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTop<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTop(top) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bottom<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bottom(::core::mem::transmute_copy(&bottom)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottom<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBottom(bottom) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Left<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Left(::core::mem::transmute_copy(&left)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeft<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLeft(left) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Right<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Right(::core::mem::transmute_copy(&right)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRight<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRight(right) {
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
            ::windows::core::GetRuntimeClassName::<Frame>,
            ::windows::core::GetTrustLevel,
            Maximize::<Impl, OFFSET>,
            Minimize::<Impl, OFFSET>,
            Restore::<Impl, OFFSET>,
            Top::<Impl, OFFSET>,
            SetTop::<Impl, OFFSET>,
            Bottom::<Impl, OFFSET>,
            SetBottom::<Impl, OFFSET>,
            Left::<Impl, OFFSET>,
            SetLeft::<Impl, OFFSET>,
            Right::<Impl, OFFSET>,
            SetRight::<Impl, OFFSET>,
        )
    }
}
pub trait IColumnDataImpl: Sized {
    fn SetColumnConfigData();
    fn GetColumnConfigData();
    fn SetColumnSortData();
    fn GetColumnSortData();
}
impl ::windows::core::RuntimeName for IColumnData {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IColumnData";
}
impl IColumnDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColumnDataImpl, const OFFSET: isize>() -> IColumnDataVtbl {
        unsafe extern "system" fn SetColumnConfigData<Impl: IColumnDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetColumnConfigData(&*(&pcolid as *const <SColumnSetID as ::windows::core::Abi>::Abi as *const <SColumnSetID as ::windows::core::DefaultType>::DefaultType), &*(&pcolsetdata as *const <MMC_COLUMN_SET_DATA as ::windows::core::Abi>::Abi as *const <MMC_COLUMN_SET_DATA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnConfigData<Impl: IColumnDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, ppcolsetdata: *mut *mut MMC_COLUMN_SET_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnConfigData(&*(&pcolid as *const <SColumnSetID as ::windows::core::Abi>::Abi as *const <SColumnSetID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcolsetdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColumnSortData<Impl: IColumnDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetColumnSortData(&*(&pcolid as *const <SColumnSetID as ::windows::core::Abi>::Abi as *const <SColumnSetID as ::windows::core::DefaultType>::DefaultType), &*(&pcolsortdata as *const <MMC_SORT_SET_DATA as ::windows::core::Abi>::Abi as *const <MMC_SORT_SET_DATA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnSortData<Impl: IColumnDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, ppcolsortdata: *mut *mut MMC_SORT_SET_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnSortData(&*(&pcolid as *const <SColumnSetID as ::windows::core::Abi>::Abi as *const <SColumnSetID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcolsortdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IColumnData>, ::windows::core::GetTrustLevel, SetColumnConfigData::<Impl, OFFSET>, GetColumnConfigData::<Impl, OFFSET>, SetColumnSortData::<Impl, OFFSET>, GetColumnSortData::<Impl, OFFSET>)
    }
}
pub trait IComponentImpl: Sized {
    fn Initialize();
    fn Notify();
    fn Destroy();
    fn QueryDataObject();
    fn GetResultViewType();
    fn GetDisplayInfo();
    fn CompareObjects();
}
impl ::windows::core::RuntimeName for IComponent {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IComponent";
}
impl IComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentImpl, const OFFSET: isize>() -> IComponentVtbl {
        unsafe extern "system" fn Initialize<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpconsole: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&lpconsole as *const <IConsole as ::windows::core::Abi>::Abi as *const <IConsole as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notify<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notify(
                &*(&lpdataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType),
                event,
                &*(&arg as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&param3 as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destroy<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destroy(cookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDataObject<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDataObject(cookie, r#type, ::core::mem::transmute_copy(&ppdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResultViewType<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, ppviewtype: *mut super::super::Foundation::PWSTR, pviewoptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResultViewType(cookie, ::core::mem::transmute_copy(&ppviewtype), ::core::mem::transmute_copy(&pviewoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayInfo<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presultdataitem: *mut RESULTDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayInfo(&*(&presultdataitem as *const <RESULTDATAITEM as ::windows::core::Abi>::Abi as *const <RESULTDATAITEM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareObjects<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobjecta: ::windows::core::RawPtr, lpdataobjectb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareObjects(&*(&lpdataobjecta as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), &*(&lpdataobjectb as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComponent>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, Notify::<Impl, OFFSET>, Destroy::<Impl, OFFSET>, QueryDataObject::<Impl, OFFSET>, GetResultViewType::<Impl, OFFSET>, GetDisplayInfo::<Impl, OFFSET>, CompareObjects::<Impl, OFFSET>)
    }
}
pub trait IComponent2Impl: Sized + IComponentImpl {
    fn QueryDispatch();
    fn GetResultViewType2();
    fn RestoreResultView();
}
impl ::windows::core::RuntimeName for IComponent2 {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IComponent2";
}
impl IComponent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponent2Impl, const OFFSET: isize>() -> IComponent2Vtbl {
        unsafe extern "system" fn QueryDispatch<Impl: IComponent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDispatch(cookie, r#type, ::core::mem::transmute_copy(&ppdispatch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResultViewType2<Impl: IComponent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResultViewType2(cookie, &*(&presultviewtype as *const <RESULT_VIEW_TYPE_INFO as ::windows::core::Abi>::Abi as *const <RESULT_VIEW_TYPE_INFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreResultView<Impl: IComponent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RestoreResultView(cookie, &*(&presultviewtype as *const <RESULT_VIEW_TYPE_INFO as ::windows::core::Abi>::Abi as *const <RESULT_VIEW_TYPE_INFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComponent2>, ::windows::core::GetTrustLevel, QueryDispatch::<Impl, OFFSET>, GetResultViewType2::<Impl, OFFSET>, RestoreResultView::<Impl, OFFSET>)
    }
}
pub trait IComponentDataImpl: Sized {
    fn Initialize();
    fn CreateComponent();
    fn Notify();
    fn Destroy();
    fn QueryDataObject();
    fn GetDisplayInfo();
    fn CompareObjects();
}
impl ::windows::core::RuntimeName for IComponentData {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IComponentData";
}
impl IComponentDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentDataImpl, const OFFSET: isize>() -> IComponentDataVtbl {
        unsafe extern "system" fn Initialize<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&punknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComponent<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateComponent(::core::mem::transmute_copy(&ppcomponent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notify<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notify(
                &*(&lpdataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType),
                event,
                &*(&arg as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&param3 as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destroy<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destroy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDataObject<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDataObject(cookie, r#type, ::core::mem::transmute_copy(&ppdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayInfo<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscopedataitem: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayInfo(&*(&pscopedataitem as *const <SCOPEDATAITEM as ::windows::core::Abi>::Abi as *const <SCOPEDATAITEM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareObjects<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobjecta: ::windows::core::RawPtr, lpdataobjectb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareObjects(&*(&lpdataobjecta as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), &*(&lpdataobjectb as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComponentData>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, CreateComponent::<Impl, OFFSET>, Notify::<Impl, OFFSET>, Destroy::<Impl, OFFSET>, QueryDataObject::<Impl, OFFSET>, GetDisplayInfo::<Impl, OFFSET>, CompareObjects::<Impl, OFFSET>)
    }
}
pub trait IComponentData2Impl: Sized + IComponentDataImpl {
    fn QueryDispatch();
}
impl ::windows::core::RuntimeName for IComponentData2 {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IComponentData2";
}
impl IComponentData2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentData2Impl, const OFFSET: isize>() -> IComponentData2Vtbl {
        unsafe extern "system" fn QueryDispatch<Impl: IComponentData2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDispatch(cookie, r#type, ::core::mem::transmute_copy(&ppdispatch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComponentData2>, ::windows::core::GetTrustLevel, QueryDispatch::<Impl, OFFSET>)
    }
}
pub trait IConsoleImpl: Sized {
    fn SetHeader();
    fn SetToolbar();
    fn QueryResultView();
    fn QueryScopeImageList();
    fn QueryResultImageList();
    fn UpdateAllViews();
    fn MessageBox();
    fn QueryConsoleVerb();
    fn SelectScopeItem();
    fn GetMainWindow();
    fn NewWindow();
}
impl ::windows::core::RuntimeName for IConsole {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IConsole";
}
impl IConsoleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsoleImpl, const OFFSET: isize>() -> IConsoleVtbl {
        unsafe extern "system" fn SetHeader<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHeader(&*(&pheader as *const <IHeaderCtrl as ::windows::core::Abi>::Abi as *const <IHeaderCtrl as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToolbar<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoolbar: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetToolbar(&*(&ptoolbar as *const <IToolbar as ::windows::core::Abi>::Abi as *const <IToolbar as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryResultView<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryResultView(::core::mem::transmute_copy(&punknown)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryScopeImageList<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimagelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryScopeImageList(::core::mem::transmute_copy(&ppimagelist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryResultImageList<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimagelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryResultImageList(::core::mem::transmute_copy(&ppimagelist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAllViews<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr, data: super::super::Foundation::LPARAM, hint: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateAllViews(&*(&lpdataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), hint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageBox<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpsztext: super::super::Foundation::PWSTR, lpsztitle: super::super::Foundation::PWSTR, fustyle: u32, piretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageBox(&*(&lpsztext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&lpsztitle as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), fustyle, ::core::mem::transmute_copy(&piretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryConsoleVerb<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconsoleverb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryConsoleVerb(::core::mem::transmute_copy(&ppconsoleverb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectScopeItem<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hscopeitem: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectScopeItem(hscopeitem) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMainWindow<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMainWindow(::core::mem::transmute_copy(&phwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewWindow<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hscopeitem: isize, loptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewWindow(hscopeitem, loptions) {
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
            ::windows::core::GetRuntimeClassName::<IConsole>,
            ::windows::core::GetTrustLevel,
            SetHeader::<Impl, OFFSET>,
            SetToolbar::<Impl, OFFSET>,
            QueryResultView::<Impl, OFFSET>,
            QueryScopeImageList::<Impl, OFFSET>,
            QueryResultImageList::<Impl, OFFSET>,
            UpdateAllViews::<Impl, OFFSET>,
            MessageBox::<Impl, OFFSET>,
            QueryConsoleVerb::<Impl, OFFSET>,
            SelectScopeItem::<Impl, OFFSET>,
            GetMainWindow::<Impl, OFFSET>,
            NewWindow::<Impl, OFFSET>,
        )
    }
}
pub trait IConsole2Impl: Sized + IConsoleImpl {
    fn Expand();
    fn IsTaskpadViewPreferred();
    fn SetStatusText();
}
impl ::windows::core::RuntimeName for IConsole2 {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IConsole2";
}
impl IConsole2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsole2Impl, const OFFSET: isize>() -> IConsole2Vtbl {
        unsafe extern "system" fn Expand<Impl: IConsole2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize, bexpand: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Expand(hitem, &*(&bexpand as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTaskpadViewPreferred<Impl: IConsole2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTaskpadViewPreferred() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatusText<Impl: IConsole2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStatusText(&*(&pszstatustext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConsole2>, ::windows::core::GetTrustLevel, Expand::<Impl, OFFSET>, IsTaskpadViewPreferred::<Impl, OFFSET>, SetStatusText::<Impl, OFFSET>)
    }
}
pub trait IConsole3Impl: Sized + IConsole2Impl + IConsoleImpl {
    fn RenameScopeItem();
}
impl ::windows::core::RuntimeName for IConsole3 {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IConsole3";
}
impl IConsole3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsole3Impl, const OFFSET: isize>() -> IConsole3Vtbl {
        unsafe extern "system" fn RenameScopeItem<Impl: IConsole3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hscopeitem: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenameScopeItem(hscopeitem) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConsole3>, ::windows::core::GetTrustLevel, RenameScopeItem::<Impl, OFFSET>)
    }
}
pub trait IConsoleNameSpaceImpl: Sized {
    fn InsertItem();
    fn DeleteItem();
    fn SetItem();
    fn GetItem();
    fn GetChildItem();
    fn GetNextItem();
    fn GetParentItem();
}
impl ::windows::core::RuntimeName for IConsoleNameSpace {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IConsoleNameSpace";
}
impl IConsoleNameSpaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsoleNameSpaceImpl, const OFFSET: isize>() -> IConsoleNameSpaceVtbl {
        unsafe extern "system" fn InsertItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertItem(&*(&item as *const <SCOPEDATAITEM as ::windows::core::Abi>::Abi as *const <SCOPEDATAITEM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize, fdeletethis: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteItem(hitem, fdeletethis) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *const SCOPEDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetItem(&*(&item as *const <SCOPEDATAITEM as ::windows::core::Abi>::Abi as *const <SCOPEDATAITEM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItem(&*(&item as *const <SCOPEDATAITEM as ::windows::core::Abi>::Abi as *const <SCOPEDATAITEM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildItem(item, ::core::mem::transmute_copy(&pitemchild), ::core::mem::transmute_copy(&pcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextItem(item, ::core::mem::transmute_copy(&pitemnext), ::core::mem::transmute_copy(&pcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentItem(item, ::core::mem::transmute_copy(&pitemparent), ::core::mem::transmute_copy(&pcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConsoleNameSpace>, ::windows::core::GetTrustLevel, InsertItem::<Impl, OFFSET>, DeleteItem::<Impl, OFFSET>, SetItem::<Impl, OFFSET>, GetItem::<Impl, OFFSET>, GetChildItem::<Impl, OFFSET>, GetNextItem::<Impl, OFFSET>, GetParentItem::<Impl, OFFSET>)
    }
}
pub trait IConsoleNameSpace2Impl: Sized + IConsoleNameSpaceImpl {
    fn Expand();
    fn AddExtension();
}
impl ::windows::core::RuntimeName for IConsoleNameSpace2 {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IConsoleNameSpace2";
}
impl IConsoleNameSpace2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsoleNameSpace2Impl, const OFFSET: isize>() -> IConsoleNameSpace2Vtbl {
        unsafe extern "system" fn Expand<Impl: IConsoleNameSpace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Expand(hitem) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddExtension<Impl: IConsoleNameSpace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize, lpclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddExtension(hitem, &*(&lpclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConsoleNameSpace2>, ::windows::core::GetTrustLevel, Expand::<Impl, OFFSET>, AddExtension::<Impl, OFFSET>)
    }
}
pub trait IConsolePowerImpl: Sized {
    fn SetExecutionState();
    fn ResetIdleTimer();
}
impl ::windows::core::RuntimeName for IConsolePower {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IConsolePower";
}
impl IConsolePowerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsolePowerImpl, const OFFSET: isize>() -> IConsolePowerVtbl {
        unsafe extern "system" fn SetExecutionState<Impl: IConsolePowerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwadd: u32, dwremove: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExecutionState(dwadd, dwremove) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetIdleTimer<Impl: IConsolePowerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetIdleTimer(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConsolePower>, ::windows::core::GetTrustLevel, SetExecutionState::<Impl, OFFSET>, ResetIdleTimer::<Impl, OFFSET>)
    }
}
pub trait IConsolePowerSinkImpl: Sized {
    fn OnPowerBroadcast();
}
impl ::windows::core::RuntimeName for IConsolePowerSink {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IConsolePowerSink";
}
impl IConsolePowerSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsolePowerSinkImpl, const OFFSET: isize>() -> IConsolePowerSinkVtbl {
        unsafe extern "system" fn OnPowerBroadcast<Impl: IConsolePowerSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nevent: u32, lparam: super::super::Foundation::LPARAM, plreturn: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnPowerBroadcast(nevent, &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plreturn)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConsolePowerSink>, ::windows::core::GetTrustLevel, OnPowerBroadcast::<Impl, OFFSET>)
    }
}
pub trait IConsoleVerbImpl: Sized {
    fn GetVerbState();
    fn SetVerbState();
    fn SetDefaultVerb();
    fn GetDefaultVerb();
}
impl ::windows::core::RuntimeName for IConsoleVerb {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IConsoleVerb";
}
impl IConsoleVerbVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsoleVerbImpl, const OFFSET: isize>() -> IConsoleVerbVtbl {
        unsafe extern "system" fn GetVerbState<Impl: IConsoleVerbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVerbState(ecmdid, nstate, ::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerbState<Impl: IConsoleVerbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVerbState(ecmdid, nstate, &*(&bstate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultVerb<Impl: IConsoleVerbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultVerb(ecmdid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultVerb<Impl: IConsoleVerbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pecmdid: *mut MMC_CONSOLE_VERB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultVerb(::core::mem::transmute_copy(&pecmdid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConsoleVerb>, ::windows::core::GetTrustLevel, GetVerbState::<Impl, OFFSET>, SetVerbState::<Impl, OFFSET>, SetDefaultVerb::<Impl, OFFSET>, GetDefaultVerb::<Impl, OFFSET>)
    }
}
pub trait IContextMenuCallbackImpl: Sized {
    fn AddItem();
}
impl ::windows::core::RuntimeName for IContextMenuCallback {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IContextMenuCallback";
}
impl IContextMenuCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextMenuCallbackImpl, const OFFSET: isize>() -> IContextMenuCallbackVtbl {
        unsafe extern "system" fn AddItem<Impl: IContextMenuCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *const CONTEXTMENUITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddItem(&*(&pitem as *const <CONTEXTMENUITEM as ::windows::core::Abi>::Abi as *const <CONTEXTMENUITEM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContextMenuCallback>, ::windows::core::GetTrustLevel, AddItem::<Impl, OFFSET>)
    }
}
pub trait IContextMenuCallback2Impl: Sized {
    fn AddItem();
}
impl ::windows::core::RuntimeName for IContextMenuCallback2 {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IContextMenuCallback2";
}
impl IContextMenuCallback2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextMenuCallback2Impl, const OFFSET: isize>() -> IContextMenuCallback2Vtbl {
        unsafe extern "system" fn AddItem<Impl: IContextMenuCallback2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *const CONTEXTMENUITEM2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddItem(&*(&pitem as *const <CONTEXTMENUITEM2 as ::windows::core::Abi>::Abi as *const <CONTEXTMENUITEM2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContextMenuCallback2>, ::windows::core::GetTrustLevel, AddItem::<Impl, OFFSET>)
    }
}
pub trait IContextMenuProviderImpl: Sized + IContextMenuCallbackImpl {
    fn EmptyMenuList();
    fn AddPrimaryExtensionItems();
    fn AddThirdPartyExtensionItems();
    fn ShowContextMenu();
}
impl ::windows::core::RuntimeName for IContextMenuProvider {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IContextMenuProvider";
}
impl IContextMenuProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextMenuProviderImpl, const OFFSET: isize>() -> IContextMenuProviderVtbl {
        unsafe extern "system" fn EmptyMenuList<Impl: IContextMenuProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmptyMenuList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPrimaryExtensionItems<Impl: IContextMenuProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piextension: *mut ::core::ffi::c_void, pidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPrimaryExtensionItems(&*(&piextension as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&pidataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddThirdPartyExtensionItems<Impl: IContextMenuProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddThirdPartyExtensionItems(&*(&pidataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowContextMenu<Impl: IContextMenuProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, xpos: i32, ypos: i32, plselected: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowContextMenu(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), xpos, ypos, ::core::mem::transmute_copy(&plselected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContextMenuProvider>, ::windows::core::GetTrustLevel, EmptyMenuList::<Impl, OFFSET>, AddPrimaryExtensionItems::<Impl, OFFSET>, AddThirdPartyExtensionItems::<Impl, OFFSET>, ShowContextMenu::<Impl, OFFSET>)
    }
}
pub trait IControlbarImpl: Sized {
    fn Create();
    fn Attach();
    fn Detach();
}
impl ::windows::core::RuntimeName for IControlbar {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IControlbar";
}
impl IControlbarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlbarImpl, const OFFSET: isize>() -> IControlbarVtbl {
        unsafe extern "system" fn Create<Impl: IControlbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: ::windows::core::RawPtr, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(ntype, &*(&pextendcontrolbar as *const <IExtendControlbar as ::windows::core::Abi>::Abi as *const <IExtendControlbar as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunknown)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attach<Impl: IControlbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntype: MMC_CONTROL_TYPE, lpunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attach(ntype, &*(&lpunknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Detach<Impl: IControlbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Detach(&*(&lpunknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IControlbar>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>, Attach::<Impl, OFFSET>, Detach::<Impl, OFFSET>)
    }
}
pub trait IDisplayHelpImpl: Sized {
    fn ShowTopic();
}
impl ::windows::core::RuntimeName for IDisplayHelp {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IDisplayHelp";
}
impl IDisplayHelpVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayHelpImpl, const OFFSET: isize>() -> IDisplayHelpVtbl {
        unsafe extern "system" fn ShowTopic<Impl: IDisplayHelpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelptopic: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowTopic(&*(&pszhelptopic as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayHelp>, ::windows::core::GetTrustLevel, ShowTopic::<Impl, OFFSET>)
    }
}
pub trait IEnumTASKImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumTASK {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IEnumTASK";
}
impl IEnumTASKVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTASKImpl, const OFFSET: isize>() -> IEnumTASKVtbl {
        unsafe extern "system" fn Next<Impl: IEnumTASKImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut MMC_TASK, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTASKImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTASKImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumTASKImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumTASK>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IExtendContextMenuImpl: Sized {
    fn AddMenuItems();
    fn Command();
}
impl ::windows::core::RuntimeName for IExtendContextMenu {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IExtendContextMenu";
}
impl IExtendContextMenuVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendContextMenuImpl, const OFFSET: isize>() -> IExtendContextMenuVtbl {
        unsafe extern "system" fn AddMenuItems<Impl: IExtendContextMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidataobject: ::windows::core::RawPtr, picallback: ::windows::core::RawPtr, pinsertionallowed: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMenuItems(&*(&pidataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), &*(&picallback as *const <IContextMenuCallback as ::windows::core::Abi>::Abi as *const <IContextMenuCallback as ::windows::core::DefaultType>::DefaultType), pinsertionallowed) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Command<Impl: IExtendContextMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcommandid: i32, pidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Command(lcommandid, &*(&pidataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IExtendContextMenu>, ::windows::core::GetTrustLevel, AddMenuItems::<Impl, OFFSET>, Command::<Impl, OFFSET>)
    }
}
pub trait IExtendControlbarImpl: Sized {
    fn SetControlbar();
    fn ControlbarNotify();
}
impl ::windows::core::RuntimeName for IExtendControlbar {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IExtendControlbar";
}
impl IExtendControlbarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendControlbarImpl, const OFFSET: isize>() -> IExtendControlbarVtbl {
        unsafe extern "system" fn SetControlbar<Impl: IExtendControlbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontrolbar: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetControlbar(&*(&pcontrolbar as *const <IControlbar as ::windows::core::Abi>::Abi as *const <IControlbar as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlbarNotify<Impl: IExtendControlbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlbarNotify(event, &*(&arg as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IExtendControlbar>, ::windows::core::GetTrustLevel, SetControlbar::<Impl, OFFSET>, ControlbarNotify::<Impl, OFFSET>)
    }
}
pub trait IExtendPropertySheetImpl: Sized {
    fn CreatePropertyPages();
    fn QueryPagesFor();
}
impl ::windows::core::RuntimeName for IExtendPropertySheet {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IExtendPropertySheet";
}
impl IExtendPropertySheetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendPropertySheetImpl, const OFFSET: isize>() -> IExtendPropertySheetVtbl {
        unsafe extern "system" fn CreatePropertyPages<Impl: IExtendPropertySheetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpprovider: ::windows::core::RawPtr, handle: isize, lpidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePropertyPages(&*(&lpprovider as *const <IPropertySheetCallback as ::windows::core::Abi>::Abi as *const <IPropertySheetCallback as ::windows::core::DefaultType>::DefaultType), handle, &*(&lpidataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPagesFor<Impl: IExtendPropertySheetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryPagesFor(&*(&lpdataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IExtendPropertySheet>, ::windows::core::GetTrustLevel, CreatePropertyPages::<Impl, OFFSET>, QueryPagesFor::<Impl, OFFSET>)
    }
}
pub trait IExtendPropertySheet2Impl: Sized + IExtendPropertySheetImpl {
    fn GetWatermarks();
}
impl ::windows::core::RuntimeName for IExtendPropertySheet2 {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IExtendPropertySheet2";
}
impl IExtendPropertySheet2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendPropertySheet2Impl, const OFFSET: isize>() -> IExtendPropertySheet2Vtbl {
        unsafe extern "system" fn GetWatermarks<Impl: IExtendPropertySheet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpidataobject: ::windows::core::RawPtr, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWatermarks(&*(&lpidataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&lphwatermark), ::core::mem::transmute_copy(&lphheader), ::core::mem::transmute_copy(&lphpalette), ::core::mem::transmute_copy(&bstretch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IExtendPropertySheet2>, ::windows::core::GetTrustLevel, GetWatermarks::<Impl, OFFSET>)
    }
}
pub trait IExtendTaskPadImpl: Sized {
    fn TaskNotify();
    fn EnumTasks();
    fn GetTitle();
    fn GetDescriptiveText();
    fn GetBackground();
    fn GetListPadInfo();
}
impl ::windows::core::RuntimeName for IExtendTaskPad {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IExtendTaskPad";
}
impl IExtendTaskPadVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendTaskPadImpl, const OFFSET: isize>() -> IExtendTaskPadVtbl {
        unsafe extern "system" fn TaskNotify<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdo: ::windows::core::RawPtr, arg: *const super::Com::VARIANT, param2: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskNotify(&*(&pdo as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), &*(&arg as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumTasks<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdo: ::windows::core::RawPtr, sztaskgroup: super::super::Foundation::PWSTR, ppenumtask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumTasks(&*(&pdo as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), &*(&sztaskgroup as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenumtask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTitle<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgroup: super::super::Foundation::PWSTR, psztitle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTitle(&*(&pszgroup as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&psztitle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptiveText<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgroup: super::super::Foundation::PWSTR, pszdescriptivetext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescriptiveText(&*(&pszgroup as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pszdescriptivetext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackground<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgroup: super::super::Foundation::PWSTR, ptdo: *mut MMC_TASK_DISPLAY_OBJECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackground(&*(&pszgroup as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptdo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetListPadInfo<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgroup: super::super::Foundation::PWSTR, lplistpadinfo: *mut MMC_LISTPAD_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetListPadInfo(&*(&pszgroup as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&lplistpadinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IExtendTaskPad>, ::windows::core::GetTrustLevel, TaskNotify::<Impl, OFFSET>, EnumTasks::<Impl, OFFSET>, GetTitle::<Impl, OFFSET>, GetDescriptiveText::<Impl, OFFSET>, GetBackground::<Impl, OFFSET>, GetListPadInfo::<Impl, OFFSET>)
    }
}
pub trait IExtendViewImpl: Sized {
    fn GetViews();
}
impl ::windows::core::RuntimeName for IExtendView {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IExtendView";
}
impl IExtendViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendViewImpl, const OFFSET: isize>() -> IExtendViewVtbl {
        unsafe extern "system" fn GetViews<Impl: IExtendViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, pviewextensioncallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViews(&*(&pdataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), &*(&pviewextensioncallback as *const <IViewExtensionCallback as ::windows::core::Abi>::Abi as *const <IViewExtensionCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IExtendView>, ::windows::core::GetTrustLevel, GetViews::<Impl, OFFSET>)
    }
}
pub trait IHeaderCtrlImpl: Sized {
    fn InsertColumn();
    fn DeleteColumn();
    fn SetColumnText();
    fn GetColumnText();
    fn SetColumnWidth();
    fn GetColumnWidth();
}
impl ::windows::core::RuntimeName for IHeaderCtrl {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IHeaderCtrl";
}
impl IHeaderCtrlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHeaderCtrlImpl, const OFFSET: isize>() -> IHeaderCtrlVtbl {
        unsafe extern "system" fn InsertColumn<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, title: super::super::Foundation::PWSTR, nformat: i32, nwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertColumn(ncol, &*(&title as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), nformat, nwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteColumn<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteColumn(ncol) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColumnText<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, title: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetColumnText(ncol, &*(&title as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnText<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, ptext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnText(ncol, ::core::mem::transmute_copy(&ptext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColumnWidth<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, nwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetColumnWidth(ncol, nwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnWidth<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, pwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnWidth(ncol, ::core::mem::transmute_copy(&pwidth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHeaderCtrl>, ::windows::core::GetTrustLevel, InsertColumn::<Impl, OFFSET>, DeleteColumn::<Impl, OFFSET>, SetColumnText::<Impl, OFFSET>, GetColumnText::<Impl, OFFSET>, SetColumnWidth::<Impl, OFFSET>, GetColumnWidth::<Impl, OFFSET>)
    }
}
pub trait IHeaderCtrl2Impl: Sized + IHeaderCtrlImpl {
    fn SetChangeTimeOut();
    fn SetColumnFilter();
    fn GetColumnFilter();
}
impl ::windows::core::RuntimeName for IHeaderCtrl2 {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IHeaderCtrl2";
}
impl IHeaderCtrl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHeaderCtrl2Impl, const OFFSET: isize>() -> IHeaderCtrl2Vtbl {
        unsafe extern "system" fn SetChangeTimeOut<Impl: IHeaderCtrl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, utimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetChangeTimeOut(utimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColumnFilter<Impl: IHeaderCtrl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetColumnFilter(ncolumn, dwtype, &*(&pfilterdata as *const <MMC_FILTERDATA as ::windows::core::Abi>::Abi as *const <MMC_FILTERDATA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnFilter<Impl: IHeaderCtrl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnFilter(ncolumn, pdwtype, &*(&pfilterdata as *const <MMC_FILTERDATA as ::windows::core::Abi>::Abi as *const <MMC_FILTERDATA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHeaderCtrl2>, ::windows::core::GetTrustLevel, SetChangeTimeOut::<Impl, OFFSET>, SetColumnFilter::<Impl, OFFSET>, GetColumnFilter::<Impl, OFFSET>)
    }
}
pub trait IImageListImpl: Sized {
    fn ImageListSetIcon();
    fn ImageListSetStrip();
}
impl ::windows::core::RuntimeName for IImageList {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IImageList";
}
impl IImageListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageListImpl, const OFFSET: isize>() -> IImageListVtbl {
        unsafe extern "system" fn ImageListSetIcon<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picon: *const isize, nloc: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageListSetIcon(picon, nloc) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageListSetStrip<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageListSetStrip(pbmapsm, pbmaplg, nstartloc, cmask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IImageList>, ::windows::core::GetTrustLevel, ImageListSetIcon::<Impl, OFFSET>, ImageListSetStrip::<Impl, OFFSET>)
    }
}
pub trait IMMCVersionInfoImpl: Sized {
    fn GetMMCVersion();
}
impl ::windows::core::RuntimeName for IMMCVersionInfo {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IMMCVersionInfo";
}
impl IMMCVersionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMMCVersionInfoImpl, const OFFSET: isize>() -> IMMCVersionInfoVtbl {
        unsafe extern "system" fn GetMMCVersion<Impl: IMMCVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversionmajor: *mut i32, pversionminor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMMCVersion(::core::mem::transmute_copy(&pversionmajor), ::core::mem::transmute_copy(&pversionminor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMMCVersionInfo>, ::windows::core::GetTrustLevel, GetMMCVersion::<Impl, OFFSET>)
    }
}
pub trait IMenuButtonImpl: Sized {
    fn AddButton();
    fn SetButton();
    fn SetButtonState();
}
impl ::windows::core::RuntimeName for IMenuButton {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IMenuButton";
}
impl IMenuButtonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuButtonImpl, const OFFSET: isize>() -> IMenuButtonVtbl {
        unsafe extern "system" fn AddButton<Impl: IMenuButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, lpbuttontext: super::super::Foundation::PWSTR, lptooltiptext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddButton(idcommand, &*(&lpbuttontext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&lptooltiptext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButton<Impl: IMenuButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, lpbuttontext: super::super::Foundation::PWSTR, lptooltiptext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetButton(idcommand, &*(&lpbuttontext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&lptooltiptext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonState<Impl: IMenuButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetButtonState(idcommand, nstate, &*(&bstate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMenuButton>, ::windows::core::GetTrustLevel, AddButton::<Impl, OFFSET>, SetButton::<Impl, OFFSET>, SetButtonState::<Impl, OFFSET>)
    }
}
pub trait IMessageViewImpl: Sized {
    fn SetTitleText();
    fn SetBodyText();
    fn SetIcon();
    fn Clear();
}
impl ::windows::core::RuntimeName for IMessageView {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IMessageView";
}
impl IMessageViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageViewImpl, const OFFSET: isize>() -> IMessageViewVtbl {
        unsafe extern "system" fn SetTitleText<Impl: IMessageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztitletext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTitleText(&*(&psztitletext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBodyText<Impl: IMessageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbodytext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBodyText(&*(&pszbodytext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIcon<Impl: IMessageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: IconIdentifier) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIcon(id) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IMessageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMessageView>, ::windows::core::GetTrustLevel, SetTitleText::<Impl, OFFSET>, SetBodyText::<Impl, OFFSET>, SetIcon::<Impl, OFFSET>, Clear::<Impl, OFFSET>)
    }
}
pub trait INodePropertiesImpl: Sized {
    fn GetProperty();
}
impl ::windows::core::RuntimeName for INodeProperties {
    const NAME: &'static str = "Windows.Win32.System.Mmc.INodeProperties";
}
impl INodePropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INodePropertiesImpl, const OFFSET: isize>() -> INodePropertiesVtbl {
        unsafe extern "system" fn GetProperty<Impl: INodePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, szpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrproperty: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&pdataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), &*(&szpropertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INodeProperties>, ::windows::core::GetTrustLevel, GetProperty::<Impl, OFFSET>)
    }
}
pub trait IPropertySheetCallbackImpl: Sized {
    fn AddPage();
    fn RemovePage();
}
impl ::windows::core::RuntimeName for IPropertySheetCallback {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IPropertySheetCallback";
}
impl IPropertySheetCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySheetCallbackImpl, const OFFSET: isize>() -> IPropertySheetCallbackVtbl {
        unsafe extern "system" fn AddPage<Impl: IPropertySheetCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPage(&*(&hpage as *const <super::super::UI::Controls::HPROPSHEETPAGE as ::windows::core::Abi>::Abi as *const <super::super::UI::Controls::HPROPSHEETPAGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePage<Impl: IPropertySheetCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovePage(&*(&hpage as *const <super::super::UI::Controls::HPROPSHEETPAGE as ::windows::core::Abi>::Abi as *const <super::super::UI::Controls::HPROPSHEETPAGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPropertySheetCallback>, ::windows::core::GetTrustLevel, AddPage::<Impl, OFFSET>, RemovePage::<Impl, OFFSET>)
    }
}
pub trait IPropertySheetProviderImpl: Sized {
    fn CreatePropertySheet();
    fn FindPropertySheet();
    fn AddPrimaryPages();
    fn AddExtensionPages();
    fn Show();
}
impl ::windows::core::RuntimeName for IPropertySheetProvider {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IPropertySheetProvider";
}
impl IPropertySheetProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySheetProviderImpl, const OFFSET: isize>() -> IPropertySheetProviderVtbl {
        unsafe extern "system" fn CreatePropertySheet<Impl: IPropertySheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: super::super::Foundation::PWSTR, r#type: u8, cookie: isize, pidataobjectm: ::windows::core::RawPtr, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePropertySheet(&*(&title as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, cookie, &*(&pidataobjectm as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), dwoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPropertySheet<Impl: IPropertySheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize, lpcomponent: ::windows::core::RawPtr, lpdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPropertySheet(hitem, &*(&lpcomponent as *const <IComponent as ::windows::core::Abi>::Abi as *const <IComponent as ::windows::core::DefaultType>::DefaultType), &*(&lpdataobject as *const <super::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::Com::IDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPrimaryPages<Impl: IPropertySheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpunknown: *mut ::core::ffi::c_void, bcreatehandle: super::super::Foundation::BOOL, hnotifywindow: super::super::Foundation::HWND, bscopepane: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPrimaryPages(
                &*(&lpunknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&bcreatehandle as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&hnotifywindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&bscopepane as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddExtensionPages<Impl: IPropertySheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddExtensionPages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: IPropertySheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: isize, page: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Show(window, page) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPropertySheetProvider>, ::windows::core::GetTrustLevel, CreatePropertySheet::<Impl, OFFSET>, FindPropertySheet::<Impl, OFFSET>, AddPrimaryPages::<Impl, OFFSET>, AddExtensionPages::<Impl, OFFSET>, Show::<Impl, OFFSET>)
    }
}
pub trait IRequiredExtensionsImpl: Sized {
    fn EnableAllExtensions();
    fn GetFirstExtension();
    fn GetNextExtension();
}
impl ::windows::core::RuntimeName for IRequiredExtensions {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IRequiredExtensions";
}
impl IRequiredExtensionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRequiredExtensionsImpl, const OFFSET: isize>() -> IRequiredExtensionsVtbl {
        unsafe extern "system" fn EnableAllExtensions<Impl: IRequiredExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableAllExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstExtension<Impl: IRequiredExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFirstExtension(::core::mem::transmute_copy(&pextclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextExtension<Impl: IRequiredExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextExtension(::core::mem::transmute_copy(&pextclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRequiredExtensions>, ::windows::core::GetTrustLevel, EnableAllExtensions::<Impl, OFFSET>, GetFirstExtension::<Impl, OFFSET>, GetNextExtension::<Impl, OFFSET>)
    }
}
pub trait IResultDataImpl: Sized {
    fn InsertItem();
    fn DeleteItem();
    fn FindItemByLParam();
    fn DeleteAllRsltItems();
    fn SetItem();
    fn GetItem();
    fn GetNextItem();
    fn ModifyItemState();
    fn ModifyViewStyle();
    fn SetViewMode();
    fn GetViewMode();
    fn UpdateItem();
    fn Sort();
    fn SetDescBarText();
    fn SetItemCount();
}
impl ::windows::core::RuntimeName for IResultData {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IResultData";
}
impl IResultDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResultDataImpl, const OFFSET: isize>() -> IResultDataVtbl {
        unsafe extern "system" fn InsertItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertItem(&*(&item as *const <RESULTDATAITEM as ::windows::core::Abi>::Abi as *const <RESULTDATAITEM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: isize, ncol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteItem(itemid, ncol) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindItemByLParam<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM, pitemid: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindItemByLParam(&*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAllRsltItems<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAllRsltItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *const RESULTDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetItem(&*(&item as *const <RESULTDATAITEM as ::windows::core::Abi>::Abi as *const <RESULTDATAITEM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItem(&*(&item as *const <RESULTDATAITEM as ::windows::core::Abi>::Abi as *const <RESULTDATAITEM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextItem(&*(&item as *const <RESULTDATAITEM as ::windows::core::Abi>::Abi as *const <RESULTDATAITEM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyItemState<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyItemState(nindex, itemid, uadd, uremove) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyViewStyle<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyViewStyle(add, remove) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewMode<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lviewmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetViewMode(lviewmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewMode<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lviewmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewMode(::core::mem::transmute_copy(&lviewmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateItem(itemid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sort<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sort(ncolumn, dwsortoptions, &*(&luserparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescBarText<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desctext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDescBarText(&*(&desctext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItemCount<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemcount: i32, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetItemCount(nitemcount, dwoptions) {
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
            ::windows::core::GetRuntimeClassName::<IResultData>,
            ::windows::core::GetTrustLevel,
            InsertItem::<Impl, OFFSET>,
            DeleteItem::<Impl, OFFSET>,
            FindItemByLParam::<Impl, OFFSET>,
            DeleteAllRsltItems::<Impl, OFFSET>,
            SetItem::<Impl, OFFSET>,
            GetItem::<Impl, OFFSET>,
            GetNextItem::<Impl, OFFSET>,
            ModifyItemState::<Impl, OFFSET>,
            ModifyViewStyle::<Impl, OFFSET>,
            SetViewMode::<Impl, OFFSET>,
            GetViewMode::<Impl, OFFSET>,
            UpdateItem::<Impl, OFFSET>,
            Sort::<Impl, OFFSET>,
            SetDescBarText::<Impl, OFFSET>,
            SetItemCount::<Impl, OFFSET>,
        )
    }
}
pub trait IResultData2Impl: Sized + IResultDataImpl {
    fn RenameResultItem();
}
impl ::windows::core::RuntimeName for IResultData2 {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IResultData2";
}
impl IResultData2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResultData2Impl, const OFFSET: isize>() -> IResultData2Vtbl {
        unsafe extern "system" fn RenameResultItem<Impl: IResultData2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenameResultItem(itemid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IResultData2>, ::windows::core::GetTrustLevel, RenameResultItem::<Impl, OFFSET>)
    }
}
pub trait IResultDataCompareImpl: Sized {
    fn Compare();
}
impl ::windows::core::RuntimeName for IResultDataCompare {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IResultDataCompare";
}
impl IResultDataCompareVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResultDataCompareImpl, const OFFSET: isize>() -> IResultDataCompareVtbl {
        unsafe extern "system" fn Compare<Impl: IResultDataCompareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, luserparam: super::super::Foundation::LPARAM, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compare(&*(&luserparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), cookiea, cookieb, pnresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IResultDataCompare>, ::windows::core::GetTrustLevel, Compare::<Impl, OFFSET>)
    }
}
pub trait IResultDataCompareExImpl: Sized {
    fn Compare();
}
impl ::windows::core::RuntimeName for IResultDataCompareEx {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IResultDataCompareEx";
}
impl IResultDataCompareExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResultDataCompareExImpl, const OFFSET: isize>() -> IResultDataCompareExVtbl {
        unsafe extern "system" fn Compare<Impl: IResultDataCompareExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prdc: *const RDCOMPARE, pnresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compare(&*(&prdc as *const <RDCOMPARE as ::windows::core::Abi>::Abi as *const <RDCOMPARE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pnresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IResultDataCompareEx>, ::windows::core::GetTrustLevel, Compare::<Impl, OFFSET>)
    }
}
pub trait IResultOwnerDataImpl: Sized {
    fn FindItem();
    fn CacheHint();
    fn SortItems();
}
impl ::windows::core::RuntimeName for IResultOwnerData {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IResultOwnerData";
}
impl IResultOwnerDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResultOwnerDataImpl, const OFFSET: isize>() -> IResultOwnerDataVtbl {
        unsafe extern "system" fn FindItem<Impl: IResultOwnerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfindinfo: *const RESULTFINDINFO, pnfoundindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindItem(&*(&pfindinfo as *const <RESULTFINDINFO as ::windows::core::Abi>::Abi as *const <RESULTFINDINFO as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pnfoundindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CacheHint<Impl: IResultOwnerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nstartindex: i32, nendindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CacheHint(nstartindex, nendindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SortItems<Impl: IResultOwnerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SortItems(ncolumn, dwsortoptions, &*(&luserparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IResultOwnerData>, ::windows::core::GetTrustLevel, FindItem::<Impl, OFFSET>, CacheHint::<Impl, OFFSET>, SortItems::<Impl, OFFSET>)
    }
}
pub trait ISnapinAboutImpl: Sized {
    fn GetSnapinDescription();
    fn GetProvider();
    fn GetSnapinVersion();
    fn GetSnapinImage();
    fn GetStaticFolderImage();
}
impl ::windows::core::RuntimeName for ISnapinAbout {
    const NAME: &'static str = "Windows.Win32.System.Mmc.ISnapinAbout";
}
impl ISnapinAboutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISnapinAboutImpl, const OFFSET: isize>() -> ISnapinAboutVtbl {
        unsafe extern "system" fn GetSnapinDescription<Impl: ISnapinAboutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapinDescription(::core::mem::transmute_copy(&lpdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProvider<Impl: ISnapinAboutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProvider(::core::mem::transmute_copy(&lpname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSnapinVersion<Impl: ISnapinAboutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpversion: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapinVersion(::core::mem::transmute_copy(&lpversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSnapinImage<Impl: ISnapinAboutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, happicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapinImage(::core::mem::transmute_copy(&happicon)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStaticFolderImage<Impl: ISnapinAboutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStaticFolderImage(::core::mem::transmute_copy(&hsmallimage), ::core::mem::transmute_copy(&hsmallimageopen), ::core::mem::transmute_copy(&hlargeimage), ::core::mem::transmute_copy(&cmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISnapinAbout>, ::windows::core::GetTrustLevel, GetSnapinDescription::<Impl, OFFSET>, GetProvider::<Impl, OFFSET>, GetSnapinVersion::<Impl, OFFSET>, GetSnapinImage::<Impl, OFFSET>, GetStaticFolderImage::<Impl, OFFSET>)
    }
}
pub trait ISnapinHelpImpl: Sized {
    fn GetHelpTopic();
}
impl ::windows::core::RuntimeName for ISnapinHelp {
    const NAME: &'static str = "Windows.Win32.System.Mmc.ISnapinHelp";
}
impl ISnapinHelpVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISnapinHelpImpl, const OFFSET: isize>() -> ISnapinHelpVtbl {
        unsafe extern "system" fn GetHelpTopic<Impl: ISnapinHelpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcompiledhelpfile: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelpTopic(::core::mem::transmute_copy(&lpcompiledhelpfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISnapinHelp>, ::windows::core::GetTrustLevel, GetHelpTopic::<Impl, OFFSET>)
    }
}
pub trait ISnapinHelp2Impl: Sized + ISnapinHelpImpl {
    fn GetLinkedTopics();
}
impl ::windows::core::RuntimeName for ISnapinHelp2 {
    const NAME: &'static str = "Windows.Win32.System.Mmc.ISnapinHelp2";
}
impl ISnapinHelp2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISnapinHelp2Impl, const OFFSET: isize>() -> ISnapinHelp2Vtbl {
        unsafe extern "system" fn GetLinkedTopics<Impl: ISnapinHelp2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcompiledhelpfiles: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLinkedTopics(::core::mem::transmute_copy(&lpcompiledhelpfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISnapinHelp2>, ::windows::core::GetTrustLevel, GetLinkedTopics::<Impl, OFFSET>)
    }
}
pub trait ISnapinPropertiesImpl: Sized {
    fn Initialize();
    fn QueryPropertyNames();
    fn PropertiesChanged();
}
impl ::windows::core::RuntimeName for ISnapinProperties {
    const NAME: &'static str = "Windows.Win32.System.Mmc.ISnapinProperties";
}
impl ISnapinPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISnapinPropertiesImpl, const OFFSET: isize>() -> ISnapinPropertiesVtbl {
        unsafe extern "system" fn Initialize<Impl: ISnapinPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pproperties as *const <Properties as ::windows::core::Abi>::Abi as *const <Properties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPropertyNames<Impl: ISnapinPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryPropertyNames(&*(&pcallback as *const <ISnapinPropertiesCallback as ::windows::core::Abi>::Abi as *const <ISnapinPropertiesCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertiesChanged<Impl: ISnapinPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: i32, pproperties: *const MMC_SNAPIN_PROPERTY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertiesChanged(cproperties, &*(&pproperties as *const <MMC_SNAPIN_PROPERTY as ::windows::core::Abi>::Abi as *const <MMC_SNAPIN_PROPERTY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISnapinProperties>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, QueryPropertyNames::<Impl, OFFSET>, PropertiesChanged::<Impl, OFFSET>)
    }
}
pub trait ISnapinPropertiesCallbackImpl: Sized {
    fn AddPropertyName();
}
impl ::windows::core::RuntimeName for ISnapinPropertiesCallback {
    const NAME: &'static str = "Windows.Win32.System.Mmc.ISnapinPropertiesCallback";
}
impl ISnapinPropertiesCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISnapinPropertiesCallbackImpl, const OFFSET: isize>() -> ISnapinPropertiesCallbackVtbl {
        unsafe extern "system" fn AddPropertyName<Impl: ISnapinPropertiesCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPropertyName(&*(&pszpropname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISnapinPropertiesCallback>, ::windows::core::GetTrustLevel, AddPropertyName::<Impl, OFFSET>)
    }
}
pub trait IStringTableImpl: Sized {
    fn AddString();
    fn GetString();
    fn GetStringLength();
    fn DeleteString();
    fn DeleteAllStrings();
    fn FindString();
    fn Enumerate();
}
impl ::windows::core::RuntimeName for IStringTable {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IStringTable";
}
impl IStringTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStringTableImpl, const OFFSET: isize>() -> IStringTableVtbl {
        unsafe extern "system" fn AddString<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszadd: super::super::Foundation::PWSTR, pstringid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddString(&*(&pszadd as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pstringid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stringid: u32, cchbuffer: u32, lpbuffer: super::super::Foundation::PWSTR, pcchout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString(stringid, cchbuffer, ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&pcchout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringLength<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stringid: u32, pcchstring: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringLength(stringid, ::core::mem::transmute_copy(&pcchstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteString<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stringid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteString(stringid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAllStrings<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAllStrings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindString<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfind: super::super::Foundation::PWSTR, pstringid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindString(&*(&pszfind as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pstringid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enumerate<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enumerate(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStringTable>, ::windows::core::GetTrustLevel, AddString::<Impl, OFFSET>, GetString::<Impl, OFFSET>, GetStringLength::<Impl, OFFSET>, DeleteString::<Impl, OFFSET>, DeleteAllStrings::<Impl, OFFSET>, FindString::<Impl, OFFSET>, Enumerate::<Impl, OFFSET>)
    }
}
pub trait IToolbarImpl: Sized {
    fn AddBitmap();
    fn AddButtons();
    fn InsertButton();
    fn DeleteButton();
    fn GetButtonState();
    fn SetButtonState();
}
impl ::windows::core::RuntimeName for IToolbar {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IToolbar";
}
impl IToolbarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToolbarImpl, const OFFSET: isize>() -> IToolbarVtbl {
        unsafe extern "system" fn AddBitmap<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nimages: i32, hbmp: super::super::Graphics::Gdi::HBITMAP, cxsize: i32, cysize: i32, crmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddBitmap(nimages, &*(&hbmp as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::DefaultType>::DefaultType), cxsize, cysize, crmask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddButtons<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddButtons(nbuttons, &*(&lpbuttons as *const <MMCBUTTON as ::windows::core::Abi>::Abi as *const <MMCBUTTON as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertButton<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, lpbutton: *const MMCBUTTON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertButton(nindex, &*(&lpbutton as *const <MMCBUTTON as ::windows::core::Abi>::Abi as *const <MMCBUTTON as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteButton<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteButton(nindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetButtonState<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetButtonState(idcommand, nstate, ::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonState<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetButtonState(idcommand, nstate, &*(&bstate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IToolbar>, ::windows::core::GetTrustLevel, AddBitmap::<Impl, OFFSET>, AddButtons::<Impl, OFFSET>, InsertButton::<Impl, OFFSET>, DeleteButton::<Impl, OFFSET>, GetButtonState::<Impl, OFFSET>, SetButtonState::<Impl, OFFSET>)
    }
}
pub trait IViewExtensionCallbackImpl: Sized {
    fn AddView();
}
impl ::windows::core::RuntimeName for IViewExtensionCallback {
    const NAME: &'static str = "Windows.Win32.System.Mmc.IViewExtensionCallback";
}
impl IViewExtensionCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewExtensionCallbackImpl, const OFFSET: isize>() -> IViewExtensionCallbackVtbl {
        unsafe extern "system" fn AddView<Impl: IViewExtensionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextviewdata: *const MMC_EXT_VIEW_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddView(&*(&pextviewdata as *const <MMC_EXT_VIEW_DATA as ::windows::core::Abi>::Abi as *const <MMC_EXT_VIEW_DATA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IViewExtensionCallback>, ::windows::core::GetTrustLevel, AddView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait MenuItemImpl: Sized + IDispatchImpl {
    fn DisplayName();
    fn LanguageIndependentName();
    fn Path();
    fn LanguageIndependentPath();
    fn Execute();
    fn Enabled();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for MenuItem {
    const NAME: &'static str = "Windows.Win32.System.Mmc.MenuItem";
}
#[cfg(feature = "Win32_System_Com")]
impl MenuItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: MenuItemImpl, const OFFSET: isize>() -> MenuItemVtbl {
        unsafe extern "system" fn DisplayName<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName(::core::mem::transmute_copy(&displayname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LanguageIndependentName<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageindependentname: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageIndependentName(::core::mem::transmute_copy(&languageindependentname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LanguageIndependentPath<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageindependentpath: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageIndependentPath(::core::mem::transmute_copy(&languageindependentpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Execute<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Execute() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&enabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<MenuItem>, ::windows::core::GetTrustLevel, DisplayName::<Impl, OFFSET>, LanguageIndependentName::<Impl, OFFSET>, Path::<Impl, OFFSET>, LanguageIndependentPath::<Impl, OFFSET>, Execute::<Impl, OFFSET>, Enabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait NodeImpl: Sized + IDispatchImpl {
    fn Name();
    fn Property();
    fn Bookmark();
    fn IsScopeNode();
    fn Nodetype();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for Node {
    const NAME: &'static str = "Windows.Win32.System.Mmc.Node";
}
#[cfg(feature = "Win32_System_Com")]
impl NodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: NodeImpl, const OFFSET: isize>() -> NodeVtbl {
        unsafe extern "system" fn Name<Impl: NodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Property<Impl: NodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Property(&*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&propertyvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bookmark<Impl: NodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bookmark: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bookmark(::core::mem::transmute_copy(&bookmark)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScopeNode<Impl: NodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isscopenode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsScopeNode(::core::mem::transmute_copy(&isscopenode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Nodetype<Impl: NodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodetype: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Nodetype(::core::mem::transmute_copy(&nodetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<Node>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, Property::<Impl, OFFSET>, Bookmark::<Impl, OFFSET>, IsScopeNode::<Impl, OFFSET>, Nodetype::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait NodesImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for Nodes {
    const NAME: &'static str = "Windows.Win32.System.Mmc.Nodes";
}
#[cfg(feature = "Win32_System_Com")]
impl NodesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: NodesImpl, const OFFSET: isize>() -> NodesVtbl {
        unsafe extern "system" fn _NewEnum<Impl: NodesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: NodesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: NodesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<Nodes>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait PropertiesImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for Properties {
    const NAME: &'static str = "Windows.Win32.System.Mmc.Properties";
}
#[cfg(feature = "Win32_System_Com")]
impl PropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: PropertiesImpl, const OFFSET: isize>() -> PropertiesVtbl {
        unsafe extern "system" fn _NewEnum<Impl: PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<Properties>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, Remove::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait PropertyImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
    fn Name();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for Property {
    const NAME: &'static str = "Windows.Win32.System.Mmc.Property";
}
#[cfg(feature = "Win32_System_Com")]
impl PropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: PropertyImpl, const OFFSET: isize>() -> PropertyVtbl {
        unsafe extern "system" fn Value<Impl: PropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: PropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&value as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: PropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<Property>, ::windows::core::GetTrustLevel, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, Name::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ScopeNamespaceImpl: Sized + IDispatchImpl {
    fn GetParent();
    fn GetChild();
    fn GetNext();
    fn GetRoot();
    fn Expand();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ScopeNamespace {
    const NAME: &'static str = "Windows.Win32.System.Mmc.ScopeNamespace";
}
#[cfg(feature = "Win32_System_Com")]
impl ScopeNamespaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ScopeNamespaceImpl, const OFFSET: isize>() -> ScopeNamespaceVtbl {
        unsafe extern "system" fn GetParent<Impl: ScopeNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParent(&*(&node as *const <Node as ::windows::core::Abi>::Abi as *const <Node as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&parent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChild<Impl: ScopeNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, child: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChild(&*(&node as *const <Node as ::windows::core::Abi>::Abi as *const <Node as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&child)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNext<Impl: ScopeNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, next: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNext(&*(&node as *const <Node as ::windows::core::Abi>::Abi as *const <Node as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&next)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRoot<Impl: ScopeNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, root: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRoot(::core::mem::transmute_copy(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Expand<Impl: ScopeNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Expand(&*(&node as *const <Node as ::windows::core::Abi>::Abi as *const <Node as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ScopeNamespace>, ::windows::core::GetTrustLevel, GetParent::<Impl, OFFSET>, GetChild::<Impl, OFFSET>, GetNext::<Impl, OFFSET>, GetRoot::<Impl, OFFSET>, Expand::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait SnapInImpl: Sized + IDispatchImpl {
    fn Name();
    fn Vendor();
    fn Version();
    fn Extensions();
    fn SnapinCLSID();
    fn Properties();
    fn EnableAllExtensions();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for SnapIn {
    const NAME: &'static str = "Windows.Win32.System.Mmc.SnapIn";
}
#[cfg(feature = "Win32_System_Com")]
impl SnapInVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: SnapInImpl, const OFFSET: isize>() -> SnapInVtbl {
        unsafe extern "system" fn Name<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Vendor<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendor: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Vendor(::core::mem::transmute_copy(&vendor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version(::core::mem::transmute_copy(&version)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extensions<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extensions(::core::mem::transmute_copy(&extensions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnapinCLSID<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapinclsid: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SnapinCLSID(::core::mem::transmute_copy(&snapinclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&properties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAllExtensions<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableAllExtensions(&*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<SnapIn>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, Vendor::<Impl, OFFSET>, Version::<Impl, OFFSET>, Extensions::<Impl, OFFSET>, SnapinCLSID::<Impl, OFFSET>, Properties::<Impl, OFFSET>, EnableAllExtensions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait SnapInsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for SnapIns {
    const NAME: &'static str = "Windows.Win32.System.Mmc.SnapIns";
}
#[cfg(feature = "Win32_System_Com")]
impl SnapInsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: SnapInsImpl, const OFFSET: isize>() -> SnapInsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: SnapInsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: SnapInsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, snapin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&snapin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: SnapInsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: SnapInsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapinnameorclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, parentsnapin: ::core::mem::ManuallyDrop<super::Com::VARIANT>, properties: ::core::mem::ManuallyDrop<super::Com::VARIANT>, snapin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(
                &*(&snapinnameorclsid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&parentsnapin as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&properties as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&snapin),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: SnapInsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&snapin as *const <SnapIn as ::windows::core::Abi>::Abi as *const <SnapIn as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<SnapIns>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ViewImpl: Sized + IDispatchImpl {
    fn ActiveScopeNode();
    fn SetActiveScopeNode();
    fn Selection();
    fn ListItems();
    fn SnapinScopeObject();
    fn SnapinSelectionObject();
    fn Is();
    fn Document();
    fn SelectAll();
    fn Select();
    fn Deselect();
    fn IsSelected();
    fn DisplayScopeNodePropertySheet();
    fn DisplaySelectionPropertySheet();
    fn CopyScopeNode();
    fn CopySelection();
    fn DeleteScopeNode();
    fn DeleteSelection();
    fn RenameScopeNode();
    fn RenameSelectedItem();
    fn ScopeNodeContextMenu();
    fn SelectionContextMenu();
    fn RefreshScopeNode();
    fn RefreshSelection();
    fn ExecuteSelectionMenuItem();
    fn ExecuteScopeNodeMenuItem();
    fn ExecuteShellCommand();
    fn Frame();
    fn Close();
    fn ScopeTreeVisible();
    fn SetScopeTreeVisible();
    fn Back();
    fn Forward();
    fn SetStatusBarText();
    fn Memento();
    fn ViewMemento();
    fn Columns();
    fn CellContents();
    fn ExportList();
    fn ListViewMode();
    fn SetListViewMode();
    fn ControlObject();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for View {
    const NAME: &'static str = "Windows.Win32.System.Mmc.View";
}
#[cfg(feature = "Win32_System_Com")]
impl ViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ViewImpl, const OFFSET: isize>() -> ViewVtbl {
        unsafe extern "system" fn ActiveScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveScopeNode(::core::mem::transmute_copy(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActiveScopeNode(&*(&node as *const <Node as ::windows::core::Abi>::Abi as *const <Node as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Selection<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Selection(::core::mem::transmute_copy(&nodes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListItems<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListItems(::core::mem::transmute_copy(&nodes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnapinScopeObject<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>, scopenodeobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SnapinScopeObject(&*(&scopenode as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&scopenodeobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnapinSelectionObject<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SnapinSelectionObject(::core::mem::transmute_copy(&selectionobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Is<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, thesame: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Is(&*(&view as *const <View as ::windows::core::Abi>::Abi as *const <View as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&thesame)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Document<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Document(::core::mem::transmute_copy(&document)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectAll<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Select<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Select(&*(&node as *const <Node as ::windows::core::Abi>::Abi as *const <Node as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deselect<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deselect(&*(&node as *const <Node as ::windows::core::Abi>::Abi as *const <Node as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSelected<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, isselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelected(&*(&node as *const <Node as ::windows::core::Abi>::Abi as *const <Node as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&isselected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayScopeNodePropertySheet<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayScopeNodePropertySheet(&*(&scopenode as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplaySelectionPropertySheet<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplaySelectionPropertySheet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyScopeNode(&*(&scopenode as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopySelection<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopySelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteScopeNode(&*(&scopenode as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteSelection<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenameScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenameScopeNode(&*(&newname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&scopenode as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenameSelectedItem<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenameSelectedItem(&*(&newname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScopeNodeContextMenu<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>, contextmenu: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScopeNodeContextMenu(&*(&scopenode as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&contextmenu)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionContextMenu<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextmenu: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionContextMenu(::core::mem::transmute_copy(&contextmenu)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RefreshScopeNode(&*(&scopenode as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshSelection<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RefreshSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecuteSelectionMenuItem<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, menuitempath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecuteSelectionMenuItem(&*(&menuitempath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecuteScopeNodeMenuItem<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, menuitempath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecuteScopeNodeMenuItem(&*(&menuitempath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&scopenode as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecuteShellCommand<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, directory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, parameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, windowstate: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecuteShellCommand(
                &*(&command as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&directory as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&parameters as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&windowstate as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Frame<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Frame(::core::mem::transmute_copy(&frame)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScopeTreeVisible<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScopeTreeVisible(::core::mem::transmute_copy(&visible)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScopeTreeVisible<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScopeTreeVisible(&*(&visible as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Back<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Back() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forward<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Forward() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatusBarText<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statusbartext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStatusBarText(&*(&statusbartext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Memento<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memento: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Memento(::core::mem::transmute_copy(&memento)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewMemento<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memento: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewMemento(&*(&memento as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Columns<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, columns: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Columns(::core::mem::transmute_copy(&columns)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CellContents<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, column: i32, cellcontents: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CellContents(&*(&node as *const <Node as ::windows::core::Abi>::Abi as *const <Node as ::windows::core::DefaultType>::DefaultType), column, ::core::mem::transmute_copy(&cellcontents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportList<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exportoptions: _ExportListOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExportList(&*(&file as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), exportoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListViewMode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut _ListViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListViewMode(::core::mem::transmute_copy(&mode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListViewMode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: _ListViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetListViewMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlObject<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, control: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlObject(::core::mem::transmute_copy(&control)) {
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
            ::windows::core::GetRuntimeClassName::<View>,
            ::windows::core::GetTrustLevel,
            ActiveScopeNode::<Impl, OFFSET>,
            SetActiveScopeNode::<Impl, OFFSET>,
            Selection::<Impl, OFFSET>,
            ListItems::<Impl, OFFSET>,
            SnapinScopeObject::<Impl, OFFSET>,
            SnapinSelectionObject::<Impl, OFFSET>,
            Is::<Impl, OFFSET>,
            Document::<Impl, OFFSET>,
            SelectAll::<Impl, OFFSET>,
            Select::<Impl, OFFSET>,
            Deselect::<Impl, OFFSET>,
            IsSelected::<Impl, OFFSET>,
            DisplayScopeNodePropertySheet::<Impl, OFFSET>,
            DisplaySelectionPropertySheet::<Impl, OFFSET>,
            CopyScopeNode::<Impl, OFFSET>,
            CopySelection::<Impl, OFFSET>,
            DeleteScopeNode::<Impl, OFFSET>,
            DeleteSelection::<Impl, OFFSET>,
            RenameScopeNode::<Impl, OFFSET>,
            RenameSelectedItem::<Impl, OFFSET>,
            ScopeNodeContextMenu::<Impl, OFFSET>,
            SelectionContextMenu::<Impl, OFFSET>,
            RefreshScopeNode::<Impl, OFFSET>,
            RefreshSelection::<Impl, OFFSET>,
            ExecuteSelectionMenuItem::<Impl, OFFSET>,
            ExecuteScopeNodeMenuItem::<Impl, OFFSET>,
            ExecuteShellCommand::<Impl, OFFSET>,
            Frame::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            ScopeTreeVisible::<Impl, OFFSET>,
            SetScopeTreeVisible::<Impl, OFFSET>,
            Back::<Impl, OFFSET>,
            Forward::<Impl, OFFSET>,
            SetStatusBarText::<Impl, OFFSET>,
            Memento::<Impl, OFFSET>,
            ViewMemento::<Impl, OFFSET>,
            Columns::<Impl, OFFSET>,
            CellContents::<Impl, OFFSET>,
            ExportList::<Impl, OFFSET>,
            ListViewMode::<Impl, OFFSET>,
            SetListViewMode::<Impl, OFFSET>,
            ControlObject::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ViewsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn Add();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for Views {
    const NAME: &'static str = "Windows.Win32.System.Mmc.Views";
}
#[cfg(feature = "Win32_System_Com")]
impl ViewsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ViewsImpl, const OFFSET: isize>() -> ViewsVtbl {
        unsafe extern "system" fn Item<Impl: ViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, view: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&view)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, viewoptions: _ViewOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&node as *const <Node as ::windows::core::Abi>::Abi as *const <Node as ::windows::core::DefaultType>::DefaultType), viewoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<Views>, ::windows::core::GetTrustLevel, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, Add::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _AppEventsImpl: Sized + IDispatchImpl {
    fn OnQuit();
    fn OnDocumentOpen();
    fn OnDocumentClose();
    fn OnSnapInAdded();
    fn OnSnapInRemoved();
    fn OnNewView();
    fn OnViewClose();
    fn OnViewChange();
    fn OnSelectionChange();
    fn OnContextMenuExecuted();
    fn OnToolbarButtonClicked();
    fn OnListUpdated();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _AppEvents {
    const NAME: &'static str = "Windows.Win32.System.Mmc._AppEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _AppEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _AppEventsImpl, const OFFSET: isize>() -> _AppEventsVtbl {
        unsafe extern "system" fn OnQuit<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnQuit(&*(&application as *const <_Application as ::windows::core::Abi>::Abi as *const <_Application as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDocumentOpen<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr, new: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDocumentOpen(&*(&document as *const <Document as ::windows::core::Abi>::Abi as *const <Document as ::windows::core::DefaultType>::DefaultType), &*(&new as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDocumentClose<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDocumentClose(&*(&document as *const <Document as ::windows::core::Abi>::Abi as *const <Document as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSnapInAdded<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr, snapin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSnapInAdded(&*(&document as *const <Document as ::windows::core::Abi>::Abi as *const <Document as ::windows::core::DefaultType>::DefaultType), &*(&snapin as *const <SnapIn as ::windows::core::Abi>::Abi as *const <SnapIn as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSnapInRemoved<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr, snapin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSnapInRemoved(&*(&document as *const <Document as ::windows::core::Abi>::Abi as *const <Document as ::windows::core::DefaultType>::DefaultType), &*(&snapin as *const <SnapIn as ::windows::core::Abi>::Abi as *const <SnapIn as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnNewView<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnNewView(&*(&view as *const <View as ::windows::core::Abi>::Abi as *const <View as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnViewClose<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnViewClose(&*(&view as *const <View as ::windows::core::Abi>::Abi as *const <View as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnViewChange<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, newownernode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnViewChange(&*(&view as *const <View as ::windows::core::Abi>::Abi as *const <View as ::windows::core::DefaultType>::DefaultType), &*(&newownernode as *const <Node as ::windows::core::Abi>::Abi as *const <Node as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSelectionChange<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, newnodes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSelectionChange(&*(&view as *const <View as ::windows::core::Abi>::Abi as *const <View as ::windows::core::DefaultType>::DefaultType), &*(&newnodes as *const <Nodes as ::windows::core::Abi>::Abi as *const <Nodes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnContextMenuExecuted<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, menuitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnContextMenuExecuted(&*(&menuitem as *const <MenuItem as ::windows::core::Abi>::Abi as *const <MenuItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnToolbarButtonClicked<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnToolbarButtonClicked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnListUpdated<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnListUpdated(&*(&view as *const <View as ::windows::core::Abi>::Abi as *const <View as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<_AppEvents>,
            ::windows::core::GetTrustLevel,
            OnQuit::<Impl, OFFSET>,
            OnDocumentOpen::<Impl, OFFSET>,
            OnDocumentClose::<Impl, OFFSET>,
            OnSnapInAdded::<Impl, OFFSET>,
            OnSnapInRemoved::<Impl, OFFSET>,
            OnNewView::<Impl, OFFSET>,
            OnViewClose::<Impl, OFFSET>,
            OnViewChange::<Impl, OFFSET>,
            OnSelectionChange::<Impl, OFFSET>,
            OnContextMenuExecuted::<Impl, OFFSET>,
            OnToolbarButtonClicked::<Impl, OFFSET>,
            OnListUpdated::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _ApplicationImpl: Sized + IDispatchImpl {
    fn Help();
    fn Quit();
    fn Document();
    fn Load();
    fn Frame();
    fn Visible();
    fn Show();
    fn Hide();
    fn UserControl();
    fn SetUserControl();
    fn VersionMajor();
    fn VersionMinor();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _Application {
    const NAME: &'static str = "Windows.Win32.System.Mmc._Application";
}
#[cfg(feature = "Win32_System_Com")]
impl _ApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ApplicationImpl, const OFFSET: isize>() -> _ApplicationVtbl {
        unsafe extern "system" fn Help<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Help().into()
        }
        unsafe extern "system" fn Quit<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Quit().into()
        }
        unsafe extern "system" fn Document<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Document(::core::mem::transmute_copy(&document)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Load(&*(&filename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Frame<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Frame(::core::mem::transmute_copy(&frame)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Visible<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Visible(::core::mem::transmute_copy(&visible)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Show() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hide<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hide() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserControl<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usercontrol: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserControl(::core::mem::transmute_copy(&usercontrol)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserControl<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usercontrol: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUserControl(&*(&usercontrol as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VersionMajor<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, versionmajor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VersionMajor(::core::mem::transmute_copy(&versionmajor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VersionMinor<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, versionminor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VersionMinor(::core::mem::transmute_copy(&versionminor)) {
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
            ::windows::core::GetRuntimeClassName::<_Application>,
            ::windows::core::GetTrustLevel,
            Help::<Impl, OFFSET>,
            Quit::<Impl, OFFSET>,
            Document::<Impl, OFFSET>,
            Load::<Impl, OFFSET>,
            Frame::<Impl, OFFSET>,
            Visible::<Impl, OFFSET>,
            Show::<Impl, OFFSET>,
            Hide::<Impl, OFFSET>,
            UserControl::<Impl, OFFSET>,
            SetUserControl::<Impl, OFFSET>,
            VersionMajor::<Impl, OFFSET>,
            VersionMinor::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _EventConnectorImpl: Sized + IDispatchImpl {
    fn ConnectTo();
    fn Disconnect();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _EventConnector {
    const NAME: &'static str = "Windows.Win32.System.Mmc._EventConnector";
}
#[cfg(feature = "Win32_System_Com")]
impl _EventConnectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _EventConnectorImpl, const OFFSET: isize>() -> _EventConnectorVtbl {
        unsafe extern "system" fn ConnectTo<Impl: _EventConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectTo(&*(&application as *const <_Application as ::windows::core::Abi>::Abi as *const <_Application as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: _EventConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_EventConnector>, ::windows::core::GetTrustLevel, ConnectTo::<Impl, OFFSET>, Disconnect::<Impl, OFFSET>)
    }
}
