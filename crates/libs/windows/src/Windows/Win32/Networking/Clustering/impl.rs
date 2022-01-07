pub trait IGetClusterDataInfoImpl: Sized {
    fn GetClusterName();
    fn GetClusterHandle();
    fn GetObjectCount();
}
impl ::windows::core::RuntimeName for IGetClusterDataInfo {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IGetClusterDataInfo";
}
impl IGetClusterDataInfoVtbl {
    pub const fn new<Impl: IGetClusterDataInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGetClusterDataInfoVtbl {
        unsafe extern "system" fn GetClusterName<Impl: IGetClusterDataInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcchname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClusterName(::core::mem::transmute_copy(&lpszname), pcchname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClusterHandle<Impl: IGetClusterDataInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> *mut _HCLUSTER {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClusterHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectCount<Impl: IGetClusterDataInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetObjectCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGetClusterDataInfo>, base.5, GetClusterName::<Impl, OFFSET>, GetClusterHandle::<Impl, OFFSET>, GetObjectCount::<Impl, OFFSET>)
    }
}
pub trait IGetClusterGroupInfoImpl: Sized {
    fn GetGroupHandle();
}
impl ::windows::core::RuntimeName for IGetClusterGroupInfo {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IGetClusterGroupInfo";
}
impl IGetClusterGroupInfoVtbl {
    pub const fn new<Impl: IGetClusterGroupInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGetClusterGroupInfoVtbl {
        unsafe extern "system" fn GetGroupHandle<Impl: IGetClusterGroupInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HGROUP {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGroupHandle(lobjindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGetClusterGroupInfo>, base.5, GetGroupHandle::<Impl, OFFSET>)
    }
}
pub trait IGetClusterNetInterfaceInfoImpl: Sized {
    fn GetNetInterfaceHandle();
}
impl ::windows::core::RuntimeName for IGetClusterNetInterfaceInfo {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IGetClusterNetInterfaceInfo";
}
impl IGetClusterNetInterfaceInfoVtbl {
    pub const fn new<Impl: IGetClusterNetInterfaceInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGetClusterNetInterfaceInfoVtbl {
        unsafe extern "system" fn GetNetInterfaceHandle<Impl: IGetClusterNetInterfaceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HNETINTERFACE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetInterfaceHandle(lobjindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGetClusterNetInterfaceInfo>, base.5, GetNetInterfaceHandle::<Impl, OFFSET>)
    }
}
pub trait IGetClusterNetworkInfoImpl: Sized {
    fn GetNetworkHandle();
}
impl ::windows::core::RuntimeName for IGetClusterNetworkInfo {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IGetClusterNetworkInfo";
}
impl IGetClusterNetworkInfoVtbl {
    pub const fn new<Impl: IGetClusterNetworkInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGetClusterNetworkInfoVtbl {
        unsafe extern "system" fn GetNetworkHandle<Impl: IGetClusterNetworkInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HNETWORK {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetworkHandle(lobjindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGetClusterNetworkInfo>, base.5, GetNetworkHandle::<Impl, OFFSET>)
    }
}
pub trait IGetClusterNodeInfoImpl: Sized {
    fn GetNodeHandle();
}
impl ::windows::core::RuntimeName for IGetClusterNodeInfo {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IGetClusterNodeInfo";
}
impl IGetClusterNodeInfoVtbl {
    pub const fn new<Impl: IGetClusterNodeInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGetClusterNodeInfoVtbl {
        unsafe extern "system" fn GetNodeHandle<Impl: IGetClusterNodeInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HNODE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNodeHandle(lobjindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGetClusterNodeInfo>, base.5, GetNodeHandle::<Impl, OFFSET>)
    }
}
pub trait IGetClusterObjectInfoImpl: Sized {
    fn GetObjectName();
    fn GetObjectType();
}
impl ::windows::core::RuntimeName for IGetClusterObjectInfo {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IGetClusterObjectInfo";
}
impl IGetClusterObjectInfoVtbl {
    pub const fn new<Impl: IGetClusterObjectInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGetClusterObjectInfoVtbl {
        unsafe extern "system" fn GetObjectName<Impl: IGetClusterObjectInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32, lpszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcchname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetObjectName(lobjindex, ::core::mem::transmute_copy(&lpszname), pcchname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectType<Impl: IGetClusterObjectInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> CLUADMEX_OBJECT_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetObjectType(lobjindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGetClusterObjectInfo>, base.5, GetObjectName::<Impl, OFFSET>, GetObjectType::<Impl, OFFSET>)
    }
}
pub trait IGetClusterResourceInfoImpl: Sized {
    fn GetResourceHandle();
    fn GetResourceTypeName();
    fn GetResourceNetworkName();
}
impl ::windows::core::RuntimeName for IGetClusterResourceInfo {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IGetClusterResourceInfo";
}
impl IGetClusterResourceInfoVtbl {
    pub const fn new<Impl: IGetClusterResourceInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGetClusterResourceInfoVtbl {
        unsafe extern "system" fn GetResourceHandle<Impl: IGetClusterResourceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HRESOURCE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetResourceHandle(lobjindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceTypeName<Impl: IGetClusterResourceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32, lpszrestypename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcchrestypename: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetResourceTypeName(lobjindex, ::core::mem::transmute_copy(&lpszrestypename), pcchrestypename) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceNetworkName<Impl: IGetClusterResourceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjindex: i32, lpsznetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcchnetname: *mut u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetResourceNetworkName(lobjindex, ::core::mem::transmute_copy(&lpsznetname), pcchnetname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGetClusterResourceInfo>, base.5, GetResourceHandle::<Impl, OFFSET>, GetResourceTypeName::<Impl, OFFSET>, GetResourceNetworkName::<Impl, OFFSET>)
    }
}
pub trait IGetClusterUIInfoImpl: Sized {
    fn GetClusterName();
    fn GetLocale();
    fn GetFont();
    fn GetIcon();
}
impl ::windows::core::RuntimeName for IGetClusterUIInfo {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IGetClusterUIInfo";
}
impl IGetClusterUIInfoVtbl {
    pub const fn new<Impl: IGetClusterUIInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGetClusterUIInfoVtbl {
        unsafe extern "system" fn GetClusterName<Impl: IGetClusterUIInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcchname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClusterName(::core::mem::transmute_copy(&lpszname), pcchname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocale<Impl: IGetClusterUIInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLocale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFont<Impl: IGetClusterUIInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Graphics::Gdi::HFONT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIcon<Impl: IGetClusterUIInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::UI::WindowsAndMessaging::HICON {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIcon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGetClusterUIInfo>, base.5, GetClusterName::<Impl, OFFSET>, GetLocale::<Impl, OFFSET>, GetFont::<Impl, OFFSET>, GetIcon::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusApplicationImpl: Sized + IDispatchImpl {
    fn DomainNames();
    fn ClusterNames();
    fn OpenCluster();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusApplication {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusApplication";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusApplicationVtbl {
    pub const fn new<Impl: ISClusApplicationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusApplicationVtbl {
        unsafe extern "system" fn DomainNames<Impl: ISClusApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdomains: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DomainNames(::core::mem::transmute_copy(&ppdomains)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClusterNames<Impl: ISClusApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdomainname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppclusters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClusterNames(&*(&bstrdomainname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclusters)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenCluster<Impl: ISClusApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrclustername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcluster: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenCluster(&*(&bstrclustername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcluster)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusApplication>, base.5, DomainNames::<Impl, OFFSET>, ClusterNames::<Impl, OFFSET>, OpenCluster::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusCryptoKeysImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn AddItem();
    fn RemoveItem();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusCryptoKeys {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusCryptoKeys";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusCryptoKeysVtbl {
    pub const fn new<Impl: ISClusCryptoKeysImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusCryptoKeysVtbl {
        unsafe extern "system" fn Count<Impl: ISClusCryptoKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusCryptoKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusCryptoKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusCryptoKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrcyrptokey: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrcyrptokey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Impl: ISClusCryptoKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcryptokey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddItem(&*(&bstrcryptokey as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItem<Impl: ISClusCryptoKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveItem(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusCryptoKeys>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>, AddItem::<Impl, OFFSET>, RemoveItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusDiskImpl: Sized + IDispatchImpl {
    fn Signature();
    fn ScsiAddress();
    fn DiskNumber();
    fn Partitions();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusDisk {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusDisk";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusDiskVtbl {
    pub const fn new<Impl: ISClusDiskImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusDiskVtbl {
        unsafe extern "system" fn Signature<Impl: ISClusDiskImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsignature: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Signature(::core::mem::transmute_copy(&plsignature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScsiAddress<Impl: ISClusDiskImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppscsiaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScsiAddress(::core::mem::transmute_copy(&ppscsiaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiskNumber<Impl: ISClusDiskImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldisknumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DiskNumber(::core::mem::transmute_copy(&pldisknumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Partitions<Impl: ISClusDiskImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppartitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Partitions(::core::mem::transmute_copy(&pppartitions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusDisk>, base.5, Signature::<Impl, OFFSET>, ScsiAddress::<Impl, OFFSET>, DiskNumber::<Impl, OFFSET>, Partitions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusDisksImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusDisks {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusDisks";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusDisksVtbl {
    pub const fn new<Impl: ISClusDisksImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusDisksVtbl {
        unsafe extern "system" fn Count<Impl: ISClusDisksImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusDisksImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusDisksImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppdisk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdisk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusDisks>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNetInterfaceImpl: Sized + IDispatchImpl {
    fn CommonProperties();
    fn PrivateProperties();
    fn CommonROProperties();
    fn PrivateROProperties();
    fn Name();
    fn Handle();
    fn State();
    fn Cluster();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusNetInterface {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusNetInterface";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetInterfaceVtbl {
    pub const fn new<Impl: ISClusNetInterfaceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusNetInterfaceVtbl {
        unsafe extern "system" fn CommonProperties<Impl: ISClusNetInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommonProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Impl: ISClusNetInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Impl: ISClusNetInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommonROProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Impl: ISClusNetInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateROProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISClusNetInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: ISClusNetInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handle(::core::mem::transmute_copy(&phandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ISClusNetInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_NETINTERFACE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&dwstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Impl: ISClusNetInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcluster: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cluster(::core::mem::transmute_copy(&ppcluster)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusNetInterface>, base.5, CommonProperties::<Impl, OFFSET>, PrivateProperties::<Impl, OFFSET>, CommonROProperties::<Impl, OFFSET>, PrivateROProperties::<Impl, OFFSET>, Name::<Impl, OFFSET>, Handle::<Impl, OFFSET>, State::<Impl, OFFSET>, Cluster::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNetInterfacesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusNetInterfaces {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusNetInterfaces";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetInterfacesVtbl {
    pub const fn new<Impl: ISClusNetInterfacesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusNetInterfacesVtbl {
        unsafe extern "system" fn Count<Impl: ISClusNetInterfacesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusNetInterfacesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusNetInterfacesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusNetInterfacesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusnetinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclusnetinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusNetInterfaces>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNetworkImpl: Sized + IDispatchImpl {
    fn CommonProperties();
    fn PrivateProperties();
    fn CommonROProperties();
    fn PrivateROProperties();
    fn Handle();
    fn Name();
    fn SetName();
    fn NetworkID();
    fn State();
    fn NetInterfaces();
    fn Cluster();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusNetwork {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusNetwork";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetworkVtbl {
    pub const fn new<Impl: ISClusNetworkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusNetworkVtbl {
        unsafe extern "system" fn CommonProperties<Impl: ISClusNetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommonProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Impl: ISClusNetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Impl: ISClusNetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommonROProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Impl: ISClusNetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateROProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: ISClusNetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handle(::core::mem::transmute_copy(&phandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISClusNetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ISClusNetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrnetworkname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&bstrnetworkname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkID<Impl: ISClusNetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrnetworkid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NetworkID(::core::mem::transmute_copy(&pbstrnetworkid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ISClusNetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_NETWORK_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&dwstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetInterfaces<Impl: ISClusNetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclusnetinterfaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NetInterfaces(::core::mem::transmute_copy(&ppclusnetinterfaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Impl: ISClusNetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcluster: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cluster(::core::mem::transmute_copy(&ppcluster)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusNetwork>, base.5, CommonProperties::<Impl, OFFSET>, PrivateProperties::<Impl, OFFSET>, CommonROProperties::<Impl, OFFSET>, PrivateROProperties::<Impl, OFFSET>, Handle::<Impl, OFFSET>, Name::<Impl, OFFSET>, SetName::<Impl, OFFSET>, NetworkID::<Impl, OFFSET>, State::<Impl, OFFSET>, NetInterfaces::<Impl, OFFSET>, Cluster::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNetworkNetInterfacesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusNetworkNetInterfaces {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusNetworkNetInterfaces";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetworkNetInterfacesVtbl {
    pub const fn new<Impl: ISClusNetworkNetInterfacesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusNetworkNetInterfacesVtbl {
        unsafe extern "system" fn Count<Impl: ISClusNetworkNetInterfacesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusNetworkNetInterfacesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusNetworkNetInterfacesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusNetworkNetInterfacesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusnetinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclusnetinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusNetworkNetInterfaces>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNetworksImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusNetworks {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusNetworks";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetworksVtbl {
    pub const fn new<Impl: ISClusNetworksImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusNetworksVtbl {
        unsafe extern "system" fn Count<Impl: ISClusNetworksImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusNetworksImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusNetworksImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusNetworksImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclusnetwork)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusNetworks>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNodeImpl: Sized + IDispatchImpl {
    fn CommonProperties();
    fn PrivateProperties();
    fn CommonROProperties();
    fn PrivateROProperties();
    fn Name();
    fn Handle();
    fn NodeID();
    fn State();
    fn Pause();
    fn Resume();
    fn Evict();
    fn ResourceGroups();
    fn Cluster();
    fn NetInterfaces();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusNode {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusNode";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusNodeVtbl {
    pub const fn new<Impl: ISClusNodeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusNodeVtbl {
        unsafe extern "system" fn CommonProperties<Impl: ISClusNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommonProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Impl: ISClusNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Impl: ISClusNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommonROProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Impl: ISClusNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateROProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISClusNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: ISClusNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handle(::core::mem::transmute_copy(&phandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NodeID<Impl: ISClusNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrnodeid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NodeID(::core::mem::transmute_copy(&pbstrnodeid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ISClusNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_NODE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&dwstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: ISClusNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: ISClusNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Evict<Impl: ISClusNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Evict() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceGroups<Impl: ISClusNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresourcegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResourceGroups(::core::mem::transmute_copy(&ppresourcegroups)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Impl: ISClusNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcluster: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cluster(::core::mem::transmute_copy(&ppcluster)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetInterfaces<Impl: ISClusNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclusnetinterfaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NetInterfaces(::core::mem::transmute_copy(&ppclusnetinterfaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISClusNode>,
            base.5,
            CommonProperties::<Impl, OFFSET>,
            PrivateProperties::<Impl, OFFSET>,
            CommonROProperties::<Impl, OFFSET>,
            PrivateROProperties::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            Handle::<Impl, OFFSET>,
            NodeID::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            Pause::<Impl, OFFSET>,
            Resume::<Impl, OFFSET>,
            Evict::<Impl, OFFSET>,
            ResourceGroups::<Impl, OFFSET>,
            Cluster::<Impl, OFFSET>,
            NetInterfaces::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNodeNetInterfacesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusNodeNetInterfaces {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusNodeNetInterfaces";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusNodeNetInterfacesVtbl {
    pub const fn new<Impl: ISClusNodeNetInterfacesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusNodeNetInterfacesVtbl {
        unsafe extern "system" fn Count<Impl: ISClusNodeNetInterfacesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusNodeNetInterfacesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusNodeNetInterfacesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusNodeNetInterfacesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusnetinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclusnetinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusNodeNetInterfaces>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNodesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusNodes {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusNodes";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusNodesVtbl {
    pub const fn new<Impl: ISClusNodesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusNodesVtbl {
        unsafe extern "system" fn Count<Impl: ISClusNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusNodes>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPartitionImpl: Sized + IDispatchImpl {
    fn Flags();
    fn DeviceName();
    fn VolumeLabel();
    fn SerialNumber();
    fn MaximumComponentLength();
    fn FileSystemFlags();
    fn FileSystem();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusPartition {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusPartition";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusPartitionVtbl {
    pub const fn new<Impl: ISClusPartitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusPartitionVtbl {
        unsafe extern "system" fn Flags<Impl: ISClusPartitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flags(::core::mem::transmute_copy(&plflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Impl: ISClusPartitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceName(::core::mem::transmute_copy(&pbstrdevicename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeLabel<Impl: ISClusPartitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrvolumelabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VolumeLabel(::core::mem::transmute_copy(&pbstrvolumelabel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerialNumber<Impl: ISClusPartitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plserialnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SerialNumber(::core::mem::transmute_copy(&plserialnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumComponentLength<Impl: ISClusPartitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmaximumcomponentlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaximumComponentLength(::core::mem::transmute_copy(&plmaximumcomponentlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSystemFlags<Impl: ISClusPartitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plfilesystemflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileSystemFlags(::core::mem::transmute_copy(&plfilesystemflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSystem<Impl: ISClusPartitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfilesystem: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileSystem(::core::mem::transmute_copy(&pbstrfilesystem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusPartition>, base.5, Flags::<Impl, OFFSET>, DeviceName::<Impl, OFFSET>, VolumeLabel::<Impl, OFFSET>, SerialNumber::<Impl, OFFSET>, MaximumComponentLength::<Impl, OFFSET>, FileSystemFlags::<Impl, OFFSET>, FileSystem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPartitionExImpl: Sized + ISClusPartitionImpl + IDispatchImpl {
    fn TotalSize();
    fn FreeSpace();
    fn DeviceNumber();
    fn PartitionNumber();
    fn VolumeGuid();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusPartitionEx {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusPartitionEx";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusPartitionExVtbl {
    pub const fn new<Impl: ISClusPartitionExImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusPartitionExVtbl {
        unsafe extern "system" fn TotalSize<Impl: ISClusPartitionExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltotalsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalSize(::core::mem::transmute_copy(&pltotalsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSpace<Impl: ISClusPartitionExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plfreespace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FreeSpace(::core::mem::transmute_copy(&plfreespace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceNumber<Impl: ISClusPartitionExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldevicenumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceNumber(::core::mem::transmute_copy(&pldevicenumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartitionNumber<Impl: ISClusPartitionExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plpartitionnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PartitionNumber(::core::mem::transmute_copy(&plpartitionnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeGuid<Impl: ISClusPartitionExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrvolumeguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VolumeGuid(::core::mem::transmute_copy(&pbstrvolumeguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusPartitionEx>, base.5, TotalSize::<Impl, OFFSET>, FreeSpace::<Impl, OFFSET>, DeviceNumber::<Impl, OFFSET>, PartitionNumber::<Impl, OFFSET>, VolumeGuid::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPartitionsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusPartitions {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusPartitions";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusPartitionsVtbl {
    pub const fn new<Impl: ISClusPartitionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusPartitionsVtbl {
        unsafe extern "system" fn Count<Impl: ISClusPartitionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusPartitionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusPartitionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pppartition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppartition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusPartitions>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPropertiesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn CreateItem();
    fn UseDefaultValue();
    fn SaveChanges();
    fn ReadOnly();
    fn Private();
    fn Common();
    fn Modified();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusProperties {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusProperties";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertiesVtbl {
    pub const fn new<Impl: ISClusPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusPropertiesVtbl {
        unsafe extern "system" fn Count<Impl: ISClusPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclusproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateItem(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseDefaultValue<Impl: ISClusPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UseDefaultValue(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveChanges<Impl: ISClusPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarstatuscode: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SaveChanges(::core::mem::transmute_copy(&pvarstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Impl: ISClusPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarreadonly: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadOnly(::core::mem::transmute_copy(&pvarreadonly)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Private<Impl: ISClusPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprivate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Private(::core::mem::transmute_copy(&pvarprivate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Common<Impl: ISClusPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarcommon: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Common(::core::mem::transmute_copy(&pvarcommon)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Impl: ISClusPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Modified(::core::mem::transmute_copy(&pvarmodified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusProperties>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>, CreateItem::<Impl, OFFSET>, UseDefaultValue::<Impl, OFFSET>, SaveChanges::<Impl, OFFSET>, ReadOnly::<Impl, OFFSET>, Private::<Impl, OFFSET>, Common::<Impl, OFFSET>, Modified::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPropertyImpl: Sized + IDispatchImpl {
    fn Name();
    fn Length();
    fn ValueCount();
    fn Values();
    fn Value();
    fn SetValue();
    fn Type();
    fn SetType();
    fn Format();
    fn SetFormat();
    fn ReadOnly();
    fn Private();
    fn Common();
    fn Modified();
    fn UseDefaultValue();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusProperty {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusProperty";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertyVtbl {
    pub const fn new<Impl: ISClusPropertyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusPropertyVtbl {
        unsafe extern "system" fn Name<Impl: ISClusPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: ISClusPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Length(::core::mem::transmute_copy(&plength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValueCount<Impl: ISClusPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValueCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Values<Impl: ISClusPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclusterpropertyvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Values(::core::mem::transmute_copy(&ppclusterpropertyvalues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: ISClusPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&pvarvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISClusPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&varvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: ISClusPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut CLUSTER_PROPERTY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: ISClusPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetType(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Format<Impl: ISClusPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformat: *mut CLUSTER_PROPERTY_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Format(::core::mem::transmute_copy(&pformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: ISClusPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: CLUSTER_PROPERTY_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFormat(format) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Impl: ISClusPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarreadonly: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadOnly(::core::mem::transmute_copy(&pvarreadonly)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Private<Impl: ISClusPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarprivate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Private(::core::mem::transmute_copy(&pvarprivate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Common<Impl: ISClusPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarcommon: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Common(::core::mem::transmute_copy(&pvarcommon)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Impl: ISClusPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Modified(::core::mem::transmute_copy(&pvarmodified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseDefaultValue<Impl: ISClusPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UseDefaultValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusProperty>, base.5, Name::<Impl, OFFSET>, Length::<Impl, OFFSET>, ValueCount::<Impl, OFFSET>, Values::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, Type::<Impl, OFFSET>, SetType::<Impl, OFFSET>, Format::<Impl, OFFSET>, SetFormat::<Impl, OFFSET>, ReadOnly::<Impl, OFFSET>, Private::<Impl, OFFSET>, Common::<Impl, OFFSET>, Modified::<Impl, OFFSET>, UseDefaultValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPropertyValueImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
    fn Type();
    fn SetType();
    fn Format();
    fn SetFormat();
    fn Length();
    fn DataCount();
    fn Data();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusPropertyValue {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusPropertyValue";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertyValueVtbl {
    pub const fn new<Impl: ISClusPropertyValueImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusPropertyValueVtbl {
        unsafe extern "system" fn Value<Impl: ISClusPropertyValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&pvarvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISClusPropertyValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&varvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: ISClusPropertyValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut CLUSTER_PROPERTY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: ISClusPropertyValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetType(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Format<Impl: ISClusPropertyValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformat: *mut CLUSTER_PROPERTY_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Format(::core::mem::transmute_copy(&pformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: ISClusPropertyValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: CLUSTER_PROPERTY_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFormat(format) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: ISClusPropertyValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Length(::core::mem::transmute_copy(&plength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataCount<Impl: ISClusPropertyValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: ISClusPropertyValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclusterpropertyvaluedata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Data(::core::mem::transmute_copy(&ppclusterpropertyvaluedata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusPropertyValue>, base.5, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, Type::<Impl, OFFSET>, SetType::<Impl, OFFSET>, Format::<Impl, OFFSET>, SetFormat::<Impl, OFFSET>, Length::<Impl, OFFSET>, DataCount::<Impl, OFFSET>, Data::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPropertyValueDataImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn CreateItem();
    fn RemoveItem();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusPropertyValueData {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusPropertyValueData";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertyValueDataVtbl {
    pub const fn new<Impl: ISClusPropertyValueDataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusPropertyValueDataVtbl {
        unsafe extern "system" fn Count<Impl: ISClusPropertyValueDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusPropertyValueDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusPropertyValueDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusPropertyValueDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvardata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateItem(&*(&varvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvardata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItem<Impl: ISClusPropertyValueDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveItem(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusPropertyValueData>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, CreateItem::<Impl, OFFSET>, RemoveItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPropertyValuesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn CreateItem();
    fn RemoveItem();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusPropertyValues {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusPropertyValues";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertyValuesVtbl {
    pub const fn new<Impl: ISClusPropertyValuesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusPropertyValuesVtbl {
        unsafe extern "system" fn Count<Impl: ISClusPropertyValuesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusPropertyValuesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusPropertyValuesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pppropertyvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppropertyvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusPropertyValuesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pppropertyvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateItem(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppropertyvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItem<Impl: ISClusPropertyValuesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveItem(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusPropertyValues>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, CreateItem::<Impl, OFFSET>, RemoveItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusRefObjectImpl: Sized + IDispatchImpl {
    fn Handle();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusRefObject {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusRefObject";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusRefObjectVtbl {
    pub const fn new<Impl: ISClusRefObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusRefObjectVtbl {
        unsafe extern "system" fn Handle<Impl: ISClusRefObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handle(::core::mem::transmute_copy(&phandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusRefObject>, base.5, Handle::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusRegistryKeysImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn AddItem();
    fn RemoveItem();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusRegistryKeys {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusRegistryKeys";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusRegistryKeysVtbl {
    pub const fn new<Impl: ISClusRegistryKeysImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusRegistryKeysVtbl {
        unsafe extern "system" fn Count<Impl: ISClusRegistryKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusRegistryKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusRegistryKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusRegistryKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrregistrykey: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrregistrykey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Impl: ISClusRegistryKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrregistrykey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddItem(&*(&bstrregistrykey as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItem<Impl: ISClusRegistryKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveItem(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusRegistryKeys>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>, AddItem::<Impl, OFFSET>, RemoveItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResDependenciesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn CreateItem();
    fn DeleteItem();
    fn AddItem();
    fn RemoveItem();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusResDependencies {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusResDependencies";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResDependenciesVtbl {
    pub const fn new<Impl: ISClusResDependenciesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusResDependenciesVtbl {
        unsafe extern "system" fn Count<Impl: ISClusResDependenciesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResDependenciesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResDependenciesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusResDependenciesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclusresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusResDependenciesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresourcetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateItem(&*(&bstrresourcename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrresourcetype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&ppclusterresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: ISClusResDependenciesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteItem(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Impl: ISClusResDependenciesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddItem(&*(&presource as *const <ISClusResource as ::windows::core::Abi>::Abi as *const <ISClusResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItem<Impl: ISClusResDependenciesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveItem(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusResDependencies>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>, CreateItem::<Impl, OFFSET>, DeleteItem::<Impl, OFFSET>, AddItem::<Impl, OFFSET>, RemoveItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResDependentsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn CreateItem();
    fn DeleteItem();
    fn AddItem();
    fn RemoveItem();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusResDependents {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusResDependents";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResDependentsVtbl {
    pub const fn new<Impl: ISClusResDependentsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusResDependentsVtbl {
        unsafe extern "system" fn Count<Impl: ISClusResDependentsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResDependentsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResDependentsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusResDependentsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclusresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusResDependentsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresourcetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateItem(&*(&bstrresourcename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrresourcetype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&ppclusterresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: ISClusResDependentsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteItem(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Impl: ISClusResDependentsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddItem(&*(&presource as *const <ISClusResource as ::windows::core::Abi>::Abi as *const <ISClusResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItem<Impl: ISClusResDependentsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveItem(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusResDependents>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>, CreateItem::<Impl, OFFSET>, DeleteItem::<Impl, OFFSET>, AddItem::<Impl, OFFSET>, RemoveItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResGroupImpl: Sized + IDispatchImpl {
    fn CommonProperties();
    fn PrivateProperties();
    fn CommonROProperties();
    fn PrivateROProperties();
    fn Handle();
    fn Name();
    fn SetName();
    fn State();
    fn OwnerNode();
    fn Resources();
    fn PreferredOwnerNodes();
    fn Delete();
    fn Online();
    fn Move();
    fn Offline();
    fn Cluster();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusResGroup {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusResGroup";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroupVtbl {
    pub const fn new<Impl: ISClusResGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusResGroupVtbl {
        unsafe extern "system" fn CommonProperties<Impl: ISClusResGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommonProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Impl: ISClusResGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Impl: ISClusResGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommonROProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Impl: ISClusResGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateROProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: ISClusResGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handle(::core::mem::transmute_copy(&phandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISClusResGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ISClusResGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ISClusResGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_GROUP_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&dwstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OwnerNode<Impl: ISClusResGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppownernode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OwnerNode(::core::mem::transmute_copy(&ppownernode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Impl: ISClusResGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclustergroupresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resources(::core::mem::transmute_copy(&ppclustergroupresources)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredOwnerNodes<Impl: ISClusResGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppownernodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreferredOwnerNodes(::core::mem::transmute_copy(&ppownernodes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: ISClusResGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Online<Impl: ISClusResGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vartimeout: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varnode: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Online(&*(&vartimeout as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varnode as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarpending)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: ISClusResGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vartimeout: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varnode: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Move(&*(&vartimeout as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varnode as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarpending)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Offline<Impl: ISClusResGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vartimeout: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Offline(&*(&vartimeout as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarpending)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Impl: ISClusResGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcluster: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cluster(::core::mem::transmute_copy(&ppcluster)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISClusResGroup>,
            base.5,
            CommonProperties::<Impl, OFFSET>,
            PrivateProperties::<Impl, OFFSET>,
            CommonROProperties::<Impl, OFFSET>,
            PrivateROProperties::<Impl, OFFSET>,
            Handle::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            OwnerNode::<Impl, OFFSET>,
            Resources::<Impl, OFFSET>,
            PreferredOwnerNodes::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            Online::<Impl, OFFSET>,
            Move::<Impl, OFFSET>,
            Offline::<Impl, OFFSET>,
            Cluster::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResGroupPreferredOwnerNodesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn InsertItem();
    fn RemoveItem();
    fn Modified();
    fn SaveChanges();
    fn AddItem();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusResGroupPreferredOwnerNodes {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusResGroupPreferredOwnerNodes";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroupPreferredOwnerNodesVtbl {
    pub const fn new<Impl: ISClusResGroupPreferredOwnerNodesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusResGroupPreferredOwnerNodesVtbl {
        unsafe extern "system" fn Count<Impl: ISClusResGroupPreferredOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResGroupPreferredOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResGroupPreferredOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusResGroupPreferredOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertItem<Impl: ISClusResGroupPreferredOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, nposition: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertItem(&*(&pnode as *const <ISClusNode as ::windows::core::Abi>::Abi as *const <ISClusNode as ::windows::core::DefaultType>::DefaultType), nposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItem<Impl: ISClusResGroupPreferredOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveItem(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Impl: ISClusResGroupPreferredOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Modified(::core::mem::transmute_copy(&pvarmodified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveChanges<Impl: ISClusResGroupPreferredOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SaveChanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Impl: ISClusResGroupPreferredOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddItem(&*(&pnode as *const <ISClusNode as ::windows::core::Abi>::Abi as *const <ISClusNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusResGroupPreferredOwnerNodes>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>, InsertItem::<Impl, OFFSET>, RemoveItem::<Impl, OFFSET>, Modified::<Impl, OFFSET>, SaveChanges::<Impl, OFFSET>, AddItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResGroupResourcesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn CreateItem();
    fn DeleteItem();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusResGroupResources {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusResGroupResources";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroupResourcesVtbl {
    pub const fn new<Impl: ISClusResGroupResourcesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusResGroupResourcesVtbl {
        unsafe extern "system" fn Count<Impl: ISClusResGroupResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResGroupResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResGroupResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusResGroupResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclusresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusResGroupResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresourcetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateItem(&*(&bstrresourcename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrresourcetype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&ppclusterresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: ISClusResGroupResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteItem(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusResGroupResources>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>, CreateItem::<Impl, OFFSET>, DeleteItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResGroupsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn CreateItem();
    fn DeleteItem();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusResGroups {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusResGroups";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroupsVtbl {
    pub const fn new<Impl: ISClusResGroupsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusResGroupsVtbl {
        unsafe extern "system" fn Count<Impl: ISClusResGroupsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResGroupsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResGroupsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusResGroupsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclusresgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusResGroupsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourcegroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppresourcegroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateItem(&*(&bstrresourcegroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppresourcegroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: ISClusResGroupsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteItem(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusResGroups>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>, CreateItem::<Impl, OFFSET>, DeleteItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResPossibleOwnerNodesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn AddItem();
    fn RemoveItem();
    fn Modified();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusResPossibleOwnerNodes {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusResPossibleOwnerNodes";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResPossibleOwnerNodesVtbl {
    pub const fn new<Impl: ISClusResPossibleOwnerNodesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusResPossibleOwnerNodesVtbl {
        unsafe extern "system" fn Count<Impl: ISClusResPossibleOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResPossibleOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResPossibleOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusResPossibleOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Impl: ISClusResPossibleOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddItem(&*(&pnode as *const <ISClusNode as ::windows::core::Abi>::Abi as *const <ISClusNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItem<Impl: ISClusResPossibleOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveItem(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Impl: ISClusResPossibleOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Modified(::core::mem::transmute_copy(&pvarmodified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusResPossibleOwnerNodes>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>, AddItem::<Impl, OFFSET>, RemoveItem::<Impl, OFFSET>, Modified::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResTypeImpl: Sized + IDispatchImpl {
    fn CommonProperties();
    fn PrivateProperties();
    fn CommonROProperties();
    fn PrivateROProperties();
    fn Name();
    fn Delete();
    fn Cluster();
    fn Resources();
    fn PossibleOwnerNodes();
    fn AvailableDisks();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusResType {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusResType";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResTypeVtbl {
    pub const fn new<Impl: ISClusResTypeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusResTypeVtbl {
        unsafe extern "system" fn CommonProperties<Impl: ISClusResTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommonProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Impl: ISClusResTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Impl: ISClusResTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommonROProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Impl: ISClusResTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateROProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISClusResTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: ISClusResTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Impl: ISClusResTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcluster: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cluster(::core::mem::transmute_copy(&ppcluster)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Impl: ISClusResTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclusterrestyperesources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resources(::core::mem::transmute_copy(&ppclusterrestyperesources)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PossibleOwnerNodes<Impl: ISClusResTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppownernodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PossibleOwnerNodes(::core::mem::transmute_copy(&ppownernodes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableDisks<Impl: ISClusResTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppavailabledisks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AvailableDisks(::core::mem::transmute_copy(&ppavailabledisks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusResType>, base.5, CommonProperties::<Impl, OFFSET>, PrivateProperties::<Impl, OFFSET>, CommonROProperties::<Impl, OFFSET>, PrivateROProperties::<Impl, OFFSET>, Name::<Impl, OFFSET>, Delete::<Impl, OFFSET>, Cluster::<Impl, OFFSET>, Resources::<Impl, OFFSET>, PossibleOwnerNodes::<Impl, OFFSET>, AvailableDisks::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResTypePossibleOwnerNodesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusResTypePossibleOwnerNodes {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusResTypePossibleOwnerNodes";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResTypePossibleOwnerNodesVtbl {
    pub const fn new<Impl: ISClusResTypePossibleOwnerNodesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusResTypePossibleOwnerNodesVtbl {
        unsafe extern "system" fn Count<Impl: ISClusResTypePossibleOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResTypePossibleOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResTypePossibleOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusResTypePossibleOwnerNodesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusResTypePossibleOwnerNodes>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResTypeResourcesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn CreateItem();
    fn DeleteItem();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusResTypeResources {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusResTypeResources";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResTypeResourcesVtbl {
    pub const fn new<Impl: ISClusResTypeResourcesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusResTypeResourcesVtbl {
        unsafe extern "system" fn Count<Impl: ISClusResTypeResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResTypeResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResTypeResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusResTypeResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclusresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusResTypeResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateItem(&*(&bstrresourcename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&ppclusterresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: ISClusResTypeResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteItem(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusResTypeResources>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>, CreateItem::<Impl, OFFSET>, DeleteItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResTypesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn CreateItem();
    fn DeleteItem();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusResTypes {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusResTypes";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResTypesVtbl {
    pub const fn new<Impl: ISClusResTypesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusResTypesVtbl {
        unsafe extern "system" fn Count<Impl: ISClusResTypesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResTypesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResTypesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusResTypesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusrestype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclusrestype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusResTypesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourcetypename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdisplayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresourcetypedll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwlooksalivepollinterval: i32, dwisalivepollinterval: i32, ppresourcetype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateItem(
                &*(&bstrresourcetypename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdisplayname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrresourcetypedll as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                dwlooksalivepollinterval,
                dwisalivepollinterval,
                ::core::mem::transmute_copy(&ppresourcetype),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: ISClusResTypesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteItem(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusResTypes>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>, CreateItem::<Impl, OFFSET>, DeleteItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResourceImpl: Sized + IDispatchImpl {
    fn CommonProperties();
    fn PrivateProperties();
    fn CommonROProperties();
    fn PrivateROProperties();
    fn Handle();
    fn Name();
    fn SetName();
    fn State();
    fn CoreFlag();
    fn BecomeQuorumResource();
    fn Delete();
    fn Fail();
    fn Online();
    fn Offline();
    fn ChangeResourceGroup();
    fn AddResourceNode();
    fn RemoveResourceNode();
    fn CanResourceBeDependent();
    fn PossibleOwnerNodes();
    fn Dependencies();
    fn Dependents();
    fn Group();
    fn OwnerNode();
    fn Cluster();
    fn ClassInfo();
    fn Disk();
    fn RegistryKeys();
    fn CryptoKeys();
    fn TypeName();
    fn Type();
    fn MaintenanceMode();
    fn SetMaintenanceMode();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusResource {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusResource";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResourceVtbl {
    pub const fn new<Impl: ISClusResourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusResourceVtbl {
        unsafe extern "system" fn CommonProperties<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommonProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommonROProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateROProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handle(::core::mem::transmute_copy(&phandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&bstrresourcename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_RESOURCE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&dwstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoreFlag<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcoreflag: *mut CLUS_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CoreFlag(::core::mem::transmute_copy(&dwcoreflag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BecomeQuorumResource<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdevicepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmaxlogsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BecomeQuorumResource(&*(&bstrdevicepath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lmaxlogsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Fail<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Fail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Online<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ntimeout: i32, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Online(ntimeout, ::core::mem::transmute_copy(&pvarpending)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Offline<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ntimeout: i32, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Offline(ntimeout, ::core::mem::transmute_copy(&pvarpending)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeResourceGroup<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presourcegroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChangeResourceGroup(&*(&presourcegroup as *const <ISClusResGroup as ::windows::core::Abi>::Abi as *const <ISClusResGroup as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddResourceNode<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddResourceNode(&*(&pnode as *const <ISClusNode as ::windows::core::Abi>::Abi as *const <ISClusNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResourceNode<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveResourceNode(&*(&pnode as *const <ISClusNode as ::windows::core::Abi>::Abi as *const <ISClusNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanResourceBeDependent<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pvardependent: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanResourceBeDependent(&*(&presource as *const <ISClusResource as ::windows::core::Abi>::Abi as *const <ISClusResource as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvardependent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PossibleOwnerNodes<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppownernodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PossibleOwnerNodes(::core::mem::transmute_copy(&ppownernodes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dependencies<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresdependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Dependencies(::core::mem::transmute_copy(&ppresdependencies)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dependents<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresdependents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Dependents(::core::mem::transmute_copy(&ppresdependents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Group(::core::mem::transmute_copy(&ppresgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OwnerNode<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppownernode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OwnerNode(::core::mem::transmute_copy(&ppownernode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcluster: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cluster(::core::mem::transmute_copy(&ppcluster)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClassInfo<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prcclassinfo: *mut CLUSTER_RESOURCE_CLASS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClassInfo(::core::mem::transmute_copy(&prcclassinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disk<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdisk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disk(::core::mem::transmute_copy(&ppdisk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegistryKeys<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppregistrykeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegistryKeys(::core::mem::transmute_copy(&ppregistrykeys)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CryptoKeys<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcryptokeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CryptoKeys(::core::mem::transmute_copy(&ppcryptokeys)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TypeName<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtypename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TypeName(::core::mem::transmute_copy(&pbstrtypename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresourcetype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&ppresourcetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaintenanceMode<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbmaintenancemode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaintenanceMode(::core::mem::transmute_copy(&pbmaintenancemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaintenanceMode<Impl: ISClusResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmaintenancemode: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMaintenanceMode(&*(&bmaintenancemode as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISClusResource>,
            base.5,
            CommonProperties::<Impl, OFFSET>,
            PrivateProperties::<Impl, OFFSET>,
            CommonROProperties::<Impl, OFFSET>,
            PrivateROProperties::<Impl, OFFSET>,
            Handle::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            CoreFlag::<Impl, OFFSET>,
            BecomeQuorumResource::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            Fail::<Impl, OFFSET>,
            Online::<Impl, OFFSET>,
            Offline::<Impl, OFFSET>,
            ChangeResourceGroup::<Impl, OFFSET>,
            AddResourceNode::<Impl, OFFSET>,
            RemoveResourceNode::<Impl, OFFSET>,
            CanResourceBeDependent::<Impl, OFFSET>,
            PossibleOwnerNodes::<Impl, OFFSET>,
            Dependencies::<Impl, OFFSET>,
            Dependents::<Impl, OFFSET>,
            Group::<Impl, OFFSET>,
            OwnerNode::<Impl, OFFSET>,
            Cluster::<Impl, OFFSET>,
            ClassInfo::<Impl, OFFSET>,
            Disk::<Impl, OFFSET>,
            RegistryKeys::<Impl, OFFSET>,
            CryptoKeys::<Impl, OFFSET>,
            TypeName::<Impl, OFFSET>,
            Type::<Impl, OFFSET>,
            MaintenanceMode::<Impl, OFFSET>,
            SetMaintenanceMode::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResourcesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn CreateItem();
    fn DeleteItem();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusResources {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusResources";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResourcesVtbl {
    pub const fn new<Impl: ISClusResourcesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusResourcesVtbl {
        unsafe extern "system" fn Count<Impl: ISClusResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclusresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclusresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Impl: ISClusResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresourcetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateItem(
                &*(&bstrresourcename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrresourcetype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                dwflags,
                ::core::mem::transmute_copy(&ppclusterresource),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: ISClusResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteItem(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusResources>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>, CreateItem::<Impl, OFFSET>, DeleteItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusScsiAddressImpl: Sized + IDispatchImpl {
    fn PortNumber();
    fn PathId();
    fn TargetId();
    fn Lun();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusScsiAddress {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusScsiAddress";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusScsiAddressVtbl {
    pub const fn new<Impl: ISClusScsiAddressImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusScsiAddressVtbl {
        unsafe extern "system" fn PortNumber<Impl: ISClusScsiAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarportnumber: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PortNumber(::core::mem::transmute_copy(&pvarportnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathId<Impl: ISClusScsiAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarpathid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PathId(::core::mem::transmute_copy(&pvarpathid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetId<Impl: ISClusScsiAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvartargetid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetId(::core::mem::transmute_copy(&pvartargetid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lun<Impl: ISClusScsiAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarlun: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Lun(::core::mem::transmute_copy(&pvarlun)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusScsiAddress>, base.5, PortNumber::<Impl, OFFSET>, PathId::<Impl, OFFSET>, TargetId::<Impl, OFFSET>, Lun::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusVersionImpl: Sized + IDispatchImpl {
    fn Name();
    fn MajorVersion();
    fn MinorVersion();
    fn BuildNumber();
    fn VendorId();
    fn CSDVersion();
    fn ClusterHighestVersion();
    fn ClusterLowestVersion();
    fn Flags();
    fn MixedVersion();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusVersion {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusVersion";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusVersionVtbl {
    pub const fn new<Impl: ISClusVersionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusVersionVtbl {
        unsafe extern "system" fn Name<Impl: ISClusVersionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrclustername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrclustername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Impl: ISClusVersionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MajorVersion(::core::mem::transmute_copy(&pnmajorversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Impl: ISClusVersionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinorVersion(::core::mem::transmute_copy(&pnminorversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildNumber<Impl: ISClusVersionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnbuildnumber: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BuildNumber(::core::mem::transmute_copy(&pnbuildnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VendorId<Impl: ISClusVersionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrvendorid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VendorId(::core::mem::transmute_copy(&pbstrvendorid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSDVersion<Impl: ISClusVersionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcsdversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CSDVersion(::core::mem::transmute_copy(&pbstrcsdversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClusterHighestVersion<Impl: ISClusVersionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnclusterhighestversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClusterHighestVersion(::core::mem::transmute_copy(&pnclusterhighestversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClusterLowestVersion<Impl: ISClusVersionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnclusterlowestversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClusterLowestVersion(::core::mem::transmute_copy(&pnclusterlowestversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Impl: ISClusVersionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flags(::core::mem::transmute_copy(&pnflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MixedVersion<Impl: ISClusVersionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmixedversion: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MixedVersion(::core::mem::transmute_copy(&pvarmixedversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusVersion>, base.5, Name::<Impl, OFFSET>, MajorVersion::<Impl, OFFSET>, MinorVersion::<Impl, OFFSET>, BuildNumber::<Impl, OFFSET>, VendorId::<Impl, OFFSET>, CSDVersion::<Impl, OFFSET>, ClusterHighestVersion::<Impl, OFFSET>, ClusterLowestVersion::<Impl, OFFSET>, Flags::<Impl, OFFSET>, MixedVersion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusterImpl: Sized + IDispatchImpl {
    fn CommonProperties();
    fn PrivateProperties();
    fn CommonROProperties();
    fn PrivateROProperties();
    fn Handle();
    fn Open();
    fn Name();
    fn SetName();
    fn Version();
    fn SetQuorumResource();
    fn QuorumResource();
    fn QuorumLogSize();
    fn SetQuorumLogSize();
    fn QuorumPath();
    fn SetQuorumPath();
    fn Nodes();
    fn ResourceGroups();
    fn Resources();
    fn ResourceTypes();
    fn Networks();
    fn NetInterfaces();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISCluster {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISCluster";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusterVtbl {
    pub const fn new<Impl: ISClusterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusterVtbl {
        unsafe extern "system" fn CommonProperties<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommonProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommonROProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateROProperties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handle(::core::mem::transmute_copy(&phandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrclustername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open(&*(&bstrclustername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrclustername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&bstrclustername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclusversion: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Version(::core::mem::transmute_copy(&ppclusversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuorumResource<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclusterresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQuorumResource(&*(&pclusterresource as *const <ISClusResource as ::windows::core::Abi>::Abi as *const <ISClusResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuorumResource<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclusterresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QuorumResource(::core::mem::transmute_copy(&pclusterresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuorumLogSize<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnlogsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QuorumLogSize(::core::mem::transmute_copy(&pnlogsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuorumLogSize<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nlogsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQuorumLogSize(nlogsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuorumPath<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QuorumPath(::core::mem::transmute_copy(&pppath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuorumPath<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQuorumPath(&*(&ppath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Nodes<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Nodes(::core::mem::transmute_copy(&ppnodes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceGroups<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclusterresourcegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResourceGroups(::core::mem::transmute_copy(&ppclusterresourcegroups)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclusterresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resources(::core::mem::transmute_copy(&ppclusterresources)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceTypes<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresourcetypes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResourceTypes(::core::mem::transmute_copy(&ppresourcetypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Networks<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnetworks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Networks(::core::mem::transmute_copy(&ppnetworks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetInterfaces<Impl: ISClusterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnetinterfaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NetInterfaces(::core::mem::transmute_copy(&ppnetinterfaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISCluster>,
            base.5,
            CommonProperties::<Impl, OFFSET>,
            PrivateProperties::<Impl, OFFSET>,
            CommonROProperties::<Impl, OFFSET>,
            PrivateROProperties::<Impl, OFFSET>,
            Handle::<Impl, OFFSET>,
            Open::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            Version::<Impl, OFFSET>,
            SetQuorumResource::<Impl, OFFSET>,
            QuorumResource::<Impl, OFFSET>,
            QuorumLogSize::<Impl, OFFSET>,
            SetQuorumLogSize::<Impl, OFFSET>,
            QuorumPath::<Impl, OFFSET>,
            SetQuorumPath::<Impl, OFFSET>,
            Nodes::<Impl, OFFSET>,
            ResourceGroups::<Impl, OFFSET>,
            Resources::<Impl, OFFSET>,
            ResourceTypes::<Impl, OFFSET>,
            Networks::<Impl, OFFSET>,
            NetInterfaces::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusterNamesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
    fn DomainName();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISClusterNames {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISClusterNames";
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusterNamesVtbl {
    pub const fn new<Impl: ISClusterNamesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISClusterNamesVtbl {
        unsafe extern "system" fn Count<Impl: ISClusterNamesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISClusterNamesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISClusterNamesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISClusterNamesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrclustername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrclustername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainName<Impl: ISClusterNamesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdomainname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DomainName(::core::mem::transmute_copy(&pbstrdomainname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISClusterNames>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>, DomainName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISDomainNamesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Refresh();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISDomainNames {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.ISDomainNames";
}
#[cfg(feature = "Win32_System_Com")]
impl ISDomainNamesVtbl {
    pub const fn new<Impl: ISDomainNamesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISDomainNamesVtbl {
        unsafe extern "system" fn Count<Impl: ISDomainNamesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISDomainNamesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISDomainNamesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISDomainNamesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrdomainname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&varindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrdomainname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISDomainNames>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
pub trait IWCContextMenuCallbackImpl: Sized {
    fn AddExtensionMenuItem();
}
impl ::windows::core::RuntimeName for IWCContextMenuCallback {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IWCContextMenuCallback";
}
impl IWCContextMenuCallbackVtbl {
    pub const fn new<Impl: IWCContextMenuCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWCContextMenuCallbackVtbl {
        unsafe extern "system" fn AddExtensionMenuItem<Impl: IWCContextMenuCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpszstatusbartext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ncommandid: u32, nsubmenucommandid: u32, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddExtensionMenuItem(&*(&lpszname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&lpszstatusbartext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ncommandid, nsubmenucommandid, uflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWCContextMenuCallback>, base.5, AddExtensionMenuItem::<Impl, OFFSET>)
    }
}
pub trait IWCPropertySheetCallbackImpl: Sized {
    fn AddPropertySheetPage();
}
impl ::windows::core::RuntimeName for IWCPropertySheetCallback {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IWCPropertySheetCallback";
}
impl IWCPropertySheetCallbackVtbl {
    pub const fn new<Impl: IWCPropertySheetCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWCPropertySheetCallbackVtbl {
        unsafe extern "system" fn AddPropertySheetPage<Impl: IWCPropertySheetCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hpage: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddPropertySheetPage(hpage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWCPropertySheetCallback>, base.5, AddPropertySheetPage::<Impl, OFFSET>)
    }
}
pub trait IWCWizard97CallbackImpl: Sized {
    fn AddWizard97Page();
    fn EnableNext();
}
impl ::windows::core::RuntimeName for IWCWizard97Callback {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IWCWizard97Callback";
}
impl IWCWizard97CallbackVtbl {
    pub const fn new<Impl: IWCWizard97CallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWCWizard97CallbackVtbl {
        unsafe extern "system" fn AddWizard97Page<Impl: IWCWizard97CallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hpage: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddWizard97Page(hpage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNext<Impl: IWCWizard97CallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hpage: *const i32, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableNext(hpage, &*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWCWizard97Callback>, base.5, AddWizard97Page::<Impl, OFFSET>, EnableNext::<Impl, OFFSET>)
    }
}
pub trait IWCWizardCallbackImpl: Sized {
    fn AddWizardPage();
    fn EnableNext();
}
impl ::windows::core::RuntimeName for IWCWizardCallback {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IWCWizardCallback";
}
impl IWCWizardCallbackVtbl {
    pub const fn new<Impl: IWCWizardCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWCWizardCallbackVtbl {
        unsafe extern "system" fn AddWizardPage<Impl: IWCWizardCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hpage: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddWizardPage(hpage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNext<Impl: IWCWizardCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hpage: *const i32, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableNext(hpage, &*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWCWizardCallback>, base.5, AddWizardPage::<Impl, OFFSET>, EnableNext::<Impl, OFFSET>)
    }
}
pub trait IWEExtendContextMenuImpl: Sized {
    fn AddContextMenuItems();
}
impl ::windows::core::RuntimeName for IWEExtendContextMenu {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IWEExtendContextMenu";
}
impl IWEExtendContextMenuVtbl {
    pub const fn new<Impl: IWEExtendContextMenuImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWEExtendContextMenuVtbl {
        unsafe extern "system" fn AddContextMenuItems<Impl: IWEExtendContextMenuImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddContextMenuItems(&*(&pidata as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&picallback as *const <IWCContextMenuCallback as ::windows::core::Abi>::Abi as *const <IWCContextMenuCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWEExtendContextMenu>, base.5, AddContextMenuItems::<Impl, OFFSET>)
    }
}
pub trait IWEExtendPropertySheetImpl: Sized {
    fn CreatePropertySheetPages();
}
impl ::windows::core::RuntimeName for IWEExtendPropertySheet {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IWEExtendPropertySheet";
}
impl IWEExtendPropertySheetVtbl {
    pub const fn new<Impl: IWEExtendPropertySheetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWEExtendPropertySheetVtbl {
        unsafe extern "system" fn CreatePropertySheetPages<Impl: IWEExtendPropertySheetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePropertySheetPages(&*(&pidata as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&picallback as *const <IWCPropertySheetCallback as ::windows::core::Abi>::Abi as *const <IWCPropertySheetCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWEExtendPropertySheet>, base.5, CreatePropertySheetPages::<Impl, OFFSET>)
    }
}
pub trait IWEExtendWizardImpl: Sized {
    fn CreateWizardPages();
}
impl ::windows::core::RuntimeName for IWEExtendWizard {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IWEExtendWizard";
}
impl IWEExtendWizardVtbl {
    pub const fn new<Impl: IWEExtendWizardImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWEExtendWizardVtbl {
        unsafe extern "system" fn CreateWizardPages<Impl: IWEExtendWizardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWizardPages(&*(&pidata as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&picallback as *const <IWCWizardCallback as ::windows::core::Abi>::Abi as *const <IWCWizardCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWEExtendWizard>, base.5, CreateWizardPages::<Impl, OFFSET>)
    }
}
pub trait IWEExtendWizard97Impl: Sized {
    fn CreateWizard97Pages();
}
impl ::windows::core::RuntimeName for IWEExtendWizard97 {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IWEExtendWizard97";
}
impl IWEExtendWizard97Vtbl {
    pub const fn new<Impl: IWEExtendWizard97Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWEExtendWizard97Vtbl {
        unsafe extern "system" fn CreateWizard97Pages<Impl: IWEExtendWizard97Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWizard97Pages(&*(&pidata as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&picallback as *const <IWCWizard97Callback as ::windows::core::Abi>::Abi as *const <IWCWizard97Callback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWEExtendWizard97>, base.5, CreateWizard97Pages::<Impl, OFFSET>)
    }
}
pub trait IWEInvokeCommandImpl: Sized {
    fn InvokeCommand();
}
impl ::windows::core::RuntimeName for IWEInvokeCommand {
    const NAME: &'static str = "Windows.Win32.Networking.Clustering.IWEInvokeCommand";
}
impl IWEInvokeCommandVtbl {
    pub const fn new<Impl: IWEInvokeCommandImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWEInvokeCommandVtbl {
        unsafe extern "system" fn InvokeCommand<Impl: IWEInvokeCommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncommandid: u32, pidata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InvokeCommand(ncommandid, &*(&pidata as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWEInvokeCommand>, base.5, InvokeCommand::<Impl, OFFSET>)
    }
}
