#[cfg(feature = "Win32_Foundation")]
pub trait IGetClusterDataInfo_Impl: Sized {
    fn GetClusterName(&mut self, lpszname: super::super::Foundation::BSTR, pcchname: *mut i32) -> ::windows::core::Result<()>;
    fn GetClusterHandle(&mut self) -> *mut _HCLUSTER;
    fn GetObjectCount(&mut self) -> i32;
}
#[cfg(feature = "Win32_Foundation")]
impl IGetClusterDataInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetClusterDataInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetClusterDataInfo_Vtbl {
        unsafe extern "system" fn GetClusterName<Impl: IGetClusterDataInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcchname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClusterName(::core::mem::transmute_copy(&lpszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetClusterHandle<Impl: IGetClusterDataInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut _HCLUSTER {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClusterHandle()
        }
        unsafe extern "system" fn GetObjectCount<Impl: IGetClusterDataInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectCount()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetClusterName: GetClusterName::<Impl, IMPL_OFFSET>,
            GetClusterHandle: GetClusterHandle::<Impl, IMPL_OFFSET>,
            GetObjectCount: GetObjectCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetClusterDataInfo as ::windows::core::Interface>::IID
    }
}
pub trait IGetClusterGroupInfo_Impl: Sized {
    fn GetGroupHandle(&mut self, lobjindex: i32) -> *mut _HGROUP;
}
impl IGetClusterGroupInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetClusterGroupInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetClusterGroupInfo_Vtbl {
        unsafe extern "system" fn GetGroupHandle<Impl: IGetClusterGroupInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HGROUP {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGroupHandle(::core::mem::transmute_copy(&lobjindex))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetGroupHandle: GetGroupHandle::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetClusterGroupInfo as ::windows::core::Interface>::IID
    }
}
pub trait IGetClusterNetInterfaceInfo_Impl: Sized {
    fn GetNetInterfaceHandle(&mut self, lobjindex: i32) -> *mut _HNETINTERFACE;
}
impl IGetClusterNetInterfaceInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetClusterNetInterfaceInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetClusterNetInterfaceInfo_Vtbl {
        unsafe extern "system" fn GetNetInterfaceHandle<Impl: IGetClusterNetInterfaceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HNETINTERFACE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNetInterfaceHandle(::core::mem::transmute_copy(&lobjindex))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetNetInterfaceHandle: GetNetInterfaceHandle::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetClusterNetInterfaceInfo as ::windows::core::Interface>::IID
    }
}
pub trait IGetClusterNetworkInfo_Impl: Sized {
    fn GetNetworkHandle(&mut self, lobjindex: i32) -> *mut _HNETWORK;
}
impl IGetClusterNetworkInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetClusterNetworkInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetClusterNetworkInfo_Vtbl {
        unsafe extern "system" fn GetNetworkHandle<Impl: IGetClusterNetworkInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HNETWORK {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNetworkHandle(::core::mem::transmute_copy(&lobjindex))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetNetworkHandle: GetNetworkHandle::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetClusterNetworkInfo as ::windows::core::Interface>::IID
    }
}
pub trait IGetClusterNodeInfo_Impl: Sized {
    fn GetNodeHandle(&mut self, lobjindex: i32) -> *mut _HNODE;
}
impl IGetClusterNodeInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetClusterNodeInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetClusterNodeInfo_Vtbl {
        unsafe extern "system" fn GetNodeHandle<Impl: IGetClusterNodeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HNODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNodeHandle(::core::mem::transmute_copy(&lobjindex))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetNodeHandle: GetNodeHandle::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetClusterNodeInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGetClusterObjectInfo_Impl: Sized {
    fn GetObjectName(&mut self, lobjindex: i32, lpszname: super::super::Foundation::BSTR, pcchname: *mut i32) -> ::windows::core::Result<()>;
    fn GetObjectType(&mut self, lobjindex: i32) -> CLUADMEX_OBJECT_TYPE;
}
#[cfg(feature = "Win32_Foundation")]
impl IGetClusterObjectInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetClusterObjectInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetClusterObjectInfo_Vtbl {
        unsafe extern "system" fn GetObjectName<Impl: IGetClusterObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32, lpszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcchname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectName(::core::mem::transmute_copy(&lobjindex), ::core::mem::transmute_copy(&lpszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetObjectType<Impl: IGetClusterObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> CLUADMEX_OBJECT_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectType(::core::mem::transmute_copy(&lobjindex))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetObjectName: GetObjectName::<Impl, IMPL_OFFSET>,
            GetObjectType: GetObjectType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetClusterObjectInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGetClusterResourceInfo_Impl: Sized {
    fn GetResourceHandle(&mut self, lobjindex: i32) -> *mut _HRESOURCE;
    fn GetResourceTypeName(&mut self, lobjindex: i32, lpszrestypename: super::super::Foundation::BSTR, pcchrestypename: *mut i32) -> ::windows::core::Result<()>;
    fn GetResourceNetworkName(&mut self, lobjindex: i32, lpsznetname: super::super::Foundation::BSTR, pcchnetname: *mut u32) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl IGetClusterResourceInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetClusterResourceInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetClusterResourceInfo_Vtbl {
        unsafe extern "system" fn GetResourceHandle<Impl: IGetClusterResourceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HRESOURCE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResourceHandle(::core::mem::transmute_copy(&lobjindex))
        }
        unsafe extern "system" fn GetResourceTypeName<Impl: IGetClusterResourceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32, lpszrestypename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcchrestypename: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResourceTypeName(::core::mem::transmute_copy(&lobjindex), ::core::mem::transmute_copy(&lpszrestypename), ::core::mem::transmute_copy(&pcchrestypename)).into()
        }
        unsafe extern "system" fn GetResourceNetworkName<Impl: IGetClusterResourceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjindex: i32, lpsznetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcchnetname: *mut u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResourceNetworkName(::core::mem::transmute_copy(&lobjindex), ::core::mem::transmute_copy(&lpsznetname), ::core::mem::transmute_copy(&pcchnetname))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetResourceHandle: GetResourceHandle::<Impl, IMPL_OFFSET>,
            GetResourceTypeName: GetResourceTypeName::<Impl, IMPL_OFFSET>,
            GetResourceNetworkName: GetResourceNetworkName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetClusterResourceInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IGetClusterUIInfo_Impl: Sized {
    fn GetClusterName(&mut self, lpszname: super::super::Foundation::BSTR, pcchname: *mut i32) -> ::windows::core::Result<()>;
    fn GetLocale(&mut self) -> u32;
    fn GetFont(&mut self) -> super::super::Graphics::Gdi::HFONT;
    fn GetIcon(&mut self) -> super::super::UI::WindowsAndMessaging::HICON;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IGetClusterUIInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetClusterUIInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetClusterUIInfo_Vtbl {
        unsafe extern "system" fn GetClusterName<Impl: IGetClusterUIInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcchname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClusterName(::core::mem::transmute_copy(&lpszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetLocale<Impl: IGetClusterUIInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocale()
        }
        unsafe extern "system" fn GetFont<Impl: IGetClusterUIInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Graphics::Gdi::HFONT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFont()
        }
        unsafe extern "system" fn GetIcon<Impl: IGetClusterUIInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::UI::WindowsAndMessaging::HICON {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIcon()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetClusterName: GetClusterName::<Impl, IMPL_OFFSET>,
            GetLocale: GetLocale::<Impl, IMPL_OFFSET>,
            GetFont: GetFont::<Impl, IMPL_OFFSET>,
            GetIcon: GetIcon::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetClusterUIInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusApplication_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DomainNames(&mut self) -> ::windows::core::Result<ISDomainNames>;
    fn ClusterNames(&mut self, bstrdomainname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISClusterNames>;
    fn OpenCluster(&mut self, bstrclustername: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISCluster>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusApplication_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusApplication_Vtbl {
        unsafe extern "system" fn DomainNames<Impl: ISClusApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdomains: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainNames() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdomains = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClusterNames<Impl: ISClusApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdomainname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppclusters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClusterNames(::core::mem::transmute_copy(&bstrdomainname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenCluster<Impl: ISClusApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclustername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcluster: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenCluster(::core::mem::transmute_copy(&bstrclustername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcluster = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DomainNames: DomainNames::<Impl, IMPL_OFFSET>,
            ClusterNames: ClusterNames::<Impl, IMPL_OFFSET>,
            OpenCluster: OpenCluster::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusApplication as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusCryptoKeys_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AddItem(&mut self, bstrcryptokey: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveItem(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusCryptoKeys_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusCryptoKeys_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusCryptoKeys_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrcyrptokey: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcyrptokey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Impl: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcryptokey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddItem(::core::mem::transmute_copy(&bstrcryptokey)).into()
        }
        unsafe extern "system" fn RemoveItem<Impl: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItem(::core::mem::transmute_copy(&varindex)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            AddItem: AddItem::<Impl, IMPL_OFFSET>,
            RemoveItem: RemoveItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusCryptoKeys as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusDisk_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Signature(&mut self) -> ::windows::core::Result<i32>;
    fn ScsiAddress(&mut self) -> ::windows::core::Result<ISClusScsiAddress>;
    fn DiskNumber(&mut self) -> ::windows::core::Result<i32>;
    fn Partitions(&mut self) -> ::windows::core::Result<ISClusPartitions>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusDisk_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusDisk_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusDisk_Vtbl {
        unsafe extern "system" fn Signature<Impl: ISClusDisk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsignature: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signature() {
                ::core::result::Result::Ok(ok__) => {
                    *plsignature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScsiAddress<Impl: ISClusDisk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscsiaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScsiAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppscsiaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiskNumber<Impl: ISClusDisk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldisknumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiskNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pldisknumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Partitions<Impl: ISClusDisk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppartitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Partitions() {
                ::core::result::Result::Ok(ok__) => {
                    *pppartitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Signature: Signature::<Impl, IMPL_OFFSET>,
            ScsiAddress: ScsiAddress::<Impl, IMPL_OFFSET>,
            DiskNumber: DiskNumber::<Impl, IMPL_OFFSET>,
            Partitions: Partitions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusDisk as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusDisks_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusDisk>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusDisks_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusDisks_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusDisks_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusDisks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusDisks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusDisks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppdisk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdisk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusDisks as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusNetInterface_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn CommonROProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateROProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Handle(&mut self) -> ::windows::core::Result<usize>;
    fn State(&mut self) -> ::windows::core::Result<CLUSTER_NETINTERFACE_STATE>;
    fn Cluster(&mut self) -> ::windows::core::Result<ISCluster>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusNetInterface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusNetInterface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusNetInterface_Vtbl {
        unsafe extern "system" fn CommonProperties<Impl: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommonProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Impl: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Impl: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommonROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Impl: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_NETINTERFACE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *dwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Impl: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcluster: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cluster() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcluster = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CommonProperties: CommonProperties::<Impl, IMPL_OFFSET>,
            PrivateProperties: PrivateProperties::<Impl, IMPL_OFFSET>,
            CommonROProperties: CommonROProperties::<Impl, IMPL_OFFSET>,
            PrivateROProperties: PrivateROProperties::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Handle: Handle::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Cluster: Cluster::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusNetInterface as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusNetInterfaces_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNetInterface>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusNetInterfaces_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusNetInterfaces_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusNetInterfaces_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusnetinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusnetinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusNetInterfaces as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusNetwork_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn CommonROProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateROProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn Handle(&mut self) -> ::windows::core::Result<usize>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrnetworkname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NetworkID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn State(&mut self) -> ::windows::core::Result<CLUSTER_NETWORK_STATE>;
    fn NetInterfaces(&mut self) -> ::windows::core::Result<ISClusNetworkNetInterfaces>;
    fn Cluster(&mut self) -> ::windows::core::Result<ISCluster>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusNetwork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusNetwork_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusNetwork_Vtbl {
        unsafe extern "system" fn CommonProperties<Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommonProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommonROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnetworkname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrnetworkname)).into()
        }
        unsafe extern "system" fn NetworkID<Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnetworkid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrnetworkid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_NETWORK_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *dwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetInterfaces<Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclusnetinterfaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusnetinterfaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Impl: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcluster: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cluster() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcluster = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CommonProperties: CommonProperties::<Impl, IMPL_OFFSET>,
            PrivateProperties: PrivateProperties::<Impl, IMPL_OFFSET>,
            CommonROProperties: CommonROProperties::<Impl, IMPL_OFFSET>,
            PrivateROProperties: PrivateROProperties::<Impl, IMPL_OFFSET>,
            Handle: Handle::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            NetworkID: NetworkID::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            NetInterfaces: NetInterfaces::<Impl, IMPL_OFFSET>,
            Cluster: Cluster::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusNetwork as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusNetworkNetInterfaces_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNetInterface>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusNetworkNetInterfaces_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusNetworkNetInterfaces_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusNetworkNetInterfaces_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusNetworkNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusNetworkNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusNetworkNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusNetworkNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusnetinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusnetinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusNetworkNetInterfaces as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusNetworks_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNetwork>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusNetworks_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusNetworks_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusNetworks_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusnetwork = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusNetworks as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusNode_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn CommonROProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateROProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Handle(&mut self) -> ::windows::core::Result<usize>;
    fn NodeID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn State(&mut self) -> ::windows::core::Result<CLUSTER_NODE_STATE>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn Evict(&mut self) -> ::windows::core::Result<()>;
    fn ResourceGroups(&mut self) -> ::windows::core::Result<ISClusResGroups>;
    fn Cluster(&mut self) -> ::windows::core::Result<ISCluster>;
    fn NetInterfaces(&mut self) -> ::windows::core::Result<ISClusNodeNetInterfaces>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusNode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusNode_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusNode_Vtbl {
        unsafe extern "system" fn CommonProperties<Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommonProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommonROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NodeID<Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnodeid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NodeID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrnodeid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_NODE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *dwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Evict<Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Evict().into()
        }
        unsafe extern "system" fn ResourceGroups<Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresourcegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresourcegroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcluster: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cluster() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcluster = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetInterfaces<Impl: ISClusNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclusnetinterfaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusnetinterfaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CommonProperties: CommonProperties::<Impl, IMPL_OFFSET>,
            PrivateProperties: PrivateProperties::<Impl, IMPL_OFFSET>,
            CommonROProperties: CommonROProperties::<Impl, IMPL_OFFSET>,
            PrivateROProperties: PrivateROProperties::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Handle: Handle::<Impl, IMPL_OFFSET>,
            NodeID: NodeID::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            Evict: Evict::<Impl, IMPL_OFFSET>,
            ResourceGroups: ResourceGroups::<Impl, IMPL_OFFSET>,
            Cluster: Cluster::<Impl, IMPL_OFFSET>,
            NetInterfaces: NetInterfaces::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusNode as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusNodeNetInterfaces_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNetInterface>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusNodeNetInterfaces_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusNodeNetInterfaces_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusNodeNetInterfaces_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusNodeNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusNodeNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusNodeNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusNodeNetInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusnetinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusnetinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusNodeNetInterfaces as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusNodes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusNodes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusNodes_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusNodes_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusNodes as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusPartition_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Flags(&mut self) -> ::windows::core::Result<i32>;
    fn DeviceName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn VolumeLabel(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SerialNumber(&mut self) -> ::windows::core::Result<i32>;
    fn MaximumComponentLength(&mut self) -> ::windows::core::Result<i32>;
    fn FileSystemFlags(&mut self) -> ::windows::core::Result<i32>;
    fn FileSystem(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusPartition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusPartition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusPartition_Vtbl {
        unsafe extern "system" fn Flags<Impl: ISClusPartition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *plflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Impl: ISClusPartition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdevicename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeLabel<Impl: ISClusPartition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrvolumelabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeLabel() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvolumelabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerialNumber<Impl: ISClusPartition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plserialnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SerialNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *plserialnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumComponentLength<Impl: ISClusPartition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaximumcomponentlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaximumComponentLength() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaximumcomponentlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSystemFlags<Impl: ISClusPartition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfilesystemflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileSystemFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *plfilesystemflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSystem<Impl: ISClusPartition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilesystem: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfilesystem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Flags: Flags::<Impl, IMPL_OFFSET>,
            DeviceName: DeviceName::<Impl, IMPL_OFFSET>,
            VolumeLabel: VolumeLabel::<Impl, IMPL_OFFSET>,
            SerialNumber: SerialNumber::<Impl, IMPL_OFFSET>,
            MaximumComponentLength: MaximumComponentLength::<Impl, IMPL_OFFSET>,
            FileSystemFlags: FileSystemFlags::<Impl, IMPL_OFFSET>,
            FileSystem: FileSystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusPartition as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusPartitionEx_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISClusPartition_Impl {
    fn TotalSize(&mut self) -> ::windows::core::Result<i32>;
    fn FreeSpace(&mut self) -> ::windows::core::Result<i32>;
    fn DeviceNumber(&mut self) -> ::windows::core::Result<i32>;
    fn PartitionNumber(&mut self) -> ::windows::core::Result<i32>;
    fn VolumeGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusPartitionEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusPartitionEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusPartitionEx_Vtbl {
        unsafe extern "system" fn TotalSize<Impl: ISClusPartitionEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltotalsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pltotalsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSpace<Impl: ISClusPartitionEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfreespace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *plfreespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceNumber<Impl: ISClusPartitionEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldevicenumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pldevicenumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartitionNumber<Impl: ISClusPartitionEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpartitionnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PartitionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *plpartitionnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeGuid<Impl: ISClusPartitionEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrvolumeguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvolumeguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISClusPartition_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TotalSize: TotalSize::<Impl, IMPL_OFFSET>,
            FreeSpace: FreeSpace::<Impl, IMPL_OFFSET>,
            DeviceNumber: DeviceNumber::<Impl, IMPL_OFFSET>,
            PartitionNumber: PartitionNumber::<Impl, IMPL_OFFSET>,
            VolumeGuid: VolumeGuid::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusPartitionEx as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISClusPartition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusPartitions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusPartition>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusPartitions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusPartitions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusPartitions_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusPartitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusPartitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusPartitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pppartition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppartition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusPartitions as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusProperties_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusProperty>;
    fn CreateItem(&mut self, bstrname: &super::super::Foundation::BSTR, varvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusProperty>;
    fn UseDefaultValue(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SaveChanges(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn ReadOnly(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Private(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Common(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Modified(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusProperties_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateItem(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&varvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseDefaultValue<Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UseDefaultValue(::core::mem::transmute_copy(&varindex)).into()
        }
        unsafe extern "system" fn SaveChanges<Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarstatuscode: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveChanges() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreadonly: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarreadonly = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Private<Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprivate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Private() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprivate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Common<Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcommon: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Common() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcommon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Impl: ISClusProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modified() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmodified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            CreateItem: CreateItem::<Impl, IMPL_OFFSET>,
            UseDefaultValue: UseDefaultValue::<Impl, IMPL_OFFSET>,
            SaveChanges: SaveChanges::<Impl, IMPL_OFFSET>,
            ReadOnly: ReadOnly::<Impl, IMPL_OFFSET>,
            Private: Private::<Impl, IMPL_OFFSET>,
            Common: Common::<Impl, IMPL_OFFSET>,
            Modified: Modified::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusProperties as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusProperty_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Length(&mut self) -> ::windows::core::Result<i32>;
    fn ValueCount(&mut self) -> ::windows::core::Result<i32>;
    fn Values(&mut self) -> ::windows::core::Result<ISClusPropertyValues>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetValue(&mut self, varvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Type(&mut self) -> ::windows::core::Result<CLUSTER_PROPERTY_TYPE>;
    fn SetType(&mut self, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows::core::Result<()>;
    fn Format(&mut self) -> ::windows::core::Result<CLUSTER_PROPERTY_FORMAT>;
    fn SetFormat(&mut self, format: CLUSTER_PROPERTY_FORMAT) -> ::windows::core::Result<()>;
    fn ReadOnly(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Private(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Common(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Modified(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn UseDefaultValue(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusProperty_Vtbl {
        unsafe extern "system" fn Name<Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *plength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValueCount<Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Values<Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclusterpropertyvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Values() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusterpropertyvalues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn Type<Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut CLUSTER_PROPERTY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn Format<Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut CLUSTER_PROPERTY_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *pformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: CLUSTER_PROPERTY_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormat(::core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn ReadOnly<Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreadonly: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarreadonly = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Private<Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprivate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Private() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprivate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Common<Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcommon: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Common() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcommon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modified() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmodified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseDefaultValue<Impl: ISClusProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UseDefaultValue().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            ValueCount: ValueCount::<Impl, IMPL_OFFSET>,
            Values: Values::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            Format: Format::<Impl, IMPL_OFFSET>,
            SetFormat: SetFormat::<Impl, IMPL_OFFSET>,
            ReadOnly: ReadOnly::<Impl, IMPL_OFFSET>,
            Private: Private::<Impl, IMPL_OFFSET>,
            Common: Common::<Impl, IMPL_OFFSET>,
            Modified: Modified::<Impl, IMPL_OFFSET>,
            UseDefaultValue: UseDefaultValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusProperty as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusPropertyValue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Value(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetValue(&mut self, varvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Type(&mut self) -> ::windows::core::Result<CLUSTER_PROPERTY_TYPE>;
    fn SetType(&mut self, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows::core::Result<()>;
    fn Format(&mut self) -> ::windows::core::Result<CLUSTER_PROPERTY_FORMAT>;
    fn SetFormat(&mut self, format: CLUSTER_PROPERTY_FORMAT) -> ::windows::core::Result<()>;
    fn Length(&mut self) -> ::windows::core::Result<i32>;
    fn DataCount(&mut self) -> ::windows::core::Result<i32>;
    fn Data(&mut self) -> ::windows::core::Result<ISClusPropertyValueData>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusPropertyValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusPropertyValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusPropertyValue_Vtbl {
        unsafe extern "system" fn Value<Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn Type<Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut CLUSTER_PROPERTY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn Format<Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut CLUSTER_PROPERTY_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *pformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: CLUSTER_PROPERTY_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormat(::core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn Length<Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *plength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataCount<Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclusterpropertyvaluedata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusterpropertyvaluedata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            Format: Format::<Impl, IMPL_OFFSET>,
            SetFormat: SetFormat::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            DataCount: DataCount::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusPropertyValue as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusPropertyValueData_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn CreateItem(&mut self, varvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn RemoveItem(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusPropertyValueData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusPropertyValueData_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusPropertyValueData_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusPropertyValueData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusPropertyValueData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusPropertyValueData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusPropertyValueData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvardata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateItem(::core::mem::transmute_copy(&varvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvardata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItem<Impl: ISClusPropertyValueData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItem(::core::mem::transmute_copy(&varindex)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            CreateItem: CreateItem::<Impl, IMPL_OFFSET>,
            RemoveItem: RemoveItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusPropertyValueData as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusPropertyValues_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusPropertyValue>;
    fn CreateItem(&mut self, bstrname: &super::super::Foundation::BSTR, varvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusPropertyValue>;
    fn RemoveItem(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusPropertyValues_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusPropertyValues_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusPropertyValues_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusPropertyValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusPropertyValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusPropertyValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pppropertyvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusPropertyValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pppropertyvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateItem(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&varvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItem<Impl: ISClusPropertyValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItem(::core::mem::transmute_copy(&varindex)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            CreateItem: CreateItem::<Impl, IMPL_OFFSET>,
            RemoveItem: RemoveItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusPropertyValues as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusRefObject_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Handle(&mut self) -> ::windows::core::Result<usize>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusRefObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusRefObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusRefObject_Vtbl {
        unsafe extern "system" fn Handle<Impl: ISClusRefObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Handle: Handle::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusRefObject as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusRegistryKeys_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AddItem(&mut self, bstrregistrykey: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveItem(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusRegistryKeys_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusRegistryKeys_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusRegistryKeys_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrregistrykey: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrregistrykey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Impl: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrregistrykey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddItem(::core::mem::transmute_copy(&bstrregistrykey)).into()
        }
        unsafe extern "system" fn RemoveItem<Impl: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItem(::core::mem::transmute_copy(&varindex)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            AddItem: AddItem::<Impl, IMPL_OFFSET>,
            RemoveItem: RemoveItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusRegistryKeys as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResDependencies_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResource>;
    fn CreateItem(&mut self, bstrresourcename: &super::super::Foundation::BSTR, bstrresourcetype: &super::super::Foundation::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows::core::Result<ISClusResource>;
    fn DeleteItem(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddItem(&mut self, presource: &::core::option::Option<ISClusResource>) -> ::windows::core::Result<()>;
    fn RemoveItem(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResDependencies_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusResDependencies_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusResDependencies_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresourcetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateItem(::core::mem::transmute_copy(&bstrresourcename), ::core::mem::transmute_copy(&bstrresourcetype), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusterresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteItem(::core::mem::transmute_copy(&varindex)).into()
        }
        unsafe extern "system" fn AddItem<Impl: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddItem(::core::mem::transmute(&presource)).into()
        }
        unsafe extern "system" fn RemoveItem<Impl: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItem(::core::mem::transmute_copy(&varindex)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            CreateItem: CreateItem::<Impl, IMPL_OFFSET>,
            DeleteItem: DeleteItem::<Impl, IMPL_OFFSET>,
            AddItem: AddItem::<Impl, IMPL_OFFSET>,
            RemoveItem: RemoveItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResDependencies as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResDependents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResource>;
    fn CreateItem(&mut self, bstrresourcename: &super::super::Foundation::BSTR, bstrresourcetype: &super::super::Foundation::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows::core::Result<ISClusResource>;
    fn DeleteItem(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddItem(&mut self, presource: &::core::option::Option<ISClusResource>) -> ::windows::core::Result<()>;
    fn RemoveItem(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResDependents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusResDependents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusResDependents_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresourcetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateItem(::core::mem::transmute_copy(&bstrresourcename), ::core::mem::transmute_copy(&bstrresourcetype), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusterresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteItem(::core::mem::transmute_copy(&varindex)).into()
        }
        unsafe extern "system" fn AddItem<Impl: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddItem(::core::mem::transmute(&presource)).into()
        }
        unsafe extern "system" fn RemoveItem<Impl: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItem(::core::mem::transmute_copy(&varindex)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            CreateItem: CreateItem::<Impl, IMPL_OFFSET>,
            DeleteItem: DeleteItem::<Impl, IMPL_OFFSET>,
            AddItem: AddItem::<Impl, IMPL_OFFSET>,
            RemoveItem: RemoveItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResDependents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResGroup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn CommonROProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateROProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn Handle(&mut self) -> ::windows::core::Result<usize>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrgroupname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<CLUSTER_GROUP_STATE>;
    fn OwnerNode(&mut self) -> ::windows::core::Result<ISClusNode>;
    fn Resources(&mut self) -> ::windows::core::Result<ISClusResGroupResources>;
    fn PreferredOwnerNodes(&mut self) -> ::windows::core::Result<ISClusResGroupPreferredOwnerNodes>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Online(&mut self, vartimeout: &super::super::System::Com::VARIANT, varnode: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Move(&mut self, vartimeout: &super::super::System::Com::VARIANT, varnode: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Offline(&mut self, vartimeout: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Cluster(&mut self) -> ::windows::core::Result<ISCluster>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusResGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusResGroup_Vtbl {
        unsafe extern "system" fn CommonProperties<Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommonProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommonROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrgroupname)).into()
        }
        unsafe extern "system" fn State<Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_GROUP_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *dwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OwnerNode<Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppownernode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OwnerNode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppownernode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclustergroupresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resources() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclustergroupresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredOwnerNodes<Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppownernodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredOwnerNodes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppownernodes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Online<Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartimeout: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varnode: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Online(::core::mem::transmute_copy(&vartimeout), ::core::mem::transmute_copy(&varnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarpending = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartimeout: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varnode: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(::core::mem::transmute_copy(&vartimeout), ::core::mem::transmute_copy(&varnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarpending = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Offline<Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartimeout: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offline(::core::mem::transmute_copy(&vartimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarpending = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Impl: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcluster: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cluster() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcluster = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CommonProperties: CommonProperties::<Impl, IMPL_OFFSET>,
            PrivateProperties: PrivateProperties::<Impl, IMPL_OFFSET>,
            CommonROProperties: CommonROProperties::<Impl, IMPL_OFFSET>,
            PrivateROProperties: PrivateROProperties::<Impl, IMPL_OFFSET>,
            Handle: Handle::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            OwnerNode: OwnerNode::<Impl, IMPL_OFFSET>,
            Resources: Resources::<Impl, IMPL_OFFSET>,
            PreferredOwnerNodes: PreferredOwnerNodes::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Online: Online::<Impl, IMPL_OFFSET>,
            Move: Move::<Impl, IMPL_OFFSET>,
            Offline: Offline::<Impl, IMPL_OFFSET>,
            Cluster: Cluster::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResGroup as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResGroupPreferredOwnerNodes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNode>;
    fn InsertItem(&mut self, pnode: &::core::option::Option<ISClusNode>, nposition: i32) -> ::windows::core::Result<()>;
    fn RemoveItem(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Modified(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SaveChanges(&mut self) -> ::windows::core::Result<()>;
    fn AddItem(&mut self, pnode: &::core::option::Option<ISClusNode>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResGroupPreferredOwnerNodes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusResGroupPreferredOwnerNodes_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusResGroupPreferredOwnerNodes_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertItem<Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, nposition: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertItem(::core::mem::transmute(&pnode), ::core::mem::transmute_copy(&nposition)).into()
        }
        unsafe extern "system" fn RemoveItem<Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItem(::core::mem::transmute_copy(&varindex)).into()
        }
        unsafe extern "system" fn Modified<Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modified() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmodified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveChanges<Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveChanges().into()
        }
        unsafe extern "system" fn AddItem<Impl: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddItem(::core::mem::transmute(&pnode)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            InsertItem: InsertItem::<Impl, IMPL_OFFSET>,
            RemoveItem: RemoveItem::<Impl, IMPL_OFFSET>,
            Modified: Modified::<Impl, IMPL_OFFSET>,
            SaveChanges: SaveChanges::<Impl, IMPL_OFFSET>,
            AddItem: AddItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResGroupPreferredOwnerNodes as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResGroupResources_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResource>;
    fn CreateItem(&mut self, bstrresourcename: &super::super::Foundation::BSTR, bstrresourcetype: &super::super::Foundation::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows::core::Result<ISClusResource>;
    fn DeleteItem(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResGroupResources_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusResGroupResources_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusResGroupResources_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresourcetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateItem(::core::mem::transmute_copy(&bstrresourcename), ::core::mem::transmute_copy(&bstrresourcetype), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusterresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteItem(::core::mem::transmute_copy(&varindex)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            CreateItem: CreateItem::<Impl, IMPL_OFFSET>,
            DeleteItem: DeleteItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResGroupResources as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResGroups_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResGroup>;
    fn CreateItem(&mut self, bstrresourcegroupname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISClusResGroup>;
    fn DeleteItem(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResGroups_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusResGroups_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusResGroups_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusresgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourcegroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppresourcegroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateItem(::core::mem::transmute_copy(&bstrresourcegroupname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresourcegroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteItem(::core::mem::transmute_copy(&varindex)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            CreateItem: CreateItem::<Impl, IMPL_OFFSET>,
            DeleteItem: DeleteItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResGroups as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResPossibleOwnerNodes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNode>;
    fn AddItem(&mut self, pnode: &::core::option::Option<ISClusNode>) -> ::windows::core::Result<()>;
    fn RemoveItem(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Modified(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResPossibleOwnerNodes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusResPossibleOwnerNodes_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusResPossibleOwnerNodes_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddItem(::core::mem::transmute(&pnode)).into()
        }
        unsafe extern "system" fn RemoveItem<Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItem(::core::mem::transmute_copy(&varindex)).into()
        }
        unsafe extern "system" fn Modified<Impl: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modified() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmodified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            AddItem: AddItem::<Impl, IMPL_OFFSET>,
            RemoveItem: RemoveItem::<Impl, IMPL_OFFSET>,
            Modified: Modified::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResPossibleOwnerNodes as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResType_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn CommonROProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateROProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Cluster(&mut self) -> ::windows::core::Result<ISCluster>;
    fn Resources(&mut self) -> ::windows::core::Result<ISClusResTypeResources>;
    fn PossibleOwnerNodes(&mut self) -> ::windows::core::Result<ISClusResTypePossibleOwnerNodes>;
    fn AvailableDisks(&mut self) -> ::windows::core::Result<ISClusDisks>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusResType_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusResType_Vtbl {
        unsafe extern "system" fn CommonProperties<Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommonProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommonROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Cluster<Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcluster: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cluster() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcluster = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclusterrestyperesources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resources() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusterrestyperesources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PossibleOwnerNodes<Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppownernodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PossibleOwnerNodes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppownernodes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableDisks<Impl: ISClusResType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppavailabledisks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableDisks() {
                ::core::result::Result::Ok(ok__) => {
                    *ppavailabledisks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CommonProperties: CommonProperties::<Impl, IMPL_OFFSET>,
            PrivateProperties: PrivateProperties::<Impl, IMPL_OFFSET>,
            CommonROProperties: CommonROProperties::<Impl, IMPL_OFFSET>,
            PrivateROProperties: PrivateROProperties::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Cluster: Cluster::<Impl, IMPL_OFFSET>,
            Resources: Resources::<Impl, IMPL_OFFSET>,
            PossibleOwnerNodes: PossibleOwnerNodes::<Impl, IMPL_OFFSET>,
            AvailableDisks: AvailableDisks::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResType as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResTypePossibleOwnerNodes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResTypePossibleOwnerNodes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusResTypePossibleOwnerNodes_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusResTypePossibleOwnerNodes_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResTypePossibleOwnerNodes as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResTypeResources_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResource>;
    fn CreateItem(&mut self, bstrresourcename: &super::super::Foundation::BSTR, bstrgroupname: &super::super::Foundation::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows::core::Result<ISClusResource>;
    fn DeleteItem(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResTypeResources_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusResTypeResources_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusResTypeResources_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateItem(::core::mem::transmute_copy(&bstrresourcename), ::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusterresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteItem(::core::mem::transmute_copy(&varindex)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            CreateItem: CreateItem::<Impl, IMPL_OFFSET>,
            DeleteItem: DeleteItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResTypeResources as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResTypes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResType>;
    fn CreateItem(&mut self, bstrresourcetypename: &super::super::Foundation::BSTR, bstrdisplayname: &super::super::Foundation::BSTR, bstrresourcetypedll: &super::super::Foundation::BSTR, dwlooksalivepollinterval: i32, dwisalivepollinterval: i32) -> ::windows::core::Result<ISClusResType>;
    fn DeleteItem(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResTypes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusResTypes_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusResTypes_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusrestype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusrestype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourcetypename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdisplayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresourcetypedll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwlooksalivepollinterval: i32, dwisalivepollinterval: i32, ppresourcetype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateItem(::core::mem::transmute_copy(&bstrresourcetypename), ::core::mem::transmute_copy(&bstrdisplayname), ::core::mem::transmute_copy(&bstrresourcetypedll), ::core::mem::transmute_copy(&dwlooksalivepollinterval), ::core::mem::transmute_copy(&dwisalivepollinterval)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresourcetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteItem(::core::mem::transmute_copy(&varindex)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            CreateItem: CreateItem::<Impl, IMPL_OFFSET>,
            DeleteItem: DeleteItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResTypes as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResource_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn CommonROProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateROProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn Handle(&mut self) -> ::windows::core::Result<usize>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrresourcename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<CLUSTER_RESOURCE_STATE>;
    fn CoreFlag(&mut self) -> ::windows::core::Result<CLUS_FLAGS>;
    fn BecomeQuorumResource(&mut self, bstrdevicepath: &super::super::Foundation::BSTR, lmaxlogsize: i32) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Fail(&mut self) -> ::windows::core::Result<()>;
    fn Online(&mut self, ntimeout: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Offline(&mut self, ntimeout: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn ChangeResourceGroup(&mut self, presourcegroup: &::core::option::Option<ISClusResGroup>) -> ::windows::core::Result<()>;
    fn AddResourceNode(&mut self, pnode: &::core::option::Option<ISClusNode>) -> ::windows::core::Result<()>;
    fn RemoveResourceNode(&mut self, pnode: &::core::option::Option<ISClusNode>) -> ::windows::core::Result<()>;
    fn CanResourceBeDependent(&mut self, presource: &::core::option::Option<ISClusResource>) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PossibleOwnerNodes(&mut self) -> ::windows::core::Result<ISClusResPossibleOwnerNodes>;
    fn Dependencies(&mut self) -> ::windows::core::Result<ISClusResDependencies>;
    fn Dependents(&mut self) -> ::windows::core::Result<ISClusResDependents>;
    fn Group(&mut self) -> ::windows::core::Result<ISClusResGroup>;
    fn OwnerNode(&mut self) -> ::windows::core::Result<ISClusNode>;
    fn Cluster(&mut self) -> ::windows::core::Result<ISCluster>;
    fn ClassInfo(&mut self) -> ::windows::core::Result<CLUSTER_RESOURCE_CLASS>;
    fn Disk(&mut self) -> ::windows::core::Result<ISClusDisk>;
    fn RegistryKeys(&mut self) -> ::windows::core::Result<ISClusRegistryKeys>;
    fn CryptoKeys(&mut self) -> ::windows::core::Result<ISClusCryptoKeys>;
    fn TypeName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Type(&mut self) -> ::windows::core::Result<ISClusResType>;
    fn MaintenanceMode(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetMaintenanceMode(&mut self, bmaintenancemode: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusResource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusResource_Vtbl {
        unsafe extern "system" fn CommonProperties<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommonProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommonROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrresourcename)).into()
        }
        unsafe extern "system" fn State<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_RESOURCE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *dwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoreFlag<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcoreflag: *mut CLUS_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoreFlag() {
                ::core::result::Result::Ok(ok__) => {
                    *dwcoreflag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BecomeQuorumResource<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmaxlogsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BecomeQuorumResource(::core::mem::transmute_copy(&bstrdevicepath), ::core::mem::transmute_copy(&lmaxlogsize)).into()
        }
        unsafe extern "system" fn Delete<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Fail<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Fail().into()
        }
        unsafe extern "system" fn Online<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntimeout: i32, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Online(::core::mem::transmute_copy(&ntimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarpending = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Offline<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntimeout: i32, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offline(::core::mem::transmute_copy(&ntimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarpending = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeResourceGroup<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcegroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangeResourceGroup(::core::mem::transmute(&presourcegroup)).into()
        }
        unsafe extern "system" fn AddResourceNode<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddResourceNode(::core::mem::transmute(&pnode)).into()
        }
        unsafe extern "system" fn RemoveResourceNode<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveResourceNode(::core::mem::transmute(&pnode)).into()
        }
        unsafe extern "system" fn CanResourceBeDependent<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pvardependent: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanResourceBeDependent(::core::mem::transmute(&presource)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvardependent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PossibleOwnerNodes<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppownernodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PossibleOwnerNodes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppownernodes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dependencies<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresdependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dependencies() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresdependencies = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dependents<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresdependents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dependents() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresdependents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OwnerNode<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppownernode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OwnerNode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppownernode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcluster: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cluster() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcluster = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClassInfo<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcclassinfo: *mut CLUSTER_RESOURCE_CLASS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClassInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *prcclassinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disk<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disk() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdisk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegistryKeys<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppregistrykeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegistryKeys() {
                ::core::result::Result::Ok(ok__) => {
                    *ppregistrykeys = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CryptoKeys<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcryptokeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CryptoKeys() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcryptokeys = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TypeName<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtypename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TypeName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtypename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresourcetype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresourcetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaintenanceMode<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmaintenancemode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaintenanceMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pbmaintenancemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaintenanceMode<Impl: ISClusResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmaintenancemode: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaintenanceMode(::core::mem::transmute_copy(&bmaintenancemode)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CommonProperties: CommonProperties::<Impl, IMPL_OFFSET>,
            PrivateProperties: PrivateProperties::<Impl, IMPL_OFFSET>,
            CommonROProperties: CommonROProperties::<Impl, IMPL_OFFSET>,
            PrivateROProperties: PrivateROProperties::<Impl, IMPL_OFFSET>,
            Handle: Handle::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            CoreFlag: CoreFlag::<Impl, IMPL_OFFSET>,
            BecomeQuorumResource: BecomeQuorumResource::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Fail: Fail::<Impl, IMPL_OFFSET>,
            Online: Online::<Impl, IMPL_OFFSET>,
            Offline: Offline::<Impl, IMPL_OFFSET>,
            ChangeResourceGroup: ChangeResourceGroup::<Impl, IMPL_OFFSET>,
            AddResourceNode: AddResourceNode::<Impl, IMPL_OFFSET>,
            RemoveResourceNode: RemoveResourceNode::<Impl, IMPL_OFFSET>,
            CanResourceBeDependent: CanResourceBeDependent::<Impl, IMPL_OFFSET>,
            PossibleOwnerNodes: PossibleOwnerNodes::<Impl, IMPL_OFFSET>,
            Dependencies: Dependencies::<Impl, IMPL_OFFSET>,
            Dependents: Dependents::<Impl, IMPL_OFFSET>,
            Group: Group::<Impl, IMPL_OFFSET>,
            OwnerNode: OwnerNode::<Impl, IMPL_OFFSET>,
            Cluster: Cluster::<Impl, IMPL_OFFSET>,
            ClassInfo: ClassInfo::<Impl, IMPL_OFFSET>,
            Disk: Disk::<Impl, IMPL_OFFSET>,
            RegistryKeys: RegistryKeys::<Impl, IMPL_OFFSET>,
            CryptoKeys: CryptoKeys::<Impl, IMPL_OFFSET>,
            TypeName: TypeName::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            MaintenanceMode: MaintenanceMode::<Impl, IMPL_OFFSET>,
            SetMaintenanceMode: SetMaintenanceMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResource as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusResources_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResource>;
    fn CreateItem(&mut self, bstrresourcename: &super::super::Foundation::BSTR, bstrresourcetype: &super::super::Foundation::BSTR, bstrgroupname: &super::super::Foundation::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows::core::Result<ISClusResource>;
    fn DeleteItem(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusResources_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusResources_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusResources_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresourcetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateItem(::core::mem::transmute_copy(&bstrresourcename), ::core::mem::transmute_copy(&bstrresourcetype), ::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusterresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: ISClusResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteItem(::core::mem::transmute_copy(&varindex)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            CreateItem: CreateItem::<Impl, IMPL_OFFSET>,
            DeleteItem: DeleteItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusResources as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusScsiAddress_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PortNumber(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PathId(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn TargetId(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Lun(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusScsiAddress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusScsiAddress_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusScsiAddress_Vtbl {
        unsafe extern "system" fn PortNumber<Impl: ISClusScsiAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarportnumber: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PortNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarportnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathId<Impl: ISClusScsiAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarpathid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarpathid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetId<Impl: ISClusScsiAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvartargetid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvartargetid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lun<Impl: ISClusScsiAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarlun: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lun() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarlun = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PortNumber: PortNumber::<Impl, IMPL_OFFSET>,
            PathId: PathId::<Impl, IMPL_OFFSET>,
            TargetId: TargetId::<Impl, IMPL_OFFSET>,
            Lun: Lun::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusScsiAddress as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusVersion_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MajorVersion(&mut self) -> ::windows::core::Result<i32>;
    fn MinorVersion(&mut self) -> ::windows::core::Result<i32>;
    fn BuildNumber(&mut self) -> ::windows::core::Result<i16>;
    fn VendorId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CSDVersion(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ClusterHighestVersion(&mut self) -> ::windows::core::Result<i32>;
    fn ClusterLowestVersion(&mut self) -> ::windows::core::Result<i32>;
    fn Flags(&mut self) -> ::windows::core::Result<i32>;
    fn MixedVersion(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusVersion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusVersion_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusVersion_Vtbl {
        unsafe extern "system" fn Name<Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrclustername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrclustername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pnmajorversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pnminorversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildNumber<Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnbuildnumber: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuildNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pnbuildnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VendorId<Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrvendorid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VendorId() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvendorid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSDVersion<Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsdversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CSDVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcsdversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClusterHighestVersion<Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnclusterhighestversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClusterHighestVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pnclusterhighestversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClusterLowestVersion<Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnclusterlowestversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClusterLowestVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pnclusterlowestversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *pnflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MixedVersion<Impl: ISClusVersion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmixedversion: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MixedVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmixedversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            MajorVersion: MajorVersion::<Impl, IMPL_OFFSET>,
            MinorVersion: MinorVersion::<Impl, IMPL_OFFSET>,
            BuildNumber: BuildNumber::<Impl, IMPL_OFFSET>,
            VendorId: VendorId::<Impl, IMPL_OFFSET>,
            CSDVersion: CSDVersion::<Impl, IMPL_OFFSET>,
            ClusterHighestVersion: ClusterHighestVersion::<Impl, IMPL_OFFSET>,
            ClusterLowestVersion: ClusterLowestVersion::<Impl, IMPL_OFFSET>,
            Flags: Flags::<Impl, IMPL_OFFSET>,
            MixedVersion: MixedVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusVersion as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISCluster_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn CommonROProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn PrivateROProperties(&mut self) -> ::windows::core::Result<ISClusProperties>;
    fn Handle(&mut self) -> ::windows::core::Result<usize>;
    fn Open(&mut self, bstrclustername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrclustername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Version(&mut self) -> ::windows::core::Result<ISClusVersion>;
    fn SetQuorumResource(&mut self, pclusterresource: &::core::option::Option<ISClusResource>) -> ::windows::core::Result<()>;
    fn QuorumResource(&mut self) -> ::windows::core::Result<ISClusResource>;
    fn QuorumLogSize(&mut self) -> ::windows::core::Result<i32>;
    fn SetQuorumLogSize(&mut self, nlogsize: i32) -> ::windows::core::Result<()>;
    fn QuorumPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetQuorumPath(&mut self, ppath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Nodes(&mut self) -> ::windows::core::Result<ISClusNodes>;
    fn ResourceGroups(&mut self) -> ::windows::core::Result<ISClusResGroups>;
    fn Resources(&mut self) -> ::windows::core::Result<ISClusResources>;
    fn ResourceTypes(&mut self) -> ::windows::core::Result<ISClusResTypes>;
    fn Networks(&mut self) -> ::windows::core::Result<ISClusNetworks>;
    fn NetInterfaces(&mut self) -> ::windows::core::Result<ISClusNetInterfaces>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISCluster_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCluster_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISCluster_Vtbl {
        unsafe extern "system" fn CommonProperties<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommonProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommonROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateROProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclustername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&bstrclustername)).into()
        }
        unsafe extern "system" fn Name<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclustername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrclustername)).into()
        }
        unsafe extern "system" fn Version<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclusversion: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuorumResource<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclusterresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuorumResource(::core::mem::transmute(&pclusterresource)).into()
        }
        unsafe extern "system" fn QuorumResource<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclusterresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuorumResource() {
                ::core::result::Result::Ok(ok__) => {
                    *pclusterresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuorumLogSize<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlogsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuorumLogSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pnlogsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuorumLogSize<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nlogsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuorumLogSize(::core::mem::transmute_copy(&nlogsize)).into()
        }
        unsafe extern "system" fn QuorumPath<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuorumPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pppath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuorumPath<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuorumPath(::core::mem::transmute_copy(&ppath)).into()
        }
        unsafe extern "system" fn Nodes<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Nodes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnodes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceGroups<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclusterresourcegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusterresourcegroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclusterresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resources() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclusterresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceTypes<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresourcetypes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresourcetypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Networks<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetworks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Networks() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetworks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetInterfaces<Impl: ISCluster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetinterfaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetinterfaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CommonProperties: CommonProperties::<Impl, IMPL_OFFSET>,
            PrivateProperties: PrivateProperties::<Impl, IMPL_OFFSET>,
            CommonROProperties: CommonROProperties::<Impl, IMPL_OFFSET>,
            PrivateROProperties: PrivateROProperties::<Impl, IMPL_OFFSET>,
            Handle: Handle::<Impl, IMPL_OFFSET>,
            Open: Open::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            SetQuorumResource: SetQuorumResource::<Impl, IMPL_OFFSET>,
            QuorumResource: QuorumResource::<Impl, IMPL_OFFSET>,
            QuorumLogSize: QuorumLogSize::<Impl, IMPL_OFFSET>,
            SetQuorumLogSize: SetQuorumLogSize::<Impl, IMPL_OFFSET>,
            QuorumPath: QuorumPath::<Impl, IMPL_OFFSET>,
            SetQuorumPath: SetQuorumPath::<Impl, IMPL_OFFSET>,
            Nodes: Nodes::<Impl, IMPL_OFFSET>,
            ResourceGroups: ResourceGroups::<Impl, IMPL_OFFSET>,
            Resources: Resources::<Impl, IMPL_OFFSET>,
            ResourceTypes: ResourceTypes::<Impl, IMPL_OFFSET>,
            Networks: Networks::<Impl, IMPL_OFFSET>,
            NetInterfaces: NetInterfaces::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCluster as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISClusterNames_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DomainName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISClusterNames_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISClusterNames_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISClusterNames_Vtbl {
        unsafe extern "system" fn Count<Impl: ISClusterNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusterNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusterNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISClusterNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrclustername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrclustername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainName<Impl: ISClusterNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdomainname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdomainname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            DomainName: DomainName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISClusterNames as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISDomainNames_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISDomainNames_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISDomainNames_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISDomainNames_Vtbl {
        unsafe extern "system" fn Count<Impl: ISDomainNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISDomainNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISDomainNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Item<Impl: ISDomainNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrdomainname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdomainname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISDomainNames as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWCContextMenuCallback_Impl: Sized {
    fn AddExtensionMenuItem(&mut self, lpszname: &super::super::Foundation::BSTR, lpszstatusbartext: &super::super::Foundation::BSTR, ncommandid: u32, nsubmenucommandid: u32, uflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWCContextMenuCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWCContextMenuCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWCContextMenuCallback_Vtbl {
        unsafe extern "system" fn AddExtensionMenuItem<Impl: IWCContextMenuCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpszstatusbartext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ncommandid: u32, nsubmenucommandid: u32, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddExtensionMenuItem(::core::mem::transmute_copy(&lpszname), ::core::mem::transmute_copy(&lpszstatusbartext), ::core::mem::transmute_copy(&ncommandid), ::core::mem::transmute_copy(&nsubmenucommandid), ::core::mem::transmute_copy(&uflags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddExtensionMenuItem: AddExtensionMenuItem::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWCContextMenuCallback as ::windows::core::Interface>::IID
    }
}
pub trait IWCPropertySheetCallback_Impl: Sized {
    fn AddPropertySheetPage(&mut self, hpage: *const i32) -> ::windows::core::Result<()>;
}
impl IWCPropertySheetCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWCPropertySheetCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWCPropertySheetCallback_Vtbl {
        unsafe extern "system" fn AddPropertySheetPage<Impl: IWCPropertySheetCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPropertySheetPage(::core::mem::transmute_copy(&hpage)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddPropertySheetPage: AddPropertySheetPage::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWCPropertySheetCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWCWizard97Callback_Impl: Sized {
    fn AddWizard97Page(&mut self, hpage: *const i32) -> ::windows::core::Result<()>;
    fn EnableNext(&mut self, hpage: *const i32, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWCWizard97Callback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWCWizard97Callback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWCWizard97Callback_Vtbl {
        unsafe extern "system" fn AddWizard97Page<Impl: IWCWizard97Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddWizard97Page(::core::mem::transmute_copy(&hpage)).into()
        }
        unsafe extern "system" fn EnableNext<Impl: IWCWizard97Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: *const i32, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNext(::core::mem::transmute_copy(&hpage), ::core::mem::transmute_copy(&benable)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddWizard97Page: AddWizard97Page::<Impl, IMPL_OFFSET>,
            EnableNext: EnableNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWCWizard97Callback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWCWizardCallback_Impl: Sized {
    fn AddWizardPage(&mut self, hpage: *const i32) -> ::windows::core::Result<()>;
    fn EnableNext(&mut self, hpage: *const i32, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWCWizardCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWCWizardCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWCWizardCallback_Vtbl {
        unsafe extern "system" fn AddWizardPage<Impl: IWCWizardCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddWizardPage(::core::mem::transmute_copy(&hpage)).into()
        }
        unsafe extern "system" fn EnableNext<Impl: IWCWizardCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpage: *const i32, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNext(::core::mem::transmute_copy(&hpage), ::core::mem::transmute_copy(&benable)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddWizardPage: AddWizardPage::<Impl, IMPL_OFFSET>,
            EnableNext: EnableNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWCWizardCallback as ::windows::core::Interface>::IID
    }
}
pub trait IWEExtendContextMenu_Impl: Sized {
    fn AddContextMenuItems(&mut self, pidata: &::core::option::Option<::windows::core::IUnknown>, picallback: &::core::option::Option<IWCContextMenuCallback>) -> ::windows::core::Result<()>;
}
impl IWEExtendContextMenu_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWEExtendContextMenu_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWEExtendContextMenu_Vtbl {
        unsafe extern "system" fn AddContextMenuItems<Impl: IWEExtendContextMenu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddContextMenuItems(::core::mem::transmute(&pidata), ::core::mem::transmute(&picallback)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddContextMenuItems: AddContextMenuItems::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWEExtendContextMenu as ::windows::core::Interface>::IID
    }
}
pub trait IWEExtendPropertySheet_Impl: Sized {
    fn CreatePropertySheetPages(&mut self, pidata: &::core::option::Option<::windows::core::IUnknown>, picallback: &::core::option::Option<IWCPropertySheetCallback>) -> ::windows::core::Result<()>;
}
impl IWEExtendPropertySheet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWEExtendPropertySheet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWEExtendPropertySheet_Vtbl {
        unsafe extern "system" fn CreatePropertySheetPages<Impl: IWEExtendPropertySheet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePropertySheetPages(::core::mem::transmute(&pidata), ::core::mem::transmute(&picallback)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreatePropertySheetPages: CreatePropertySheetPages::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWEExtendPropertySheet as ::windows::core::Interface>::IID
    }
}
pub trait IWEExtendWizard_Impl: Sized {
    fn CreateWizardPages(&mut self, pidata: &::core::option::Option<::windows::core::IUnknown>, picallback: &::core::option::Option<IWCWizardCallback>) -> ::windows::core::Result<()>;
}
impl IWEExtendWizard_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWEExtendWizard_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWEExtendWizard_Vtbl {
        unsafe extern "system" fn CreateWizardPages<Impl: IWEExtendWizard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateWizardPages(::core::mem::transmute(&pidata), ::core::mem::transmute(&picallback)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateWizardPages: CreateWizardPages::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWEExtendWizard as ::windows::core::Interface>::IID
    }
}
pub trait IWEExtendWizard97_Impl: Sized {
    fn CreateWizard97Pages(&mut self, pidata: &::core::option::Option<::windows::core::IUnknown>, picallback: &::core::option::Option<IWCWizard97Callback>) -> ::windows::core::Result<()>;
}
impl IWEExtendWizard97_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWEExtendWizard97_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWEExtendWizard97_Vtbl {
        unsafe extern "system" fn CreateWizard97Pages<Impl: IWEExtendWizard97_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateWizard97Pages(::core::mem::transmute(&pidata), ::core::mem::transmute(&picallback)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateWizard97Pages: CreateWizard97Pages::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWEExtendWizard97 as ::windows::core::Interface>::IID
    }
}
pub trait IWEInvokeCommand_Impl: Sized {
    fn InvokeCommand(&mut self, ncommandid: u32, pidata: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IWEInvokeCommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWEInvokeCommand_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWEInvokeCommand_Vtbl {
        unsafe extern "system" fn InvokeCommand<Impl: IWEInvokeCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncommandid: u32, pidata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvokeCommand(::core::mem::transmute_copy(&ncommandid), ::core::mem::transmute(&pidata)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), InvokeCommand: InvokeCommand::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWEInvokeCommand as ::windows::core::Interface>::IID
    }
}
