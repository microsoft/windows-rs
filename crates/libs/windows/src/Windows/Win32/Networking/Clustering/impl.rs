#[cfg(feature = "Win32_Foundation")]
pub trait IGetClusterDataInfo_Impl: Sized {
    fn GetClusterName(&self, lpszname: super::super::Foundation::BSTR, pcchname: *mut i32) -> ::windows::core::Result<()>;
    fn GetClusterHandle(&self) -> *mut _HCLUSTER;
    fn GetObjectCount(&self) -> i32;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IGetClusterDataInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IGetClusterDataInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterDataInfo_Impl, const OFFSET: isize>() -> IGetClusterDataInfo_Vtbl {
        unsafe extern "system" fn GetClusterName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterDataInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcchname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClusterName(::core::mem::transmute_copy(&lpszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetClusterHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterDataInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut _HCLUSTER {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClusterHandle()
        }
        unsafe extern "system" fn GetObjectCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterDataInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectCount()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetClusterName: GetClusterName::<Identity, Impl, OFFSET>,
            GetClusterHandle: GetClusterHandle::<Identity, Impl, OFFSET>,
            GetObjectCount: GetObjectCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetClusterDataInfo as ::windows::core::Interface>::IID
    }
}
pub trait IGetClusterGroupInfo_Impl: Sized {
    fn GetGroupHandle(&self, lobjindex: i32) -> *mut _HGROUP;
}
impl ::windows::core::RuntimeName for IGetClusterGroupInfo {}
impl IGetClusterGroupInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterGroupInfo_Impl, const OFFSET: isize>() -> IGetClusterGroupInfo_Vtbl {
        unsafe extern "system" fn GetGroupHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterGroupInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HGROUP {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGroupHandle(::core::mem::transmute_copy(&lobjindex))
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetGroupHandle: GetGroupHandle::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetClusterGroupInfo as ::windows::core::Interface>::IID
    }
}
pub trait IGetClusterNetInterfaceInfo_Impl: Sized {
    fn GetNetInterfaceHandle(&self, lobjindex: i32) -> *mut _HNETINTERFACE;
}
impl ::windows::core::RuntimeName for IGetClusterNetInterfaceInfo {}
impl IGetClusterNetInterfaceInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterNetInterfaceInfo_Impl, const OFFSET: isize>() -> IGetClusterNetInterfaceInfo_Vtbl {
        unsafe extern "system" fn GetNetInterfaceHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterNetInterfaceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HNETINTERFACE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNetInterfaceHandle(::core::mem::transmute_copy(&lobjindex))
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetNetInterfaceHandle: GetNetInterfaceHandle::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetClusterNetInterfaceInfo as ::windows::core::Interface>::IID
    }
}
pub trait IGetClusterNetworkInfo_Impl: Sized {
    fn GetNetworkHandle(&self, lobjindex: i32) -> *mut _HNETWORK;
}
impl ::windows::core::RuntimeName for IGetClusterNetworkInfo {}
impl IGetClusterNetworkInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterNetworkInfo_Impl, const OFFSET: isize>() -> IGetClusterNetworkInfo_Vtbl {
        unsafe extern "system" fn GetNetworkHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterNetworkInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HNETWORK {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNetworkHandle(::core::mem::transmute_copy(&lobjindex))
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetNetworkHandle: GetNetworkHandle::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetClusterNetworkInfo as ::windows::core::Interface>::IID
    }
}
pub trait IGetClusterNodeInfo_Impl: Sized {
    fn GetNodeHandle(&self, lobjindex: i32) -> *mut _HNODE;
}
impl ::windows::core::RuntimeName for IGetClusterNodeInfo {}
impl IGetClusterNodeInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterNodeInfo_Impl, const OFFSET: isize>() -> IGetClusterNodeInfo_Vtbl {
        unsafe extern "system" fn GetNodeHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterNodeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HNODE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNodeHandle(::core::mem::transmute_copy(&lobjindex))
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetNodeHandle: GetNodeHandle::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetClusterNodeInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGetClusterObjectInfo_Impl: Sized {
    fn GetObjectName(&self, lobjindex: i32, lpszname: super::super::Foundation::BSTR, pcchname: *mut i32) -> ::windows::core::Result<()>;
    fn GetObjectType(&self, lobjindex: i32) -> CLUADMEX_OBJECT_TYPE;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IGetClusterObjectInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IGetClusterObjectInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterObjectInfo_Impl, const OFFSET: isize>() -> IGetClusterObjectInfo_Vtbl {
        unsafe extern "system" fn GetObjectName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32, lpszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcchname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectName(::core::mem::transmute_copy(&lobjindex), ::core::mem::transmute_copy(&lpszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetObjectType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> CLUADMEX_OBJECT_TYPE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectType(::core::mem::transmute_copy(&lobjindex))
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetObjectName: GetObjectName::<Identity, Impl, OFFSET>,
            GetObjectType: GetObjectType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetClusterObjectInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGetClusterResourceInfo_Impl: Sized {
    fn GetResourceHandle(&self, lobjindex: i32) -> *mut _HRESOURCE;
    fn GetResourceTypeName(&self, lobjindex: i32, lpszrestypename: super::super::Foundation::BSTR, pcchrestypename: *mut i32) -> ::windows::core::Result<()>;
    fn GetResourceNetworkName(&self, lobjindex: i32, lpsznetname: super::super::Foundation::BSTR, pcchnetname: *mut u32) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IGetClusterResourceInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IGetClusterResourceInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterResourceInfo_Impl, const OFFSET: isize>() -> IGetClusterResourceInfo_Vtbl {
        unsafe extern "system" fn GetResourceHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterResourceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HRESOURCE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetResourceHandle(::core::mem::transmute_copy(&lobjindex))
        }
        unsafe extern "system" fn GetResourceTypeName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterResourceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32, lpszrestypename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcchrestypename: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetResourceTypeName(::core::mem::transmute_copy(&lobjindex), ::core::mem::transmute_copy(&lpszrestypename), ::core::mem::transmute_copy(&pcchrestypename)).into()
        }
        unsafe extern "system" fn GetResourceNetworkName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterResourceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32, lpsznetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcchnetname: *mut u32) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetResourceNetworkName(::core::mem::transmute_copy(&lobjindex), ::core::mem::transmute_copy(&lpsznetname), ::core::mem::transmute_copy(&pcchnetname))
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetResourceHandle: GetResourceHandle::<Identity, Impl, OFFSET>,
            GetResourceTypeName: GetResourceTypeName::<Identity, Impl, OFFSET>,
            GetResourceNetworkName: GetResourceNetworkName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetClusterResourceInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IGetClusterUIInfo_Impl: Sized {
    fn GetClusterName(&self, lpszname: super::super::Foundation::BSTR, pcchname: *mut i32) -> ::windows::core::Result<()>;
    fn GetLocale(&self) -> u32;
    fn GetFont(&self) -> super::super::Graphics::Gdi::HFONT;
    fn GetIcon(&self) -> super::super::UI::WindowsAndMessaging::HICON;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::RuntimeName for IGetClusterUIInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IGetClusterUIInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterUIInfo_Impl, const OFFSET: isize>() -> IGetClusterUIInfo_Vtbl {
        unsafe extern "system" fn GetClusterName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterUIInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcchname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClusterName(::core::mem::transmute_copy(&lpszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetLocale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterUIInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocale()
        }
        unsafe extern "system" fn GetFont<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterUIInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Graphics::Gdi::HFONT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFont()
        }
        unsafe extern "system" fn GetIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetClusterUIInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::UI::WindowsAndMessaging::HICON {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIcon()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetClusterName: GetClusterName::<Identity, Impl, OFFSET>,
            GetLocale: GetLocale::<Identity, Impl, OFFSET>,
            GetFont: GetFont::<Identity, Impl, OFFSET>,
            GetIcon: GetIcon::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetClusterUIInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusApplication_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DomainNames(&self) -> ::windows::core::Result<ISDomainNames>;
    fn get_ClusterNames(&self, bstrdomainname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISClusterNames>;
    fn OpenCluster(&self, bstrclustername: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISCluster>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusApplication {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusApplication_Impl, const OFFSET: isize>() -> ISClusApplication_Vtbl {
        unsafe extern "system" fn DomainNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdomains: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DomainNames() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdomains, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ClusterNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdomainname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppclusters: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_ClusterNames(::core::mem::transmute(&bstrdomainname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusters, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenCluster<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclustername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcluster: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenCluster(::core::mem::transmute(&bstrclustername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcluster, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DomainNames: DomainNames::<Identity, Impl, OFFSET>,
            get_ClusterNames: get_ClusterNames::<Identity, Impl, OFFSET>,
            OpenCluster: OpenCluster::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusApplication as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusCryptoKeys_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AddItem(&self, bstrcryptokey: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveItem(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusCryptoKeys {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusCryptoKeys_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusCryptoKeys_Impl, const OFFSET: isize>() -> ISClusCryptoKeys_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrcyrptokey: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcyrptokey, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcryptokey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddItem(::core::mem::transmute(&bstrcryptokey)).into()
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveItem(::core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            AddItem: AddItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusCryptoKeys as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusDisk_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Signature(&self) -> ::windows::core::Result<i32>;
    fn ScsiAddress(&self) -> ::windows::core::Result<ISClusScsiAddress>;
    fn DiskNumber(&self) -> ::windows::core::Result<i32>;
    fn Partitions(&self) -> ::windows::core::Result<ISClusPartitions>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusDisk {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusDisk_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusDisk_Impl, const OFFSET: isize>() -> ISClusDisk_Vtbl {
        unsafe extern "system" fn Signature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusDisk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsignature: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Signature() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsignature, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScsiAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusDisk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscsiaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScsiAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscsiaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiskNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusDisk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldisknumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DiskNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldisknumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Partitions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusDisk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppartitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Partitions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppartitions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Signature: Signature::<Identity, Impl, OFFSET>,
            ScsiAddress: ScsiAddress::<Identity, Impl, OFFSET>,
            DiskNumber: DiskNumber::<Identity, Impl, OFFSET>,
            Partitions: Partitions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusDisk as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusDisks_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusDisk>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusDisks {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusDisks_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusDisks_Impl, const OFFSET: isize>() -> ISClusDisks_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusDisks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusDisks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusDisks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppdisk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusDisks as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusNetInterface_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Handle(&self) -> ::windows::core::Result<usize>;
    fn State(&self) -> ::windows::core::Result<CLUSTER_NETINTERFACE_STATE>;
    fn Cluster(&self) -> ::windows::core::Result<ISCluster>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusNetInterface {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusNetInterface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: isize>() -> ISClusNetInterface_Vtbl {
        unsafe extern "system" fn CommonProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommonProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommonROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivateROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_NETINTERFACE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cluster() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcluster, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, Impl, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, Impl, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, Impl, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Cluster: Cluster::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusNetInterface as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusNetInterfaces_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNetInterface>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusNetInterfaces {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusNetInterfaces_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetInterfaces_Impl, const OFFSET: isize>() -> ISClusNetInterfaces_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusnetinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusnetinterface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusNetInterfaces as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusNetwork_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn Handle(&self) -> ::windows::core::Result<usize>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, bstrnetworkname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NetworkID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn State(&self) -> ::windows::core::Result<CLUSTER_NETWORK_STATE>;
    fn NetInterfaces(&self) -> ::windows::core::Result<ISClusNetworkNetInterfaces>;
    fn Cluster(&self) -> ::windows::core::Result<ISCluster>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusNetwork {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusNetwork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: isize>() -> ISClusNetwork_Vtbl {
        unsafe extern "system" fn CommonProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommonProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommonROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivateROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnetworkname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrnetworkname)).into()
        }
        unsafe extern "system" fn NetworkID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnetworkid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NetworkID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrnetworkid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_NETWORK_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetInterfaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclusnetinterfaces: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NetInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusnetinterfaces, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cluster() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcluster, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, Impl, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, Impl, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, Impl, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            NetworkID: NetworkID::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            NetInterfaces: NetInterfaces::<Identity, Impl, OFFSET>,
            Cluster: Cluster::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusNetwork as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusNetworkNetInterfaces_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNetInterface>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusNetworkNetInterfaces {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusNetworkNetInterfaces_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetworkNetInterfaces_Impl, const OFFSET: isize>() -> ISClusNetworkNetInterfaces_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetworkNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetworkNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetworkNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetworkNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusnetinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusnetinterface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusNetworkNetInterfaces as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusNetworks_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNetwork>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusNetworks {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusNetworks_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetworks_Impl, const OFFSET: isize>() -> ISClusNetworks_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusnetwork: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusnetwork, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusNetworks as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusNode_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Handle(&self) -> ::windows::core::Result<usize>;
    fn NodeID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn State(&self) -> ::windows::core::Result<CLUSTER_NODE_STATE>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Evict(&self) -> ::windows::core::Result<()>;
    fn ResourceGroups(&self) -> ::windows::core::Result<ISClusResGroups>;
    fn Cluster(&self) -> ::windows::core::Result<ISCluster>;
    fn NetInterfaces(&self) -> ::windows::core::Result<ISClusNodeNetInterfaces>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusNode {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusNode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: isize>() -> ISClusNode_Vtbl {
        unsafe extern "system" fn CommonProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommonProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommonROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivateROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NodeID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnodeid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NodeID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrnodeid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_NODE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume().into()
        }
        unsafe extern "system" fn Evict<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Evict().into()
        }
        unsafe extern "system" fn ResourceGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresourcegroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResourceGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresourcegroups, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cluster() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcluster, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetInterfaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclusnetinterfaces: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NetInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusnetinterfaces, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, Impl, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, Impl, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, Impl, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            NodeID: NodeID::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Evict: Evict::<Identity, Impl, OFFSET>,
            ResourceGroups: ResourceGroups::<Identity, Impl, OFFSET>,
            Cluster: Cluster::<Identity, Impl, OFFSET>,
            NetInterfaces: NetInterfaces::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusNode as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusNodeNetInterfaces_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNetInterface>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusNodeNetInterfaces {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusNodeNetInterfaces_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNodeNetInterfaces_Impl, const OFFSET: isize>() -> ISClusNodeNetInterfaces_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNodeNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNodeNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNodeNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNodeNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusnetinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusnetinterface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusNodeNetInterfaces as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusNodes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusNodes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusNodes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNodes_Impl, const OFFSET: isize>() -> ISClusNodes_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusNodes as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusPartition_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Flags(&self) -> ::windows::core::Result<i32>;
    fn DeviceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn VolumeLabel(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SerialNumber(&self) -> ::windows::core::Result<i32>;
    fn MaximumComponentLength(&self) -> ::windows::core::Result<i32>;
    fn FileSystemFlags(&self) -> ::windows::core::Result<i32>;
    fn FileSystem(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusPartition {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusPartition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartition_Impl, const OFFSET: isize>() -> ISClusPartition_Vtbl {
        unsafe extern "system" fn Flags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Flags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdevicename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeLabel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrvolumelabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VolumeLabel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvolumelabel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerialNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plserialnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SerialNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plserialnumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumComponentLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaximumcomponentlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaximumComponentLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaximumcomponentlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSystemFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfilesystemflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FileSystemFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plfilesystemflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSystem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilesystem: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FileSystem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfilesystem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Flags: Flags::<Identity, Impl, OFFSET>,
            DeviceName: DeviceName::<Identity, Impl, OFFSET>,
            VolumeLabel: VolumeLabel::<Identity, Impl, OFFSET>,
            SerialNumber: SerialNumber::<Identity, Impl, OFFSET>,
            MaximumComponentLength: MaximumComponentLength::<Identity, Impl, OFFSET>,
            FileSystemFlags: FileSystemFlags::<Identity, Impl, OFFSET>,
            FileSystem: FileSystem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusPartition as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusPartitionEx_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISClusPartition_Impl {
    fn TotalSize(&self) -> ::windows::core::Result<i32>;
    fn FreeSpace(&self) -> ::windows::core::Result<i32>;
    fn DeviceNumber(&self) -> ::windows::core::Result<i32>;
    fn PartitionNumber(&self) -> ::windows::core::Result<i32>;
    fn VolumeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusPartitionEx {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusPartitionEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartitionEx_Impl, const OFFSET: isize>() -> ISClusPartitionEx_Vtbl {
        unsafe extern "system" fn TotalSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartitionEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltotalsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TotalSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltotalsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSpace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartitionEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfreespace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FreeSpace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plfreespace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartitionEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldevicenumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldevicenumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartitionNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartitionEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpartitionnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PartitionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpartitionnumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartitionEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrvolumeguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VolumeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvolumeguid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISClusPartition_Vtbl::new::<Identity, Impl, OFFSET>(),
            TotalSize: TotalSize::<Identity, Impl, OFFSET>,
            FreeSpace: FreeSpace::<Identity, Impl, OFFSET>,
            DeviceNumber: DeviceNumber::<Identity, Impl, OFFSET>,
            PartitionNumber: PartitionNumber::<Identity, Impl, OFFSET>,
            VolumeGuid: VolumeGuid::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusPartitionEx as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISClusPartition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusPartitions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusPartition>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusPartitions {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusPartitions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartitions_Impl, const OFFSET: isize>() -> ISClusPartitions_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPartitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pppartition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppartition, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusPartitions as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusProperties_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusProperty>;
    fn CreateItem(&self, bstrname: &super::super::Foundation::BSTR, varvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusProperty>;
    fn UseDefaultValue(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SaveChanges(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn ReadOnly(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Private(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Common(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Modified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusProperties {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: isize>() -> ISClusProperties_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusproperty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateItem(::core::mem::transmute(&bstrname), ::core::mem::transmute(&varvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproperty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseDefaultValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UseDefaultValue(::core::mem::transmute(&varindex)).into()
        }
        unsafe extern "system" fn SaveChanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarstatuscode: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SaveChanges() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreadonly: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreadonly, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Private<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprivate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Private() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprivate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Common<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcommon: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Common() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcommon, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Modified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmodified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            UseDefaultValue: UseDefaultValue::<Identity, Impl, OFFSET>,
            SaveChanges: SaveChanges::<Identity, Impl, OFFSET>,
            ReadOnly: ReadOnly::<Identity, Impl, OFFSET>,
            Private: Private::<Identity, Impl, OFFSET>,
            Common: Common::<Identity, Impl, OFFSET>,
            Modified: Modified::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusProperties as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusProperty_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Length(&self) -> ::windows::core::Result<i32>;
    fn ValueCount(&self) -> ::windows::core::Result<i32>;
    fn Values(&self) -> ::windows::core::Result<ISClusPropertyValues>;
    fn Value(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetValue(&self, varvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Type(&self) -> ::windows::core::Result<CLUSTER_PROPERTY_TYPE>;
    fn SetType(&self, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows::core::Result<()>;
    fn Format(&self) -> ::windows::core::Result<CLUSTER_PROPERTY_FORMAT>;
    fn SetFormat(&self, format: CLUSTER_PROPERTY_FORMAT) -> ::windows::core::Result<()>;
    fn ReadOnly(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Private(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Common(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Modified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn UseDefaultValue(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: isize>() -> ISClusProperty_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValueCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ValueCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Values<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclusterpropertyvalues: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Values() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterpropertyvalues, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute(&varvalue)).into()
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut CLUSTER_PROPERTY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetType(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn Format<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut CLUSTER_PROPERTY_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Format() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: CLUSTER_PROPERTY_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFormat(::core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreadonly: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreadonly, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Private<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprivate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Private() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprivate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Common<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcommon: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Common() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcommon, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Modified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmodified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseDefaultValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UseDefaultValue().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            ValueCount: ValueCount::<Identity, Impl, OFFSET>,
            Values: Values::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            Format: Format::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            ReadOnly: ReadOnly::<Identity, Impl, OFFSET>,
            Private: Private::<Identity, Impl, OFFSET>,
            Common: Common::<Identity, Impl, OFFSET>,
            Modified: Modified::<Identity, Impl, OFFSET>,
            UseDefaultValue: UseDefaultValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusProperty as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusPropertyValue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Value(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetValue(&self, varvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Type(&self) -> ::windows::core::Result<CLUSTER_PROPERTY_TYPE>;
    fn SetType(&self, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows::core::Result<()>;
    fn Format(&self) -> ::windows::core::Result<CLUSTER_PROPERTY_FORMAT>;
    fn SetFormat(&self, format: CLUSTER_PROPERTY_FORMAT) -> ::windows::core::Result<()>;
    fn Length(&self) -> ::windows::core::Result<i32>;
    fn DataCount(&self) -> ::windows::core::Result<i32>;
    fn Data(&self) -> ::windows::core::Result<ISClusPropertyValueData>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusPropertyValue {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusPropertyValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: isize>() -> ISClusPropertyValue_Vtbl {
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute(&varvalue)).into()
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut CLUSTER_PROPERTY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetType(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn Format<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut CLUSTER_PROPERTY_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Format() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: CLUSTER_PROPERTY_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFormat(::core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn Length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DataCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclusterpropertyvaluedata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Data() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterpropertyvaluedata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            Format: Format::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            DataCount: DataCount::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusPropertyValue as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusPropertyValueData_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn CreateItem(&self, varvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn RemoveItem(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusPropertyValueData {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusPropertyValueData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValueData_Impl, const OFFSET: isize>() -> ISClusPropertyValueData_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValueData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValueData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValueData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValueData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvardata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateItem(::core::mem::transmute(&varvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValueData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveItem(::core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusPropertyValueData as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusPropertyValues_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusPropertyValue>;
    fn CreateItem(&self, bstrname: &super::super::Foundation::BSTR, varvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusPropertyValue>;
    fn RemoveItem(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusPropertyValues {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusPropertyValues_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValues_Impl, const OFFSET: isize>() -> ISClusPropertyValues_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pppropertyvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertyvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pppropertyvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateItem(::core::mem::transmute(&bstrname), ::core::mem::transmute(&varvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertyvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusPropertyValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveItem(::core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusPropertyValues as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusRefObject_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Handle(&self) -> ::windows::core::Result<usize>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusRefObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusRefObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusRefObject_Impl, const OFFSET: isize>() -> ISClusRefObject_Vtbl {
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusRefObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Handle: Handle::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusRefObject as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusRegistryKeys_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AddItem(&self, bstrregistrykey: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveItem(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusRegistryKeys {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusRegistryKeys_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusRegistryKeys_Impl, const OFFSET: isize>() -> ISClusRegistryKeys_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrregistrykey: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrregistrykey, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrregistrykey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddItem(::core::mem::transmute(&bstrregistrykey)).into()
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveItem(::core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            AddItem: AddItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusRegistryKeys as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResDependencies_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResource>;
    fn CreateItem(&self, bstrresourcename: &super::super::Foundation::BSTR, bstrresourcetype: &super::super::Foundation::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows::core::Result<ISClusResource>;
    fn DeleteItem(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddItem(&self, presource: &::core::option::Option<ISClusResource>) -> ::windows::core::Result<()>;
    fn RemoveItem(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusResDependencies {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResDependencies_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: isize>() -> ISClusResDependencies_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresourcetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateItem(::core::mem::transmute(&bstrresourcename), ::core::mem::transmute(&bstrresourcetype), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteItem(::core::mem::transmute(&varindex)).into()
        }
        unsafe extern "system" fn AddItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddItem(::core::mem::transmute(&presource)).into()
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveItem(::core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
            AddItem: AddItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResDependencies as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResDependents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResource>;
    fn CreateItem(&self, bstrresourcename: &super::super::Foundation::BSTR, bstrresourcetype: &super::super::Foundation::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows::core::Result<ISClusResource>;
    fn DeleteItem(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddItem(&self, presource: &::core::option::Option<ISClusResource>) -> ::windows::core::Result<()>;
    fn RemoveItem(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusResDependents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResDependents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: isize>() -> ISClusResDependents_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresourcetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateItem(::core::mem::transmute(&bstrresourcename), ::core::mem::transmute(&bstrresourcetype), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteItem(::core::mem::transmute(&varindex)).into()
        }
        unsafe extern "system" fn AddItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddItem(::core::mem::transmute(&presource)).into()
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveItem(::core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
            AddItem: AddItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResDependents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResGroup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn Handle(&self) -> ::windows::core::Result<usize>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, bstrgroupname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<CLUSTER_GROUP_STATE>;
    fn OwnerNode(&self) -> ::windows::core::Result<ISClusNode>;
    fn Resources(&self) -> ::windows::core::Result<ISClusResGroupResources>;
    fn PreferredOwnerNodes(&self) -> ::windows::core::Result<ISClusResGroupPreferredOwnerNodes>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Online(&self, vartimeout: &super::super::System::Com::VARIANT, varnode: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Move(&self, vartimeout: &super::super::System::Com::VARIANT, varnode: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Offline(&self, vartimeout: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Cluster(&self) -> ::windows::core::Result<ISCluster>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusResGroup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>() -> ISClusResGroup_Vtbl {
        unsafe extern "system" fn CommonProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommonProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommonROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivateROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrgroupname)).into()
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_GROUP_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OwnerNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppownernode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OwnerNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppownernode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclustergroupresources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Resources() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclustergroupresources, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredOwnerNodes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppownernodes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PreferredOwnerNodes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppownernodes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Online<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartimeout: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varnode: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Online(::core::mem::transmute(&vartimeout), ::core::mem::transmute(&varnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarpending, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartimeout: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varnode: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Move(::core::mem::transmute(&vartimeout), ::core::mem::transmute(&varnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarpending, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Offline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartimeout: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Offline(::core::mem::transmute(&vartimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarpending, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cluster() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcluster, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, Impl, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, Impl, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, Impl, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            OwnerNode: OwnerNode::<Identity, Impl, OFFSET>,
            Resources: Resources::<Identity, Impl, OFFSET>,
            PreferredOwnerNodes: PreferredOwnerNodes::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Online: Online::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Offline: Offline::<Identity, Impl, OFFSET>,
            Cluster: Cluster::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResGroup as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResGroupPreferredOwnerNodes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNode>;
    fn InsertItem(&self, pnode: &::core::option::Option<ISClusNode>, nposition: i32) -> ::windows::core::Result<()>;
    fn RemoveItem(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Modified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SaveChanges(&self) -> ::windows::core::Result<()>;
    fn AddItem(&self, pnode: &::core::option::Option<ISClusNode>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusResGroupPreferredOwnerNodes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResGroupPreferredOwnerNodes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>() -> ISClusResGroupPreferredOwnerNodes_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, nposition: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertItem(::core::mem::transmute(&pnode), ::core::mem::transmute_copy(&nposition)).into()
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveItem(::core::mem::transmute(&varindex)).into()
        }
        unsafe extern "system" fn Modified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Modified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmodified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveChanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveChanges().into()
        }
        unsafe extern "system" fn AddItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddItem(::core::mem::transmute(&pnode)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            InsertItem: InsertItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
            Modified: Modified::<Identity, Impl, OFFSET>,
            SaveChanges: SaveChanges::<Identity, Impl, OFFSET>,
            AddItem: AddItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResGroupPreferredOwnerNodes as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResGroupResources_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResource>;
    fn CreateItem(&self, bstrresourcename: &super::super::Foundation::BSTR, bstrresourcetype: &super::super::Foundation::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows::core::Result<ISClusResource>;
    fn DeleteItem(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusResGroupResources {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResGroupResources_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupResources_Impl, const OFFSET: isize>() -> ISClusResGroupResources_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresourcetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateItem(::core::mem::transmute(&bstrresourcename), ::core::mem::transmute(&bstrresourcetype), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteItem(::core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResGroupResources as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResGroups_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResGroup>;
    fn CreateItem(&self, bstrresourcegroupname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISClusResGroup>;
    fn DeleteItem(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusResGroups {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResGroups_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroups_Impl, const OFFSET: isize>() -> ISClusResGroups_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusresgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourcegroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppresourcegroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateItem(::core::mem::transmute(&bstrresourcegroupname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresourcegroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteItem(::core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResGroups as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResPossibleOwnerNodes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNode>;
    fn AddItem(&self, pnode: &::core::option::Option<ISClusNode>) -> ::windows::core::Result<()>;
    fn RemoveItem(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Modified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusResPossibleOwnerNodes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResPossibleOwnerNodes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>() -> ISClusResPossibleOwnerNodes_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddItem(::core::mem::transmute(&pnode)).into()
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveItem(::core::mem::transmute(&varindex)).into()
        }
        unsafe extern "system" fn Modified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Modified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmodified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            AddItem: AddItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
            Modified: Modified::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResPossibleOwnerNodes as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResType_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Cluster(&self) -> ::windows::core::Result<ISCluster>;
    fn Resources(&self) -> ::windows::core::Result<ISClusResTypeResources>;
    fn PossibleOwnerNodes(&self) -> ::windows::core::Result<ISClusResTypePossibleOwnerNodes>;
    fn AvailableDisks(&self) -> ::windows::core::Result<ISClusDisks>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusResType {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: isize>() -> ISClusResType_Vtbl {
        unsafe extern "system" fn CommonProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommonProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommonROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivateROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Cluster<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cluster() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcluster, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclusterrestyperesources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Resources() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterrestyperesources, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PossibleOwnerNodes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppownernodes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PossibleOwnerNodes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppownernodes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableDisks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppavailabledisks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AvailableDisks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppavailabledisks, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, Impl, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, Impl, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, Impl, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Cluster: Cluster::<Identity, Impl, OFFSET>,
            Resources: Resources::<Identity, Impl, OFFSET>,
            PossibleOwnerNodes: PossibleOwnerNodes::<Identity, Impl, OFFSET>,
            AvailableDisks: AvailableDisks::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResType as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResTypePossibleOwnerNodes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusResTypePossibleOwnerNodes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResTypePossibleOwnerNodes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: isize>() -> ISClusResTypePossibleOwnerNodes_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResTypePossibleOwnerNodes as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResTypeResources_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResource>;
    fn CreateItem(&self, bstrresourcename: &super::super::Foundation::BSTR, bstrgroupname: &super::super::Foundation::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows::core::Result<ISClusResource>;
    fn DeleteItem(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusResTypeResources {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResTypeResources_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypeResources_Impl, const OFFSET: isize>() -> ISClusResTypeResources_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateItem(::core::mem::transmute(&bstrresourcename), ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteItem(::core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResTypeResources as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResTypes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResType>;
    fn CreateItem(&self, bstrresourcetypename: &super::super::Foundation::BSTR, bstrdisplayname: &super::super::Foundation::BSTR, bstrresourcetypedll: &super::super::Foundation::BSTR, dwlooksalivepollinterval: i32, dwisalivepollinterval: i32) -> ::windows::core::Result<ISClusResType>;
    fn DeleteItem(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusResTypes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResTypes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypes_Impl, const OFFSET: isize>() -> ISClusResTypes_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusrestype: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusrestype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourcetypename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdisplayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresourcetypedll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwlooksalivepollinterval: i32, dwisalivepollinterval: i32, ppresourcetype: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateItem(::core::mem::transmute(&bstrresourcetypename), ::core::mem::transmute(&bstrdisplayname), ::core::mem::transmute(&bstrresourcetypedll), ::core::mem::transmute_copy(&dwlooksalivepollinterval), ::core::mem::transmute_copy(&dwisalivepollinterval)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresourcetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteItem(::core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResTypes as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResource_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn Handle(&self) -> ::windows::core::Result<usize>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, bstrresourcename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<CLUSTER_RESOURCE_STATE>;
    fn CoreFlag(&self) -> ::windows::core::Result<CLUS_FLAGS>;
    fn BecomeQuorumResource(&self, bstrdevicepath: &super::super::Foundation::BSTR, lmaxlogsize: i32) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Fail(&self) -> ::windows::core::Result<()>;
    fn Online(&self, ntimeout: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Offline(&self, ntimeout: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn ChangeResourceGroup(&self, presourcegroup: &::core::option::Option<ISClusResGroup>) -> ::windows::core::Result<()>;
    fn AddResourceNode(&self, pnode: &::core::option::Option<ISClusNode>) -> ::windows::core::Result<()>;
    fn RemoveResourceNode(&self, pnode: &::core::option::Option<ISClusNode>) -> ::windows::core::Result<()>;
    fn CanResourceBeDependent(&self, presource: &::core::option::Option<ISClusResource>) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PossibleOwnerNodes(&self) -> ::windows::core::Result<ISClusResPossibleOwnerNodes>;
    fn Dependencies(&self) -> ::windows::core::Result<ISClusResDependencies>;
    fn Dependents(&self) -> ::windows::core::Result<ISClusResDependents>;
    fn Group(&self) -> ::windows::core::Result<ISClusResGroup>;
    fn OwnerNode(&self) -> ::windows::core::Result<ISClusNode>;
    fn Cluster(&self) -> ::windows::core::Result<ISCluster>;
    fn ClassInfo(&self) -> ::windows::core::Result<CLUSTER_RESOURCE_CLASS>;
    fn Disk(&self) -> ::windows::core::Result<ISClusDisk>;
    fn RegistryKeys(&self) -> ::windows::core::Result<ISClusRegistryKeys>;
    fn CryptoKeys(&self) -> ::windows::core::Result<ISClusCryptoKeys>;
    fn TypeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Type(&self) -> ::windows::core::Result<ISClusResType>;
    fn MaintenanceMode(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetMaintenanceMode(&self, bmaintenancemode: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusResource {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>() -> ISClusResource_Vtbl {
        unsafe extern "system" fn CommonProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommonProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommonROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivateROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrresourcename)).into()
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_RESOURCE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoreFlag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcoreflag: *mut CLUS_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CoreFlag() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwcoreflag, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BecomeQuorumResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmaxlogsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BecomeQuorumResource(::core::mem::transmute(&bstrdevicepath), ::core::mem::transmute_copy(&lmaxlogsize)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Fail<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Fail().into()
        }
        unsafe extern "system" fn Online<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntimeout: i32, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Online(::core::mem::transmute_copy(&ntimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarpending, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Offline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntimeout: i32, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Offline(::core::mem::transmute_copy(&ntimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarpending, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeResourceGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcegroup: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChangeResourceGroup(::core::mem::transmute(&presourcegroup)).into()
        }
        unsafe extern "system" fn AddResourceNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddResourceNode(::core::mem::transmute(&pnode)).into()
        }
        unsafe extern "system" fn RemoveResourceNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveResourceNode(::core::mem::transmute(&pnode)).into()
        }
        unsafe extern "system" fn CanResourceBeDependent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pvardependent: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanResourceBeDependent(::core::mem::transmute(&presource)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardependent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PossibleOwnerNodes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppownernodes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PossibleOwnerNodes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppownernodes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dependencies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresdependencies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Dependencies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresdependencies, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dependents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresdependents: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Dependents() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresdependents, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Group() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OwnerNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppownernode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OwnerNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppownernode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cluster() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcluster, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClassInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcclassinfo: *mut CLUSTER_RESOURCE_CLASS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClassInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prcclassinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disk<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Disk() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegistryKeys<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppregistrykeys: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegistryKeys() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppregistrykeys, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CryptoKeys<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcryptokeys: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CryptoKeys() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcryptokeys, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TypeName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtypename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TypeName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtypename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresourcetype: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresourcetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaintenanceMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmaintenancemode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaintenanceMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmaintenancemode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaintenanceMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmaintenancemode: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaintenanceMode(::core::mem::transmute_copy(&bmaintenancemode)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, Impl, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, Impl, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, Impl, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            CoreFlag: CoreFlag::<Identity, Impl, OFFSET>,
            BecomeQuorumResource: BecomeQuorumResource::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Fail: Fail::<Identity, Impl, OFFSET>,
            Online: Online::<Identity, Impl, OFFSET>,
            Offline: Offline::<Identity, Impl, OFFSET>,
            ChangeResourceGroup: ChangeResourceGroup::<Identity, Impl, OFFSET>,
            AddResourceNode: AddResourceNode::<Identity, Impl, OFFSET>,
            RemoveResourceNode: RemoveResourceNode::<Identity, Impl, OFFSET>,
            CanResourceBeDependent: CanResourceBeDependent::<Identity, Impl, OFFSET>,
            PossibleOwnerNodes: PossibleOwnerNodes::<Identity, Impl, OFFSET>,
            Dependencies: Dependencies::<Identity, Impl, OFFSET>,
            Dependents: Dependents::<Identity, Impl, OFFSET>,
            Group: Group::<Identity, Impl, OFFSET>,
            OwnerNode: OwnerNode::<Identity, Impl, OFFSET>,
            Cluster: Cluster::<Identity, Impl, OFFSET>,
            ClassInfo: ClassInfo::<Identity, Impl, OFFSET>,
            Disk: Disk::<Identity, Impl, OFFSET>,
            RegistryKeys: RegistryKeys::<Identity, Impl, OFFSET>,
            CryptoKeys: CryptoKeys::<Identity, Impl, OFFSET>,
            TypeName: TypeName::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            MaintenanceMode: MaintenanceMode::<Identity, Impl, OFFSET>,
            SetMaintenanceMode: SetMaintenanceMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResource as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResources_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResource>;
    fn CreateItem(&self, bstrresourcename: &super::super::Foundation::BSTR, bstrresourcetype: &super::super::Foundation::BSTR, bstrgroupname: &super::super::Foundation::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows::core::Result<ISClusResource>;
    fn DeleteItem(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusResources {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResources_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResources_Impl, const OFFSET: isize>() -> ISClusResources_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresourcetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateItem(::core::mem::transmute(&bstrresourcename), ::core::mem::transmute(&bstrresourcetype), ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteItem(::core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            CreateItem: CreateItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResources as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusScsiAddress_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PortNumber(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PathId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn TargetId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Lun(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusScsiAddress {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusScsiAddress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusScsiAddress_Impl, const OFFSET: isize>() -> ISClusScsiAddress_Vtbl {
        unsafe extern "system" fn PortNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusScsiAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarportnumber: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PortNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarportnumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusScsiAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarpathid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PathId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarpathid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusScsiAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvartargetid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TargetId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvartargetid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lun<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusScsiAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarlun: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Lun() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarlun, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PortNumber: PortNumber::<Identity, Impl, OFFSET>,
            PathId: PathId::<Identity, Impl, OFFSET>,
            TargetId: TargetId::<Identity, Impl, OFFSET>,
            Lun: Lun::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusScsiAddress as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusVersion_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MajorVersion(&self) -> ::windows::core::Result<i32>;
    fn MinorVersion(&self) -> ::windows::core::Result<i32>;
    fn BuildNumber(&self) -> ::windows::core::Result<i16>;
    fn VendorId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CSDVersion(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ClusterHighestVersion(&self) -> ::windows::core::Result<i32>;
    fn ClusterLowestVersion(&self) -> ::windows::core::Result<i32>;
    fn Flags(&self) -> ::windows::core::Result<i32>;
    fn MixedVersion(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusVersion {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusVersion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: isize>() -> ISClusVersion_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrclustername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrclustername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnmajorversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnminorversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnbuildnumber: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BuildNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnbuildnumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VendorId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrvendorid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VendorId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvendorid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSDVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsdversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CSDVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcsdversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClusterHighestVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnclusterhighestversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClusterHighestVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnclusterhighestversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClusterLowestVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnclusterlowestversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClusterLowestVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnclusterlowestversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Flags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MixedVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmixedversion: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MixedVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmixedversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
            BuildNumber: BuildNumber::<Identity, Impl, OFFSET>,
            VendorId: VendorId::<Identity, Impl, OFFSET>,
            CSDVersion: CSDVersion::<Identity, Impl, OFFSET>,
            ClusterHighestVersion: ClusterHighestVersion::<Identity, Impl, OFFSET>,
            ClusterLowestVersion: ClusterLowestVersion::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            MixedVersion: MixedVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusVersion as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISCluster_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> ::windows::core::Result<ISClusProperties>;
    fn Handle(&self) -> ::windows::core::Result<usize>;
    fn Open(&self, bstrclustername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, bstrclustername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Version(&self) -> ::windows::core::Result<ISClusVersion>;
    fn SetQuorumResource(&self, pclusterresource: &::core::option::Option<ISClusResource>) -> ::windows::core::Result<()>;
    fn QuorumResource(&self) -> ::windows::core::Result<ISClusResource>;
    fn QuorumLogSize(&self) -> ::windows::core::Result<i32>;
    fn SetQuorumLogSize(&self, nlogsize: i32) -> ::windows::core::Result<()>;
    fn QuorumPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetQuorumPath(&self, ppath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Nodes(&self) -> ::windows::core::Result<ISClusNodes>;
    fn ResourceGroups(&self) -> ::windows::core::Result<ISClusResGroups>;
    fn Resources(&self) -> ::windows::core::Result<ISClusResources>;
    fn ResourceTypes(&self) -> ::windows::core::Result<ISClusResTypes>;
    fn Networks(&self) -> ::windows::core::Result<ISClusNetworks>;
    fn NetInterfaces(&self) -> ::windows::core::Result<ISClusNetInterfaces>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISCluster {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISCluster_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>() -> ISCluster_Vtbl {
        unsafe extern "system" fn CommonProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommonProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommonROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivateROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclustername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::core::mem::transmute(&bstrclustername)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclustername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrclustername)).into()
        }
        unsafe extern "system" fn Version<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclusversion: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuorumResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclusterresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuorumResource(::core::mem::transmute(&pclusterresource)).into()
        }
        unsafe extern "system" fn QuorumResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QuorumResource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclusterresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuorumLogSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlogsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QuorumLogSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnlogsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuorumLogSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nlogsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuorumLogSize(::core::mem::transmute_copy(&nlogsize)).into()
        }
        unsafe extern "system" fn QuorumPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QuorumPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuorumPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuorumPath(::core::mem::transmute(&ppath)).into()
        }
        unsafe extern "system" fn Nodes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnodes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Nodes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnodes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclusterresourcegroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResourceGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterresourcegroups, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclusterresources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Resources() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclusterresources, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresourcetypes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResourceTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresourcetypes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Networks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetworks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Networks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetworks, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetInterfaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetinterfaces: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NetInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetinterfaces, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, Impl, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, Impl, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, Impl, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            SetQuorumResource: SetQuorumResource::<Identity, Impl, OFFSET>,
            QuorumResource: QuorumResource::<Identity, Impl, OFFSET>,
            QuorumLogSize: QuorumLogSize::<Identity, Impl, OFFSET>,
            SetQuorumLogSize: SetQuorumLogSize::<Identity, Impl, OFFSET>,
            QuorumPath: QuorumPath::<Identity, Impl, OFFSET>,
            SetQuorumPath: SetQuorumPath::<Identity, Impl, OFFSET>,
            Nodes: Nodes::<Identity, Impl, OFFSET>,
            ResourceGroups: ResourceGroups::<Identity, Impl, OFFSET>,
            Resources: Resources::<Identity, Impl, OFFSET>,
            ResourceTypes: ResourceTypes::<Identity, Impl, OFFSET>,
            Networks: Networks::<Identity, Impl, OFFSET>,
            NetInterfaces: NetInterfaces::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCluster as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusterNames_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DomainName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISClusterNames {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusterNames_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusterNames_Impl, const OFFSET: isize>() -> ISClusterNames_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusterNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusterNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusterNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusterNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrclustername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrclustername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISClusterNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdomainname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DomainName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdomainname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            DomainName: DomainName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusterNames as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISDomainNames_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISDomainNames {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISDomainNames_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISDomainNames_Impl, const OFFSET: isize>() -> ISDomainNames_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISDomainNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISDomainNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISDomainNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISDomainNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrdomainname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdomainname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISDomainNames as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWCContextMenuCallback_Impl: Sized {
    fn AddExtensionMenuItem(&self, lpszname: &super::super::Foundation::BSTR, lpszstatusbartext: &super::super::Foundation::BSTR, ncommandid: u32, nsubmenucommandid: u32, uflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWCContextMenuCallback {}
#[cfg(feature = "Win32_Foundation")]
impl IWCContextMenuCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCContextMenuCallback_Impl, const OFFSET: isize>() -> IWCContextMenuCallback_Vtbl {
        unsafe extern "system" fn AddExtensionMenuItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCContextMenuCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpszstatusbartext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ncommandid: u32, nsubmenucommandid: u32, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddExtensionMenuItem(::core::mem::transmute(&lpszname), ::core::mem::transmute(&lpszstatusbartext), ::core::mem::transmute_copy(&ncommandid), ::core::mem::transmute_copy(&nsubmenucommandid), ::core::mem::transmute_copy(&uflags)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AddExtensionMenuItem: AddExtensionMenuItem::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWCContextMenuCallback as ::windows::core::Interface>::IID
    }
}
pub trait IWCPropertySheetCallback_Impl: Sized {
    fn AddPropertySheetPage(&self, hpage: *const i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWCPropertySheetCallback {}
impl IWCPropertySheetCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCPropertySheetCallback_Impl, const OFFSET: isize>() -> IWCPropertySheetCallback_Vtbl {
        unsafe extern "system" fn AddPropertySheetPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCPropertySheetCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertySheetPage(::core::mem::transmute_copy(&hpage)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AddPropertySheetPage: AddPropertySheetPage::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWCPropertySheetCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWCWizard97Callback_Impl: Sized {
    fn AddWizard97Page(&self, hpage: *const i32) -> ::windows::core::Result<()>;
    fn EnableNext(&self, hpage: *const i32, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWCWizard97Callback {}
#[cfg(feature = "Win32_Foundation")]
impl IWCWizard97Callback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCWizard97Callback_Impl, const OFFSET: isize>() -> IWCWizard97Callback_Vtbl {
        unsafe extern "system" fn AddWizard97Page<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCWizard97Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddWizard97Page(::core::mem::transmute_copy(&hpage)).into()
        }
        unsafe extern "system" fn EnableNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCWizard97Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: *const i32, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableNext(::core::mem::transmute_copy(&hpage), ::core::mem::transmute_copy(&benable)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddWizard97Page: AddWizard97Page::<Identity, Impl, OFFSET>,
            EnableNext: EnableNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWCWizard97Callback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWCWizardCallback_Impl: Sized {
    fn AddWizardPage(&self, hpage: *const i32) -> ::windows::core::Result<()>;
    fn EnableNext(&self, hpage: *const i32, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWCWizardCallback {}
#[cfg(feature = "Win32_Foundation")]
impl IWCWizardCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCWizardCallback_Impl, const OFFSET: isize>() -> IWCWizardCallback_Vtbl {
        unsafe extern "system" fn AddWizardPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCWizardCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddWizardPage(::core::mem::transmute_copy(&hpage)).into()
        }
        unsafe extern "system" fn EnableNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCWizardCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: *const i32, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableNext(::core::mem::transmute_copy(&hpage), ::core::mem::transmute_copy(&benable)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddWizardPage: AddWizardPage::<Identity, Impl, OFFSET>,
            EnableNext: EnableNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWCWizardCallback as ::windows::core::Interface>::IID
    }
}
pub trait IWEExtendContextMenu_Impl: Sized {
    fn AddContextMenuItems(&self, pidata: &::core::option::Option<::windows::core::IUnknown>, picallback: &::core::option::Option<IWCContextMenuCallback>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWEExtendContextMenu {}
impl IWEExtendContextMenu_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWEExtendContextMenu_Impl, const OFFSET: isize>() -> IWEExtendContextMenu_Vtbl {
        unsafe extern "system" fn AddContextMenuItems<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWEExtendContextMenu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddContextMenuItems(::core::mem::transmute(&pidata), ::core::mem::transmute(&picallback)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AddContextMenuItems: AddContextMenuItems::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWEExtendContextMenu as ::windows::core::Interface>::IID
    }
}
pub trait IWEExtendPropertySheet_Impl: Sized {
    fn CreatePropertySheetPages(&self, pidata: &::core::option::Option<::windows::core::IUnknown>, picallback: &::core::option::Option<IWCPropertySheetCallback>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWEExtendPropertySheet {}
impl IWEExtendPropertySheet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWEExtendPropertySheet_Impl, const OFFSET: isize>() -> IWEExtendPropertySheet_Vtbl {
        unsafe extern "system" fn CreatePropertySheetPages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWEExtendPropertySheet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreatePropertySheetPages(::core::mem::transmute(&pidata), ::core::mem::transmute(&picallback)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreatePropertySheetPages: CreatePropertySheetPages::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWEExtendPropertySheet as ::windows::core::Interface>::IID
    }
}
pub trait IWEExtendWizard_Impl: Sized {
    fn CreateWizardPages(&self, pidata: &::core::option::Option<::windows::core::IUnknown>, picallback: &::core::option::Option<IWCWizardCallback>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWEExtendWizard {}
impl IWEExtendWizard_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWEExtendWizard_Impl, const OFFSET: isize>() -> IWEExtendWizard_Vtbl {
        unsafe extern "system" fn CreateWizardPages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWEExtendWizard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateWizardPages(::core::mem::transmute(&pidata), ::core::mem::transmute(&picallback)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateWizardPages: CreateWizardPages::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWEExtendWizard as ::windows::core::Interface>::IID
    }
}
pub trait IWEExtendWizard97_Impl: Sized {
    fn CreateWizard97Pages(&self, pidata: &::core::option::Option<::windows::core::IUnknown>, picallback: &::core::option::Option<IWCWizard97Callback>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWEExtendWizard97 {}
impl IWEExtendWizard97_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWEExtendWizard97_Impl, const OFFSET: isize>() -> IWEExtendWizard97_Vtbl {
        unsafe extern "system" fn CreateWizard97Pages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWEExtendWizard97_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateWizard97Pages(::core::mem::transmute(&pidata), ::core::mem::transmute(&picallback)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateWizard97Pages: CreateWizard97Pages::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWEExtendWizard97 as ::windows::core::Interface>::IID
    }
}
pub trait IWEInvokeCommand_Impl: Sized {
    fn InvokeCommand(&self, ncommandid: u32, pidata: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWEInvokeCommand {}
impl IWEInvokeCommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWEInvokeCommand_Impl, const OFFSET: isize>() -> IWEInvokeCommand_Vtbl {
        unsafe extern "system" fn InvokeCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWEInvokeCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncommandid: u32, pidata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvokeCommand(::core::mem::transmute_copy(&ncommandid), ::core::mem::transmute(&pidata)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), InvokeCommand: InvokeCommand::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWEInvokeCommand as ::windows::core::Interface>::IID
    }
}
