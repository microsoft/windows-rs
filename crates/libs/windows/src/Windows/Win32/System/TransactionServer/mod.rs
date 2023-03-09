#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ICatalog(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ICatalog {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCollection<P0>(&self, bstrcollname: P0) -> ::windows::core::Result<super::Com::IDispatch>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).GetCollection)(::windows::core::Interface::as_raw(self), bstrcollname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Connect<P0>(&self, bstrconnectstring: P0) -> ::windows::core::Result<super::Com::IDispatch>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).Connect)(::windows::core::Interface::as_raw(self), bstrconnectstring.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn MajorVersion(&self, retval: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MajorVersion)(::windows::core::Interface::as_raw(self), retval).ok()
    }
    pub unsafe fn MinorVersion(&self, retval: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MinorVersion)(::windows::core::Interface::as_raw(self), retval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ICatalog, ::windows::core::IUnknown, super::Com::IDispatch);
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
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ICatalog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ICatalog {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22870_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICatalog_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcollname: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrconnectstring: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
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
    pub unsafe fn InstallComponent<P0, P1, P2>(&self, bstrdllfile: P0, bstrtypelibfile: P1, bstrproxystubdllfile: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).InstallComponent)(::windows::core::Interface::as_raw(self), bstrdllfile.into_param().abi(), bstrtypelibfile.into_param().abi(), bstrproxystubdllfile.into_param().abi()).ok()
    }
    pub unsafe fn ImportComponent<P0>(&self, bstrclsid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).ImportComponent)(::windows::core::Interface::as_raw(self), bstrclsid.into_param().abi()).ok()
    }
    pub unsafe fn ImportComponentByName<P0>(&self, bstrprogid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).ImportComponentByName)(::windows::core::Interface::as_raw(self), bstrprogid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCLSIDs<P0, P1>(&self, bstrdllfile: P0, bstrtypelibfile: P1, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).GetCLSIDs)(::windows::core::Interface::as_raw(self), bstrdllfile.into_param().abi(), bstrtypelibfile.into_param().abi(), aclsids).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IComponentUtil, ::windows::core::IUnknown, super::Com::IDispatch);
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
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IComponentUtil {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IComponentUtil {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22873_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IComponentUtil_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub InstallComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdllfile: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrtypelibfile: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrproxystubdllfile: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ImportComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclsid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ImportComponentByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprogid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCLSIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdllfile: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrtypelibfile: ::std::mem::MaybeUninit<::windows::core::BSTR>, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCLSIDs: usize,
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPackageUtil(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPackageUtil {
    pub unsafe fn InstallPackage<P0, P1>(&self, bstrpackagefile: P0, bstrinstallpath: P1, loptions: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).InstallPackage)(::windows::core::Interface::as_raw(self), bstrpackagefile.into_param().abi(), bstrinstallpath.into_param().abi(), loptions).ok()
    }
    pub unsafe fn ExportPackage<P0, P1>(&self, bstrpackageid: P0, bstrpackagefile: P1, loptions: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).ExportPackage)(::windows::core::Interface::as_raw(self), bstrpackageid.into_param().abi(), bstrpackagefile.into_param().abi(), loptions).ok()
    }
    pub unsafe fn ShutdownPackage<P0>(&self, bstrpackageid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).ShutdownPackage)(::windows::core::Interface::as_raw(self), bstrpackageid.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IPackageUtil, ::windows::core::IUnknown, super::Com::IDispatch);
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
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPackageUtil {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IPackageUtil {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22874_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUtil_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub InstallPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpackagefile: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrinstallpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, loptions: i32) -> ::windows::core::HRESULT,
    pub ExportPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpackageid: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrpackagefile: ::std::mem::MaybeUninit<::windows::core::BSTR>, loptions: i32) -> ::windows::core::HRESULT,
    pub ShutdownPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpackageid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRemoteComponentUtil(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRemoteComponentUtil {
    pub unsafe fn InstallRemoteComponent<P0, P1, P2>(&self, bstrserver: P0, bstrpackageid: P1, bstrclsid: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).InstallRemoteComponent)(::windows::core::Interface::as_raw(self), bstrserver.into_param().abi(), bstrpackageid.into_param().abi(), bstrclsid.into_param().abi()).ok()
    }
    pub unsafe fn InstallRemoteComponentByName<P0, P1, P2>(&self, bstrserver: P0, bstrpackagename: P1, bstrprogid: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).InstallRemoteComponentByName)(::windows::core::Interface::as_raw(self), bstrserver.into_param().abi(), bstrpackagename.into_param().abi(), bstrprogid.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRemoteComponentUtil, ::windows::core::IUnknown, super::Com::IDispatch);
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
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRemoteComponentUtil {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRemoteComponentUtil {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22875_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteComponentUtil_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub InstallRemoteComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrserver: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrpackageid: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrclsid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub InstallRemoteComponentByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrserver: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrpackagename: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrprogid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRoleAssociationUtil(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRoleAssociationUtil {
    pub unsafe fn AssociateRole<P0>(&self, bstrroleid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AssociateRole)(::windows::core::Interface::as_raw(self), bstrroleid.into_param().abi()).ok()
    }
    pub unsafe fn AssociateRoleByName<P0>(&self, bstrrolename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AssociateRoleByName)(::windows::core::Interface::as_raw(self), bstrrolename.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRoleAssociationUtil, ::windows::core::IUnknown, super::Com::IDispatch);
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
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRoleAssociationUtil {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRoleAssociationUtil {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22876_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRoleAssociationUtil_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub AssociateRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroleid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub AssociateRoleByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const Catalog: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22881_8a19_11d0_81b6_00a0c9231c29);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const CatalogCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22883_8a19_11d0_81b6_00a0c9231c29);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const CatalogObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22882_8a19_11d0_81b6_00a0c9231c29);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const ComponentUtil: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22884_8a19_11d0_81b6_00a0c9231c29);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const PackageUtil: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22885_8a19_11d0_81b6_00a0c9231c29);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const RemoteComponentUtil: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22886_8a19_11d0_81b6_00a0c9231c29);
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const RoleAssociationUtil: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22887_8a19_11d0_81b6_00a0c9231c29);
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
impl ::windows::core::TypeKind for MTSAdminErrorCodes {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for MTSPackageExportOptions {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for MTSPackageInstallOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MTSPackageInstallOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MTSPackageInstallOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
