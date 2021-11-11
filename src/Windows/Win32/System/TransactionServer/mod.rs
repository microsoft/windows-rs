#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const Catalog: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22881_8a19_11d0_81b6_00a0c9231c29);
pub const CatalogCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22883_8a19_11d0_81b6_00a0c9231c29);
pub const CatalogObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22882_8a19_11d0_81b6_00a0c9231c29);
pub const ComponentUtil: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22884_8a19_11d0_81b6_00a0c9231c29);
#[doc = "*Required features: `Win32_System_TransactionServer`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICatalog(pub ::windows::core::IUnknown);
impl ICatalog {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetCollection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcollname: Param0) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrcollname.into_param().abi(), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrconnectstring: Param0) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrconnectstring.into_param().abi(), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `Win32_System_TransactionServer`*"]
    pub unsafe fn MajorVersion(&self, retval: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(retval)).ok()
    }
    #[doc = "*Required features: `Win32_System_TransactionServer`*"]
    pub unsafe fn MinorVersion(&self, retval: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(retval)).ok()
    }
}
unsafe impl ::windows::core::Interface for ICatalog {
    type Vtable = ICatalog_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22870_8a19_11d0_81b6_00a0c9231c29);
}
impl ::core::convert::From<ICatalog> for ::windows::core::IUnknown {
    fn from(value: ICatalog) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICatalog> for ::windows::core::IUnknown {
    fn from(value: &ICatalog) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICatalog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICatalog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ICatalog {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatalog_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrconnectstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_TransactionServer`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComponentUtil(pub ::windows::core::IUnknown);
impl IComponentUtil {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn InstallComponent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdllfile: Param0, bstrtypelibfile: Param1, bstrproxystubdllfile: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrdllfile.into_param().abi(), bstrtypelibfile.into_param().abi(), bstrproxystubdllfile.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn ImportComponent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrclsid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn ImportComponentByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprogid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrprogid.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetCLSIDs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdllfile: Param0, bstrtypelibfile: Param1, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bstrdllfile.into_param().abi(), bstrtypelibfile.into_param().abi(), ::core::mem::transmute(aclsids)).ok()
    }
}
unsafe impl ::windows::core::Interface for IComponentUtil {
    type Vtable = IComponentUtil_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22873_8a19_11d0_81b6_00a0c9231c29);
}
impl ::core::convert::From<IComponentUtil> for ::windows::core::IUnknown {
    fn from(value: IComponentUtil) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComponentUtil> for ::windows::core::IUnknown {
    fn from(value: &IComponentUtil) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IComponentUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IComponentUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IComponentUtil {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentUtil_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrdllfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtypelibfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrproxystubdllfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrdllfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtypelibfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_System_TransactionServer`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPackageUtil(pub ::windows::core::IUnknown);
impl IPackageUtil {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn InstallPackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpackagefile: Param0, bstrinstallpath: Param1, loptions: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrpackagefile.into_param().abi(), bstrinstallpath.into_param().abi(), ::core::mem::transmute(loptions)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn ExportPackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpackageid: Param0, bstrpackagefile: Param1, loptions: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrpackageid.into_param().abi(), bstrpackagefile.into_param().abi(), ::core::mem::transmute(loptions)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn ShutdownPackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpackageid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrpackageid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IPackageUtil {
    type Vtable = IPackageUtil_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22874_8a19_11d0_81b6_00a0c9231c29);
}
impl ::core::convert::From<IPackageUtil> for ::windows::core::IUnknown {
    fn from(value: IPackageUtil) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPackageUtil> for ::windows::core::IUnknown {
    fn from(value: &IPackageUtil) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPackageUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPackageUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IPackageUtil {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUtil_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpackagefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinstallpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackagefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_TransactionServer`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRemoteComponentUtil(pub ::windows::core::IUnknown);
impl IRemoteComponentUtil {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn InstallRemoteComponent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrserver: Param0, bstrpackageid: Param1, bstrclsid: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrserver.into_param().abi(), bstrpackageid.into_param().abi(), bstrclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn InstallRemoteComponentByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrserver: Param0, bstrpackagename: Param1, bstrprogid: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrserver.into_param().abi(), bstrpackagename.into_param().abi(), bstrprogid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IRemoteComponentUtil {
    type Vtable = IRemoteComponentUtil_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22875_8a19_11d0_81b6_00a0c9231c29);
}
impl ::core::convert::From<IRemoteComponentUtil> for ::windows::core::IUnknown {
    fn from(value: IRemoteComponentUtil) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRemoteComponentUtil> for ::windows::core::IUnknown {
    fn from(value: &IRemoteComponentUtil) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRemoteComponentUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRemoteComponentUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRemoteComponentUtil {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteComponentUtil_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_TransactionServer`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRoleAssociationUtil(pub ::windows::core::IUnknown);
impl IRoleAssociationUtil {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn AssociateRole<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrroleid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrroleid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn AssociateRoleByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrolename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrrolename.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IRoleAssociationUtil {
    type Vtable = IRoleAssociationUtil_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22876_8a19_11d0_81b6_00a0c9231c29);
}
impl ::core::convert::From<IRoleAssociationUtil> for ::windows::core::IUnknown {
    fn from(value: IRoleAssociationUtil) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRoleAssociationUtil> for ::windows::core::IUnknown {
    fn from(value: &IRoleAssociationUtil) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRoleAssociationUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRoleAssociationUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRoleAssociationUtil {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoleAssociationUtil_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrroleid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const PackageUtil: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22885_8a19_11d0_81b6_00a0c9231c29);
pub const RemoteComponentUtil: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22886_8a19_11d0_81b6_00a0c9231c29);
pub const RoleAssociationUtil: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22887_8a19_11d0_81b6_00a0c9231c29);
#[doc = "*Required features: `Win32_System_TransactionServer`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0001(pub i32);
pub const mtsInstallUsers: __MIDL___MIDL_itf_mtxadmin_0107_0001 = __MIDL___MIDL_itf_mtxadmin_0107_0001(1i32);
impl ::core::convert::From<i32> for __MIDL___MIDL_itf_mtxadmin_0107_0001 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for __MIDL___MIDL_itf_mtxadmin_0107_0001 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_TransactionServer`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0002(pub i32);
pub const mtsExportUsers: __MIDL___MIDL_itf_mtxadmin_0107_0002 = __MIDL___MIDL_itf_mtxadmin_0107_0002(1i32);
impl ::core::convert::From<i32> for __MIDL___MIDL_itf_mtxadmin_0107_0002 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for __MIDL___MIDL_itf_mtxadmin_0107_0002 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_TransactionServer`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0003(pub i32);
pub const mtsErrObjectErrors: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368511i32);
pub const mtsErrObjectInvalid: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368510i32);
pub const mtsErrKeyMissing: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368509i32);
pub const mtsErrAlreadyInstalled: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368508i32);
pub const mtsErrDownloadFailed: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368507i32);
pub const mtsErrPDFWriteFail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368505i32);
pub const mtsErrPDFReadFail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368504i32);
pub const mtsErrPDFVersion: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368503i32);
pub const mtsErrCoReqCompInstalled: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368496i32);
pub const mtsErrBadPath: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368502i32);
pub const mtsErrPackageExists: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368501i32);
pub const mtsErrRoleExists: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368500i32);
pub const mtsErrCantCopyFile: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368499i32);
pub const mtsErrNoTypeLib: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368498i32);
pub const mtsErrNoUser: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368497i32);
pub const mtsErrInvalidUserids: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368496i32);
pub const mtsErrNoRegistryCLSID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368495i32);
pub const mtsErrBadRegistryProgID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368494i32);
pub const mtsErrAuthenticationLevel: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368493i32);
pub const mtsErrUserPasswdNotValid: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368492i32);
pub const mtsErrNoRegistryRead: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368491i32);
pub const mtsErrNoRegistryWrite: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368490i32);
pub const mtsErrNoRegistryRepair: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368489i32);
pub const mtsErrCLSIDOrIIDMismatch: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368488i32);
pub const mtsErrRemoteInterface: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368487i32);
pub const mtsErrDllRegisterServer: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368486i32);
pub const mtsErrNoServerShare: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368485i32);
pub const mtsErrNoAccessToUNC: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368484i32);
pub const mtsErrDllLoadFailed: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368483i32);
pub const mtsErrBadRegistryLibID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368482i32);
pub const mtsErrPackDirNotFound: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368481i32);
pub const mtsErrTreatAs: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368480i32);
pub const mtsErrBadForward: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368479i32);
pub const mtsErrBadIID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368478i32);
pub const mtsErrRegistrarFailed: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368477i32);
pub const mtsErrCompFileDoesNotExist: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368476i32);
pub const mtsErrCompFileLoadDLLFail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368475i32);
pub const mtsErrCompFileGetClassObj: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368474i32);
pub const mtsErrCompFileClassNotAvail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368473i32);
pub const mtsErrCompFileBadTLB: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368472i32);
pub const mtsErrCompFileNotInstallable: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368471i32);
pub const mtsErrNotChangeable: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368470i32);
pub const mtsErrNotDeletable: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368469i32);
pub const mtsErrSession: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368468i32);
pub const mtsErrCompFileNoRegistrar: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368460i32);
impl ::core::convert::From<i32> for __MIDL___MIDL_itf_mtxadmin_0107_0003 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for __MIDL___MIDL_itf_mtxadmin_0107_0003 {
    type Abi = Self;
}
