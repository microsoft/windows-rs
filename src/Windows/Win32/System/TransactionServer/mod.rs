#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const Catalog: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1857169537, 35353, 4560, [129, 182, 0, 160, 201, 35, 28, 41]);
pub const CatalogCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1857169539, 35353, 4560, [129, 182, 0, 160, 201, 35, 28, 41]);
pub const CatalogObject: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1857169538, 35353, 4560, [129, 182, 0, 160, 201, 35, 28, 41]);
pub const ComponentUtil: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1857169540, 35353, 4560, [129, 182, 0, 160, 201, 35, 28, 41]);
#[doc = "*Required features: `Win32_System_TransactionServer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICatalog(pub ::windows::runtime::IUnknown);
impl ICatalog {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetCollection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcollname: Param0) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), bstrcollname.into_param().abi(), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Connect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrconnectstring: Param0) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrconnectstring.into_param().abi(), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[doc = "*Required features: `Win32_System_TransactionServer`*"]
    pub unsafe fn MajorVersion(&self, retval: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(retval)).ok()
    }
    #[doc = "*Required features: `Win32_System_TransactionServer`*"]
    pub unsafe fn MinorVersion(&self, retval: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(retval)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICatalog {
    type Vtable = ICatalog_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1857169520, 35353, 4560, [129, 182, 0, 160, 201, 35, 28, 41]);
}
impl ::std::convert::From<ICatalog> for ::windows::runtime::IUnknown {
    fn from(value: ICatalog) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ICatalog> for ::windows::runtime::IUnknown {
    fn from(value: &ICatalog) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICatalog {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICatalog {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<ICatalog> for super::Ole::Automation::IDispatch {
    fn from(value: ICatalog) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&ICatalog> for super::Ole::Automation::IDispatch {
    fn from(value: &ICatalog) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for ICatalog {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &ICatalog {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatalog_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcollname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrconnectstring: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, retval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, retval: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_TransactionServer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IComponentUtil(pub ::windows::runtime::IUnknown);
impl IComponentUtil {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn InstallComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdllfile: Param0, bstrtypelibfile: Param1, bstrproxystubdllfile: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), bstrdllfile.into_param().abi(), bstrtypelibfile.into_param().abi(), bstrproxystubdllfile.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn ImportComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrclsid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn ImportComponentByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprogid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), bstrprogid.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetCLSIDs<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdllfile: Param0, bstrtypelibfile: Param1, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bstrdllfile.into_param().abi(), bstrtypelibfile.into_param().abi(), ::std::mem::transmute(aclsids)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComponentUtil {
    type Vtable = IComponentUtil_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1857169523, 35353, 4560, [129, 182, 0, 160, 201, 35, 28, 41]);
}
impl ::std::convert::From<IComponentUtil> for ::windows::runtime::IUnknown {
    fn from(value: IComponentUtil) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IComponentUtil> for ::windows::runtime::IUnknown {
    fn from(value: &IComponentUtil) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComponentUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComponentUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IComponentUtil> for super::Ole::Automation::IDispatch {
    fn from(value: IComponentUtil) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IComponentUtil> for super::Ole::Automation::IDispatch {
    fn from(value: &IComponentUtil) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IComponentUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IComponentUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentUtil_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdllfile: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtypelibfile: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrproxystubdllfile: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrclsid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrprogid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdllfile: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtypelibfile: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_System_TransactionServer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPackageUtil(pub ::windows::runtime::IUnknown);
impl IPackageUtil {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn InstallPackage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpackagefile: Param0, bstrinstallpath: Param1, loptions: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), bstrpackagefile.into_param().abi(), bstrinstallpath.into_param().abi(), ::std::mem::transmute(loptions)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn ExportPackage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpackageid: Param0, bstrpackagefile: Param1, loptions: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrpackageid.into_param().abi(), bstrpackagefile.into_param().abi(), ::std::mem::transmute(loptions)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn ShutdownPackage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpackageid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), bstrpackageid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPackageUtil {
    type Vtable = IPackageUtil_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1857169524, 35353, 4560, [129, 182, 0, 160, 201, 35, 28, 41]);
}
impl ::std::convert::From<IPackageUtil> for ::windows::runtime::IUnknown {
    fn from(value: IPackageUtil) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPackageUtil> for ::windows::runtime::IUnknown {
    fn from(value: &IPackageUtil) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPackageUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPackageUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IPackageUtil> for super::Ole::Automation::IDispatch {
    fn from(value: IPackageUtil) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IPackageUtil> for super::Ole::Automation::IDispatch {
    fn from(value: &IPackageUtil) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IPackageUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IPackageUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUtil_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpackagefile: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinstallpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpackageid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackagefile: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpackageid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_TransactionServer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRemoteComponentUtil(pub ::windows::runtime::IUnknown);
impl IRemoteComponentUtil {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn InstallRemoteComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrserver: Param0, bstrpackageid: Param1, bstrclsid: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), bstrserver.into_param().abi(), bstrpackageid.into_param().abi(), bstrclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn InstallRemoteComponentByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrserver: Param0, bstrpackagename: Param1, bstrprogid: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrserver.into_param().abi(), bstrpackagename.into_param().abi(), bstrprogid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRemoteComponentUtil {
    type Vtable = IRemoteComponentUtil_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1857169525, 35353, 4560, [129, 182, 0, 160, 201, 35, 28, 41]);
}
impl ::std::convert::From<IRemoteComponentUtil> for ::windows::runtime::IUnknown {
    fn from(value: IRemoteComponentUtil) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRemoteComponentUtil> for ::windows::runtime::IUnknown {
    fn from(value: &IRemoteComponentUtil) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRemoteComponentUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRemoteComponentUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IRemoteComponentUtil> for super::Ole::Automation::IDispatch {
    fn from(value: IRemoteComponentUtil) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IRemoteComponentUtil> for super::Ole::Automation::IDispatch {
    fn from(value: &IRemoteComponentUtil) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IRemoteComponentUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IRemoteComponentUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteComponentUtil_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrserver: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackageid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrserver: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackagename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrprogid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_TransactionServer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRoleAssociationUtil(pub ::windows::runtime::IUnknown);
impl IRoleAssociationUtil {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn AssociateRole<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrroleid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), bstrroleid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_TransactionServer`, `Win32_Foundation`*"]
    pub unsafe fn AssociateRoleByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrolename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrrolename.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRoleAssociationUtil {
    type Vtable = IRoleAssociationUtil_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1857169526, 35353, 4560, [129, 182, 0, 160, 201, 35, 28, 41]);
}
impl ::std::convert::From<IRoleAssociationUtil> for ::windows::runtime::IUnknown {
    fn from(value: IRoleAssociationUtil) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRoleAssociationUtil> for ::windows::runtime::IUnknown {
    fn from(value: &IRoleAssociationUtil) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRoleAssociationUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRoleAssociationUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IRoleAssociationUtil> for super::Ole::Automation::IDispatch {
    fn from(value: IRoleAssociationUtil) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IRoleAssociationUtil> for super::Ole::Automation::IDispatch {
    fn from(value: &IRoleAssociationUtil) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IRoleAssociationUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IRoleAssociationUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoleAssociationUtil_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrroleid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrolename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const PackageUtil: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1857169541, 35353, 4560, [129, 182, 0, 160, 201, 35, 28, 41]);
pub const RemoteComponentUtil: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1857169542, 35353, 4560, [129, 182, 0, 160, 201, 35, 28, 41]);
pub const RoleAssociationUtil: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1857169543, 35353, 4560, [129, 182, 0, 160, 201, 35, 28, 41]);
#[doc = "*Required features: `Win32_System_TransactionServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0001(pub i32);
pub const mtsInstallUsers: __MIDL___MIDL_itf_mtxadmin_0107_0001 = __MIDL___MIDL_itf_mtxadmin_0107_0001(1i32);
impl ::std::convert::From<i32> for __MIDL___MIDL_itf_mtxadmin_0107_0001 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for __MIDL___MIDL_itf_mtxadmin_0107_0001 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_TransactionServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0002(pub i32);
pub const mtsExportUsers: __MIDL___MIDL_itf_mtxadmin_0107_0002 = __MIDL___MIDL_itf_mtxadmin_0107_0002(1i32);
impl ::std::convert::From<i32> for __MIDL___MIDL_itf_mtxadmin_0107_0002 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for __MIDL___MIDL_itf_mtxadmin_0107_0002 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_TransactionServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for __MIDL___MIDL_itf_mtxadmin_0107_0003 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for __MIDL___MIDL_itf_mtxadmin_0107_0003 {
    type Abi = Self;
}
