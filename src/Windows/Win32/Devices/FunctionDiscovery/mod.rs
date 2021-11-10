#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const E_FDPAIRING_AUTHFAILURE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1882193917i32 as _);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const E_FDPAIRING_AUTHNOTALLOWED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1882193914i32 as _);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const E_FDPAIRING_CONNECTTIMEOUT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1882193916i32 as _);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const E_FDPAIRING_HWFAILURE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1882193918i32 as _);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const E_FDPAIRING_IPBUSDISABLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1882193913i32 as _);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const E_FDPAIRING_NOCONNECTION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1882193919i32 as _);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const E_FDPAIRING_NOPROFILES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1882193912i32 as _);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const E_FDPAIRING_TOOMANYCONNECTIONS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1882193915i32 as _);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_EVENTID: u32 = 1000u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_EVENTID_ASYNCTHREADEXIT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_EVENTID_IPADDRESSCHANGE: u32 = 1003u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_EVENTID_PRIVATE: u32 = 100u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_EVENTID_QUERYREFRESH: u32 = 1004u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_EVENTID_SEARCHCOMPLETE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_EVENTID_SEARCHSTART: u32 = 1002u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_LONGHORN: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_Visibility_Default: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_Visibility_Hidden: u32 = 1u32;
pub const FMTID_Device: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57);
pub const FMTID_DeviceInterface: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x53808008_07bb_4661_bc3c_b5953e708560);
pub const FMTID_FD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x904b03a2_471d_423c_a584_f3483238a146);
pub const FMTID_PNPX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd);
pub const FMTID_PNPXDynamicProperty: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd);
pub const FMTID_Pairing: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc);
pub const FMTID_WSD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x92506491_ff95_4724_a05a_5b81885a7c92);
pub const FunctionDiscovery: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc72be2ec_8e90_452c_b29a_ab8ff1c071fc);
pub const FunctionInstanceCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xba818ce5_b55f_443f_ad39_2fe89be6191f);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFunctionDiscovery(pub ::windows::runtime::IUnknown);
impl IFunctionDiscovery {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn GetInstanceCollection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszcategory: Param0, pszsubcategory: Param1, fincludeallsubcategories: Param2) -> ::windows::runtime::Result<IFunctionInstanceCollection> {
        let mut result__: <IFunctionInstanceCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszcategory.into_param().abi(), pszsubcategory.into_param().abi(), fincludeallsubcategories.into_param().abi(), &mut result__).from_abi::<IFunctionInstanceCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn GetInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszfunctioninstanceidentity: Param0) -> ::windows::runtime::Result<IFunctionInstance> {
        let mut result__: <IFunctionInstance as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszfunctioninstanceidentity.into_param().abi(), &mut result__).from_abi::<IFunctionInstance>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn CreateInstanceCollectionQuery<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryNotification>>(
        &self,
        pszcategory: Param0,
        pszsubcategory: Param1,
        fincludeallsubcategories: Param2,
        pifunctiondiscoverynotification: Param3,
        pfdqcquerycontext: *mut u64,
        ppifunctioninstancecollectionquery: *mut ::core::option::Option<IFunctionInstanceCollectionQuery>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszcategory.into_param().abi(), pszsubcategory.into_param().abi(), fincludeallsubcategories.into_param().abi(), pifunctiondiscoverynotification.into_param().abi(), ::core::mem::transmute(pfdqcquerycontext), ::core::mem::transmute(ppifunctioninstancecollectionquery)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn CreateInstanceQuery<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryNotification>>(&self, pszfunctioninstanceidentity: Param0, pifunctiondiscoverynotification: Param1, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut ::core::option::Option<IFunctionInstanceQuery>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszfunctioninstanceidentity.into_param().abi(), pifunctiondiscoverynotification.into_param().abi(), ::core::mem::transmute(pfdqcquerycontext), ::core::mem::transmute(ppifunctioninstancequery)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn AddInstance<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: Param1, pszsubcategory: Param2, pszcategoryidentity: Param3) -> ::windows::runtime::Result<IFunctionInstance> {
        let mut result__: <IFunctionInstance as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumsystemvisibility), pszcategory.into_param().abi(), pszsubcategory.into_param().abi(), pszcategoryidentity.into_param().abi(), &mut result__).from_abi::<IFunctionInstance>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn RemoveInstance<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: Param1, pszsubcategory: Param2, pszcategoryidentity: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumsystemvisibility), pszcategory.into_param().abi(), pszsubcategory.into_param().abi(), pszcategoryidentity.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionDiscovery {
    type Vtable = IFunctionDiscovery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4df99b70_e148_4432_b004_4c9eeb535a5e);
}
impl ::core::convert::From<IFunctionDiscovery> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionDiscovery) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFunctionDiscovery> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionDiscovery) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionDiscovery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFunctionDiscovery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscovery_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, ppifunctioninstancecollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfunctioninstanceidentity: super::super::Foundation::PWSTR, ppifunctioninstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, pifunctiondiscoverynotification: ::windows::runtime::RawPtr, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfunctioninstanceidentity: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::runtime::RawPtr, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumsystemvisibility: SystemVisibilityFlags, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, pszcategoryidentity: super::super::Foundation::PWSTR, ppifunctioninstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumsystemvisibility: SystemVisibilityFlags, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, pszcategoryidentity: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFunctionDiscoveryNotification(pub ::windows::runtime::IUnknown);
impl IFunctionDiscoveryNotification {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn OnUpdate<'a, Param2: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumqueryupdateaction), ::core::mem::transmute(fdqcquerycontext), pifunctioninstance.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn OnError<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hr: ::windows::runtime::HRESULT, fdqcquerycontext: u64, pszprovider: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr), ::core::mem::transmute(fdqcquerycontext), pszprovider.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn OnEvent<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dweventid: u32, fdqcquerycontext: u64, pszprovider: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dweventid), ::core::mem::transmute(fdqcquerycontext), pszprovider.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionDiscoveryNotification {
    type Vtable = IFunctionDiscoveryNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5f6c1ba8_5330_422e_a368_572b244d3f87);
}
impl ::core::convert::From<IFunctionDiscoveryNotification> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionDiscoveryNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFunctionDiscoveryNotification> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionDiscoveryNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionDiscoveryNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFunctionDiscoveryNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT, fdqcquerycontext: u64, pszprovider: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dweventid: u32, fdqcquerycontext: u64, pszprovider: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFunctionDiscoveryProvider(pub ::windows::runtime::IUnknown);
impl IFunctionDiscoveryProvider {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryProviderFactory>, Param1: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryNotification>>(&self, pifunctiondiscoveryproviderfactory: Param0, pifunctiondiscoverynotification: Param1, lciduserdefault: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pifunctiondiscoveryproviderfactory.into_param().abi(), pifunctiondiscoverynotification.into_param().abi(), ::core::mem::transmute(lciduserdefault), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Query<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryProviderQuery>>(&self, pifunctiondiscoveryproviderquery: Param0) -> ::windows::runtime::Result<IFunctionInstanceCollection> {
        let mut result__: <IFunctionInstanceCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pifunctiondiscoveryproviderquery.into_param().abi(), &mut result__).from_abi::<IFunctionInstanceCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn EndQuery(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn InstancePropertyStoreValidateAccess<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext), ::core::mem::transmute(dwstgaccess)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn InstancePropertyStoreOpen<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::runtime::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext), ::core::mem::transmute(dwstgaccess), &mut result__).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn InstancePropertyStoreFlush<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn InstanceQueryService<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, guidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext), ::core::mem::transmute(guidservice), ::core::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn InstanceReleased<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionDiscoveryProvider {
    type Vtable = IFunctionDiscoveryProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdcde394f_1478_4813_a402_f6fb10657222);
}
impl ::core::convert::From<IFunctionDiscoveryProvider> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionDiscoveryProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFunctionDiscoveryProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionDiscoveryProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionDiscoveryProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFunctionDiscoveryProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctiondiscoveryproviderfactory: ::windows::runtime::RawPtr, pifunctiondiscoverynotification: ::windows::runtime::RawPtr, lciduserdefault: u32, pdwstgaccesscapabilities: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctiondiscoveryproviderquery: ::windows::runtime::RawPtr, ppifunctioninstancecollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize, dwstgaccess: u32, ppipropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize, guidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppiunknown: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFunctionDiscoveryProviderFactory(pub ::windows::runtime::IUnknown);
impl IFunctionDiscoveryProviderFactory {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn CreatePropertyStore(&self) -> ::windows::runtime::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::UI::Shell::PropertiesSystem::IPropertyStore>, Param4: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryProvider>>(
        &self,
        pszsubcategory: Param0,
        pszproviderinstanceidentity: Param1,
        iproviderinstancecontext: isize,
        pipropertystore: Param3,
        pifunctiondiscoveryprovider: Param4,
    ) -> ::windows::runtime::Result<IFunctionInstance> {
        let mut result__: <IFunctionInstance as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszsubcategory.into_param().abi(), pszproviderinstanceidentity.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext), pipropertystore.into_param().abi(), pifunctiondiscoveryprovider.into_param().abi(), &mut result__).from_abi::<IFunctionInstance>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn CreateFunctionInstanceCollection(&self) -> ::windows::runtime::Result<IFunctionInstanceCollection> {
        let mut result__: <IFunctionInstanceCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IFunctionInstanceCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionDiscoveryProviderFactory {
    type Vtable = IFunctionDiscoveryProviderFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x86443ff0_1ad5_4e68_a45a_40c2c329de3b);
}
impl ::core::convert::From<IFunctionDiscoveryProviderFactory> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionDiscoveryProviderFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFunctionDiscoveryProviderFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionDiscoveryProviderFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionDiscoveryProviderFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFunctionDiscoveryProviderFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryProviderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppipropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR, iproviderinstancecontext: isize, pipropertystore: ::windows::runtime::RawPtr, pifunctiondiscoveryprovider: ::windows::runtime::RawPtr, ppifunctioninstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppifunctioninstancecollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFunctionDiscoveryProviderQuery(pub ::windows::runtime::IUnknown);
impl IFunctionDiscoveryProviderQuery {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn IsInstanceQuery(&self, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pisinstancequery), ::core::mem::transmute(ppszconstraintvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn IsSubcategoryQuery(&self, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pissubcategoryquery), ::core::mem::transmute(ppszconstraintvalue)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetQueryConstraints(&self) -> ::windows::runtime::Result<IProviderQueryConstraintCollection> {
        let mut result__: <IProviderQueryConstraintCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IProviderQueryConstraintCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetPropertyConstraints(&self) -> ::windows::runtime::Result<IProviderPropertyConstraintCollection> {
        let mut result__: <IProviderPropertyConstraintCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IProviderPropertyConstraintCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionDiscoveryProviderQuery {
    type Vtable = IFunctionDiscoveryProviderQuery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6876ea98_baec_46db_bc20_75a76e267a3a);
}
impl ::core::convert::From<IFunctionDiscoveryProviderQuery> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionDiscoveryProviderQuery) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFunctionDiscoveryProviderQuery> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionDiscoveryProviderQuery) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionDiscoveryProviderQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFunctionDiscoveryProviderQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryProviderQuery_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppiproviderqueryconstraints: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppiproviderpropertyconstraints: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFunctionDiscoveryServiceProvider(pub ::windows::runtime::IUnknown);
impl IFunctionDiscoveryServiceProvider {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>, T: ::windows::runtime::Interface>(&self, pifunctioninstance: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionDiscoveryServiceProvider {
    type Vtable = IFunctionDiscoveryServiceProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4c81ed02_1b04_43f2_a451_69966cbcd1c2);
}
impl ::core::convert::From<IFunctionDiscoveryServiceProvider> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionDiscoveryServiceProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFunctionDiscoveryServiceProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionDiscoveryServiceProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionDiscoveryServiceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFunctionDiscoveryServiceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryServiceProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFunctionInstance(pub ::windows::runtime::IUnknown);
impl IFunctionInstance {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn QueryService(&self, guidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidservice), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobject)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetID(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetProviderInstanceID(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut u16>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn OpenPropertyStore(&self, dwstgaccess: u32) -> ::windows::runtime::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstgaccess), &mut result__).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetCategory(&self, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppszcomemcategory), ::core::mem::transmute(ppszcomemsubcategory)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionInstance {
    type Vtable = IFunctionInstance_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x33591c10_0bed_4f02_b0ab_1530d5533ee9);
}
impl ::core::convert::From<IFunctionInstance> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionInstance) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFunctionInstance> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionInstance) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionInstance {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFunctionInstance {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFunctionInstance> for super::super::System::Com::IServiceProvider {
    fn from(value: IFunctionInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFunctionInstance> for super::super::System::Com::IServiceProvider {
    fn from(value: &IFunctionInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IServiceProvider> for IFunctionInstance {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IServiceProvider> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IServiceProvider> for &IFunctionInstance {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IServiceProvider> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstance_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszcomemidentity: *mut *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszcomemproviderinstanceidentity: *mut *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwstgaccess: u32, ppipropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFunctionInstanceCollection(pub ::windows::runtime::IUnknown);
impl IFunctionInstanceCollection {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn Get<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszinstanceidentity: Param0, pdwindex: *mut u32, ppifunctioninstance: *mut ::core::option::Option<IFunctionInstance>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszinstanceidentity.into_param().abi(), ::core::mem::transmute(pdwindex), ::core::mem::transmute(ppifunctioninstance)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Item(&self, dwindex: u32) -> ::windows::runtime::Result<IFunctionInstance> {
        let mut result__: <IFunctionInstance as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), &mut result__).from_abi::<IFunctionInstance>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pifunctioninstance.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Remove(&self, dwindex: u32) -> ::windows::runtime::Result<IFunctionInstance> {
        let mut result__: <IFunctionInstance as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), &mut result__).from_abi::<IFunctionInstance>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Delete(&self, dwindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn DeleteAll(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionInstanceCollection {
    type Vtable = IFunctionInstanceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf0a3d895_855c_42a2_948d_2f97d450ecb1);
}
impl ::core::convert::From<IFunctionInstanceCollection> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionInstanceCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFunctionInstanceCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionInstanceCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionInstanceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFunctionInstanceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszinstanceidentity: super::super::Foundation::PWSTR, pdwindex: *mut u32, ppifunctioninstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, ppifunctioninstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, ppifunctioninstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFunctionInstanceCollectionQuery(pub ::windows::runtime::IUnknown);
impl IFunctionInstanceCollectionQuery {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn AddQueryConstraint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszconstraintname: Param0, pszconstraintvalue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszconstraintname.into_param().abi(), pszconstraintvalue.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn AddPropertyConstraint(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(pv), ::core::mem::transmute(enumpropertyconstraint)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Execute(&self) -> ::windows::runtime::Result<IFunctionInstanceCollection> {
        let mut result__: <IFunctionInstanceCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IFunctionInstanceCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionInstanceCollectionQuery {
    type Vtable = IFunctionInstanceCollectionQuery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x57cc6fd2_c09a_4289_bb72_25f04142058e);
}
impl ::core::convert::From<IFunctionInstanceCollectionQuery> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionInstanceCollectionQuery) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFunctionInstanceCollectionQuery> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionInstanceCollectionQuery) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionInstanceCollectionQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFunctionInstanceCollectionQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceCollectionQuery_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszconstraintname: super::super::Foundation::PWSTR, pszconstraintvalue: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, enumpropertyconstraint: PropertyConstraint) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppifunctioninstancecollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFunctionInstanceQuery(pub ::windows::runtime::IUnknown);
impl IFunctionInstanceQuery {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Execute(&self) -> ::windows::runtime::Result<IFunctionInstance> {
        let mut result__: <IFunctionInstance as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IFunctionInstance>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionInstanceQuery {
    type Vtable = IFunctionInstanceQuery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6242bc6b_90ec_4b37_bb46_e229fd84ed95);
}
impl ::core::convert::From<IFunctionInstanceQuery> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionInstanceQuery) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFunctionInstanceQuery> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionInstanceQuery) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionInstanceQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFunctionInstanceQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceQuery_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppifunctioninstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPNPXAssociation(pub ::windows::runtime::IUnknown);
impl IPNPXAssociation {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn Associate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszsubcategory: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszsubcategory.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn Unassociate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszsubcategory: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszsubcategory.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn Delete<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszsubcategory: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszsubcategory.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPNPXAssociation {
    type Vtable = IPNPXAssociation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0bd7e521_4da6_42d5_81ba_1981b6b94075);
}
impl ::core::convert::From<IPNPXAssociation> for ::windows::runtime::IUnknown {
    fn from(value: IPNPXAssociation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPNPXAssociation> for ::windows::runtime::IUnknown {
    fn from(value: &IPNPXAssociation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPNPXAssociation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPNPXAssociation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPNPXAssociation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPNPXDeviceAssociation(pub ::windows::runtime::IUnknown);
impl IPNPXDeviceAssociation {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn Associate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryNotification>>(&self, pszsubcategory: Param0, pifunctiondiscoverynotification: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszsubcategory.into_param().abi(), pifunctiondiscoverynotification.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn Unassociate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryNotification>>(&self, pszsubcategory: Param0, pifunctiondiscoverynotification: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszsubcategory.into_param().abi(), pifunctiondiscoverynotification.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn Delete<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryNotification>>(&self, pszsubcategory: Param0, pifunctiondiscoverynotification: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszsubcategory.into_param().abi(), pifunctiondiscoverynotification.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPNPXDeviceAssociation {
    type Vtable = IPNPXDeviceAssociation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xeed366d0_35b8_4fc5_8d20_7e5bd31f6ded);
}
impl ::core::convert::From<IPNPXDeviceAssociation> for ::windows::runtime::IUnknown {
    fn from(value: IPNPXDeviceAssociation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPNPXDeviceAssociation> for ::windows::runtime::IUnknown {
    fn from(value: &IPNPXDeviceAssociation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPNPXDeviceAssociation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPNPXDeviceAssociation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPNPXDeviceAssociation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyStoreCollection(pub ::windows::runtime::IUnknown);
impl IPropertyStoreCollection {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn Get<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszinstanceidentity: Param0, pdwindex: *mut u32, ppipropertystore: *mut ::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszinstanceidentity.into_param().abi(), ::core::mem::transmute(pdwindex), ::core::mem::transmute(ppipropertystore)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn Item(&self, dwindex: u32) -> ::windows::runtime::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), &mut result__).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Shell::PropertiesSystem::IPropertyStore>>(&self, pipropertystore: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pipropertystore.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn Remove(&self, dwindex: u32) -> ::windows::runtime::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), &mut result__).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Delete(&self, dwindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn DeleteAll(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyStoreCollection {
    type Vtable = IPropertyStoreCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd14d9c30_12d2_42d8_bce4_c60c2bb226fa);
}
impl ::core::convert::From<IPropertyStoreCollection> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyStoreCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyStoreCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyStoreCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyStoreCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPropertyStoreCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszinstanceidentity: super::super::Foundation::PWSTR, pdwindex: *mut u32, ppipropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, ppipropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pipropertystore: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pipropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProviderProperties(pub ::windows::runtime::IUnknown);
impl IProviderProperties {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetCount<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetAt<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, dwindex: u32) -> ::windows::runtime::Result<super::super::UI::Shell::PropertiesSystem::PROPERTYKEY> {
        let mut result__: <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext), ::core::mem::transmute(dwindex), &mut result__).from_abi::<super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext), ::core::mem::transmute(key), &mut result__).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext), ::core::mem::transmute(key), ::core::mem::transmute(ppropvar)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IProviderProperties {
    type Vtable = IProviderProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcf986ea6_3b5f_4c5f_b88a_2f8b20ceef17);
}
impl ::core::convert::From<IProviderProperties> for ::windows::runtime::IUnknown {
    fn from(value: IProviderProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProviderProperties> for ::windows::runtime::IUnknown {
    fn from(value: &IProviderProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProviderProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProviderProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProviderPropertyConstraintCollection(pub ::windows::runtime::IUnknown);
impl IProviderPropertyConstraintCollection {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn Get(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(ppropvar), ::core::mem::transmute(pdwpropertyconstraint)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn Item(&self, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pkey), ::core::mem::transmute(ppropvar), ::core::mem::transmute(pdwpropertyconstraint)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn Next(&self, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pkey), ::core::mem::transmute(ppropvar), ::core::mem::transmute(pdwpropertyconstraint)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Skip(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IProviderPropertyConstraintCollection {
    type Vtable = IProviderPropertyConstraintCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf4fae42f_5778_4a13_8540_b5fd8c1398dd);
}
impl ::core::convert::From<IProviderPropertyConstraintCollection> for ::windows::runtime::IUnknown {
    fn from(value: IProviderPropertyConstraintCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProviderPropertyConstraintCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IProviderPropertyConstraintCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProviderPropertyConstraintCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProviderPropertyConstraintCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderPropertyConstraintCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, pdwpropertyconstraint: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, pdwpropertyconstraint: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, pdwpropertyconstraint: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProviderPublishing(pub ::windows::runtime::IUnknown);
impl IProviderPublishing {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn CreateInstance<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: Param1, pszproviderinstanceidentity: Param2) -> ::windows::runtime::Result<IFunctionInstance> {
        let mut result__: <IFunctionInstance as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumvisibilityflags), pszsubcategory.into_param().abi(), pszproviderinstanceidentity.into_param().abi(), &mut result__).from_abi::<IFunctionInstance>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn RemoveInstance<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: Param1, pszproviderinstanceidentity: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumvisibilityflags), pszsubcategory.into_param().abi(), pszproviderinstanceidentity.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IProviderPublishing {
    type Vtable = IProviderPublishing_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcd1b9a04_206c_4a05_a0c8_1635a21a2b7c);
}
impl ::core::convert::From<IProviderPublishing> for ::windows::runtime::IUnknown {
    fn from(value: IProviderPublishing) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProviderPublishing> for ::windows::runtime::IUnknown {
    fn from(value: &IProviderPublishing) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProviderPublishing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProviderPublishing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderPublishing_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR, ppifunctioninstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProviderQueryConstraintCollection(pub ::windows::runtime::IUnknown);
impl IProviderQueryConstraintCollection {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn Get<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszconstraintname: Param0) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszconstraintname.into_param().abi(), &mut result__).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Item(&self, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(ppszconstraintname), ::core::mem::transmute(ppszconstraintvalue)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Next(&self, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppszconstraintname), ::core::mem::transmute(ppszconstraintvalue)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Skip(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IProviderQueryConstraintCollection {
    type Vtable = IProviderQueryConstraintCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9c243e11_3261_4bcd_b922_84a873d460ae);
}
impl ::core::convert::From<IProviderQueryConstraintCollection> for ::windows::runtime::IUnknown {
    fn from(value: IProviderQueryConstraintCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProviderQueryConstraintCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IProviderQueryConstraintCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProviderQueryConstraintCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProviderQueryConstraintCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderQueryConstraintCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszconstraintname: super::super::Foundation::PWSTR, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const MAX_FDCONSTRAINTNAME_LENGTH: u32 = 100u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const MAX_FDCONSTRAINTVALUE_LENGTH: u32 = 1000u32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_Characteristics: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b),
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_ClassCoInstallers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x713d1703_a2e2_49f5_9214_56472ef3da5c),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_ClassInstaller: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_ClassName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_DefaultService: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_DevType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b),
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_Exclusive: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b),
    pid: 28u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_IconPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_LowerFilters: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b),
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_Name: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_NoDisplayClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_NoInstallClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_NoUseClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_PropPageProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_Security: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b),
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_SecuritySDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b),
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_SilentInstall: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceClass_UpperFilters: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b),
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_Address: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 51u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_AlwaysShowDeviceAsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_AssociationArray: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 80u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_BaselineExperienceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 78u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_Category: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 90u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_CategoryGroup_Desc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 94u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_CategoryGroup_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 95u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_Category_Desc_Plural: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 92u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_Category_Desc_Singular: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 91u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_Category_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 93u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_DeviceDescription1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 81u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_DeviceDescription2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 82u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_DeviceFunctionSubRank: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_DiscoveryMethod: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 52u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_ExperienceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 89u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_FriendlyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 12288u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 57u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_InstallInProgress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x83da6326_97a6_4088_9453_a1923f573b29),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_IsAuthenticated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 54u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_IsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 55u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_IsDefaultDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 86u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_IsDeviceUniquelyIdentifiable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 79u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_IsEncrypted: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 53u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_IsLocalMachine: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 70u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_IsMetadataSearchInProgress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 72u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_IsNetworkDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 85u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_IsNotInterestingForDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 74u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_IsNotWorkingProperly: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 83u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_IsPaired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 56u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_IsSharedDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 84u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_IsShowInDisconnectedState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 68u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_Last_Connected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 67u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_Last_Seen: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 66u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_LaunchDeviceStageFromExplorer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 77u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_LaunchDeviceStageOnDeviceConnect: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 76u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 8192u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_MetadataCabinet: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 87u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_MetadataChecksum: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 73u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_MetadataPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 71u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_ModelName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 8194u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_ModelNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 8195u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_PrimaryCategory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 97u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_RequiresPairingElevation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 88u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_RequiresUninstallElevation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 99u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_UnpairUninstall: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 98u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceDisplay_Version: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 65u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterfaceClass_DefaultInterface: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x14c83a99_0b3f_44b7_be4c_a178d3990564),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_ClassGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Enabled: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_FriendlyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_AdditionalSoftwareRequested: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_Address: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 30u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_BIOSVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xeaee7f1d_6a33_44d1_9441_5f46def23198),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_BaseContainerId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 38u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_BusNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_BusRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_BusReportedDeviceDesc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_BusTypeGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_Capabilities: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_Characteristics: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_Children: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_Class: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_ClassGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_CompatibleIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_ConfigFlags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_ContainerId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x8c7ed206_3f8a_4827_b3ab_ae9e1faefc6c),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_DHP_Rebalance_Policy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_DevNodeStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_DevType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_DeviceDesc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_Driver: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_DriverCoInstallers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_DriverDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_DriverDesc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_DriverInfPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_DriverInfSection: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_DriverInfSectionExt: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_DriverLogoLevel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_DriverPropPageProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_DriverProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_DriverRank: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_DriverVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_EjectionRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_EnumeratorName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_Exclusive: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 28u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_FriendlyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_FriendlyNameAttributes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_GenericDriverInstalled: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_HardwareIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_InstallInProgress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x83da6326_97a6_4088_9453_a1923f573b29),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_InstallState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 36u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_InstanceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57),
    pid: 256u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_IsAssociateableByUserAction: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_Legacy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x80497100_8c73_48b9_aad9_ce387e19c56e),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_LegacyBusType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 22u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_LocationInfo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_LocationPaths: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 37u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_LowerFilters: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_ManufacturerAttributes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_MatchingDeviceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_ModelId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_NoConnectSound: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_Numa_Node: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_PDOName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_Parent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_PowerData: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 32u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_PowerRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_PresenceNotForDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_ProblemCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_RemovalPolicy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 33u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_RemovalPolicyDefault: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 34u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_RemovalPolicyOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 35u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_RemovalRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_Reported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x80497100_8c73_48b9_aad9_ce387e19c56e),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_ResourcePickerExceptions: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_ResourcePickerTags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_SafeRemovalRequired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xafd97640_86a3_4210_b67c_289c41aabe55),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_SafeRemovalRequiredOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xafd97640_86a3_4210_b67c_289c41aabe55),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_Security: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_SecuritySDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_Service: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_Siblings: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_SignalStrength: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_TransportRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_UINumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_UINumberDescFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 31u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_UpperFilters: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DrvPkg_BrandingIcon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DrvPkg_DetailedDescription: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DrvPkg_DocumentationLink: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DrvPkg_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DrvPkg_Model: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DrvPkg_VendorWebSite: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FunctionInstance: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x08c0c253_a154_4746_9005_82de5317148b),
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Hardware_Devinst: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953),
    pid: 4097u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Hardware_DisplayAttribute: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Hardware_DriverDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Hardware_DriverProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Hardware_DriverVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Hardware_Function: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953),
    pid: 4099u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Hardware_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Hardware_Image: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953),
    pid: 4098u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Hardware_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Hardware_Model: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Hardware_Name: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Hardware_SerialNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Hardware_ShellAttributes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953),
    pid: 4100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Hardware_Status: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953),
    pid: 4096u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Numa_Proximity_Domain: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2),
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_Associated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_Category_Desc_NonPlural: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 12304u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_CompactSignature: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 28674u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_CompatibleTypes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_DeviceCategory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 12292u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_DeviceCategory_Desc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 12293u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_DeviceCertHash: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 28675u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_DomainName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 20480u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_FirmwareVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 12289u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_GlobalIdentity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 4096u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 4101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_IPBusEnumerated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 28688u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_InstallState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_Installable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd),
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_IpAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 12297u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_ManufacturerUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 8193u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_MetadataVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 4100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_ModelUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 8196u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_NetworkInterfaceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 12296u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_NetworkInterfaceLuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 12295u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_PhysicalAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 12294u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_PresentationUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 8198u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_RemoteAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 4102u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_Removable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 28672u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_RootProxy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 4103u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_Scopes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 4098u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_SecureChannel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 28673u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_SerialNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 12290u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_ServiceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 16384u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_ServiceControlUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 16388u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_ServiceDescUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 16389u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_ServiceEventSubUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 16390u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_ServiceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 16385u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_ServiceTypes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 16386u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_ShareName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 20482u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_Types: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 4097u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_Upc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 8197u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PNPX_XAddrs: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 4099u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Pairing_IsWifiOnlyDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Pairing_ListItemDefault: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Pairing_ListItemDescription: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Pairing_ListItemIcon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Pairing_ListItemText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc),
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_SSDP_AltLocationInfo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 24576u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_SSDP_DevLifeTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 24577u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_SSDP_NetworkInterface: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd),
    pid: 24578u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WCN_AssocState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x88190b88_4684_11da_a26a_0002b3988e81),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WCN_AuthType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x88190b82_4684_11da_a26a_0002b3988e81),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WCN_ConfigError: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WCN_ConfigMethods: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x88190b85_4684_11da_a26a_0002b3988e81),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WCN_ConfigState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WCN_ConnType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x88190b84_4684_11da_a26a_0002b3988e81),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WCN_DevicePasswordId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WCN_EncryptType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x88190b83_4684_11da_a26a_0002b3988e81),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WCN_OSVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WCN_RegistrarType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x88190b8b_4684_11da_a26a_0002b3988e81),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WCN_RequestType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x88190b81_4684_11da_a26a_0002b3988e81),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WCN_RfBand: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x88190b87_4684_11da_a26a_0002b3988e81),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WCN_VendorExtension: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x88190b8a_4684_11da_a26a_0002b3988e81),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WCN_Version: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0x88190b80_4684_11da_a26a_0002b3988e81),
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WNET_Comment: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WNET_DisplayType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WNET_LocalName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WNET_Provider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WNET_RemoteName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WNET_Scope: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab),
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WNET_Type: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_WNET_Usage: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab),
    pid: 4u32,
};
pub const PNPXAssociation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcee8ccc9_4f6b_4469_a235_5a22869eef03);
pub const PNPXPairingHandler: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb8a27942_ade7_4085_aa6e_4fadc7ada1ef);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const PNPX_INSTALLSTATE_FAILED: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const PNPX_INSTALLSTATE_INSTALLED: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const PNPX_INSTALLSTATE_INSTALLING: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const PNPX_INSTALLSTATE_NOTINSTALLED: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PropertyConstraint(pub i32);
pub const QC_EQUALS: PropertyConstraint = PropertyConstraint(0i32);
pub const QC_NOTEQUAL: PropertyConstraint = PropertyConstraint(1i32);
pub const QC_LESSTHAN: PropertyConstraint = PropertyConstraint(2i32);
pub const QC_LESSTHANOREQUAL: PropertyConstraint = PropertyConstraint(3i32);
pub const QC_GREATERTHAN: PropertyConstraint = PropertyConstraint(4i32);
pub const QC_GREATERTHANOREQUAL: PropertyConstraint = PropertyConstraint(5i32);
pub const QC_STARTSWITH: PropertyConstraint = PropertyConstraint(6i32);
pub const QC_EXISTS: PropertyConstraint = PropertyConstraint(7i32);
pub const QC_DOESNOTEXIST: PropertyConstraint = PropertyConstraint(8i32);
pub const QC_CONTAINS: PropertyConstraint = PropertyConstraint(9i32);
impl ::core::convert::From<i32> for PropertyConstraint {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PropertyConstraint {
    type Abi = Self;
}
pub const PropertyStore: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe4796550_df61_448b_9193_13fc1341b163);
pub const PropertyStoreCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xedd36029_d753_4862_aa5b_5bccad2a4d29);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct QueryCategoryType(pub i32);
pub const QCT_PROVIDER: QueryCategoryType = QueryCategoryType(0i32);
pub const QCT_LAYERED: QueryCategoryType = QueryCategoryType(1i32);
impl ::core::convert::From<i32> for QueryCategoryType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for QueryCategoryType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct QueryUpdateAction(pub i32);
pub const QUA_ADD: QueryUpdateAction = QueryUpdateAction(0i32);
pub const QUA_REMOVE: QueryUpdateAction = QueryUpdateAction(1i32);
pub const QUA_CHANGE: QueryUpdateAction = QueryUpdateAction(2i32);
impl ::core::convert::From<i32> for QueryUpdateAction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for QueryUpdateAction {
    type Abi = Self;
}
pub const SID_DeviceDisplayStatusManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf59aa553_8309_46ca_9736_1ac3c62d6031);
pub const SID_EnumDeviceFunction: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x13e0e9e2_c3fa_4e3c_906e_64502fa4dc95);
pub const SID_EnumInterface: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x40eab0b9_4d7f_4b53_a334_1581dd9041f4);
pub const SID_FDPairingHandler: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x383b69fa_5486_49da_91f5_d63c24c8e9d0);
pub const SID_FunctionDiscoveryProviderRefresh: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2b4cbdc9_31c4_40d4_a62d_772aa174ed52);
pub const SID_PNPXAssociation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcee8ccc9_4f6b_4469_a235_5a22869eef03);
pub const SID_PNPXPropertyStore: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa86530b1_542f_439f_b71c_b0756b13677a);
pub const SID_PNPXServiceCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x439e80ee_a217_4712_9fa6_deabd9c2a727);
pub const SID_PnpProvider: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8101368e_cabb_4426_acff_96c410812000);
pub const SID_UPnPActivator: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0d0d66eb_cf74_4164_b52f_08344672dd46);
pub const SID_UninstallDeviceFunction: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc920566e_5671_4496_8025_bf0b89bd44cd);
pub const SID_UnpairProvider: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x89a502fc_857b_4698_a0b7_027192002f9e);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SystemVisibilityFlags(pub i32);
pub const SVF_SYSTEM: SystemVisibilityFlags = SystemVisibilityFlags(0i32);
pub const SVF_USER: SystemVisibilityFlags = SystemVisibilityFlags(1i32);
impl ::core::convert::From<i32> for SystemVisibilityFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SystemVisibilityFlags {
    type Abi = Self;
}
