#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait AppEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl AppEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AppEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AppEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AppEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ColumnVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ColumnImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ColumnVtbl {
        unsafe extern "system" fn Name<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Width<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWidth<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayPosition<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayPosition<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Hidden<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hidden: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHidden<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hidden: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAsSortColumn<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sortorder: _ColumnSortOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSortColumn<Impl: ColumnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issortcolumn: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Width::<Impl, IMPL_OFFSET>,
            SetWidth::<Impl, IMPL_OFFSET>,
            DisplayPosition::<Impl, IMPL_OFFSET>,
            SetDisplayPosition::<Impl, IMPL_OFFSET>,
            Hidden::<Impl, IMPL_OFFSET>,
            SetHidden::<Impl, IMPL_OFFSET>,
            SetAsSortColumn::<Impl, IMPL_OFFSET>,
            IsSortColumn::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Column as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ColumnsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ColumnsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ColumnsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ColumnsVtbl {
        unsafe extern "system" fn Item<Impl: ColumnsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, column: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ColumnsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ColumnsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Columns as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ContextMenuImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ContextMenuVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ContextMenuImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ContextMenuVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ContextMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ContextMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexorpath: ::core::mem::ManuallyDrop<super::Com::VARIANT>, menuitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ContextMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ContextMenu as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DocumentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DocumentVtbl {
        unsafe extern "system" fn Save<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveAs<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, savechanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Views<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, views: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SnapIns<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapins: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ActiveView<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Location<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSaved<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issaved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Mode<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut _DocumentMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMode<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: _DocumentMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RootNode<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScopeNamespace<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateProperties<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Application<Impl: DocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Save::<Impl, IMPL_OFFSET>,
            SaveAs::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            Views::<Impl, IMPL_OFFSET>,
            SnapIns::<Impl, IMPL_OFFSET>,
            ActiveView::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            Location::<Impl, IMPL_OFFSET>,
            IsSaved::<Impl, IMPL_OFFSET>,
            Mode::<Impl, IMPL_OFFSET>,
            SetMode::<Impl, IMPL_OFFSET>,
            RootNode::<Impl, IMPL_OFFSET>,
            ScopeNamespace::<Impl, IMPL_OFFSET>,
            CreateProperties::<Impl, IMPL_OFFSET>,
            Application::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Document as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ExtensionImpl: Sized + IDispatchImpl {
    fn Name();
    fn Vendor();
    fn Version();
    fn Extensions();
    fn SnapinCLSID();
    fn EnableAllExtensions();
    fn Enable();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ExtensionVtbl {
        unsafe extern "system" fn Name<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Vendor<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendor: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Version<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Extensions<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SnapinCLSID<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapinclsid: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableAllExtensions<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enable<Impl: ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Vendor::<Impl, IMPL_OFFSET>,
            Version::<Impl, IMPL_OFFSET>,
            Extensions::<Impl, IMPL_OFFSET>,
            SnapinCLSID::<Impl, IMPL_OFFSET>,
            EnableAllExtensions::<Impl, IMPL_OFFSET>,
            Enable::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Extension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ExtensionsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ExtensionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ExtensionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ExtensionsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, extension: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Extensions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl FrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: FrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> FrameVtbl {
        unsafe extern "system" fn Maximize<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Minimize<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Restore<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Top<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTop<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Bottom<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBottom<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Left<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLeft<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Right<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRight<Impl: FrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Maximize::<Impl, IMPL_OFFSET>,
            Minimize::<Impl, IMPL_OFFSET>,
            Restore::<Impl, IMPL_OFFSET>,
            Top::<Impl, IMPL_OFFSET>,
            SetTop::<Impl, IMPL_OFFSET>,
            Bottom::<Impl, IMPL_OFFSET>,
            SetBottom::<Impl, IMPL_OFFSET>,
            Left::<Impl, IMPL_OFFSET>,
            SetLeft::<Impl, IMPL_OFFSET>,
            Right::<Impl, IMPL_OFFSET>,
            SetRight::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Frame as ::windows::core::Interface>::IID
    }
}
pub trait IColumnDataImpl: Sized {
    fn SetColumnConfigData();
    fn GetColumnConfigData();
    fn SetColumnSortData();
    fn GetColumnSortData();
}
impl IColumnDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColumnDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColumnDataVtbl {
        unsafe extern "system" fn SetColumnConfigData<Impl: IColumnDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnConfigData<Impl: IColumnDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, ppcolsetdata: *mut *mut MMC_COLUMN_SET_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColumnSortData<Impl: IColumnDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnSortData<Impl: IColumnDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, ppcolsortdata: *mut *mut MMC_SORT_SET_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetColumnConfigData::<Impl, IMPL_OFFSET>, GetColumnConfigData::<Impl, IMPL_OFFSET>, SetColumnSortData::<Impl, IMPL_OFFSET>, GetColumnSortData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColumnData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IComponentImpl: Sized {
    fn Initialize();
    fn Notify();
    fn Destroy();
    fn QueryDataObject();
    fn GetResultViewType();
    fn GetDisplayInfo();
    fn CompareObjects();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComponentVtbl {
        unsafe extern "system" fn Initialize<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpconsole: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Notify<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Destroy<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryDataObject<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResultViewType<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, ppviewtype: *mut super::super::Foundation::PWSTR, pviewoptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayInfo<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presultdataitem: *mut RESULTDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompareObjects<Impl: IComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobjecta: ::windows::core::RawPtr, lpdataobjectb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Notify::<Impl, IMPL_OFFSET>, Destroy::<Impl, IMPL_OFFSET>, QueryDataObject::<Impl, IMPL_OFFSET>, GetResultViewType::<Impl, IMPL_OFFSET>, GetDisplayInfo::<Impl, IMPL_OFFSET>, CompareObjects::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IComponent2Impl: Sized + IComponentImpl {
    fn QueryDispatch();
    fn GetResultViewType2();
    fn RestoreResultView();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IComponent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComponent2Vtbl {
        unsafe extern "system" fn QueryDispatch<Impl: IComponent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResultViewType2<Impl: IComponent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestoreResultView<Impl: IComponent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            Notify::<Impl, IMPL_OFFSET>,
            Destroy::<Impl, IMPL_OFFSET>,
            QueryDataObject::<Impl, IMPL_OFFSET>,
            GetResultViewType::<Impl, IMPL_OFFSET>,
            GetDisplayInfo::<Impl, IMPL_OFFSET>,
            CompareObjects::<Impl, IMPL_OFFSET>,
            QueryDispatch::<Impl, IMPL_OFFSET>,
            GetResultViewType2::<Impl, IMPL_OFFSET>,
            RestoreResultView::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IComponentDataImpl: Sized {
    fn Initialize();
    fn CreateComponent();
    fn Notify();
    fn Destroy();
    fn QueryDataObject();
    fn GetDisplayInfo();
    fn CompareObjects();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IComponentDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComponentDataVtbl {
        unsafe extern "system" fn Initialize<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateComponent<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Notify<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Destroy<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryDataObject<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayInfo<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscopedataitem: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompareObjects<Impl: IComponentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobjecta: ::windows::core::RawPtr, lpdataobjectb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, CreateComponent::<Impl, IMPL_OFFSET>, Notify::<Impl, IMPL_OFFSET>, Destroy::<Impl, IMPL_OFFSET>, QueryDataObject::<Impl, IMPL_OFFSET>, GetDisplayInfo::<Impl, IMPL_OFFSET>, CompareObjects::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponentData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IComponentData2Impl: Sized + IComponentDataImpl {
    fn QueryDispatch();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IComponentData2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentData2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComponentData2Vtbl {
        unsafe extern "system" fn QueryDispatch<Impl: IComponentData2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, CreateComponent::<Impl, IMPL_OFFSET>, Notify::<Impl, IMPL_OFFSET>, Destroy::<Impl, IMPL_OFFSET>, QueryDataObject::<Impl, IMPL_OFFSET>, GetDisplayInfo::<Impl, IMPL_OFFSET>, CompareObjects::<Impl, IMPL_OFFSET>, QueryDispatch::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponentData2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IConsoleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsoleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConsoleVtbl {
        unsafe extern "system" fn SetHeader<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetToolbar<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoolbar: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryResultView<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryScopeImageList<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimagelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryResultImageList<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimagelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateAllViews<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr, data: super::super::Foundation::LPARAM, hint: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MessageBox<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpsztext: super::super::Foundation::PWSTR, lpsztitle: super::super::Foundation::PWSTR, fustyle: u32, piretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryConsoleVerb<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconsoleverb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectScopeItem<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hscopeitem: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMainWindow<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NewWindow<Impl: IConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hscopeitem: isize, loptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetHeader::<Impl, IMPL_OFFSET>,
            SetToolbar::<Impl, IMPL_OFFSET>,
            QueryResultView::<Impl, IMPL_OFFSET>,
            QueryScopeImageList::<Impl, IMPL_OFFSET>,
            QueryResultImageList::<Impl, IMPL_OFFSET>,
            UpdateAllViews::<Impl, IMPL_OFFSET>,
            MessageBox::<Impl, IMPL_OFFSET>,
            QueryConsoleVerb::<Impl, IMPL_OFFSET>,
            SelectScopeItem::<Impl, IMPL_OFFSET>,
            GetMainWindow::<Impl, IMPL_OFFSET>,
            NewWindow::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConsole as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IConsole2Impl: Sized + IConsoleImpl {
    fn Expand();
    fn IsTaskpadViewPreferred();
    fn SetStatusText();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IConsole2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsole2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConsole2Vtbl {
        unsafe extern "system" fn Expand<Impl: IConsole2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize, bexpand: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsTaskpadViewPreferred<Impl: IConsole2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStatusText<Impl: IConsole2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetHeader::<Impl, IMPL_OFFSET>,
            SetToolbar::<Impl, IMPL_OFFSET>,
            QueryResultView::<Impl, IMPL_OFFSET>,
            QueryScopeImageList::<Impl, IMPL_OFFSET>,
            QueryResultImageList::<Impl, IMPL_OFFSET>,
            UpdateAllViews::<Impl, IMPL_OFFSET>,
            MessageBox::<Impl, IMPL_OFFSET>,
            QueryConsoleVerb::<Impl, IMPL_OFFSET>,
            SelectScopeItem::<Impl, IMPL_OFFSET>,
            GetMainWindow::<Impl, IMPL_OFFSET>,
            NewWindow::<Impl, IMPL_OFFSET>,
            Expand::<Impl, IMPL_OFFSET>,
            IsTaskpadViewPreferred::<Impl, IMPL_OFFSET>,
            SetStatusText::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConsole2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IConsole3Impl: Sized + IConsole2Impl + IConsoleImpl {
    fn RenameScopeItem();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IConsole3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsole3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConsole3Vtbl {
        unsafe extern "system" fn RenameScopeItem<Impl: IConsole3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hscopeitem: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetHeader::<Impl, IMPL_OFFSET>,
            SetToolbar::<Impl, IMPL_OFFSET>,
            QueryResultView::<Impl, IMPL_OFFSET>,
            QueryScopeImageList::<Impl, IMPL_OFFSET>,
            QueryResultImageList::<Impl, IMPL_OFFSET>,
            UpdateAllViews::<Impl, IMPL_OFFSET>,
            MessageBox::<Impl, IMPL_OFFSET>,
            QueryConsoleVerb::<Impl, IMPL_OFFSET>,
            SelectScopeItem::<Impl, IMPL_OFFSET>,
            GetMainWindow::<Impl, IMPL_OFFSET>,
            NewWindow::<Impl, IMPL_OFFSET>,
            Expand::<Impl, IMPL_OFFSET>,
            IsTaskpadViewPreferred::<Impl, IMPL_OFFSET>,
            SetStatusText::<Impl, IMPL_OFFSET>,
            RenameScopeItem::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConsole3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IConsoleNameSpaceImpl: Sized {
    fn InsertItem();
    fn DeleteItem();
    fn SetItem();
    fn GetItem();
    fn GetChildItem();
    fn GetNextItem();
    fn GetParentItem();
}
#[cfg(feature = "Win32_Foundation")]
impl IConsoleNameSpaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsoleNameSpaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConsoleNameSpaceVtbl {
        unsafe extern "system" fn InsertItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize, fdeletethis: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *const SCOPEDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChildItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParentItem<Impl: IConsoleNameSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, InsertItem::<Impl, IMPL_OFFSET>, DeleteItem::<Impl, IMPL_OFFSET>, SetItem::<Impl, IMPL_OFFSET>, GetItem::<Impl, IMPL_OFFSET>, GetChildItem::<Impl, IMPL_OFFSET>, GetNextItem::<Impl, IMPL_OFFSET>, GetParentItem::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConsoleNameSpace as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IConsoleNameSpace2Impl: Sized + IConsoleNameSpaceImpl {
    fn Expand();
    fn AddExtension();
}
#[cfg(feature = "Win32_Foundation")]
impl IConsoleNameSpace2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsoleNameSpace2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConsoleNameSpace2Vtbl {
        unsafe extern "system" fn Expand<Impl: IConsoleNameSpace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddExtension<Impl: IConsoleNameSpace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize, lpclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, InsertItem::<Impl, IMPL_OFFSET>, DeleteItem::<Impl, IMPL_OFFSET>, SetItem::<Impl, IMPL_OFFSET>, GetItem::<Impl, IMPL_OFFSET>, GetChildItem::<Impl, IMPL_OFFSET>, GetNextItem::<Impl, IMPL_OFFSET>, GetParentItem::<Impl, IMPL_OFFSET>, Expand::<Impl, IMPL_OFFSET>, AddExtension::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConsoleNameSpace2 as ::windows::core::Interface>::IID
    }
}
pub trait IConsolePowerImpl: Sized {
    fn SetExecutionState();
    fn ResetIdleTimer();
}
impl IConsolePowerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsolePowerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConsolePowerVtbl {
        unsafe extern "system" fn SetExecutionState<Impl: IConsolePowerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwadd: u32, dwremove: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResetIdleTimer<Impl: IConsolePowerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetExecutionState::<Impl, IMPL_OFFSET>, ResetIdleTimer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConsolePower as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IConsolePowerSinkImpl: Sized {
    fn OnPowerBroadcast();
}
#[cfg(feature = "Win32_Foundation")]
impl IConsolePowerSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsolePowerSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConsolePowerSinkVtbl {
        unsafe extern "system" fn OnPowerBroadcast<Impl: IConsolePowerSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nevent: u32, lparam: super::super::Foundation::LPARAM, plreturn: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnPowerBroadcast::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConsolePowerSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IConsoleVerbImpl: Sized {
    fn GetVerbState();
    fn SetVerbState();
    fn SetDefaultVerb();
    fn GetDefaultVerb();
}
#[cfg(feature = "Win32_Foundation")]
impl IConsoleVerbVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConsoleVerbImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConsoleVerbVtbl {
        unsafe extern "system" fn GetVerbState<Impl: IConsoleVerbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVerbState<Impl: IConsoleVerbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultVerb<Impl: IConsoleVerbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultVerb<Impl: IConsoleVerbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pecmdid: *mut MMC_CONSOLE_VERB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetVerbState::<Impl, IMPL_OFFSET>, SetVerbState::<Impl, IMPL_OFFSET>, SetDefaultVerb::<Impl, IMPL_OFFSET>, GetDefaultVerb::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConsoleVerb as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContextMenuCallbackImpl: Sized {
    fn AddItem();
}
#[cfg(feature = "Win32_Foundation")]
impl IContextMenuCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextMenuCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContextMenuCallbackVtbl {
        unsafe extern "system" fn AddItem<Impl: IContextMenuCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *const CONTEXTMENUITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddItem::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextMenuCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContextMenuCallback2Impl: Sized {
    fn AddItem();
}
#[cfg(feature = "Win32_Foundation")]
impl IContextMenuCallback2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextMenuCallback2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContextMenuCallback2Vtbl {
        unsafe extern "system" fn AddItem<Impl: IContextMenuCallback2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *const CONTEXTMENUITEM2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddItem::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextMenuCallback2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IContextMenuProviderImpl: Sized + IContextMenuCallbackImpl {
    fn EmptyMenuList();
    fn AddPrimaryExtensionItems();
    fn AddThirdPartyExtensionItems();
    fn ShowContextMenu();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IContextMenuProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextMenuProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContextMenuProviderVtbl {
        unsafe extern "system" fn EmptyMenuList<Impl: IContextMenuProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddPrimaryExtensionItems<Impl: IContextMenuProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piextension: *mut ::core::ffi::c_void, pidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddThirdPartyExtensionItems<Impl: IContextMenuProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShowContextMenu<Impl: IContextMenuProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, xpos: i32, ypos: i32, plselected: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddItem::<Impl, IMPL_OFFSET>, EmptyMenuList::<Impl, IMPL_OFFSET>, AddPrimaryExtensionItems::<Impl, IMPL_OFFSET>, AddThirdPartyExtensionItems::<Impl, IMPL_OFFSET>, ShowContextMenu::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextMenuProvider as ::windows::core::Interface>::IID
    }
}
pub trait IControlbarImpl: Sized {
    fn Create();
    fn Attach();
    fn Detach();
}
impl IControlbarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlbarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IControlbarVtbl {
        unsafe extern "system" fn Create<Impl: IControlbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: ::windows::core::RawPtr, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Attach<Impl: IControlbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntype: MMC_CONTROL_TYPE, lpunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Detach<Impl: IControlbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Create::<Impl, IMPL_OFFSET>, Attach::<Impl, IMPL_OFFSET>, Detach::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlbar as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDisplayHelpImpl: Sized {
    fn ShowTopic();
}
#[cfg(feature = "Win32_Foundation")]
impl IDisplayHelpVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayHelpImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayHelpVtbl {
        unsafe extern "system" fn ShowTopic<Impl: IDisplayHelpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelptopic: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ShowTopic::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayHelp as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumTASKImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumTASKVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTASKImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTASKVtbl {
        unsafe extern "system" fn Next<Impl: IEnumTASKImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut MMC_TASK, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTASKImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTASKImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumTASKImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTASK as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IExtendContextMenuImpl: Sized {
    fn AddMenuItems();
    fn Command();
}
#[cfg(feature = "Win32_System_Com")]
impl IExtendContextMenuVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendContextMenuImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExtendContextMenuVtbl {
        unsafe extern "system" fn AddMenuItems<Impl: IExtendContextMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidataobject: ::windows::core::RawPtr, picallback: ::windows::core::RawPtr, pinsertionallowed: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Command<Impl: IExtendContextMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcommandid: i32, pidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddMenuItems::<Impl, IMPL_OFFSET>, Command::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExtendContextMenu as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IExtendControlbarImpl: Sized {
    fn SetControlbar();
    fn ControlbarNotify();
}
#[cfg(feature = "Win32_Foundation")]
impl IExtendControlbarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendControlbarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExtendControlbarVtbl {
        unsafe extern "system" fn SetControlbar<Impl: IExtendControlbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontrolbar: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ControlbarNotify<Impl: IExtendControlbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetControlbar::<Impl, IMPL_OFFSET>, ControlbarNotify::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExtendControlbar as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IExtendPropertySheetImpl: Sized {
    fn CreatePropertyPages();
    fn QueryPagesFor();
}
#[cfg(feature = "Win32_System_Com")]
impl IExtendPropertySheetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendPropertySheetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExtendPropertySheetVtbl {
        unsafe extern "system" fn CreatePropertyPages<Impl: IExtendPropertySheetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpprovider: ::windows::core::RawPtr, handle: isize, lpidataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryPagesFor<Impl: IExtendPropertySheetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreatePropertyPages::<Impl, IMPL_OFFSET>, QueryPagesFor::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExtendPropertySheet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IExtendPropertySheet2Impl: Sized + IExtendPropertySheetImpl {
    fn GetWatermarks();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IExtendPropertySheet2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendPropertySheet2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExtendPropertySheet2Vtbl {
        unsafe extern "system" fn GetWatermarks<Impl: IExtendPropertySheet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpidataobject: ::windows::core::RawPtr, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreatePropertyPages::<Impl, IMPL_OFFSET>, QueryPagesFor::<Impl, IMPL_OFFSET>, GetWatermarks::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExtendPropertySheet2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IExtendTaskPadImpl: Sized {
    fn TaskNotify();
    fn EnumTasks();
    fn GetTitle();
    fn GetDescriptiveText();
    fn GetBackground();
    fn GetListPadInfo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IExtendTaskPadVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendTaskPadImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExtendTaskPadVtbl {
        unsafe extern "system" fn TaskNotify<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdo: ::windows::core::RawPtr, arg: *const super::Com::VARIANT, param2: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumTasks<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdo: ::windows::core::RawPtr, sztaskgroup: super::super::Foundation::PWSTR, ppenumtask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTitle<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgroup: super::super::Foundation::PWSTR, psztitle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescriptiveText<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgroup: super::super::Foundation::PWSTR, pszdescriptivetext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackground<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgroup: super::super::Foundation::PWSTR, ptdo: *mut MMC_TASK_DISPLAY_OBJECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetListPadInfo<Impl: IExtendTaskPadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgroup: super::super::Foundation::PWSTR, lplistpadinfo: *mut MMC_LISTPAD_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, TaskNotify::<Impl, IMPL_OFFSET>, EnumTasks::<Impl, IMPL_OFFSET>, GetTitle::<Impl, IMPL_OFFSET>, GetDescriptiveText::<Impl, IMPL_OFFSET>, GetBackground::<Impl, IMPL_OFFSET>, GetListPadInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExtendTaskPad as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IExtendViewImpl: Sized {
    fn GetViews();
}
#[cfg(feature = "Win32_System_Com")]
impl IExtendViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExtendViewVtbl {
        unsafe extern "system" fn GetViews<Impl: IExtendViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, pviewextensioncallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetViews::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExtendView as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHeaderCtrlImpl: Sized {
    fn InsertColumn();
    fn DeleteColumn();
    fn SetColumnText();
    fn GetColumnText();
    fn SetColumnWidth();
    fn GetColumnWidth();
}
#[cfg(feature = "Win32_Foundation")]
impl IHeaderCtrlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHeaderCtrlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHeaderCtrlVtbl {
        unsafe extern "system" fn InsertColumn<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, title: super::super::Foundation::PWSTR, nformat: i32, nwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteColumn<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColumnText<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, title: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnText<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, ptext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColumnWidth<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, nwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnWidth<Impl: IHeaderCtrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncol: i32, pwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, InsertColumn::<Impl, IMPL_OFFSET>, DeleteColumn::<Impl, IMPL_OFFSET>, SetColumnText::<Impl, IMPL_OFFSET>, GetColumnText::<Impl, IMPL_OFFSET>, SetColumnWidth::<Impl, IMPL_OFFSET>, GetColumnWidth::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHeaderCtrl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHeaderCtrl2Impl: Sized + IHeaderCtrlImpl {
    fn SetChangeTimeOut();
    fn SetColumnFilter();
    fn GetColumnFilter();
}
#[cfg(feature = "Win32_Foundation")]
impl IHeaderCtrl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHeaderCtrl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHeaderCtrl2Vtbl {
        unsafe extern "system" fn SetChangeTimeOut<Impl: IHeaderCtrl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, utimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColumnFilter<Impl: IHeaderCtrl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnFilter<Impl: IHeaderCtrl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            InsertColumn::<Impl, IMPL_OFFSET>,
            DeleteColumn::<Impl, IMPL_OFFSET>,
            SetColumnText::<Impl, IMPL_OFFSET>,
            GetColumnText::<Impl, IMPL_OFFSET>,
            SetColumnWidth::<Impl, IMPL_OFFSET>,
            GetColumnWidth::<Impl, IMPL_OFFSET>,
            SetChangeTimeOut::<Impl, IMPL_OFFSET>,
            SetColumnFilter::<Impl, IMPL_OFFSET>,
            GetColumnFilter::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHeaderCtrl2 as ::windows::core::Interface>::IID
    }
}
pub trait IImageListImpl: Sized {
    fn ImageListSetIcon();
    fn ImageListSetStrip();
}
impl IImageListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageListVtbl {
        unsafe extern "system" fn ImageListSetIcon<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picon: *const isize, nloc: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImageListSetStrip<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ImageListSetIcon::<Impl, IMPL_OFFSET>, ImageListSetStrip::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageList as ::windows::core::Interface>::IID
    }
}
pub trait IMMCVersionInfoImpl: Sized {
    fn GetMMCVersion();
}
impl IMMCVersionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMMCVersionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMMCVersionInfoVtbl {
        unsafe extern "system" fn GetMMCVersion<Impl: IMMCVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversionmajor: *mut i32, pversionminor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetMMCVersion::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMMCVersionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMenuButtonImpl: Sized {
    fn AddButton();
    fn SetButton();
    fn SetButtonState();
}
#[cfg(feature = "Win32_Foundation")]
impl IMenuButtonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuButtonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuButtonVtbl {
        unsafe extern "system" fn AddButton<Impl: IMenuButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, lpbuttontext: super::super::Foundation::PWSTR, lptooltiptext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetButton<Impl: IMenuButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, lpbuttontext: super::super::Foundation::PWSTR, lptooltiptext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetButtonState<Impl: IMenuButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddButton::<Impl, IMPL_OFFSET>, SetButton::<Impl, IMPL_OFFSET>, SetButtonState::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMenuButton as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMessageViewImpl: Sized {
    fn SetTitleText();
    fn SetBodyText();
    fn SetIcon();
    fn Clear();
}
#[cfg(feature = "Win32_Foundation")]
impl IMessageViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageViewVtbl {
        unsafe extern "system" fn SetTitleText<Impl: IMessageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztitletext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBodyText<Impl: IMessageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbodytext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIcon<Impl: IMessageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: IconIdentifier) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IMessageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetTitleText::<Impl, IMPL_OFFSET>, SetBodyText::<Impl, IMPL_OFFSET>, SetIcon::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait INodePropertiesImpl: Sized {
    fn GetProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl INodePropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INodePropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INodePropertiesVtbl {
        unsafe extern "system" fn GetProperty<Impl: INodePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, szpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrproperty: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INodeProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Controls")]
pub trait IPropertySheetCallbackImpl: Sized {
    fn AddPage();
    fn RemovePage();
}
#[cfg(feature = "Win32_UI_Controls")]
impl IPropertySheetCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySheetCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertySheetCallbackVtbl {
        unsafe extern "system" fn AddPage<Impl: IPropertySheetCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemovePage<Impl: IPropertySheetCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddPage::<Impl, IMPL_OFFSET>, RemovePage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertySheetCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPropertySheetProviderImpl: Sized {
    fn CreatePropertySheet();
    fn FindPropertySheet();
    fn AddPrimaryPages();
    fn AddExtensionPages();
    fn Show();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPropertySheetProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySheetProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertySheetProviderVtbl {
        unsafe extern "system" fn CreatePropertySheet<Impl: IPropertySheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: super::super::Foundation::PWSTR, r#type: u8, cookie: isize, pidataobjectm: ::windows::core::RawPtr, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindPropertySheet<Impl: IPropertySheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hitem: isize, lpcomponent: ::windows::core::RawPtr, lpdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddPrimaryPages<Impl: IPropertySheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpunknown: *mut ::core::ffi::c_void, bcreatehandle: super::super::Foundation::BOOL, hnotifywindow: super::super::Foundation::HWND, bscopepane: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddExtensionPages<Impl: IPropertySheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Show<Impl: IPropertySheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: isize, page: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreatePropertySheet::<Impl, IMPL_OFFSET>, FindPropertySheet::<Impl, IMPL_OFFSET>, AddPrimaryPages::<Impl, IMPL_OFFSET>, AddExtensionPages::<Impl, IMPL_OFFSET>, Show::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertySheetProvider as ::windows::core::Interface>::IID
    }
}
pub trait IRequiredExtensionsImpl: Sized {
    fn EnableAllExtensions();
    fn GetFirstExtension();
    fn GetNextExtension();
}
impl IRequiredExtensionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRequiredExtensionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRequiredExtensionsVtbl {
        unsafe extern "system" fn EnableAllExtensions<Impl: IRequiredExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFirstExtension<Impl: IRequiredExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextExtension<Impl: IRequiredExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, EnableAllExtensions::<Impl, IMPL_OFFSET>, GetFirstExtension::<Impl, IMPL_OFFSET>, GetNextExtension::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRequiredExtensions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IResultDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResultDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResultDataVtbl {
        unsafe extern "system" fn InsertItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: isize, ncol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindItemByLParam<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM, pitemid: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAllRsltItems<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *const RESULTDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModifyItemState<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModifyViewStyle<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetViewMode<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lviewmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetViewMode<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lviewmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateItem<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Sort<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescBarText<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desctext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetItemCount<Impl: IResultDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemcount: i32, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            InsertItem::<Impl, IMPL_OFFSET>,
            DeleteItem::<Impl, IMPL_OFFSET>,
            FindItemByLParam::<Impl, IMPL_OFFSET>,
            DeleteAllRsltItems::<Impl, IMPL_OFFSET>,
            SetItem::<Impl, IMPL_OFFSET>,
            GetItem::<Impl, IMPL_OFFSET>,
            GetNextItem::<Impl, IMPL_OFFSET>,
            ModifyItemState::<Impl, IMPL_OFFSET>,
            ModifyViewStyle::<Impl, IMPL_OFFSET>,
            SetViewMode::<Impl, IMPL_OFFSET>,
            GetViewMode::<Impl, IMPL_OFFSET>,
            UpdateItem::<Impl, IMPL_OFFSET>,
            Sort::<Impl, IMPL_OFFSET>,
            SetDescBarText::<Impl, IMPL_OFFSET>,
            SetItemCount::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResultData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResultData2Impl: Sized + IResultDataImpl {
    fn RenameResultItem();
}
#[cfg(feature = "Win32_Foundation")]
impl IResultData2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResultData2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResultData2Vtbl {
        unsafe extern "system" fn RenameResultItem<Impl: IResultData2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            InsertItem::<Impl, IMPL_OFFSET>,
            DeleteItem::<Impl, IMPL_OFFSET>,
            FindItemByLParam::<Impl, IMPL_OFFSET>,
            DeleteAllRsltItems::<Impl, IMPL_OFFSET>,
            SetItem::<Impl, IMPL_OFFSET>,
            GetItem::<Impl, IMPL_OFFSET>,
            GetNextItem::<Impl, IMPL_OFFSET>,
            ModifyItemState::<Impl, IMPL_OFFSET>,
            ModifyViewStyle::<Impl, IMPL_OFFSET>,
            SetViewMode::<Impl, IMPL_OFFSET>,
            GetViewMode::<Impl, IMPL_OFFSET>,
            UpdateItem::<Impl, IMPL_OFFSET>,
            Sort::<Impl, IMPL_OFFSET>,
            SetDescBarText::<Impl, IMPL_OFFSET>,
            SetItemCount::<Impl, IMPL_OFFSET>,
            RenameResultItem::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResultData2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResultDataCompareImpl: Sized {
    fn Compare();
}
#[cfg(feature = "Win32_Foundation")]
impl IResultDataCompareVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResultDataCompareImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResultDataCompareVtbl {
        unsafe extern "system" fn Compare<Impl: IResultDataCompareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, luserparam: super::super::Foundation::LPARAM, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Compare::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResultDataCompare as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResultDataCompareExImpl: Sized {
    fn Compare();
}
#[cfg(feature = "Win32_Foundation")]
impl IResultDataCompareExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResultDataCompareExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResultDataCompareExVtbl {
        unsafe extern "system" fn Compare<Impl: IResultDataCompareExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prdc: *const RDCOMPARE, pnresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Compare::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResultDataCompareEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResultOwnerDataImpl: Sized {
    fn FindItem();
    fn CacheHint();
    fn SortItems();
}
#[cfg(feature = "Win32_Foundation")]
impl IResultOwnerDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResultOwnerDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResultOwnerDataVtbl {
        unsafe extern "system" fn FindItem<Impl: IResultOwnerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfindinfo: *const RESULTFINDINFO, pnfoundindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CacheHint<Impl: IResultOwnerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nstartindex: i32, nendindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SortItems<Impl: IResultOwnerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, FindItem::<Impl, IMPL_OFFSET>, CacheHint::<Impl, IMPL_OFFSET>, SortItems::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResultOwnerData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ISnapinAboutImpl: Sized {
    fn GetSnapinDescription();
    fn GetProvider();
    fn GetSnapinVersion();
    fn GetSnapinImage();
    fn GetStaticFolderImage();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ISnapinAboutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISnapinAboutImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISnapinAboutVtbl {
        unsafe extern "system" fn GetSnapinDescription<Impl: ISnapinAboutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProvider<Impl: ISnapinAboutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSnapinVersion<Impl: ISnapinAboutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpversion: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSnapinImage<Impl: ISnapinAboutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, happicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStaticFolderImage<Impl: ISnapinAboutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSnapinDescription::<Impl, IMPL_OFFSET>, GetProvider::<Impl, IMPL_OFFSET>, GetSnapinVersion::<Impl, IMPL_OFFSET>, GetSnapinImage::<Impl, IMPL_OFFSET>, GetStaticFolderImage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISnapinAbout as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISnapinHelpImpl: Sized {
    fn GetHelpTopic();
}
#[cfg(feature = "Win32_Foundation")]
impl ISnapinHelpVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISnapinHelpImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISnapinHelpVtbl {
        unsafe extern "system" fn GetHelpTopic<Impl: ISnapinHelpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcompiledhelpfile: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetHelpTopic::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISnapinHelp as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISnapinHelp2Impl: Sized + ISnapinHelpImpl {
    fn GetLinkedTopics();
}
#[cfg(feature = "Win32_Foundation")]
impl ISnapinHelp2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISnapinHelp2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISnapinHelp2Vtbl {
        unsafe extern "system" fn GetLinkedTopics<Impl: ISnapinHelp2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcompiledhelpfiles: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetHelpTopic::<Impl, IMPL_OFFSET>, GetLinkedTopics::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISnapinHelp2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISnapinPropertiesImpl: Sized {
    fn Initialize();
    fn QueryPropertyNames();
    fn PropertiesChanged();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISnapinPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISnapinPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISnapinPropertiesVtbl {
        unsafe extern "system" fn Initialize<Impl: ISnapinPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryPropertyNames<Impl: ISnapinPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PropertiesChanged<Impl: ISnapinPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: i32, pproperties: *const MMC_SNAPIN_PROPERTY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, QueryPropertyNames::<Impl, IMPL_OFFSET>, PropertiesChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISnapinProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISnapinPropertiesCallbackImpl: Sized {
    fn AddPropertyName();
}
#[cfg(feature = "Win32_Foundation")]
impl ISnapinPropertiesCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISnapinPropertiesCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISnapinPropertiesCallbackVtbl {
        unsafe extern "system" fn AddPropertyName<Impl: ISnapinPropertiesCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddPropertyName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISnapinPropertiesCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IStringTableImpl: Sized {
    fn AddString();
    fn GetString();
    fn GetStringLength();
    fn DeleteString();
    fn DeleteAllStrings();
    fn FindString();
    fn Enumerate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IStringTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStringTableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStringTableVtbl {
        unsafe extern "system" fn AddString<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszadd: super::super::Foundation::PWSTR, pstringid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetString<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stringid: u32, cchbuffer: u32, lpbuffer: super::super::Foundation::PWSTR, pcchout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStringLength<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stringid: u32, pcchstring: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteString<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stringid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAllStrings<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindString<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfind: super::super::Foundation::PWSTR, pstringid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enumerate<Impl: IStringTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddString::<Impl, IMPL_OFFSET>, GetString::<Impl, IMPL_OFFSET>, GetStringLength::<Impl, IMPL_OFFSET>, DeleteString::<Impl, IMPL_OFFSET>, DeleteAllStrings::<Impl, IMPL_OFFSET>, FindString::<Impl, IMPL_OFFSET>, Enumerate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStringTable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IToolbarImpl: Sized {
    fn AddBitmap();
    fn AddButtons();
    fn InsertButton();
    fn DeleteButton();
    fn GetButtonState();
    fn SetButtonState();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IToolbarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToolbarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToolbarVtbl {
        unsafe extern "system" fn AddBitmap<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nimages: i32, hbmp: super::super::Graphics::Gdi::HBITMAP, cxsize: i32, cysize: i32, crmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddButtons<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertButton<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, lpbutton: *const MMCBUTTON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteButton<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetButtonState<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetButtonState<Impl: IToolbarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddBitmap::<Impl, IMPL_OFFSET>, AddButtons::<Impl, IMPL_OFFSET>, InsertButton::<Impl, IMPL_OFFSET>, DeleteButton::<Impl, IMPL_OFFSET>, GetButtonState::<Impl, IMPL_OFFSET>, SetButtonState::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToolbar as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IViewExtensionCallbackImpl: Sized {
    fn AddView();
}
#[cfg(feature = "Win32_Foundation")]
impl IViewExtensionCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewExtensionCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IViewExtensionCallbackVtbl {
        unsafe extern "system" fn AddView<Impl: IViewExtensionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextviewdata: *const MMC_EXT_VIEW_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddView::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewExtensionCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait MenuItemImpl: Sized + IDispatchImpl {
    fn DisplayName();
    fn LanguageIndependentName();
    fn Path();
    fn LanguageIndependentPath();
    fn Execute();
    fn Enabled();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl MenuItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: MenuItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> MenuItemVtbl {
        unsafe extern "system" fn DisplayName<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LanguageIndependentName<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageindependentname: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Path<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LanguageIndependentPath<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageindependentpath: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Execute<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: MenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            LanguageIndependentName::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            LanguageIndependentPath::<Impl, IMPL_OFFSET>,
            Execute::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<MenuItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait NodeImpl: Sized + IDispatchImpl {
    fn Name();
    fn Property();
    fn Bookmark();
    fn IsScopeNode();
    fn Nodetype();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl NodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: NodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> NodeVtbl {
        unsafe extern "system" fn Name<Impl: NodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Property<Impl: NodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Bookmark<Impl: NodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bookmark: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsScopeNode<Impl: NodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isscopenode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Nodetype<Impl: NodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodetype: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, Property::<Impl, IMPL_OFFSET>, Bookmark::<Impl, IMPL_OFFSET>, IsScopeNode::<Impl, IMPL_OFFSET>, Nodetype::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Node as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait NodesImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl NodesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: NodesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> NodesVtbl {
        unsafe extern "system" fn _NewEnum<Impl: NodesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: NodesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: NodesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Nodes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait PropertiesImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Remove();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl PropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: PropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> PropertiesVtbl {
        unsafe extern "system" fn _NewEnum<Impl: PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Properties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait PropertyImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
    fn Name();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl PropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: PropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> PropertyVtbl {
        unsafe extern "system" fn Value<Impl: PropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: PropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: PropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Value::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Property as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ScopeNamespaceImpl: Sized + IDispatchImpl {
    fn GetParent();
    fn GetChild();
    fn GetNext();
    fn GetRoot();
    fn Expand();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ScopeNamespaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ScopeNamespaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ScopeNamespaceVtbl {
        unsafe extern "system" fn GetParent<Impl: ScopeNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChild<Impl: ScopeNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, child: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNext<Impl: ScopeNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, next: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRoot<Impl: ScopeNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, root: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Expand<Impl: ScopeNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, GetParent::<Impl, IMPL_OFFSET>, GetChild::<Impl, IMPL_OFFSET>, GetNext::<Impl, IMPL_OFFSET>, GetRoot::<Impl, IMPL_OFFSET>, Expand::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ScopeNamespace as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait SnapInImpl: Sized + IDispatchImpl {
    fn Name();
    fn Vendor();
    fn Version();
    fn Extensions();
    fn SnapinCLSID();
    fn Properties();
    fn EnableAllExtensions();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl SnapInVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: SnapInImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> SnapInVtbl {
        unsafe extern "system" fn Name<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Vendor<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendor: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Version<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Extensions<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SnapinCLSID<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapinclsid: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableAllExtensions<Impl: SnapInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Vendor::<Impl, IMPL_OFFSET>,
            Version::<Impl, IMPL_OFFSET>,
            Extensions::<Impl, IMPL_OFFSET>,
            SnapinCLSID::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
            EnableAllExtensions::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<SnapIn as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait SnapInsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl SnapInsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: SnapInsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> SnapInsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: SnapInsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: SnapInsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, snapin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: SnapInsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: SnapInsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapinnameorclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, parentsnapin: ::core::mem::ManuallyDrop<super::Com::VARIANT>, properties: ::core::mem::ManuallyDrop<super::Com::VARIANT>, snapin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: SnapInsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<SnapIns as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ViewVtbl {
        unsafe extern "system" fn ActiveScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActiveScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Selection<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ListItems<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SnapinScopeObject<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>, scopenodeobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SnapinSelectionObject<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Is<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, thesame: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Document<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectAll<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Select<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Deselect<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSelected<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, isselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayScopeNodePropertySheet<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplaySelectionPropertySheet<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopySelection<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteSelection<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RenameScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RenameSelectedItem<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScopeNodeContextMenu<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>, contextmenu: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectionContextMenu<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextmenu: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RefreshScopeNode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RefreshSelection<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecuteSelectionMenuItem<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, menuitempath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecuteScopeNodeMenuItem<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, menuitempath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecuteShellCommand<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, directory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, parameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, windowstate: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Frame<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScopeTreeVisible<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScopeTreeVisible<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Back<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Forward<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStatusBarText<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statusbartext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Memento<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memento: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ViewMemento<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memento: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Columns<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, columns: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CellContents<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, column: i32, cellcontents: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExportList<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exportoptions: _ExportListOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ListViewMode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut _ListViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetListViewMode<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: _ListViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ControlObject<Impl: ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, control: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            ActiveScopeNode::<Impl, IMPL_OFFSET>,
            SetActiveScopeNode::<Impl, IMPL_OFFSET>,
            Selection::<Impl, IMPL_OFFSET>,
            ListItems::<Impl, IMPL_OFFSET>,
            SnapinScopeObject::<Impl, IMPL_OFFSET>,
            SnapinSelectionObject::<Impl, IMPL_OFFSET>,
            Is::<Impl, IMPL_OFFSET>,
            Document::<Impl, IMPL_OFFSET>,
            SelectAll::<Impl, IMPL_OFFSET>,
            Select::<Impl, IMPL_OFFSET>,
            Deselect::<Impl, IMPL_OFFSET>,
            IsSelected::<Impl, IMPL_OFFSET>,
            DisplayScopeNodePropertySheet::<Impl, IMPL_OFFSET>,
            DisplaySelectionPropertySheet::<Impl, IMPL_OFFSET>,
            CopyScopeNode::<Impl, IMPL_OFFSET>,
            CopySelection::<Impl, IMPL_OFFSET>,
            DeleteScopeNode::<Impl, IMPL_OFFSET>,
            DeleteSelection::<Impl, IMPL_OFFSET>,
            RenameScopeNode::<Impl, IMPL_OFFSET>,
            RenameSelectedItem::<Impl, IMPL_OFFSET>,
            ScopeNodeContextMenu::<Impl, IMPL_OFFSET>,
            SelectionContextMenu::<Impl, IMPL_OFFSET>,
            RefreshScopeNode::<Impl, IMPL_OFFSET>,
            RefreshSelection::<Impl, IMPL_OFFSET>,
            ExecuteSelectionMenuItem::<Impl, IMPL_OFFSET>,
            ExecuteScopeNodeMenuItem::<Impl, IMPL_OFFSET>,
            ExecuteShellCommand::<Impl, IMPL_OFFSET>,
            Frame::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            ScopeTreeVisible::<Impl, IMPL_OFFSET>,
            SetScopeTreeVisible::<Impl, IMPL_OFFSET>,
            Back::<Impl, IMPL_OFFSET>,
            Forward::<Impl, IMPL_OFFSET>,
            SetStatusBarText::<Impl, IMPL_OFFSET>,
            Memento::<Impl, IMPL_OFFSET>,
            ViewMemento::<Impl, IMPL_OFFSET>,
            Columns::<Impl, IMPL_OFFSET>,
            CellContents::<Impl, IMPL_OFFSET>,
            ExportList::<Impl, IMPL_OFFSET>,
            ListViewMode::<Impl, IMPL_OFFSET>,
            SetListViewMode::<Impl, IMPL_OFFSET>,
            ControlObject::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<View as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ViewsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn Add();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ViewsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ViewsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ViewsVtbl {
        unsafe extern "system" fn Item<Impl: ViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, view: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, viewoptions: _ViewOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<Views as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _AppEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _AppEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _AppEventsVtbl {
        unsafe extern "system" fn OnQuit<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDocumentOpen<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr, new: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDocumentClose<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSnapInAdded<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr, snapin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSnapInRemoved<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr, snapin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnNewView<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnViewClose<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnViewChange<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, newownernode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSelectionChange<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, newnodes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnContextMenuExecuted<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, menuitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnToolbarButtonClicked<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnListUpdated<Impl: _AppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            OnQuit::<Impl, IMPL_OFFSET>,
            OnDocumentOpen::<Impl, IMPL_OFFSET>,
            OnDocumentClose::<Impl, IMPL_OFFSET>,
            OnSnapInAdded::<Impl, IMPL_OFFSET>,
            OnSnapInRemoved::<Impl, IMPL_OFFSET>,
            OnNewView::<Impl, IMPL_OFFSET>,
            OnViewClose::<Impl, IMPL_OFFSET>,
            OnViewChange::<Impl, IMPL_OFFSET>,
            OnSelectionChange::<Impl, IMPL_OFFSET>,
            OnContextMenuExecuted::<Impl, IMPL_OFFSET>,
            OnToolbarButtonClicked::<Impl, IMPL_OFFSET>,
            OnListUpdated::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_AppEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _ApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ApplicationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _ApplicationVtbl {
        unsafe extern "system" fn Help<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Quit<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Document<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Load<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Frame<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Visible<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Show<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Hide<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserControl<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usercontrol: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUserControl<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usercontrol: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VersionMajor<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, versionmajor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VersionMinor<Impl: _ApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, versionminor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Help::<Impl, IMPL_OFFSET>,
            Quit::<Impl, IMPL_OFFSET>,
            Document::<Impl, IMPL_OFFSET>,
            Load::<Impl, IMPL_OFFSET>,
            Frame::<Impl, IMPL_OFFSET>,
            Visible::<Impl, IMPL_OFFSET>,
            Show::<Impl, IMPL_OFFSET>,
            Hide::<Impl, IMPL_OFFSET>,
            UserControl::<Impl, IMPL_OFFSET>,
            SetUserControl::<Impl, IMPL_OFFSET>,
            VersionMajor::<Impl, IMPL_OFFSET>,
            VersionMinor::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_Application as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _EventConnectorImpl: Sized + IDispatchImpl {
    fn ConnectTo();
    fn Disconnect();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _EventConnectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _EventConnectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _EventConnectorVtbl {
        unsafe extern "system" fn ConnectTo<Impl: _EventConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disconnect<Impl: _EventConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, ConnectTo::<Impl, IMPL_OFFSET>, Disconnect::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_EventConnector as ::windows::core::Interface>::IID
    }
}
