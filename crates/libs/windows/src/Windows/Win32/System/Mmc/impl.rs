#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait AppEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl AppEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AppEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AppEventsVtbl {
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AppEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ColumnImpl: Sized + IDispatchImpl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Width(&mut self) -> ::windows::core::Result<i32>;
    fn SetWidth(&mut self, width: i32) -> ::windows::core::Result<()>;
    fn DisplayPosition(&mut self) -> ::windows::core::Result<i32>;
    fn SetDisplayPosition(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Hidden(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetHidden(&mut self, hidden: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetAsSortColumn(&mut self, sortorder: _ColumnSortOrder) -> ::windows::core::Result<()>;
    fn IsSortColumn(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ColumnVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ColumnImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ColumnVtbl {
        unsafe extern "system" fn Name<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *width = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWidth(::core::mem::transmute_copy(&width)).into()
        }
        unsafe extern "system" fn DisplayPosition<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *displayposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayPosition<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayPosition(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Hidden<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hidden: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hidden() {
                ::core::result::Result::Ok(ok__) => {
                    *hidden = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHidden<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hidden: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHidden(::core::mem::transmute_copy(&hidden)).into()
        }
        unsafe extern "system" fn SetAsSortColumn<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sortorder: _ColumnSortOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAsSortColumn(::core::mem::transmute_copy(&sortorder)).into()
        }
        unsafe extern "system" fn IsSortColumn<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issortcolumn: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSortColumn() {
                ::core::result::Result::Ok(ok__) => {
                    *issortcolumn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Width: Width::<Impl, IMPL_OFFSET>,
            SetWidth: SetWidth::<Impl, IMPL_OFFSET>,
            DisplayPosition: DisplayPosition::<Impl, IMPL_OFFSET>,
            SetDisplayPosition: SetDisplayPosition::<Impl, IMPL_OFFSET>,
            Hidden: Hidden::<Impl, IMPL_OFFSET>,
            SetHidden: SetHidden::<Impl, IMPL_OFFSET>,
            SetAsSortColumn: SetAsSortColumn::<Impl, IMPL_OFFSET>,
            IsSortColumn: IsSortColumn::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Column as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ColumnsImpl: Sized + IDispatchImpl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<Column>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ColumnsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ColumnsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ColumnsVtbl {
        unsafe extern "system" fn Item<Impl: ColumnsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, column: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *column = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ColumnsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ColumnsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Columns as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ContextMenuImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, indexorpath: super::Com::VARIANT) -> ::windows::core::Result<MenuItem>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ContextMenuVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ContextMenuImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ContextMenuVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ContextMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ContextMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexorpath: ::core::mem::ManuallyDrop<super::Com::VARIANT>, menuitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&indexorpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *menuitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ContextMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ContextMenu as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DocumentImpl: Sized + IDispatchImpl {
    fn Save(&mut self) -> ::windows::core::Result<()>;
    fn SaveAs(&mut self, filename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Close(&mut self, savechanges: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Views(&mut self) -> ::windows::core::Result<Views>;
    fn SnapIns(&mut self) -> ::windows::core::Result<SnapIns>;
    fn ActiveView(&mut self) -> ::windows::core::Result<View>;
    fn Name(&mut self) -> ::windows::core::Result<*mut u16>;
    fn SetName(&mut self, name: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Location(&mut self) -> ::windows::core::Result<*mut u16>;
    fn IsSaved(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Mode(&mut self) -> ::windows::core::Result<_DocumentMode>;
    fn SetMode(&mut self, mode: _DocumentMode) -> ::windows::core::Result<()>;
    fn RootNode(&mut self) -> ::windows::core::Result<Node>;
    fn ScopeNamespace(&mut self) -> ::windows::core::Result<ScopeNamespace>;
    fn CreateProperties(&mut self) -> ::windows::core::Result<Properties>;
    fn Application(&mut self) -> ::windows::core::Result<_Application>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DocumentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DocumentVtbl {
        unsafe extern "system" fn Save<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        unsafe extern "system" fn SaveAs<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveAs(::core::mem::transmute_copy(&filename)).into()
        }
        unsafe extern "system" fn Close<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, savechanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close(::core::mem::transmute_copy(&savechanges)).into()
        }
        unsafe extern "system" fn Views<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, views: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Views() {
                ::core::result::Result::Ok(ok__) => {
                    *views = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnapIns<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapins: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SnapIns() {
                ::core::result::Result::Ok(ok__) => {
                    *snapins = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveView<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    *view = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Location<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *location = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSaved<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issaved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSaved() {
                ::core::result::Result::Ok(ok__) => {
                    *issaved = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut _DocumentMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *mode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: _DocumentMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn RootNode<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RootNode() {
                ::core::result::Result::Ok(ok__) => {
                    *node = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScopeNamespace<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScopeNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    *scopenamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProperties<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *properties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Application<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Application() {
                ::core::result::Result::Ok(ok__) => {
                    *application = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Save: Save::<Impl, IMPL_OFFSET>,
            SaveAs: SaveAs::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            Views: Views::<Impl, IMPL_OFFSET>,
            SnapIns: SnapIns::<Impl, IMPL_OFFSET>,
            ActiveView: ActiveView::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            IsSaved: IsSaved::<Impl, IMPL_OFFSET>,
            Mode: Mode::<Impl, IMPL_OFFSET>,
            SetMode: SetMode::<Impl, IMPL_OFFSET>,
            RootNode: RootNode::<Impl, IMPL_OFFSET>,
            ScopeNamespace: ScopeNamespace::<Impl, IMPL_OFFSET>,
            CreateProperties: CreateProperties::<Impl, IMPL_OFFSET>,
            Application: Application::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Document as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ExtensionImpl: Sized + IDispatchImpl {
    fn Name(&mut self) -> ::windows::core::Result<*mut u16>;
    fn Vendor(&mut self) -> ::windows::core::Result<*mut u16>;
    fn Version(&mut self) -> ::windows::core::Result<*mut u16>;
    fn Extensions(&mut self) -> ::windows::core::Result<Extensions>;
    fn SnapinCLSID(&mut self) -> ::windows::core::Result<*mut u16>;
    fn EnableAllExtensions(&mut self, enable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Enable(&mut self, enable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ExtensionVtbl {
        unsafe extern "system" fn Name<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Vendor<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendor: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Vendor() {
                ::core::result::Result::Ok(ok__) => {
                    *vendor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *version = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extensions<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extensions() {
                ::core::result::Result::Ok(ok__) => {
                    *extensions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnapinCLSID<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapinclsid: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SnapinCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *snapinclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAllExtensions<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableAllExtensions(::core::mem::transmute_copy(&enable)).into()
        }
        unsafe extern "system" fn Enable<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable(::core::mem::transmute_copy(&enable)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Vendor: Vendor::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            Extensions: Extensions::<Impl, IMPL_OFFSET>,
            SnapinCLSID: SnapinCLSID::<Impl, IMPL_OFFSET>,
            EnableAllExtensions: EnableAllExtensions::<Impl, IMPL_OFFSET>,
            Enable: Enable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Extension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ExtensionsImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<Extension>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ExtensionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ExtensionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ExtensionsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, extension: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *extension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Extensions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait FrameImpl: Sized + IDispatchImpl {
    fn Maximize(&mut self) -> ::windows::core::Result<()>;
    fn Minimize(&mut self) -> ::windows::core::Result<()>;
    fn Restore(&mut self) -> ::windows::core::Result<()>;
    fn Top(&mut self) -> ::windows::core::Result<i32>;
    fn SetTop(&mut self, top: i32) -> ::windows::core::Result<()>;
    fn Bottom(&mut self) -> ::windows::core::Result<i32>;
    fn SetBottom(&mut self, bottom: i32) -> ::windows::core::Result<()>;
    fn Left(&mut self) -> ::windows::core::Result<i32>;
    fn SetLeft(&mut self, left: i32) -> ::windows::core::Result<()>;
    fn Right(&mut self) -> ::windows::core::Result<i32>;
    fn SetRight(&mut self, right: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl FrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: FrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> FrameVtbl {
        unsafe extern "system" fn Maximize<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Maximize().into()
        }
        unsafe extern "system" fn Minimize<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Minimize().into()
        }
        unsafe extern "system" fn Restore<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Restore().into()
        }
        unsafe extern "system" fn Top<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Top() {
                ::core::result::Result::Ok(ok__) => {
                    *top = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTop<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTop(::core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn Bottom<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bottom() {
                ::core::result::Result::Ok(ok__) => {
                    *bottom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottom<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottom(::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn Left<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Left() {
                ::core::result::Result::Ok(ok__) => {
                    *left = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeft<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeft(::core::mem::transmute_copy(&left)).into()
        }
        unsafe extern "system" fn Right<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Right() {
                ::core::result::Result::Ok(ok__) => {
                    *right = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRight<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRight(::core::mem::transmute_copy(&right)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Maximize: Maximize::<Impl, IMPL_OFFSET>,
            Minimize: Minimize::<Impl, IMPL_OFFSET>,
            Restore: Restore::<Impl, IMPL_OFFSET>,
            Top: Top::<Impl, IMPL_OFFSET>,
            SetTop: SetTop::<Impl, IMPL_OFFSET>,
            Bottom: Bottom::<Impl, IMPL_OFFSET>,
            SetBottom: SetBottom::<Impl, IMPL_OFFSET>,
            Left: Left::<Impl, IMPL_OFFSET>,
            SetLeft: SetLeft::<Impl, IMPL_OFFSET>,
            Right: Right::<Impl, IMPL_OFFSET>,
            SetRight: SetRight::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Frame as ::windows::core::Interface>::IID
    }
}
pub trait IColumnDataImpl: Sized {
    fn SetColumnConfigData(&mut self, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> ::windows::core::Result<()>;
    fn GetColumnConfigData(&mut self, pcolid: *const SColumnSetID) -> ::windows::core::Result<*mut MMC_COLUMN_SET_DATA>;
    fn SetColumnSortData(&mut self, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> ::windows::core::Result<()>;
    fn GetColumnSortData(&mut self, pcolid: *const SColumnSetID) -> ::windows::core::Result<*mut MMC_SORT_SET_DATA>;
}
impl IColumnDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColumnDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColumnDataVtbl {
        unsafe extern "system" fn SetColumnConfigData<Impl: IColumnDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColumnConfigData(::core::mem::transmute_copy(&pcolid), ::core::mem::transmute_copy(&pcolsetdata)).into()
        }
        unsafe extern "system" fn GetColumnConfigData<Impl: IColumnDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, ppcolsetdata: *mut *mut MMC_COLUMN_SET_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnConfigData(::core::mem::transmute_copy(&pcolid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolsetdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColumnSortData<Impl: IColumnDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColumnSortData(::core::mem::transmute_copy(&pcolid), ::core::mem::transmute_copy(&pcolsortdata)).into()
        }
        unsafe extern "system" fn GetColumnSortData<Impl: IColumnDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, ppcolsortdata: *mut *mut MMC_SORT_SET_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnSortData(::core::mem::transmute_copy(&pcolid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolsortdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetColumnConfigData: SetColumnConfigData::<Impl, IMPL_OFFSET>,
            GetColumnConfigData: GetColumnConfigData::<Impl, IMPL_OFFSET>,
            SetColumnSortData: SetColumnSortData::<Impl, IMPL_OFFSET>,
            GetColumnSortData: GetColumnSortData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColumnData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IComponentImpl: Sized {
    fn Initialize(&mut self, lpconsole: ::core::option::Option<IConsole>) -> ::windows::core::Result<()>;
    fn Notify(&mut self, lpdataobject: ::core::option::Option<super::Com::IDataObject>, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn Destroy(&mut self, cookie: isize) -> ::windows::core::Result<()>;
    fn QueryDataObject(&mut self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows::core::Result<super::Com::IDataObject>;
    fn GetResultViewType(&mut self, cookie: isize, ppviewtype: *mut super::super::Foundation::PWSTR, pviewoptions: *mut i32) -> ::windows::core::Result<()>;
    fn GetDisplayInfo(&mut self, presultdataitem: *mut RESULTDATAITEM) -> ::windows::core::Result<()>;
    fn CompareObjects(&mut self, lpdataobjecta: ::core::option::Option<super::Com::IDataObject>, lpdataobjectb: ::core::option::Option<super::Com::IDataObject>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComponentVtbl {
        unsafe extern "system" fn Initialize<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpconsole: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&lpconsole)).into()
        }
        unsafe extern "system" fn Notify<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Notify(::core::mem::transmute(&lpdataobject), ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&arg), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn Destroy<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Destroy(::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn QueryDataObject<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDataObject(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdataobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResultViewType<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, ppviewtype: *mut super::super::Foundation::PWSTR, pviewoptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResultViewType(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&ppviewtype), ::core::mem::transmute_copy(&pviewoptions)).into()
        }
        unsafe extern "system" fn GetDisplayInfo<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presultdataitem: *mut RESULTDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayInfo(::core::mem::transmute_copy(&presultdataitem)).into()
        }
        unsafe extern "system" fn CompareObjects<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobjecta: ::windows::core::RawPtr, lpdataobjectb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompareObjects(::core::mem::transmute(&lpdataobjecta), ::core::mem::transmute(&lpdataobjectb)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Notify: Notify::<Impl, IMPL_OFFSET>,
            Destroy: Destroy::<Impl, IMPL_OFFSET>,
            QueryDataObject: QueryDataObject::<Impl, IMPL_OFFSET>,
            GetResultViewType: GetResultViewType::<Impl, IMPL_OFFSET>,
            GetDisplayInfo: GetDisplayInfo::<Impl, IMPL_OFFSET>,
            CompareObjects: CompareObjects::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IComponent2Impl: Sized + IComponentImpl {
    fn QueryDispatch(&mut self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows::core::Result<super::Com::IDispatch>;
    fn GetResultViewType2(&mut self, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> ::windows::core::Result<()>;
    fn RestoreResultView(&mut self, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IComponent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComponent2Vtbl {
        unsafe extern "system" fn QueryDispatch<Impl: IComponent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDispatch(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdispatch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResultViewType2<Impl: IComponent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResultViewType2(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&presultviewtype)).into()
        }
        unsafe extern "system" fn RestoreResultView<Impl: IComponent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreResultView(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&presultviewtype)).into()
        }
        Self {
            base: IComponentVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            QueryDispatch: QueryDispatch::<Impl, IMPL_OFFSET>,
            GetResultViewType2: GetResultViewType2::<Impl, IMPL_OFFSET>,
            RestoreResultView: RestoreResultView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IComponentDataImpl: Sized {
    fn Initialize(&mut self, punknown: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CreateComponent(&mut self) -> ::windows::core::Result<IComponent>;
    fn Notify(&mut self, lpdataobject: ::core::option::Option<super::Com::IDataObject>, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn Destroy(&mut self) -> ::windows::core::Result<()>;
    fn QueryDataObject(&mut self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows::core::Result<super::Com::IDataObject>;
    fn GetDisplayInfo(&mut self, pscopedataitem: *mut SCOPEDATAITEM) -> ::windows::core::Result<()>;
    fn CompareObjects(&mut self, lpdataobjecta: ::core::option::Option<super::Com::IDataObject>, lpdataobjectb: ::core::option::Option<super::Com::IDataObject>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IComponentDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComponentDataVtbl {
        unsafe extern "system" fn Initialize<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&punknown)).into()
        }
        unsafe extern "system" fn CreateComponent<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateComponent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomponent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notify<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Notify(::core::mem::transmute(&lpdataobject), ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&arg), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn Destroy<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Destroy().into()
        }
        unsafe extern "system" fn QueryDataObject<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDataObject(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdataobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayInfo<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscopedataitem: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayInfo(::core::mem::transmute_copy(&pscopedataitem)).into()
        }
        unsafe extern "system" fn CompareObjects<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobjecta: ::windows::core::RawPtr, lpdataobjectb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompareObjects(::core::mem::transmute(&lpdataobjecta), ::core::mem::transmute(&lpdataobjectb)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            CreateComponent: CreateComponent::<Impl, IMPL_OFFSET>,
            Notify: Notify::<Impl, IMPL_OFFSET>,
            Destroy: Destroy::<Impl, IMPL_OFFSET>,
            QueryDataObject: QueryDataObject::<Impl, IMPL_OFFSET>,
            GetDisplayInfo: GetDisplayInfo::<Impl, IMPL_OFFSET>,
            CompareObjects: CompareObjects::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponentData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IComponentData2Impl: Sized + IComponentDataImpl {
    fn QueryDispatch(&mut self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IComponentData2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentData2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComponentData2Vtbl {
        unsafe extern "system" fn QueryDispatch<Impl: IComponentData2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDispatch(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdispatch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IComponentDataVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), QueryDispatch: QueryDispatch::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponentData2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IConsoleImpl: Sized {
    fn SetHeader(&mut self, pheader: ::core::option::Option<IHeaderCtrl>) -> ::windows::core::Result<()>;
    fn SetToolbar(&mut self, ptoolbar: ::core::option::Option<IToolbar>) -> ::windows::core::Result<()>;
    fn QueryResultView(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn QueryScopeImageList(&mut self) -> ::windows::core::Result<IImageList>;
    fn QueryResultImageList(&mut self) -> ::windows::core::Result<IImageList>;
    fn UpdateAllViews(&mut self, lpdataobject: ::core::option::Option<super::Com::IDataObject>, data: super::super::Foundation::LPARAM, hint: isize) -> ::windows::core::Result<()>;
    fn MessageBox(&mut self, lpsztext: super::super::Foundation::PWSTR, lpsztitle: super::super::Foundation::PWSTR, fustyle: u32) -> ::windows::core::Result<i32>;
    fn QueryConsoleVerb(&mut self) -> ::windows::core::Result<IConsoleVerb>;
    fn SelectScopeItem(&mut self, hscopeitem: isize) -> ::windows::core::Result<()>;
    fn GetMainWindow(&mut self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn NewWindow(&mut self, hscopeitem: isize, loptions: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IConsoleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsoleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConsoleVtbl {
        unsafe extern "system" fn SetHeader<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeader(::core::mem::transmute(&pheader)).into()
        }
        unsafe extern "system" fn SetToolbar<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoolbar: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToolbar(::core::mem::transmute(&ptoolbar)).into()
        }
        unsafe extern "system" fn QueryResultView<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryResultView() {
                ::core::result::Result::Ok(ok__) => {
                    *punknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryScopeImageList<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimagelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryScopeImageList() {
                ::core::result::Result::Ok(ok__) => {
                    *ppimagelist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryResultImageList<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimagelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryResultImageList() {
                ::core::result::Result::Ok(ok__) => {
                    *ppimagelist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAllViews<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr, data: super::super::Foundation::LPARAM, hint: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateAllViews(::core::mem::transmute(&lpdataobject), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&hint)).into()
        }
        unsafe extern "system" fn MessageBox<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpsztext: super::super::Foundation::PWSTR, lpsztitle: super::super::Foundation::PWSTR, fustyle: u32, piretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageBox(::core::mem::transmute_copy(&lpsztext), ::core::mem::transmute_copy(&lpsztitle), ::core::mem::transmute_copy(&fustyle)) {
                ::core::result::Result::Ok(ok__) => {
                    *piretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryConsoleVerb<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconsoleverb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryConsoleVerb() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconsoleverb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectScopeItem<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hscopeitem: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectScopeItem(::core::mem::transmute_copy(&hscopeitem)).into()
        }
        unsafe extern "system" fn GetMainWindow<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMainWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewWindow<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hscopeitem: isize, loptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NewWindow(::core::mem::transmute_copy(&hscopeitem), ::core::mem::transmute_copy(&loptions)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetHeader: SetHeader::<Impl, IMPL_OFFSET>,
            SetToolbar: SetToolbar::<Impl, IMPL_OFFSET>,
            QueryResultView: QueryResultView::<Impl, IMPL_OFFSET>,
            QueryScopeImageList: QueryScopeImageList::<Impl, IMPL_OFFSET>,
            QueryResultImageList: QueryResultImageList::<Impl, IMPL_OFFSET>,
            UpdateAllViews: UpdateAllViews::<Impl, IMPL_OFFSET>,
            MessageBox: MessageBox::<Impl, IMPL_OFFSET>,
            QueryConsoleVerb: QueryConsoleVerb::<Impl, IMPL_OFFSET>,
            SelectScopeItem: SelectScopeItem::<Impl, IMPL_OFFSET>,
            GetMainWindow: GetMainWindow::<Impl, IMPL_OFFSET>,
            NewWindow: NewWindow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConsole as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IConsole2Impl: Sized + IConsoleImpl {
    fn Expand(&mut self, hitem: isize, bexpand: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsTaskpadViewPreferred(&mut self) -> ::windows::core::Result<()>;
    fn SetStatusText(&mut self, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IConsole2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsole2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConsole2Vtbl {
        unsafe extern "system" fn Expand<Impl: IConsole2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize, bexpand: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Expand(::core::mem::transmute_copy(&hitem), ::core::mem::transmute_copy(&bexpand)).into()
        }
        unsafe extern "system" fn IsTaskpadViewPreferred<Impl: IConsole2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsTaskpadViewPreferred().into()
        }
        unsafe extern "system" fn SetStatusText<Impl: IConsole2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatusText(::core::mem::transmute_copy(&pszstatustext)).into()
        }
        Self {
            base: IConsoleVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Expand: Expand::<Impl, IMPL_OFFSET>,
            IsTaskpadViewPreferred: IsTaskpadViewPreferred::<Impl, IMPL_OFFSET>,
            SetStatusText: SetStatusText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConsole2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IConsole3Impl: Sized + IConsoleImpl + IConsole2Impl {
    fn RenameScopeItem(&mut self, hscopeitem: isize) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IConsole3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsole3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConsole3Vtbl {
        unsafe extern "system" fn RenameScopeItem<Impl: IConsole3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hscopeitem: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RenameScopeItem(::core::mem::transmute_copy(&hscopeitem)).into()
        }
        Self { base: IConsole2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), RenameScopeItem: RenameScopeItem::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConsole3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IConsoleNameSpaceImpl: Sized {
    fn InsertItem(&mut self, item: *mut SCOPEDATAITEM) -> ::windows::core::Result<()>;
    fn DeleteItem(&mut self, hitem: isize, fdeletethis: i32) -> ::windows::core::Result<()>;
    fn SetItem(&mut self, item: *const SCOPEDATAITEM) -> ::windows::core::Result<()>;
    fn GetItem(&mut self, item: *mut SCOPEDATAITEM) -> ::windows::core::Result<()>;
    fn GetChildItem(&mut self, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> ::windows::core::Result<()>;
    fn GetNextItem(&mut self, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> ::windows::core::Result<()>;
    fn GetParentItem(&mut self, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IConsoleNameSpaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsoleNameSpaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConsoleNameSpaceVtbl {
        unsafe extern "system" fn InsertItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertItem(::core::mem::transmute_copy(&item)).into()
        }
        unsafe extern "system" fn DeleteItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize, fdeletethis: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteItem(::core::mem::transmute_copy(&hitem), ::core::mem::transmute_copy(&fdeletethis)).into()
        }
        unsafe extern "system" fn SetItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *const SCOPEDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItem(::core::mem::transmute_copy(&item)).into()
        }
        unsafe extern "system" fn GetItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetItem(::core::mem::transmute_copy(&item)).into()
        }
        unsafe extern "system" fn GetChildItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChildItem(::core::mem::transmute_copy(&item), ::core::mem::transmute_copy(&pitemchild), ::core::mem::transmute_copy(&pcookie)).into()
        }
        unsafe extern "system" fn GetNextItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNextItem(::core::mem::transmute_copy(&item), ::core::mem::transmute_copy(&pitemnext), ::core::mem::transmute_copy(&pcookie)).into()
        }
        unsafe extern "system" fn GetParentItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetParentItem(::core::mem::transmute_copy(&item), ::core::mem::transmute_copy(&pitemparent), ::core::mem::transmute_copy(&pcookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InsertItem: InsertItem::<Impl, IMPL_OFFSET>,
            DeleteItem: DeleteItem::<Impl, IMPL_OFFSET>,
            SetItem: SetItem::<Impl, IMPL_OFFSET>,
            GetItem: GetItem::<Impl, IMPL_OFFSET>,
            GetChildItem: GetChildItem::<Impl, IMPL_OFFSET>,
            GetNextItem: GetNextItem::<Impl, IMPL_OFFSET>,
            GetParentItem: GetParentItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConsoleNameSpace as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IConsoleNameSpace2Impl: Sized + IConsoleNameSpaceImpl {
    fn Expand(&mut self, hitem: isize) -> ::windows::core::Result<()>;
    fn AddExtension(&mut self, hitem: isize, lpclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IConsoleNameSpace2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsoleNameSpace2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConsoleNameSpace2Vtbl {
        unsafe extern "system" fn Expand<Impl: IConsoleNameSpace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Expand(::core::mem::transmute_copy(&hitem)).into()
        }
        unsafe extern "system" fn AddExtension<Impl: IConsoleNameSpace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize, lpclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddExtension(::core::mem::transmute_copy(&hitem), ::core::mem::transmute_copy(&lpclsid)).into()
        }
        Self {
            base: IConsoleNameSpaceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Expand: Expand::<Impl, IMPL_OFFSET>,
            AddExtension: AddExtension::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConsoleNameSpace2 as ::windows::core::Interface>::IID
    }
}
pub trait IConsolePowerImpl: Sized {
    fn SetExecutionState(&mut self, dwadd: u32, dwremove: u32) -> ::windows::core::Result<()>;
    fn ResetIdleTimer(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
}
impl IConsolePowerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsolePowerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConsolePowerVtbl {
        unsafe extern "system" fn SetExecutionState<Impl: IConsolePowerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwadd: u32, dwremove: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExecutionState(::core::mem::transmute_copy(&dwadd), ::core::mem::transmute_copy(&dwremove)).into()
        }
        unsafe extern "system" fn ResetIdleTimer<Impl: IConsolePowerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetIdleTimer(::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetExecutionState: SetExecutionState::<Impl, IMPL_OFFSET>,
            ResetIdleTimer: ResetIdleTimer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConsolePower as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IConsolePowerSinkImpl: Sized {
    fn OnPowerBroadcast(&mut self, nevent: u32, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::LRESULT>;
}
#[cfg(feature = "Win32_Foundation")]
impl IConsolePowerSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsolePowerSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConsolePowerSinkVtbl {
        unsafe extern "system" fn OnPowerBroadcast<Impl: IConsolePowerSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nevent: u32, lparam: super::super::Foundation::LPARAM, plreturn: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnPowerBroadcast(::core::mem::transmute_copy(&nevent), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *plreturn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnPowerBroadcast: OnPowerBroadcast::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConsolePowerSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IConsoleVerbImpl: Sized {
    fn GetVerbState(&mut self, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetVerbState(&mut self, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetDefaultVerb(&mut self, ecmdid: MMC_CONSOLE_VERB) -> ::windows::core::Result<()>;
    fn GetDefaultVerb(&mut self) -> ::windows::core::Result<MMC_CONSOLE_VERB>;
}
#[cfg(feature = "Win32_Foundation")]
impl IConsoleVerbVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsoleVerbImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConsoleVerbVtbl {
        unsafe extern "system" fn GetVerbState<Impl: IConsoleVerbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVerbState(::core::mem::transmute_copy(&ecmdid), ::core::mem::transmute_copy(&nstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerbState<Impl: IConsoleVerbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVerbState(::core::mem::transmute_copy(&ecmdid), ::core::mem::transmute_copy(&nstate), ::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn SetDefaultVerb<Impl: IConsoleVerbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultVerb(::core::mem::transmute_copy(&ecmdid)).into()
        }
        unsafe extern "system" fn GetDefaultVerb<Impl: IConsoleVerbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pecmdid: *mut MMC_CONSOLE_VERB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultVerb() {
                ::core::result::Result::Ok(ok__) => {
                    *pecmdid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetVerbState: GetVerbState::<Impl, IMPL_OFFSET>,
            SetVerbState: SetVerbState::<Impl, IMPL_OFFSET>,
            SetDefaultVerb: SetDefaultVerb::<Impl, IMPL_OFFSET>,
            GetDefaultVerb: GetDefaultVerb::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConsoleVerb as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContextMenuCallbackImpl: Sized {
    fn AddItem(&mut self, pitem: *const CONTEXTMENUITEM) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContextMenuCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextMenuCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContextMenuCallbackVtbl {
        unsafe extern "system" fn AddItem<Impl: IContextMenuCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *const CONTEXTMENUITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddItem(::core::mem::transmute_copy(&pitem)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddItem: AddItem::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextMenuCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContextMenuCallback2Impl: Sized {
    fn AddItem(&mut self, pitem: *const CONTEXTMENUITEM2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContextMenuCallback2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextMenuCallback2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContextMenuCallback2Vtbl {
        unsafe extern "system" fn AddItem<Impl: IContextMenuCallback2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *const CONTEXTMENUITEM2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddItem(::core::mem::transmute_copy(&pitem)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddItem: AddItem::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextMenuCallback2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IContextMenuProviderImpl: Sized + IContextMenuCallbackImpl {
    fn EmptyMenuList(&mut self) -> ::windows::core::Result<()>;
    fn AddPrimaryExtensionItems(&mut self, piextension: ::core::option::Option<::windows::core::IUnknown>, pidataobject: ::core::option::Option<super::Com::IDataObject>) -> ::windows::core::Result<()>;
    fn AddThirdPartyExtensionItems(&mut self, pidataobject: ::core::option::Option<super::Com::IDataObject>) -> ::windows::core::Result<()>;
    fn ShowContextMenu(&mut self, hwndparent: super::super::Foundation::HWND, xpos: i32, ypos: i32) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IContextMenuProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextMenuProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContextMenuProviderVtbl {
        unsafe extern "system" fn EmptyMenuList<Impl: IContextMenuProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EmptyMenuList().into()
        }
        unsafe extern "system" fn AddPrimaryExtensionItems<Impl: IContextMenuProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piextension: *mut ::core::ffi::c_void, pidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPrimaryExtensionItems(::core::mem::transmute(&piextension), ::core::mem::transmute(&pidataobject)).into()
        }
        unsafe extern "system" fn AddThirdPartyExtensionItems<Impl: IContextMenuProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddThirdPartyExtensionItems(::core::mem::transmute(&pidataobject)).into()
        }
        unsafe extern "system" fn ShowContextMenu<Impl: IContextMenuProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, xpos: i32, ypos: i32, plselected: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowContextMenu(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&xpos), ::core::mem::transmute_copy(&ypos)) {
                ::core::result::Result::Ok(ok__) => {
                    *plselected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IContextMenuCallbackVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EmptyMenuList: EmptyMenuList::<Impl, IMPL_OFFSET>,
            AddPrimaryExtensionItems: AddPrimaryExtensionItems::<Impl, IMPL_OFFSET>,
            AddThirdPartyExtensionItems: AddThirdPartyExtensionItems::<Impl, IMPL_OFFSET>,
            ShowContextMenu: ShowContextMenu::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextMenuProvider as ::windows::core::Interface>::IID
    }
}
pub trait IControlbarImpl: Sized {
    fn Create(&mut self, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: ::core::option::Option<IExtendControlbar>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Attach(&mut self, ntype: MMC_CONTROL_TYPE, lpunknown: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Detach(&mut self, lpunknown: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IControlbarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlbarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IControlbarVtbl {
        unsafe extern "system" fn Create<Impl: IControlbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: ::windows::core::RawPtr, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&ntype), ::core::mem::transmute(&pextendcontrolbar)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attach<Impl: IControlbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntype: MMC_CONTROL_TYPE, lpunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Attach(::core::mem::transmute_copy(&ntype), ::core::mem::transmute(&lpunknown)).into()
        }
        unsafe extern "system" fn Detach<Impl: IControlbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Detach(::core::mem::transmute(&lpunknown)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Attach: Attach::<Impl, IMPL_OFFSET>,
            Detach: Detach::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlbar as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDisplayHelpImpl: Sized {
    fn ShowTopic(&mut self, pszhelptopic: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDisplayHelpVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayHelpImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayHelpVtbl {
        unsafe extern "system" fn ShowTopic<Impl: IDisplayHelpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelptopic: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowTopic(::core::mem::transmute_copy(&pszhelptopic)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ShowTopic: ShowTopic::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayHelp as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumTASKImpl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut MMC_TASK, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTASK>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumTASKVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTASKImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTASKVtbl {
        unsafe extern "system" fn Next<Impl: IEnumTASKImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut MMC_TASK, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTASKImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTASKImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumTASKImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTASK as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IExtendContextMenuImpl: Sized {
    fn AddMenuItems(&mut self, pidataobject: ::core::option::Option<super::Com::IDataObject>, picallback: ::core::option::Option<IContextMenuCallback>, pinsertionallowed: *mut i32) -> ::windows::core::Result<()>;
    fn Command(&mut self, lcommandid: i32, pidataobject: ::core::option::Option<super::Com::IDataObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IExtendContextMenuVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendContextMenuImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExtendContextMenuVtbl {
        unsafe extern "system" fn AddMenuItems<Impl: IExtendContextMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidataobject: ::windows::core::RawPtr, picallback: ::windows::core::RawPtr, pinsertionallowed: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddMenuItems(::core::mem::transmute(&pidataobject), ::core::mem::transmute(&picallback), ::core::mem::transmute_copy(&pinsertionallowed)).into()
        }
        unsafe extern "system" fn Command<Impl: IExtendContextMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcommandid: i32, pidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Command(::core::mem::transmute_copy(&lcommandid), ::core::mem::transmute(&pidataobject)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddMenuItems: AddMenuItems::<Impl, IMPL_OFFSET>,
            Command: Command::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExtendContextMenu as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IExtendControlbarImpl: Sized {
    fn SetControlbar(&mut self, pcontrolbar: ::core::option::Option<IControlbar>) -> ::windows::core::Result<()>;
    fn ControlbarNotify(&mut self, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IExtendControlbarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendControlbarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExtendControlbarVtbl {
        unsafe extern "system" fn SetControlbar<Impl: IExtendControlbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontrolbar: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControlbar(::core::mem::transmute(&pcontrolbar)).into()
        }
        unsafe extern "system" fn ControlbarNotify<Impl: IExtendControlbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ControlbarNotify(::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&arg), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetControlbar: SetControlbar::<Impl, IMPL_OFFSET>,
            ControlbarNotify: ControlbarNotify::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExtendControlbar as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IExtendPropertySheetImpl: Sized {
    fn CreatePropertyPages(&mut self, lpprovider: ::core::option::Option<IPropertySheetCallback>, handle: isize, lpidataobject: ::core::option::Option<super::Com::IDataObject>) -> ::windows::core::Result<()>;
    fn QueryPagesFor(&mut self, lpdataobject: ::core::option::Option<super::Com::IDataObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IExtendPropertySheetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendPropertySheetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExtendPropertySheetVtbl {
        unsafe extern "system" fn CreatePropertyPages<Impl: IExtendPropertySheetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpprovider: ::windows::core::RawPtr, handle: isize, lpidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePropertyPages(::core::mem::transmute(&lpprovider), ::core::mem::transmute_copy(&handle), ::core::mem::transmute(&lpidataobject)).into()
        }
        unsafe extern "system" fn QueryPagesFor<Impl: IExtendPropertySheetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryPagesFor(::core::mem::transmute(&lpdataobject)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreatePropertyPages: CreatePropertyPages::<Impl, IMPL_OFFSET>,
            QueryPagesFor: QueryPagesFor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExtendPropertySheet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IExtendPropertySheet2Impl: Sized + IExtendPropertySheetImpl {
    fn GetWatermarks(&mut self, lpidataobject: ::core::option::Option<super::Com::IDataObject>, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IExtendPropertySheet2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendPropertySheet2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExtendPropertySheet2Vtbl {
        unsafe extern "system" fn GetWatermarks<Impl: IExtendPropertySheet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpidataobject: ::windows::core::RawPtr, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWatermarks(::core::mem::transmute(&lpidataobject), ::core::mem::transmute_copy(&lphwatermark), ::core::mem::transmute_copy(&lphheader), ::core::mem::transmute_copy(&lphpalette), ::core::mem::transmute_copy(&bstretch)).into()
        }
        Self { base: IExtendPropertySheetVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetWatermarks: GetWatermarks::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExtendPropertySheet2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IExtendTaskPadImpl: Sized {
    fn TaskNotify(&mut self, pdo: ::core::option::Option<super::Com::IDataObject>, arg: *const super::Com::VARIANT, param2: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn EnumTasks(&mut self, pdo: ::core::option::Option<super::Com::IDataObject>, sztaskgroup: super::super::Foundation::PWSTR) -> ::windows::core::Result<IEnumTASK>;
    fn GetTitle(&mut self, pszgroup: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetDescriptiveText(&mut self, pszgroup: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetBackground(&mut self, pszgroup: super::super::Foundation::PWSTR) -> ::windows::core::Result<MMC_TASK_DISPLAY_OBJECT>;
    fn GetListPadInfo(&mut self, pszgroup: super::super::Foundation::PWSTR) -> ::windows::core::Result<MMC_LISTPAD_INFO>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IExtendTaskPadVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendTaskPadImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExtendTaskPadVtbl {
        unsafe extern "system" fn TaskNotify<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdo: ::windows::core::RawPtr, arg: *const super::Com::VARIANT, param2: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TaskNotify(::core::mem::transmute(&pdo), ::core::mem::transmute_copy(&arg), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn EnumTasks<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdo: ::windows::core::RawPtr, sztaskgroup: super::super::Foundation::PWSTR, ppenumtask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumTasks(::core::mem::transmute(&pdo), ::core::mem::transmute_copy(&sztaskgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumtask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTitle<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgroup: super::super::Foundation::PWSTR, psztitle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTitle(::core::mem::transmute_copy(&pszgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *psztitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptiveText<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgroup: super::super::Foundation::PWSTR, pszdescriptivetext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescriptiveText(::core::mem::transmute_copy(&pszgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszdescriptivetext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackground<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgroup: super::super::Foundation::PWSTR, ptdo: *mut MMC_TASK_DISPLAY_OBJECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackground(::core::mem::transmute_copy(&pszgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *ptdo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetListPadInfo<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgroup: super::super::Foundation::PWSTR, lplistpadinfo: *mut MMC_LISTPAD_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetListPadInfo(::core::mem::transmute_copy(&pszgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *lplistpadinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            TaskNotify: TaskNotify::<Impl, IMPL_OFFSET>,
            EnumTasks: EnumTasks::<Impl, IMPL_OFFSET>,
            GetTitle: GetTitle::<Impl, IMPL_OFFSET>,
            GetDescriptiveText: GetDescriptiveText::<Impl, IMPL_OFFSET>,
            GetBackground: GetBackground::<Impl, IMPL_OFFSET>,
            GetListPadInfo: GetListPadInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExtendTaskPad as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IExtendViewImpl: Sized {
    fn GetViews(&mut self, pdataobject: ::core::option::Option<super::Com::IDataObject>, pviewextensioncallback: ::core::option::Option<IViewExtensionCallback>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IExtendViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExtendViewVtbl {
        unsafe extern "system" fn GetViews<Impl: IExtendViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, pviewextensioncallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetViews(::core::mem::transmute(&pdataobject), ::core::mem::transmute(&pviewextensioncallback)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetViews: GetViews::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExtendView as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHeaderCtrlImpl: Sized {
    fn InsertColumn(&mut self, ncol: i32, title: super::super::Foundation::PWSTR, nformat: i32, nwidth: i32) -> ::windows::core::Result<()>;
    fn DeleteColumn(&mut self, ncol: i32) -> ::windows::core::Result<()>;
    fn SetColumnText(&mut self, ncol: i32, title: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetColumnText(&mut self, ncol: i32) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetColumnWidth(&mut self, ncol: i32, nwidth: i32) -> ::windows::core::Result<()>;
    fn GetColumnWidth(&mut self, ncol: i32) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IHeaderCtrlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHeaderCtrlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHeaderCtrlVtbl {
        unsafe extern "system" fn InsertColumn<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, title: super::super::Foundation::PWSTR, nformat: i32, nwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertColumn(::core::mem::transmute_copy(&ncol), ::core::mem::transmute_copy(&title), ::core::mem::transmute_copy(&nformat), ::core::mem::transmute_copy(&nwidth)).into()
        }
        unsafe extern "system" fn DeleteColumn<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteColumn(::core::mem::transmute_copy(&ncol)).into()
        }
        unsafe extern "system" fn SetColumnText<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, title: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColumnText(::core::mem::transmute_copy(&ncol), ::core::mem::transmute_copy(&title)).into()
        }
        unsafe extern "system" fn GetColumnText<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, ptext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnText(::core::mem::transmute_copy(&ncol)) {
                ::core::result::Result::Ok(ok__) => {
                    *ptext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColumnWidth<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, nwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColumnWidth(::core::mem::transmute_copy(&ncol), ::core::mem::transmute_copy(&nwidth)).into()
        }
        unsafe extern "system" fn GetColumnWidth<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, pwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnWidth(::core::mem::transmute_copy(&ncol)) {
                ::core::result::Result::Ok(ok__) => {
                    *pwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InsertColumn: InsertColumn::<Impl, IMPL_OFFSET>,
            DeleteColumn: DeleteColumn::<Impl, IMPL_OFFSET>,
            SetColumnText: SetColumnText::<Impl, IMPL_OFFSET>,
            GetColumnText: GetColumnText::<Impl, IMPL_OFFSET>,
            SetColumnWidth: SetColumnWidth::<Impl, IMPL_OFFSET>,
            GetColumnWidth: GetColumnWidth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHeaderCtrl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHeaderCtrl2Impl: Sized + IHeaderCtrlImpl {
    fn SetChangeTimeOut(&mut self, utimeout: u32) -> ::windows::core::Result<()>;
    fn SetColumnFilter(&mut self, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> ::windows::core::Result<()>;
    fn GetColumnFilter(&mut self, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IHeaderCtrl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHeaderCtrl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHeaderCtrl2Vtbl {
        unsafe extern "system" fn SetChangeTimeOut<Impl: IHeaderCtrl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, utimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChangeTimeOut(::core::mem::transmute_copy(&utimeout)).into()
        }
        unsafe extern "system" fn SetColumnFilter<Impl: IHeaderCtrl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColumnFilter(::core::mem::transmute_copy(&ncolumn), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pfilterdata)).into()
        }
        unsafe extern "system" fn GetColumnFilter<Impl: IHeaderCtrl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColumnFilter(::core::mem::transmute_copy(&ncolumn), ::core::mem::transmute_copy(&pdwtype), ::core::mem::transmute_copy(&pfilterdata)).into()
        }
        Self {
            base: IHeaderCtrlVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetChangeTimeOut: SetChangeTimeOut::<Impl, IMPL_OFFSET>,
            SetColumnFilter: SetColumnFilter::<Impl, IMPL_OFFSET>,
            GetColumnFilter: GetColumnFilter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHeaderCtrl2 as ::windows::core::Interface>::IID
    }
}
pub trait IImageListImpl: Sized {
    fn ImageListSetIcon(&mut self, picon: *const isize, nloc: i32) -> ::windows::core::Result<()>;
    fn ImageListSetStrip(&mut self, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: u32) -> ::windows::core::Result<()>;
}
impl IImageListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageListVtbl {
        unsafe extern "system" fn ImageListSetIcon<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picon: *const isize, nloc: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImageListSetIcon(::core::mem::transmute_copy(&picon), ::core::mem::transmute_copy(&nloc)).into()
        }
        unsafe extern "system" fn ImageListSetStrip<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImageListSetStrip(::core::mem::transmute_copy(&pbmapsm), ::core::mem::transmute_copy(&pbmaplg), ::core::mem::transmute_copy(&nstartloc), ::core::mem::transmute_copy(&cmask)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ImageListSetIcon: ImageListSetIcon::<Impl, IMPL_OFFSET>,
            ImageListSetStrip: ImageListSetStrip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageList as ::windows::core::Interface>::IID
    }
}
pub trait IMMCVersionInfoImpl: Sized {
    fn GetMMCVersion(&mut self, pversionmajor: *mut i32, pversionminor: *mut i32) -> ::windows::core::Result<()>;
}
impl IMMCVersionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMMCVersionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMMCVersionInfoVtbl {
        unsafe extern "system" fn GetMMCVersion<Impl: IMMCVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversionmajor: *mut i32, pversionminor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMMCVersion(::core::mem::transmute_copy(&pversionmajor), ::core::mem::transmute_copy(&pversionminor)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetMMCVersion: GetMMCVersion::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMMCVersionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMenuButtonImpl: Sized {
    fn AddButton(&mut self, idcommand: i32, lpbuttontext: super::super::Foundation::PWSTR, lptooltiptext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetButton(&mut self, idcommand: i32, lpbuttontext: super::super::Foundation::PWSTR, lptooltiptext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetButtonState(&mut self, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMenuButtonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuButtonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuButtonVtbl {
        unsafe extern "system" fn AddButton<Impl: IMenuButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, lpbuttontext: super::super::Foundation::PWSTR, lptooltiptext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddButton(::core::mem::transmute_copy(&idcommand), ::core::mem::transmute_copy(&lpbuttontext), ::core::mem::transmute_copy(&lptooltiptext)).into()
        }
        unsafe extern "system" fn SetButton<Impl: IMenuButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, lpbuttontext: super::super::Foundation::PWSTR, lptooltiptext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButton(::core::mem::transmute_copy(&idcommand), ::core::mem::transmute_copy(&lpbuttontext), ::core::mem::transmute_copy(&lptooltiptext)).into()
        }
        unsafe extern "system" fn SetButtonState<Impl: IMenuButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonState(::core::mem::transmute_copy(&idcommand), ::core::mem::transmute_copy(&nstate), ::core::mem::transmute_copy(&bstate)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddButton: AddButton::<Impl, IMPL_OFFSET>,
            SetButton: SetButton::<Impl, IMPL_OFFSET>,
            SetButtonState: SetButtonState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMenuButton as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMessageViewImpl: Sized {
    fn SetTitleText(&mut self, psztitletext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetBodyText(&mut self, pszbodytext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetIcon(&mut self, id: IconIdentifier) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMessageViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageViewVtbl {
        unsafe extern "system" fn SetTitleText<Impl: IMessageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztitletext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitleText(::core::mem::transmute_copy(&psztitletext)).into()
        }
        unsafe extern "system" fn SetBodyText<Impl: IMessageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbodytext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBodyText(::core::mem::transmute_copy(&pszbodytext)).into()
        }
        unsafe extern "system" fn SetIcon<Impl: IMessageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: IconIdentifier) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIcon(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn Clear<Impl: IMessageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetTitleText: SetTitleText::<Impl, IMPL_OFFSET>,
            SetBodyText: SetBodyText::<Impl, IMPL_OFFSET>,
            SetIcon: SetIcon::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait INodePropertiesImpl: Sized {
    fn GetProperty(&mut self, pdataobject: ::core::option::Option<super::Com::IDataObject>, szpropertyname: super::super::Foundation::BSTR) -> ::windows::core::Result<*mut u16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl INodePropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INodePropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INodePropertiesVtbl {
        unsafe extern "system" fn GetProperty<Impl: INodePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, szpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrproperty: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute(&pdataobject), ::core::mem::transmute_copy(&szpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetProperty: GetProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INodeProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Controls")]
pub trait IPropertySheetCallbackImpl: Sized {
    fn AddPage(&mut self, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::Result<()>;
    fn RemovePage(&mut self, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Controls")]
impl IPropertySheetCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySheetCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertySheetCallbackVtbl {
        unsafe extern "system" fn AddPage<Impl: IPropertySheetCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPage(::core::mem::transmute_copy(&hpage)).into()
        }
        unsafe extern "system" fn RemovePage<Impl: IPropertySheetCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePage(::core::mem::transmute_copy(&hpage)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddPage: AddPage::<Impl, IMPL_OFFSET>,
            RemovePage: RemovePage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertySheetCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPropertySheetProviderImpl: Sized {
    fn CreatePropertySheet(&mut self, title: super::super::Foundation::PWSTR, r#type: u8, cookie: isize, pidataobjectm: ::core::option::Option<super::Com::IDataObject>, dwoptions: u32) -> ::windows::core::Result<()>;
    fn FindPropertySheet(&mut self, hitem: isize, lpcomponent: ::core::option::Option<IComponent>, lpdataobject: ::core::option::Option<super::Com::IDataObject>) -> ::windows::core::Result<()>;
    fn AddPrimaryPages(&mut self, lpunknown: ::core::option::Option<::windows::core::IUnknown>, bcreatehandle: super::super::Foundation::BOOL, hnotifywindow: super::super::Foundation::HWND, bscopepane: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn AddExtensionPages(&mut self) -> ::windows::core::Result<()>;
    fn Show(&mut self, window: isize, page: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPropertySheetProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySheetProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertySheetProviderVtbl {
        unsafe extern "system" fn CreatePropertySheet<Impl: IPropertySheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: super::super::Foundation::PWSTR, r#type: u8, cookie: isize, pidataobjectm: ::windows::core::RawPtr, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePropertySheet(::core::mem::transmute_copy(&title), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&cookie), ::core::mem::transmute(&pidataobjectm), ::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn FindPropertySheet<Impl: IPropertySheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize, lpcomponent: ::windows::core::RawPtr, lpdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindPropertySheet(::core::mem::transmute_copy(&hitem), ::core::mem::transmute(&lpcomponent), ::core::mem::transmute(&lpdataobject)).into()
        }
        unsafe extern "system" fn AddPrimaryPages<Impl: IPropertySheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpunknown: *mut ::core::ffi::c_void, bcreatehandle: super::super::Foundation::BOOL, hnotifywindow: super::super::Foundation::HWND, bscopepane: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPrimaryPages(::core::mem::transmute(&lpunknown), ::core::mem::transmute_copy(&bcreatehandle), ::core::mem::transmute_copy(&hnotifywindow), ::core::mem::transmute_copy(&bscopepane)).into()
        }
        unsafe extern "system" fn AddExtensionPages<Impl: IPropertySheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddExtensionPages().into()
        }
        unsafe extern "system" fn Show<Impl: IPropertySheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: isize, page: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&page)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreatePropertySheet: CreatePropertySheet::<Impl, IMPL_OFFSET>,
            FindPropertySheet: FindPropertySheet::<Impl, IMPL_OFFSET>,
            AddPrimaryPages: AddPrimaryPages::<Impl, IMPL_OFFSET>,
            AddExtensionPages: AddExtensionPages::<Impl, IMPL_OFFSET>,
            Show: Show::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertySheetProvider as ::windows::core::Interface>::IID
    }
}
pub trait IRequiredExtensionsImpl: Sized {
    fn EnableAllExtensions(&mut self) -> ::windows::core::Result<()>;
    fn GetFirstExtension(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetNextExtension(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl IRequiredExtensionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRequiredExtensionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRequiredExtensionsVtbl {
        unsafe extern "system" fn EnableAllExtensions<Impl: IRequiredExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableAllExtensions().into()
        }
        unsafe extern "system" fn GetFirstExtension<Impl: IRequiredExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFirstExtension() {
                ::core::result::Result::Ok(ok__) => {
                    *pextclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextExtension<Impl: IRequiredExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextExtension() {
                ::core::result::Result::Ok(ok__) => {
                    *pextclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnableAllExtensions: EnableAllExtensions::<Impl, IMPL_OFFSET>,
            GetFirstExtension: GetFirstExtension::<Impl, IMPL_OFFSET>,
            GetNextExtension: GetNextExtension::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRequiredExtensions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResultDataImpl: Sized {
    fn InsertItem(&mut self, item: *mut RESULTDATAITEM) -> ::windows::core::Result<()>;
    fn DeleteItem(&mut self, itemid: isize, ncol: i32) -> ::windows::core::Result<()>;
    fn FindItemByLParam(&mut self, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<isize>;
    fn DeleteAllRsltItems(&mut self) -> ::windows::core::Result<()>;
    fn SetItem(&mut self, item: *const RESULTDATAITEM) -> ::windows::core::Result<()>;
    fn GetItem(&mut self, item: *mut RESULTDATAITEM) -> ::windows::core::Result<()>;
    fn GetNextItem(&mut self, item: *mut RESULTDATAITEM) -> ::windows::core::Result<()>;
    fn ModifyItemState(&mut self, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> ::windows::core::Result<()>;
    fn ModifyViewStyle(&mut self, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> ::windows::core::Result<()>;
    fn SetViewMode(&mut self, lviewmode: i32) -> ::windows::core::Result<()>;
    fn GetViewMode(&mut self) -> ::windows::core::Result<i32>;
    fn UpdateItem(&mut self, itemid: isize) -> ::windows::core::Result<()>;
    fn Sort(&mut self, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn SetDescBarText(&mut self, desctext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetItemCount(&mut self, nitemcount: i32, dwoptions: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IResultDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResultDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResultDataVtbl {
        unsafe extern "system" fn InsertItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertItem(::core::mem::transmute_copy(&item)).into()
        }
        unsafe extern "system" fn DeleteItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: isize, ncol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteItem(::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&ncol)).into()
        }
        unsafe extern "system" fn FindItemByLParam<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM, pitemid: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindItemByLParam(::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pitemid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAllRsltItems<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAllRsltItems().into()
        }
        unsafe extern "system" fn SetItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *const RESULTDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItem(::core::mem::transmute_copy(&item)).into()
        }
        unsafe extern "system" fn GetItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetItem(::core::mem::transmute_copy(&item)).into()
        }
        unsafe extern "system" fn GetNextItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNextItem(::core::mem::transmute_copy(&item)).into()
        }
        unsafe extern "system" fn ModifyItemState<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ModifyItemState(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&uadd), ::core::mem::transmute_copy(&uremove)).into()
        }
        unsafe extern "system" fn ModifyViewStyle<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ModifyViewStyle(::core::mem::transmute_copy(&add), ::core::mem::transmute_copy(&remove)).into()
        }
        unsafe extern "system" fn SetViewMode<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lviewmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewMode(::core::mem::transmute_copy(&lviewmode)).into()
        }
        unsafe extern "system" fn GetViewMode<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lviewmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewMode() {
                ::core::result::Result::Ok(ok__) => {
                    *lviewmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateItem(::core::mem::transmute_copy(&itemid)).into()
        }
        unsafe extern "system" fn Sort<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Sort(::core::mem::transmute_copy(&ncolumn), ::core::mem::transmute_copy(&dwsortoptions), ::core::mem::transmute_copy(&luserparam)).into()
        }
        unsafe extern "system" fn SetDescBarText<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desctext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescBarText(::core::mem::transmute_copy(&desctext)).into()
        }
        unsafe extern "system" fn SetItemCount<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemcount: i32, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemCount(::core::mem::transmute_copy(&nitemcount), ::core::mem::transmute_copy(&dwoptions)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InsertItem: InsertItem::<Impl, IMPL_OFFSET>,
            DeleteItem: DeleteItem::<Impl, IMPL_OFFSET>,
            FindItemByLParam: FindItemByLParam::<Impl, IMPL_OFFSET>,
            DeleteAllRsltItems: DeleteAllRsltItems::<Impl, IMPL_OFFSET>,
            SetItem: SetItem::<Impl, IMPL_OFFSET>,
            GetItem: GetItem::<Impl, IMPL_OFFSET>,
            GetNextItem: GetNextItem::<Impl, IMPL_OFFSET>,
            ModifyItemState: ModifyItemState::<Impl, IMPL_OFFSET>,
            ModifyViewStyle: ModifyViewStyle::<Impl, IMPL_OFFSET>,
            SetViewMode: SetViewMode::<Impl, IMPL_OFFSET>,
            GetViewMode: GetViewMode::<Impl, IMPL_OFFSET>,
            UpdateItem: UpdateItem::<Impl, IMPL_OFFSET>,
            Sort: Sort::<Impl, IMPL_OFFSET>,
            SetDescBarText: SetDescBarText::<Impl, IMPL_OFFSET>,
            SetItemCount: SetItemCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResultData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResultData2Impl: Sized + IResultDataImpl {
    fn RenameResultItem(&mut self, itemid: isize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IResultData2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResultData2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResultData2Vtbl {
        unsafe extern "system" fn RenameResultItem<Impl: IResultData2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RenameResultItem(::core::mem::transmute_copy(&itemid)).into()
        }
        Self { base: IResultDataVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), RenameResultItem: RenameResultItem::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResultData2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResultDataCompareImpl: Sized {
    fn Compare(&mut self, luserparam: super::super::Foundation::LPARAM, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IResultDataCompareVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResultDataCompareImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResultDataCompareVtbl {
        unsafe extern "system" fn Compare<Impl: IResultDataCompareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, luserparam: super::super::Foundation::LPARAM, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Compare(::core::mem::transmute_copy(&luserparam), ::core::mem::transmute_copy(&cookiea), ::core::mem::transmute_copy(&cookieb), ::core::mem::transmute_copy(&pnresult)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Compare: Compare::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResultDataCompare as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResultDataCompareExImpl: Sized {
    fn Compare(&mut self, prdc: *const RDCOMPARE) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IResultDataCompareExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResultDataCompareExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResultDataCompareExVtbl {
        unsafe extern "system" fn Compare<Impl: IResultDataCompareExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prdc: *const RDCOMPARE, pnresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compare(::core::mem::transmute_copy(&prdc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Compare: Compare::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResultDataCompareEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResultOwnerDataImpl: Sized {
    fn FindItem(&mut self, pfindinfo: *const RESULTFINDINFO) -> ::windows::core::Result<i32>;
    fn CacheHint(&mut self, nstartindex: i32, nendindex: i32) -> ::windows::core::Result<()>;
    fn SortItems(&mut self, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IResultOwnerDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResultOwnerDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResultOwnerDataVtbl {
        unsafe extern "system" fn FindItem<Impl: IResultOwnerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfindinfo: *const RESULTFINDINFO, pnfoundindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindItem(::core::mem::transmute_copy(&pfindinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnfoundindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CacheHint<Impl: IResultOwnerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nstartindex: i32, nendindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CacheHint(::core::mem::transmute_copy(&nstartindex), ::core::mem::transmute_copy(&nendindex)).into()
        }
        unsafe extern "system" fn SortItems<Impl: IResultOwnerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SortItems(::core::mem::transmute_copy(&ncolumn), ::core::mem::transmute_copy(&dwsortoptions), ::core::mem::transmute_copy(&luserparam)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            FindItem: FindItem::<Impl, IMPL_OFFSET>,
            CacheHint: CacheHint::<Impl, IMPL_OFFSET>,
            SortItems: SortItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResultOwnerData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ISnapinAboutImpl: Sized {
    fn GetSnapinDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetProvider(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetSnapinVersion(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetSnapinImage(&mut self) -> ::windows::core::Result<super::super::UI::WindowsAndMessaging::HICON>;
    fn GetStaticFolderImage(&mut self, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ISnapinAboutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISnapinAboutImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISnapinAboutVtbl {
        unsafe extern "system" fn GetSnapinDescription<Impl: ISnapinAboutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapinDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *lpdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProvider<Impl: ISnapinAboutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *lpname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSnapinVersion<Impl: ISnapinAboutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpversion: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapinVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *lpversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSnapinImage<Impl: ISnapinAboutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, happicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapinImage() {
                ::core::result::Result::Ok(ok__) => {
                    *happicon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStaticFolderImage<Impl: ISnapinAboutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStaticFolderImage(::core::mem::transmute_copy(&hsmallimage), ::core::mem::transmute_copy(&hsmallimageopen), ::core::mem::transmute_copy(&hlargeimage), ::core::mem::transmute_copy(&cmask)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSnapinDescription: GetSnapinDescription::<Impl, IMPL_OFFSET>,
            GetProvider: GetProvider::<Impl, IMPL_OFFSET>,
            GetSnapinVersion: GetSnapinVersion::<Impl, IMPL_OFFSET>,
            GetSnapinImage: GetSnapinImage::<Impl, IMPL_OFFSET>,
            GetStaticFolderImage: GetStaticFolderImage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISnapinAbout as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISnapinHelpImpl: Sized {
    fn GetHelpTopic(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISnapinHelpVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISnapinHelpImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISnapinHelpVtbl {
        unsafe extern "system" fn GetHelpTopic<Impl: ISnapinHelpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcompiledhelpfile: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelpTopic() {
                ::core::result::Result::Ok(ok__) => {
                    *lpcompiledhelpfile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetHelpTopic: GetHelpTopic::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISnapinHelp as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISnapinHelp2Impl: Sized + ISnapinHelpImpl {
    fn GetLinkedTopics(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISnapinHelp2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISnapinHelp2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISnapinHelp2Vtbl {
        unsafe extern "system" fn GetLinkedTopics<Impl: ISnapinHelp2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcompiledhelpfiles: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLinkedTopics() {
                ::core::result::Result::Ok(ok__) => {
                    *lpcompiledhelpfiles = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ISnapinHelpVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetLinkedTopics: GetLinkedTopics::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISnapinHelp2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISnapinPropertiesImpl: Sized {
    fn Initialize(&mut self, pproperties: ::core::option::Option<Properties>) -> ::windows::core::Result<()>;
    fn QueryPropertyNames(&mut self, pcallback: ::core::option::Option<ISnapinPropertiesCallback>) -> ::windows::core::Result<()>;
    fn PropertiesChanged(&mut self, cproperties: i32, pproperties: *const MMC_SNAPIN_PROPERTY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISnapinPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISnapinPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISnapinPropertiesVtbl {
        unsafe extern "system" fn Initialize<Impl: ISnapinPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pproperties)).into()
        }
        unsafe extern "system" fn QueryPropertyNames<Impl: ISnapinPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryPropertyNames(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn PropertiesChanged<Impl: ISnapinPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: i32, pproperties: *const MMC_SNAPIN_PROPERTY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PropertiesChanged(::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&pproperties)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            QueryPropertyNames: QueryPropertyNames::<Impl, IMPL_OFFSET>,
            PropertiesChanged: PropertiesChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISnapinProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISnapinPropertiesCallbackImpl: Sized {
    fn AddPropertyName(&mut self, pszpropname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISnapinPropertiesCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISnapinPropertiesCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISnapinPropertiesCallbackVtbl {
        unsafe extern "system" fn AddPropertyName<Impl: ISnapinPropertiesCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPropertyName(::core::mem::transmute_copy(&pszpropname), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddPropertyName: AddPropertyName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISnapinPropertiesCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IStringTableImpl: Sized {
    fn AddString(&mut self, pszadd: super::super::Foundation::PWSTR) -> ::windows::core::Result<u32>;
    fn GetString(&mut self, stringid: u32, cchbuffer: u32, lpbuffer: super::super::Foundation::PWSTR, pcchout: *mut u32) -> ::windows::core::Result<()>;
    fn GetStringLength(&mut self, stringid: u32) -> ::windows::core::Result<u32>;
    fn DeleteString(&mut self, stringid: u32) -> ::windows::core::Result<()>;
    fn DeleteAllStrings(&mut self) -> ::windows::core::Result<()>;
    fn FindString(&mut self, pszfind: super::super::Foundation::PWSTR) -> ::windows::core::Result<u32>;
    fn Enumerate(&mut self) -> ::windows::core::Result<super::Com::IEnumString>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IStringTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStringTableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStringTableVtbl {
        unsafe extern "system" fn AddString<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszadd: super::super::Foundation::PWSTR, pstringid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddString(::core::mem::transmute_copy(&pszadd)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstringid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stringid: u32, cchbuffer: u32, lpbuffer: super::super::Foundation::PWSTR, pcchout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetString(::core::mem::transmute_copy(&stringid), ::core::mem::transmute_copy(&cchbuffer), ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&pcchout)).into()
        }
        unsafe extern "system" fn GetStringLength<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stringid: u32, pcchstring: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringLength(::core::mem::transmute_copy(&stringid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcchstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteString<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stringid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteString(::core::mem::transmute_copy(&stringid)).into()
        }
        unsafe extern "system" fn DeleteAllStrings<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAllStrings().into()
        }
        unsafe extern "system" fn FindString<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfind: super::super::Foundation::PWSTR, pstringid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindString(::core::mem::transmute_copy(&pszfind)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstringid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enumerate<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enumerate() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddString: AddString::<Impl, IMPL_OFFSET>,
            GetString: GetString::<Impl, IMPL_OFFSET>,
            GetStringLength: GetStringLength::<Impl, IMPL_OFFSET>,
            DeleteString: DeleteString::<Impl, IMPL_OFFSET>,
            DeleteAllStrings: DeleteAllStrings::<Impl, IMPL_OFFSET>,
            FindString: FindString::<Impl, IMPL_OFFSET>,
            Enumerate: Enumerate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStringTable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IToolbarImpl: Sized {
    fn AddBitmap(&mut self, nimages: i32, hbmp: super::super::Graphics::Gdi::HBITMAP, cxsize: i32, cysize: i32, crmask: u32) -> ::windows::core::Result<()>;
    fn AddButtons(&mut self, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> ::windows::core::Result<()>;
    fn InsertButton(&mut self, nindex: i32, lpbutton: *const MMCBUTTON) -> ::windows::core::Result<()>;
    fn DeleteButton(&mut self, nindex: i32) -> ::windows::core::Result<()>;
    fn GetButtonState(&mut self, idcommand: i32, nstate: MMC_BUTTON_STATE) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetButtonState(&mut self, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IToolbarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToolbarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToolbarVtbl {
        unsafe extern "system" fn AddBitmap<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nimages: i32, hbmp: super::super::Graphics::Gdi::HBITMAP, cxsize: i32, cysize: i32, crmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBitmap(::core::mem::transmute_copy(&nimages), ::core::mem::transmute_copy(&hbmp), ::core::mem::transmute_copy(&cxsize), ::core::mem::transmute_copy(&cysize), ::core::mem::transmute_copy(&crmask)).into()
        }
        unsafe extern "system" fn AddButtons<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddButtons(::core::mem::transmute_copy(&nbuttons), ::core::mem::transmute_copy(&lpbuttons)).into()
        }
        unsafe extern "system" fn InsertButton<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, lpbutton: *const MMCBUTTON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertButton(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&lpbutton)).into()
        }
        unsafe extern "system" fn DeleteButton<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteButton(::core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn GetButtonState<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetButtonState(::core::mem::transmute_copy(&idcommand), ::core::mem::transmute_copy(&nstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonState<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonState(::core::mem::transmute_copy(&idcommand), ::core::mem::transmute_copy(&nstate), ::core::mem::transmute_copy(&bstate)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddBitmap: AddBitmap::<Impl, IMPL_OFFSET>,
            AddButtons: AddButtons::<Impl, IMPL_OFFSET>,
            InsertButton: InsertButton::<Impl, IMPL_OFFSET>,
            DeleteButton: DeleteButton::<Impl, IMPL_OFFSET>,
            GetButtonState: GetButtonState::<Impl, IMPL_OFFSET>,
            SetButtonState: SetButtonState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToolbar as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IViewExtensionCallbackImpl: Sized {
    fn AddView(&mut self, pextviewdata: *const MMC_EXT_VIEW_DATA) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IViewExtensionCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewExtensionCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IViewExtensionCallbackVtbl {
        unsafe extern "system" fn AddView<Impl: IViewExtensionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextviewdata: *const MMC_EXT_VIEW_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddView(::core::mem::transmute_copy(&pextviewdata)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddView: AddView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewExtensionCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait MenuItemImpl: Sized + IDispatchImpl {
    fn DisplayName(&mut self) -> ::windows::core::Result<*mut u16>;
    fn LanguageIndependentName(&mut self) -> ::windows::core::Result<*mut u16>;
    fn Path(&mut self) -> ::windows::core::Result<*mut u16>;
    fn LanguageIndependentPath(&mut self) -> ::windows::core::Result<*mut u16>;
    fn Execute(&mut self) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl MenuItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: MenuItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> MenuItemVtbl {
        unsafe extern "system" fn DisplayName<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *displayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LanguageIndependentName<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageindependentname: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageIndependentName() {
                ::core::result::Result::Ok(ok__) => {
                    *languageindependentname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LanguageIndependentPath<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageindependentpath: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageIndependentPath() {
                ::core::result::Result::Ok(ok__) => {
                    *languageindependentpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Execute<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Execute().into()
        }
        unsafe extern "system" fn Enabled<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            LanguageIndependentName: LanguageIndependentName::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            LanguageIndependentPath: LanguageIndependentPath::<Impl, IMPL_OFFSET>,
            Execute: Execute::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<MenuItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait NodeImpl: Sized + IDispatchImpl {
    fn Name(&mut self) -> ::windows::core::Result<*mut u16>;
    fn Property(&mut self, propertyname: super::super::Foundation::BSTR) -> ::windows::core::Result<*mut u16>;
    fn Bookmark(&mut self) -> ::windows::core::Result<*mut u16>;
    fn IsScopeNode(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Nodetype(&mut self) -> ::windows::core::Result<*mut u16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl NodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: NodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> NodeVtbl {
        unsafe extern "system" fn Name<Impl: NodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Property<Impl: NodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Property(::core::mem::transmute_copy(&propertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bookmark<Impl: NodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bookmark: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bookmark() {
                ::core::result::Result::Ok(ok__) => {
                    *bookmark = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScopeNode<Impl: NodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isscopenode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsScopeNode() {
                ::core::result::Result::Ok(ok__) => {
                    *isscopenode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Nodetype<Impl: NodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodetype: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Nodetype() {
                ::core::result::Result::Ok(ok__) => {
                    *nodetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Property: Property::<Impl, IMPL_OFFSET>,
            Bookmark: Bookmark::<Impl, IMPL_OFFSET>,
            IsScopeNode: IsScopeNode::<Impl, IMPL_OFFSET>,
            Nodetype: Nodetype::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Node as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait NodesImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<Node>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl NodesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: NodesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> NodesVtbl {
        unsafe extern "system" fn _NewEnum<Impl: NodesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: NodesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *node = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: NodesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Nodes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait PropertiesImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, name: super::super::Foundation::BSTR) -> ::windows::core::Result<Property>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Remove(&mut self, name: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl PropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: PropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> PropertiesVtbl {
        unsafe extern "system" fn _NewEnum<Impl: PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *property = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&name)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Properties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait PropertyImpl: Sized + IDispatchImpl {
    fn Value(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetValue(&mut self, value: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<*mut u16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl PropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: PropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> PropertyVtbl {
        unsafe extern "system" fn Value<Impl: PropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: PropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Name<Impl: PropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Property as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ScopeNamespaceImpl: Sized + IDispatchImpl {
    fn GetParent(&mut self, node: ::core::option::Option<Node>) -> ::windows::core::Result<Node>;
    fn GetChild(&mut self, node: ::core::option::Option<Node>) -> ::windows::core::Result<Node>;
    fn GetNext(&mut self, node: ::core::option::Option<Node>) -> ::windows::core::Result<Node>;
    fn GetRoot(&mut self) -> ::windows::core::Result<Node>;
    fn Expand(&mut self, node: ::core::option::Option<Node>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ScopeNamespaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ScopeNamespaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ScopeNamespaceVtbl {
        unsafe extern "system" fn GetParent<Impl: ScopeNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParent(::core::mem::transmute(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    *parent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChild<Impl: ScopeNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, child: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChild(::core::mem::transmute(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    *child = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNext<Impl: ScopeNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, next: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNext(::core::mem::transmute(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    *next = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRoot<Impl: ScopeNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, root: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *root = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Expand<Impl: ScopeNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Expand(::core::mem::transmute(&node)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetParent: GetParent::<Impl, IMPL_OFFSET>,
            GetChild: GetChild::<Impl, IMPL_OFFSET>,
            GetNext: GetNext::<Impl, IMPL_OFFSET>,
            GetRoot: GetRoot::<Impl, IMPL_OFFSET>,
            Expand: Expand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ScopeNamespace as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait SnapInImpl: Sized + IDispatchImpl {
    fn Name(&mut self) -> ::windows::core::Result<*mut u16>;
    fn Vendor(&mut self) -> ::windows::core::Result<*mut u16>;
    fn Version(&mut self) -> ::windows::core::Result<*mut u16>;
    fn Extensions(&mut self) -> ::windows::core::Result<Extensions>;
    fn SnapinCLSID(&mut self) -> ::windows::core::Result<*mut u16>;
    fn Properties(&mut self) -> ::windows::core::Result<Properties>;
    fn EnableAllExtensions(&mut self, enable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl SnapInVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: SnapInImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> SnapInVtbl {
        unsafe extern "system" fn Name<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Vendor<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendor: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Vendor() {
                ::core::result::Result::Ok(ok__) => {
                    *vendor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *version = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extensions<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extensions() {
                ::core::result::Result::Ok(ok__) => {
                    *extensions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnapinCLSID<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapinclsid: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SnapinCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *snapinclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *properties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAllExtensions<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableAllExtensions(::core::mem::transmute_copy(&enable)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Vendor: Vendor::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            Extensions: Extensions::<Impl, IMPL_OFFSET>,
            SnapinCLSID: SnapinCLSID::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            EnableAllExtensions: EnableAllExtensions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<SnapIn as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait SnapInsImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<SnapIn>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, snapinnameorclsid: super::super::Foundation::BSTR, parentsnapin: super::Com::VARIANT, properties: super::Com::VARIANT) -> ::windows::core::Result<SnapIn>;
    fn Remove(&mut self, snapin: ::core::option::Option<SnapIn>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl SnapInsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: SnapInsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> SnapInsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: SnapInsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: SnapInsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, snapin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *snapin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: SnapInsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: SnapInsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapinnameorclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, parentsnapin: ::core::mem::ManuallyDrop<super::Com::VARIANT>, properties: ::core::mem::ManuallyDrop<super::Com::VARIANT>, snapin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&snapinnameorclsid), ::core::mem::transmute_copy(&parentsnapin), ::core::mem::transmute_copy(&properties)) {
                ::core::result::Result::Ok(ok__) => {
                    *snapin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: SnapInsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute(&snapin)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<SnapIns as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ViewImpl: Sized + IDispatchImpl {
    fn ActiveScopeNode(&mut self) -> ::windows::core::Result<Node>;
    fn SetActiveScopeNode(&mut self, node: ::core::option::Option<Node>) -> ::windows::core::Result<()>;
    fn Selection(&mut self) -> ::windows::core::Result<Nodes>;
    fn ListItems(&mut self) -> ::windows::core::Result<Nodes>;
    fn SnapinScopeObject(&mut self, scopenode: super::Com::VARIANT) -> ::windows::core::Result<super::Com::IDispatch>;
    fn SnapinSelectionObject(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Is(&mut self, view: ::core::option::Option<View>) -> ::windows::core::Result<i16>;
    fn Document(&mut self) -> ::windows::core::Result<Document>;
    fn SelectAll(&mut self) -> ::windows::core::Result<()>;
    fn Select(&mut self, node: ::core::option::Option<Node>) -> ::windows::core::Result<()>;
    fn Deselect(&mut self, node: ::core::option::Option<Node>) -> ::windows::core::Result<()>;
    fn IsSelected(&mut self, node: ::core::option::Option<Node>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn DisplayScopeNodePropertySheet(&mut self, scopenode: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DisplaySelectionPropertySheet(&mut self) -> ::windows::core::Result<()>;
    fn CopyScopeNode(&mut self, scopenode: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn CopySelection(&mut self) -> ::windows::core::Result<()>;
    fn DeleteScopeNode(&mut self, scopenode: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteSelection(&mut self) -> ::windows::core::Result<()>;
    fn RenameScopeNode(&mut self, newname: super::super::Foundation::BSTR, scopenode: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn RenameSelectedItem(&mut self, newname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ScopeNodeContextMenu(&mut self, scopenode: super::Com::VARIANT) -> ::windows::core::Result<ContextMenu>;
    fn SelectionContextMenu(&mut self) -> ::windows::core::Result<ContextMenu>;
    fn RefreshScopeNode(&mut self, scopenode: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn RefreshSelection(&mut self) -> ::windows::core::Result<()>;
    fn ExecuteSelectionMenuItem(&mut self, menuitempath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ExecuteScopeNodeMenuItem(&mut self, menuitempath: super::super::Foundation::BSTR, scopenode: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ExecuteShellCommand(&mut self, command: super::super::Foundation::BSTR, directory: super::super::Foundation::BSTR, parameters: super::super::Foundation::BSTR, windowstate: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Frame(&mut self) -> ::windows::core::Result<Frame>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn ScopeTreeVisible(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetScopeTreeVisible(&mut self, visible: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Back(&mut self) -> ::windows::core::Result<()>;
    fn Forward(&mut self) -> ::windows::core::Result<()>;
    fn SetStatusBarText(&mut self, statusbartext: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Memento(&mut self) -> ::windows::core::Result<*mut u16>;
    fn ViewMemento(&mut self, memento: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Columns(&mut self) -> ::windows::core::Result<Columns>;
    fn CellContents(&mut self, node: ::core::option::Option<Node>, column: i32) -> ::windows::core::Result<*mut u16>;
    fn ExportList(&mut self, file: super::super::Foundation::BSTR, exportoptions: _ExportListOptions) -> ::windows::core::Result<()>;
    fn ListViewMode(&mut self) -> ::windows::core::Result<_ListViewMode>;
    fn SetListViewMode(&mut self, mode: _ListViewMode) -> ::windows::core::Result<()>;
    fn ControlObject(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ViewVtbl {
        unsafe extern "system" fn ActiveScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveScopeNode() {
                ::core::result::Result::Ok(ok__) => {
                    *node = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActiveScopeNode(::core::mem::transmute(&node)).into()
        }
        unsafe extern "system" fn Selection<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Selection() {
                ::core::result::Result::Ok(ok__) => {
                    *nodes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListItems<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListItems() {
                ::core::result::Result::Ok(ok__) => {
                    *nodes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnapinScopeObject<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>, scopenodeobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SnapinScopeObject(::core::mem::transmute_copy(&scopenode)) {
                ::core::result::Result::Ok(ok__) => {
                    *scopenodeobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnapinSelectionObject<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SnapinSelectionObject() {
                ::core::result::Result::Ok(ok__) => {
                    *selectionobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Is<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, thesame: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Is(::core::mem::transmute(&view)) {
                ::core::result::Result::Ok(ok__) => {
                    *thesame = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Document<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Document() {
                ::core::result::Result::Ok(ok__) => {
                    *document = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectAll<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectAll().into()
        }
        unsafe extern "system" fn Select<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Select(::core::mem::transmute(&node)).into()
        }
        unsafe extern "system" fn Deselect<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deselect(::core::mem::transmute(&node)).into()
        }
        unsafe extern "system" fn IsSelected<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, isselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelected(::core::mem::transmute(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    *isselected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayScopeNodePropertySheet<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayScopeNodePropertySheet(::core::mem::transmute_copy(&scopenode)).into()
        }
        unsafe extern "system" fn DisplaySelectionPropertySheet<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplaySelectionPropertySheet().into()
        }
        unsafe extern "system" fn CopyScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyScopeNode(::core::mem::transmute_copy(&scopenode)).into()
        }
        unsafe extern "system" fn CopySelection<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopySelection().into()
        }
        unsafe extern "system" fn DeleteScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteScopeNode(::core::mem::transmute_copy(&scopenode)).into()
        }
        unsafe extern "system" fn DeleteSelection<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteSelection().into()
        }
        unsafe extern "system" fn RenameScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RenameScopeNode(::core::mem::transmute_copy(&newname), ::core::mem::transmute_copy(&scopenode)).into()
        }
        unsafe extern "system" fn RenameSelectedItem<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RenameSelectedItem(::core::mem::transmute_copy(&newname)).into()
        }
        unsafe extern "system" fn ScopeNodeContextMenu<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>, contextmenu: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScopeNodeContextMenu(::core::mem::transmute_copy(&scopenode)) {
                ::core::result::Result::Ok(ok__) => {
                    *contextmenu = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionContextMenu<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextmenu: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionContextMenu() {
                ::core::result::Result::Ok(ok__) => {
                    *contextmenu = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RefreshScopeNode(::core::mem::transmute_copy(&scopenode)).into()
        }
        unsafe extern "system" fn RefreshSelection<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RefreshSelection().into()
        }
        unsafe extern "system" fn ExecuteSelectionMenuItem<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, menuitempath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecuteSelectionMenuItem(::core::mem::transmute_copy(&menuitempath)).into()
        }
        unsafe extern "system" fn ExecuteScopeNodeMenuItem<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, menuitempath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecuteScopeNodeMenuItem(::core::mem::transmute_copy(&menuitempath), ::core::mem::transmute_copy(&scopenode)).into()
        }
        unsafe extern "system" fn ExecuteShellCommand<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, directory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, parameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, windowstate: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecuteShellCommand(::core::mem::transmute_copy(&command), ::core::mem::transmute_copy(&directory), ::core::mem::transmute_copy(&parameters), ::core::mem::transmute_copy(&windowstate)).into()
        }
        unsafe extern "system" fn Frame<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Frame() {
                ::core::result::Result::Ok(ok__) => {
                    *frame = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn ScopeTreeVisible<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScopeTreeVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *visible = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScopeTreeVisible<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScopeTreeVisible(::core::mem::transmute_copy(&visible)).into()
        }
        unsafe extern "system" fn Back<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Back().into()
        }
        unsafe extern "system" fn Forward<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Forward().into()
        }
        unsafe extern "system" fn SetStatusBarText<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statusbartext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatusBarText(::core::mem::transmute_copy(&statusbartext)).into()
        }
        unsafe extern "system" fn Memento<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memento: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Memento() {
                ::core::result::Result::Ok(ok__) => {
                    *memento = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewMemento<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memento: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ViewMemento(::core::mem::transmute_copy(&memento)).into()
        }
        unsafe extern "system" fn Columns<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, columns: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Columns() {
                ::core::result::Result::Ok(ok__) => {
                    *columns = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CellContents<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, column: i32, cellcontents: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CellContents(::core::mem::transmute(&node), ::core::mem::transmute_copy(&column)) {
                ::core::result::Result::Ok(ok__) => {
                    *cellcontents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportList<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exportoptions: _ExportListOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExportList(::core::mem::transmute_copy(&file), ::core::mem::transmute_copy(&exportoptions)).into()
        }
        unsafe extern "system" fn ListViewMode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut _ListViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListViewMode() {
                ::core::result::Result::Ok(ok__) => {
                    *mode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListViewMode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: _ListViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListViewMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn ControlObject<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, control: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlObject() {
                ::core::result::Result::Ok(ok__) => {
                    *control = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ActiveScopeNode: ActiveScopeNode::<Impl, IMPL_OFFSET>,
            SetActiveScopeNode: SetActiveScopeNode::<Impl, IMPL_OFFSET>,
            Selection: Selection::<Impl, IMPL_OFFSET>,
            ListItems: ListItems::<Impl, IMPL_OFFSET>,
            SnapinScopeObject: SnapinScopeObject::<Impl, IMPL_OFFSET>,
            SnapinSelectionObject: SnapinSelectionObject::<Impl, IMPL_OFFSET>,
            Is: Is::<Impl, IMPL_OFFSET>,
            Document: Document::<Impl, IMPL_OFFSET>,
            SelectAll: SelectAll::<Impl, IMPL_OFFSET>,
            Select: Select::<Impl, IMPL_OFFSET>,
            Deselect: Deselect::<Impl, IMPL_OFFSET>,
            IsSelected: IsSelected::<Impl, IMPL_OFFSET>,
            DisplayScopeNodePropertySheet: DisplayScopeNodePropertySheet::<Impl, IMPL_OFFSET>,
            DisplaySelectionPropertySheet: DisplaySelectionPropertySheet::<Impl, IMPL_OFFSET>,
            CopyScopeNode: CopyScopeNode::<Impl, IMPL_OFFSET>,
            CopySelection: CopySelection::<Impl, IMPL_OFFSET>,
            DeleteScopeNode: DeleteScopeNode::<Impl, IMPL_OFFSET>,
            DeleteSelection: DeleteSelection::<Impl, IMPL_OFFSET>,
            RenameScopeNode: RenameScopeNode::<Impl, IMPL_OFFSET>,
            RenameSelectedItem: RenameSelectedItem::<Impl, IMPL_OFFSET>,
            ScopeNodeContextMenu: ScopeNodeContextMenu::<Impl, IMPL_OFFSET>,
            SelectionContextMenu: SelectionContextMenu::<Impl, IMPL_OFFSET>,
            RefreshScopeNode: RefreshScopeNode::<Impl, IMPL_OFFSET>,
            RefreshSelection: RefreshSelection::<Impl, IMPL_OFFSET>,
            ExecuteSelectionMenuItem: ExecuteSelectionMenuItem::<Impl, IMPL_OFFSET>,
            ExecuteScopeNodeMenuItem: ExecuteScopeNodeMenuItem::<Impl, IMPL_OFFSET>,
            ExecuteShellCommand: ExecuteShellCommand::<Impl, IMPL_OFFSET>,
            Frame: Frame::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            ScopeTreeVisible: ScopeTreeVisible::<Impl, IMPL_OFFSET>,
            SetScopeTreeVisible: SetScopeTreeVisible::<Impl, IMPL_OFFSET>,
            Back: Back::<Impl, IMPL_OFFSET>,
            Forward: Forward::<Impl, IMPL_OFFSET>,
            SetStatusBarText: SetStatusBarText::<Impl, IMPL_OFFSET>,
            Memento: Memento::<Impl, IMPL_OFFSET>,
            ViewMemento: ViewMemento::<Impl, IMPL_OFFSET>,
            Columns: Columns::<Impl, IMPL_OFFSET>,
            CellContents: CellContents::<Impl, IMPL_OFFSET>,
            ExportList: ExportList::<Impl, IMPL_OFFSET>,
            ListViewMode: ListViewMode::<Impl, IMPL_OFFSET>,
            SetListViewMode: SetListViewMode::<Impl, IMPL_OFFSET>,
            ControlObject: ControlObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<View as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ViewsImpl: Sized + IDispatchImpl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<View>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, node: ::core::option::Option<Node>, viewoptions: _ViewOptions) -> ::windows::core::Result<()>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ViewsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ViewsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ViewsVtbl {
        unsafe extern "system" fn Item<Impl: ViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, view: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *view = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, viewoptions: _ViewOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&node), ::core::mem::transmute_copy(&viewoptions)).into()
        }
        unsafe extern "system" fn _NewEnum<Impl: ViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Views as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _AppEventsImpl: Sized + IDispatchImpl {
    fn OnQuit(&mut self, application: ::core::option::Option<_Application>) -> ::windows::core::Result<()>;
    fn OnDocumentOpen(&mut self, document: ::core::option::Option<Document>, new: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnDocumentClose(&mut self, document: ::core::option::Option<Document>) -> ::windows::core::Result<()>;
    fn OnSnapInAdded(&mut self, document: ::core::option::Option<Document>, snapin: ::core::option::Option<SnapIn>) -> ::windows::core::Result<()>;
    fn OnSnapInRemoved(&mut self, document: ::core::option::Option<Document>, snapin: ::core::option::Option<SnapIn>) -> ::windows::core::Result<()>;
    fn OnNewView(&mut self, view: ::core::option::Option<View>) -> ::windows::core::Result<()>;
    fn OnViewClose(&mut self, view: ::core::option::Option<View>) -> ::windows::core::Result<()>;
    fn OnViewChange(&mut self, view: ::core::option::Option<View>, newownernode: ::core::option::Option<Node>) -> ::windows::core::Result<()>;
    fn OnSelectionChange(&mut self, view: ::core::option::Option<View>, newnodes: ::core::option::Option<Nodes>) -> ::windows::core::Result<()>;
    fn OnContextMenuExecuted(&mut self, menuitem: ::core::option::Option<MenuItem>) -> ::windows::core::Result<()>;
    fn OnToolbarButtonClicked(&mut self) -> ::windows::core::Result<()>;
    fn OnListUpdated(&mut self, view: ::core::option::Option<View>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _AppEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _AppEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _AppEventsVtbl {
        unsafe extern "system" fn OnQuit<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnQuit(::core::mem::transmute(&application)).into()
        }
        unsafe extern "system" fn OnDocumentOpen<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr, new: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDocumentOpen(::core::mem::transmute(&document), ::core::mem::transmute_copy(&new)).into()
        }
        unsafe extern "system" fn OnDocumentClose<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDocumentClose(::core::mem::transmute(&document)).into()
        }
        unsafe extern "system" fn OnSnapInAdded<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr, snapin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSnapInAdded(::core::mem::transmute(&document), ::core::mem::transmute(&snapin)).into()
        }
        unsafe extern "system" fn OnSnapInRemoved<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr, snapin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSnapInRemoved(::core::mem::transmute(&document), ::core::mem::transmute(&snapin)).into()
        }
        unsafe extern "system" fn OnNewView<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnNewView(::core::mem::transmute(&view)).into()
        }
        unsafe extern "system" fn OnViewClose<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnViewClose(::core::mem::transmute(&view)).into()
        }
        unsafe extern "system" fn OnViewChange<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, newownernode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnViewChange(::core::mem::transmute(&view), ::core::mem::transmute(&newownernode)).into()
        }
        unsafe extern "system" fn OnSelectionChange<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, newnodes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSelectionChange(::core::mem::transmute(&view), ::core::mem::transmute(&newnodes)).into()
        }
        unsafe extern "system" fn OnContextMenuExecuted<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, menuitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnContextMenuExecuted(::core::mem::transmute(&menuitem)).into()
        }
        unsafe extern "system" fn OnToolbarButtonClicked<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnToolbarButtonClicked().into()
        }
        unsafe extern "system" fn OnListUpdated<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnListUpdated(::core::mem::transmute(&view)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnQuit: OnQuit::<Impl, IMPL_OFFSET>,
            OnDocumentOpen: OnDocumentOpen::<Impl, IMPL_OFFSET>,
            OnDocumentClose: OnDocumentClose::<Impl, IMPL_OFFSET>,
            OnSnapInAdded: OnSnapInAdded::<Impl, IMPL_OFFSET>,
            OnSnapInRemoved: OnSnapInRemoved::<Impl, IMPL_OFFSET>,
            OnNewView: OnNewView::<Impl, IMPL_OFFSET>,
            OnViewClose: OnViewClose::<Impl, IMPL_OFFSET>,
            OnViewChange: OnViewChange::<Impl, IMPL_OFFSET>,
            OnSelectionChange: OnSelectionChange::<Impl, IMPL_OFFSET>,
            OnContextMenuExecuted: OnContextMenuExecuted::<Impl, IMPL_OFFSET>,
            OnToolbarButtonClicked: OnToolbarButtonClicked::<Impl, IMPL_OFFSET>,
            OnListUpdated: OnListUpdated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_AppEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _ApplicationImpl: Sized + IDispatchImpl {
    fn Help(&mut self);
    fn Quit(&mut self);
    fn Document(&mut self) -> ::windows::core::Result<Document>;
    fn Load(&mut self, filename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Frame(&mut self) -> ::windows::core::Result<Frame>;
    fn Visible(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Show(&mut self) -> ::windows::core::Result<()>;
    fn Hide(&mut self) -> ::windows::core::Result<()>;
    fn UserControl(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetUserControl(&mut self, usercontrol: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn VersionMajor(&mut self) -> ::windows::core::Result<i32>;
    fn VersionMinor(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _ApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ApplicationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _ApplicationVtbl {
        unsafe extern "system" fn Help<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Help()
        }
        unsafe extern "system" fn Quit<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Quit()
        }
        unsafe extern "system" fn Document<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Document() {
                ::core::result::Result::Ok(ok__) => {
                    *document = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Load(::core::mem::transmute_copy(&filename)).into()
        }
        unsafe extern "system" fn Frame<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Frame() {
                ::core::result::Result::Ok(ok__) => {
                    *frame = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Visible<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *visible = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show().into()
        }
        unsafe extern "system" fn Hide<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Hide().into()
        }
        unsafe extern "system" fn UserControl<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usercontrol: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserControl() {
                ::core::result::Result::Ok(ok__) => {
                    *usercontrol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserControl<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usercontrol: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserControl(::core::mem::transmute_copy(&usercontrol)).into()
        }
        unsafe extern "system" fn VersionMajor<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, versionmajor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VersionMajor() {
                ::core::result::Result::Ok(ok__) => {
                    *versionmajor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VersionMinor<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, versionminor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VersionMinor() {
                ::core::result::Result::Ok(ok__) => {
                    *versionminor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Help: Help::<Impl, IMPL_OFFSET>,
            Quit: Quit::<Impl, IMPL_OFFSET>,
            Document: Document::<Impl, IMPL_OFFSET>,
            Load: Load::<Impl, IMPL_OFFSET>,
            Frame: Frame::<Impl, IMPL_OFFSET>,
            Visible: Visible::<Impl, IMPL_OFFSET>,
            Show: Show::<Impl, IMPL_OFFSET>,
            Hide: Hide::<Impl, IMPL_OFFSET>,
            UserControl: UserControl::<Impl, IMPL_OFFSET>,
            SetUserControl: SetUserControl::<Impl, IMPL_OFFSET>,
            VersionMajor: VersionMajor::<Impl, IMPL_OFFSET>,
            VersionMinor: VersionMinor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_Application as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _EventConnectorImpl: Sized + IDispatchImpl {
    fn ConnectTo(&mut self, application: ::core::option::Option<_Application>) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _EventConnectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _EventConnectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _EventConnectorVtbl {
        unsafe extern "system" fn ConnectTo<Impl: _EventConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectTo(::core::mem::transmute(&application)).into()
        }
        unsafe extern "system" fn Disconnect<Impl: _EventConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ConnectTo: ConnectTo::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_EventConnector as ::windows::core::Interface>::IID
    }
}
