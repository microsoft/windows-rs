#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const Catalog: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22881_8a19_11d0_81b6_00a0c9231c29);
pub const CatalogCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22883_8a19_11d0_81b6_00a0c9231c29);
pub const CatalogObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22882_8a19_11d0_81b6_00a0c9231c29);
pub const ComponentUtil: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22884_8a19_11d0_81b6_00a0c9231c29);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ICatalog(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ICatalog {
    #[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetCollection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcollname: Param0) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCollection)(::core::mem::transmute_copy(self), bstrcollname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrconnectstring: Param0) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Connect)(::core::mem::transmute_copy(self), bstrconnectstring.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
    pub unsafe fn MajorVersion(&self, retval: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MajorVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(retval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
    pub unsafe fn MinorVersion(&self, retval: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MinorVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(retval)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICatalog> for ::windows::core::IUnknown {
    fn from(value: ICatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICatalog> for ::windows::core::IUnknown {
    fn from(value: &ICatalog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICatalog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICatalog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICatalog> for super::Com::IDispatch {
    fn from(value: ICatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICatalog> for super::Com::IDispatch {
    fn from(value: &ICatalog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ICatalog {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a ICatalog {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ICatalog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICatalog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICatalog {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICatalog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatalog").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ICatalog {
    type Vtable = ICatalog_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22870_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICatalog_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetCollection: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrconnectstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Connect: usize,
    pub MajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IComponentUtil(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IComponentUtil {
    #[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallComponent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdllfile: Param0, bstrtypelibfile: Param1, bstrproxystubdllfile: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InstallComponent)(::core::mem::transmute_copy(self), bstrdllfile.into_param().abi(), bstrtypelibfile.into_param().abi(), bstrproxystubdllfile.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportComponent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrclsid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ImportComponent)(::core::mem::transmute_copy(self), bstrclsid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportComponentByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprogid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ImportComponentByName)(::core::mem::transmute_copy(self), bstrprogid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetCLSIDs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdllfile: Param0, bstrtypelibfile: Param1, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCLSIDs)(::core::mem::transmute_copy(self), bstrdllfile.into_param().abi(), bstrtypelibfile.into_param().abi(), ::core::mem::transmute(aclsids)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IComponentUtil> for ::windows::core::IUnknown {
    fn from(value: IComponentUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IComponentUtil> for ::windows::core::IUnknown {
    fn from(value: &IComponentUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IComponentUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IComponentUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IComponentUtil> for super::Com::IDispatch {
    fn from(value: IComponentUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IComponentUtil> for super::Com::IDispatch {
    fn from(value: &IComponentUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IComponentUtil {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IComponentUtil {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IComponentUtil {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IComponentUtil {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IComponentUtil {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IComponentUtil {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComponentUtil").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IComponentUtil {
    type Vtable = IComponentUtil_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22873_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IComponentUtil_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdllfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtypelibfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrproxystubdllfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallComponent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImportComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImportComponent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImportComponentByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImportComponentByName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetCLSIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdllfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtypelibfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetCLSIDs: usize,
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPackageUtil(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPackageUtil {
    #[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallPackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpackagefile: Param0, bstrinstallpath: Param1, loptions: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InstallPackage)(::core::mem::transmute_copy(self), bstrpackagefile.into_param().abi(), bstrinstallpath.into_param().abi(), ::core::mem::transmute(loptions)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExportPackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpackageid: Param0, bstrpackagefile: Param1, loptions: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExportPackage)(::core::mem::transmute_copy(self), bstrpackageid.into_param().abi(), bstrpackagefile.into_param().abi(), ::core::mem::transmute(loptions)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShutdownPackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpackageid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShutdownPackage)(::core::mem::transmute_copy(self), bstrpackageid.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPackageUtil> for ::windows::core::IUnknown {
    fn from(value: IPackageUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPackageUtil> for ::windows::core::IUnknown {
    fn from(value: &IPackageUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPackageUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPackageUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPackageUtil> for super::Com::IDispatch {
    fn from(value: IPackageUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPackageUtil> for super::Com::IDispatch {
    fn from(value: &IPackageUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IPackageUtil {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IPackageUtil {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPackageUtil {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPackageUtil {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPackageUtil {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPackageUtil {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPackageUtil").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPackageUtil {
    type Vtable = IPackageUtil_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22874_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUtil_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpackagefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinstallpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallPackage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExportPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackagefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExportPackage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShutdownPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShutdownPackage: usize,
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRemoteComponentUtil(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRemoteComponentUtil {
    #[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallRemoteComponent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrserver: Param0, bstrpackageid: Param1, bstrclsid: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InstallRemoteComponent)(::core::mem::transmute_copy(self), bstrserver.into_param().abi(), bstrpackageid.into_param().abi(), bstrclsid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallRemoteComponentByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrserver: Param0, bstrpackagename: Param1, bstrprogid: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InstallRemoteComponentByName)(::core::mem::transmute_copy(self), bstrserver.into_param().abi(), bstrpackagename.into_param().abi(), bstrprogid.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRemoteComponentUtil> for ::windows::core::IUnknown {
    fn from(value: IRemoteComponentUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRemoteComponentUtil> for ::windows::core::IUnknown {
    fn from(value: &IRemoteComponentUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRemoteComponentUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRemoteComponentUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRemoteComponentUtil> for super::Com::IDispatch {
    fn from(value: IRemoteComponentUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRemoteComponentUtil> for super::Com::IDispatch {
    fn from(value: &IRemoteComponentUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRemoteComponentUtil {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IRemoteComponentUtil {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRemoteComponentUtil {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRemoteComponentUtil {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRemoteComponentUtil {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRemoteComponentUtil {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteComponentUtil").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRemoteComponentUtil {
    type Vtable = IRemoteComponentUtil_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22875_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteComponentUtil_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallRemoteComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallRemoteComponent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallRemoteComponentByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallRemoteComponentByName: usize,
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRoleAssociationUtil(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRoleAssociationUtil {
    #[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AssociateRole<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrroleid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AssociateRole)(::core::mem::transmute_copy(self), bstrroleid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AssociateRoleByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrolename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AssociateRoleByName)(::core::mem::transmute_copy(self), bstrrolename.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRoleAssociationUtil> for ::windows::core::IUnknown {
    fn from(value: IRoleAssociationUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRoleAssociationUtil> for ::windows::core::IUnknown {
    fn from(value: &IRoleAssociationUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRoleAssociationUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRoleAssociationUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRoleAssociationUtil> for super::Com::IDispatch {
    fn from(value: IRoleAssociationUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRoleAssociationUtil> for super::Com::IDispatch {
    fn from(value: &IRoleAssociationUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRoleAssociationUtil {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IRoleAssociationUtil {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRoleAssociationUtil {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRoleAssociationUtil {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRoleAssociationUtil {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRoleAssociationUtil {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRoleAssociationUtil").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRoleAssociationUtil {
    type Vtable = IRoleAssociationUtil_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22876_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRoleAssociationUtil_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AssociateRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroleid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AssociateRole: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AssociateRoleByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AssociateRoleByName: usize,
}
pub const PackageUtil: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22885_8a19_11d0_81b6_00a0c9231c29);
pub const RemoteComponentUtil: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22886_8a19_11d0_81b6_00a0c9231c29);
pub const RoleAssociationUtil: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22887_8a19_11d0_81b6_00a0c9231c29);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0001(pub i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsInstallUsers: __MIDL___MIDL_itf_mtxadmin_0107_0001 = __MIDL___MIDL_itf_mtxadmin_0107_0001(1i32);
impl ::core::marker::Copy for __MIDL___MIDL_itf_mtxadmin_0107_0001 {}
impl ::core::clone::Clone for __MIDL___MIDL_itf_mtxadmin_0107_0001 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for __MIDL___MIDL_itf_mtxadmin_0107_0001 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for __MIDL___MIDL_itf_mtxadmin_0107_0001 {
    type Abi = Self;
}
impl ::core::fmt::Debug for __MIDL___MIDL_itf_mtxadmin_0107_0001 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("__MIDL___MIDL_itf_mtxadmin_0107_0001").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0002(pub i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsExportUsers: __MIDL___MIDL_itf_mtxadmin_0107_0002 = __MIDL___MIDL_itf_mtxadmin_0107_0002(1i32);
impl ::core::marker::Copy for __MIDL___MIDL_itf_mtxadmin_0107_0002 {}
impl ::core::clone::Clone for __MIDL___MIDL_itf_mtxadmin_0107_0002 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for __MIDL___MIDL_itf_mtxadmin_0107_0002 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for __MIDL___MIDL_itf_mtxadmin_0107_0002 {
    type Abi = Self;
}
impl ::core::fmt::Debug for __MIDL___MIDL_itf_mtxadmin_0107_0002 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("__MIDL___MIDL_itf_mtxadmin_0107_0002").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0003(pub i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrObjectErrors: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368511i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrObjectInvalid: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368510i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrKeyMissing: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368509i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrAlreadyInstalled: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368508i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrDownloadFailed: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368507i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPDFWriteFail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368505i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPDFReadFail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368504i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPDFVersion: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368503i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCoReqCompInstalled: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368496i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadPath: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368502i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPackageExists: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368501i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrRoleExists: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368500i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCantCopyFile: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368499i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoTypeLib: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368498i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoUser: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368497i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrInvalidUserids: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368496i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoRegistryCLSID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368495i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadRegistryProgID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368494i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrAuthenticationLevel: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368493i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrUserPasswdNotValid: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368492i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoRegistryRead: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368491i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoRegistryWrite: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368490i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoRegistryRepair: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368489i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCLSIDOrIIDMismatch: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368488i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrRemoteInterface: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368487i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrDllRegisterServer: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368486i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoServerShare: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368485i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoAccessToUNC: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368484i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrDllLoadFailed: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368483i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadRegistryLibID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368482i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPackDirNotFound: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368481i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrTreatAs: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368480i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadForward: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368479i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadIID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368478i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrRegistrarFailed: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368477i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileDoesNotExist: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368476i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileLoadDLLFail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368475i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileGetClassObj: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368474i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileClassNotAvail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368473i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileBadTLB: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368472i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileNotInstallable: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368471i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNotChangeable: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368470i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNotDeletable: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368469i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrSession: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368468i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileNoRegistrar: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368460i32);
impl ::core::marker::Copy for __MIDL___MIDL_itf_mtxadmin_0107_0003 {}
impl ::core::clone::Clone for __MIDL___MIDL_itf_mtxadmin_0107_0003 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for __MIDL___MIDL_itf_mtxadmin_0107_0003 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for __MIDL___MIDL_itf_mtxadmin_0107_0003 {
    type Abi = Self;
}
impl ::core::fmt::Debug for __MIDL___MIDL_itf_mtxadmin_0107_0003 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("__MIDL___MIDL_itf_mtxadmin_0107_0003").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
