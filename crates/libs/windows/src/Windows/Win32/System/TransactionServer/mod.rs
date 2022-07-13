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
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetCollection<'a, P0>(&self, bstrcollname: P0) -> ::windows::core::Result<super::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCollection)(::windows::core::Interface::as_raw(self), bstrcollname.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Connect<'a, P0>(&self, bstrconnectstring: P0) -> ::windows::core::Result<super::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Connect)(::windows::core::Interface::as_raw(self), bstrconnectstring.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn MajorVersion(&self, retval: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MajorVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(retval)).ok()
    }
    pub unsafe fn MinorVersion(&self, retval: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MinorVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(retval)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICatalog> for ::windows::core::IUnknown {
    fn from(value: ICatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ICatalog> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICatalog) -> Self {
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
impl ::core::convert::From<ICatalog> for super::Com::IDispatch {
    fn from(value: ICatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ICatalog> for &'a super::Com::IDispatch {
    fn from(value: &'a ICatalog) -> Self {
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
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetCollection: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrconnectstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallComponent<'a, P0, P1, P2>(&self, bstrdllfile: P0, bstrtypelibfile: P1, bstrproxystubdllfile: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).InstallComponent)(::windows::core::Interface::as_raw(self), bstrdllfile.into().abi(), bstrtypelibfile.into().abi(), bstrproxystubdllfile.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportComponent<'a, P0>(&self, bstrclsid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).ImportComponent)(::windows::core::Interface::as_raw(self), bstrclsid.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportComponentByName<'a, P0>(&self, bstrprogid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).ImportComponentByName)(::windows::core::Interface::as_raw(self), bstrprogid.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetCLSIDs<'a, P0, P1>(&self, bstrdllfile: P0, bstrtypelibfile: P1, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).GetCLSIDs)(::windows::core::Interface::as_raw(self), bstrdllfile.into().abi(), bstrtypelibfile.into().abi(), ::core::mem::transmute(aclsids)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IComponentUtil> for ::windows::core::IUnknown {
    fn from(value: IComponentUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IComponentUtil> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComponentUtil) -> Self {
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
impl ::core::convert::From<IComponentUtil> for super::Com::IDispatch {
    fn from(value: IComponentUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IComponentUtil> for &'a super::Com::IDispatch {
    fn from(value: &'a IComponentUtil) -> Self {
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
    pub base__: super::Com::IDispatch_Vtbl,
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
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallPackage<'a, P0, P1>(&self, bstrpackagefile: P0, bstrinstallpath: P1, loptions: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).InstallPackage)(::windows::core::Interface::as_raw(self), bstrpackagefile.into().abi(), bstrinstallpath.into().abi(), loptions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExportPackage<'a, P0, P1>(&self, bstrpackageid: P0, bstrpackagefile: P1, loptions: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).ExportPackage)(::windows::core::Interface::as_raw(self), bstrpackageid.into().abi(), bstrpackagefile.into().abi(), loptions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShutdownPackage<'a, P0>(&self, bstrpackageid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).ShutdownPackage)(::windows::core::Interface::as_raw(self), bstrpackageid.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPackageUtil> for ::windows::core::IUnknown {
    fn from(value: IPackageUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IPackageUtil> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPackageUtil) -> Self {
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
impl ::core::convert::From<IPackageUtil> for super::Com::IDispatch {
    fn from(value: IPackageUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IPackageUtil> for &'a super::Com::IDispatch {
    fn from(value: &'a IPackageUtil) -> Self {
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
    pub base__: super::Com::IDispatch_Vtbl,
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
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallRemoteComponent<'a, P0, P1, P2>(&self, bstrserver: P0, bstrpackageid: P1, bstrclsid: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).InstallRemoteComponent)(::windows::core::Interface::as_raw(self), bstrserver.into().abi(), bstrpackageid.into().abi(), bstrclsid.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallRemoteComponentByName<'a, P0, P1, P2>(&self, bstrserver: P0, bstrpackagename: P1, bstrprogid: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).InstallRemoteComponentByName)(::windows::core::Interface::as_raw(self), bstrserver.into().abi(), bstrpackagename.into().abi(), bstrprogid.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRemoteComponentUtil> for ::windows::core::IUnknown {
    fn from(value: IRemoteComponentUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRemoteComponentUtil> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRemoteComponentUtil) -> Self {
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
impl ::core::convert::From<IRemoteComponentUtil> for super::Com::IDispatch {
    fn from(value: IRemoteComponentUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRemoteComponentUtil> for &'a super::Com::IDispatch {
    fn from(value: &'a IRemoteComponentUtil) -> Self {
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
    pub base__: super::Com::IDispatch_Vtbl,
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
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AssociateRole<'a, P0>(&self, bstrroleid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).AssociateRole)(::windows::core::Interface::as_raw(self), bstrroleid.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AssociateRoleByName<'a, P0>(&self, bstrrolename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).AssociateRoleByName)(::windows::core::Interface::as_raw(self), bstrrolename.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRoleAssociationUtil> for ::windows::core::IUnknown {
    fn from(value: IRoleAssociationUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRoleAssociationUtil> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRoleAssociationUtil) -> Self {
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
impl ::core::convert::From<IRoleAssociationUtil> for super::Com::IDispatch {
    fn from(value: IRoleAssociationUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRoleAssociationUtil> for &'a super::Com::IDispatch {
    fn from(value: &'a IRoleAssociationUtil) -> Self {
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
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AssociateRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroleid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AssociateRole: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AssociateRoleByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AssociateRoleByName: usize,
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MTSAdminErrorCodes(pub i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrObjectErrors: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368511i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrObjectInvalid: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368510i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrKeyMissing: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368509i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrAlreadyInstalled: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368508i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrDownloadFailed: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368507i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPDFWriteFail: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368505i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPDFReadFail: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368504i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPDFVersion: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368503i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCoReqCompInstalled: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368496i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadPath: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368502i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPackageExists: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368501i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrRoleExists: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368500i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCantCopyFile: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368499i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoTypeLib: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368498i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoUser: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368497i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrInvalidUserids: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368496i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoRegistryCLSID: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368495i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadRegistryProgID: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368494i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrAuthenticationLevel: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368493i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrUserPasswdNotValid: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368492i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoRegistryRead: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368491i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoRegistryWrite: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368490i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoRegistryRepair: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368489i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCLSIDOrIIDMismatch: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368488i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrRemoteInterface: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368487i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrDllRegisterServer: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368486i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoServerShare: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368485i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoAccessToUNC: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368484i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrDllLoadFailed: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368483i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadRegistryLibID: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368482i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPackDirNotFound: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368481i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrTreatAs: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368480i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadForward: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368479i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadIID: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368478i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrRegistrarFailed: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368477i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileDoesNotExist: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368476i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileLoadDLLFail: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368475i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileGetClassObj: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368474i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileClassNotAvail: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368473i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileBadTLB: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368472i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileNotInstallable: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368471i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNotChangeable: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368470i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNotDeletable: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368469i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrSession: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368468i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileNoRegistrar: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368460i32);
impl ::core::marker::Copy for MTSAdminErrorCodes {}
impl ::core::clone::Clone for MTSAdminErrorCodes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MTSAdminErrorCodes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MTSAdminErrorCodes {
    type Abi = Self;
}
impl ::core::fmt::Debug for MTSAdminErrorCodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MTSAdminErrorCodes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MTSPackageExportOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsExportUsers: MTSPackageExportOptions = MTSPackageExportOptions(1i32);
impl ::core::marker::Copy for MTSPackageExportOptions {}
impl ::core::clone::Clone for MTSPackageExportOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MTSPackageExportOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MTSPackageExportOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for MTSPackageExportOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MTSPackageExportOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MTSPackageInstallOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsInstallUsers: MTSPackageInstallOptions = MTSPackageInstallOptions(1i32);
impl ::core::marker::Copy for MTSPackageInstallOptions {}
impl ::core::clone::Clone for MTSPackageInstallOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MTSPackageInstallOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MTSPackageInstallOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for MTSPackageInstallOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MTSPackageInstallOptions").field(&self.0).finish()
    }
}
pub const PackageUtil: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22885_8a19_11d0_81b6_00a0c9231c29);
pub const RemoteComponentUtil: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22886_8a19_11d0_81b6_00a0c9231c29);
pub const RoleAssociationUtil: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22887_8a19_11d0_81b6_00a0c9231c29);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
