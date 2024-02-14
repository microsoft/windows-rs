::windows_core::imp::com_interface!(IDontSupportEventSubscription, IDontSupportEventSubscription_Vtbl, 0x784121f1_62a6_4b89_855f_d65f296de83a);
::windows_core::imp::interface_hierarchy!(IDontSupportEventSubscription, ::windows_core::IUnknown);
impl IDontSupportEventSubscription {}
#[repr(C)]
pub struct IDontSupportEventSubscription_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
}
::windows_core::imp::com_interface!(IEnumEventObject, IEnumEventObject_Vtbl, 0xf4a07d63_2e25_11d1_9964_00c04fbbb345);
::windows_core::imp::interface_hierarchy!(IEnumEventObject, ::windows_core::IUnknown);
impl IEnumEventObject {
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumEventObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, ppinterface: &mut [::core::option::Option<::windows_core::IUnknown>], cretelem: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppinterface.len().try_into().unwrap(), ::core::mem::transmute(ppinterface.as_ptr()), cretelem).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Skip(&self, cskipelem: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), cskipelem).ok()
    }
}
#[repr(C)]
pub struct IEnumEventObject_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut *mut ::core::ffi::c_void, *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEventClass, IEventClass_Vtbl, 0xfb2b72a0_7a68_11d1_88f9_0080c7d771bf);
::windows_core::imp::interface_hierarchy!(IEventClass, ::windows_core::IUnknown, super::IDispatch);
impl IEventClass {
    pub unsafe fn EventClassID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EventClassID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEventClassID<P0>(&self, bstreventclassid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetEventClassID)(::windows_core::Interface::as_raw(self), bstreventclassid.into_param().abi()).ok()
    }
    pub unsafe fn EventClassName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EventClassName)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEventClassName<P0>(&self, bstreventclassname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetEventClassName)(::windows_core::Interface::as_raw(self), bstreventclassname.into_param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OwnerSID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOwnerSID<P0>(&self, bstrownersid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOwnerSID)(::windows_core::Interface::as_raw(self), bstrownersid.into_param().abi()).ok()
    }
    pub unsafe fn FiringInterfaceID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FiringInterfaceID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFiringInterfaceID<P0>(&self, bstrfiringinterfaceid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFiringInterfaceID)(::windows_core::Interface::as_raw(self), bstrfiringinterfaceid.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn CustomConfigCLSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CustomConfigCLSID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCustomConfigCLSID<P0>(&self, bstrcustomconfigclsid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCustomConfigCLSID)(::windows_core::Interface::as_raw(self), bstrcustomconfigclsid.into_param().abi()).ok()
    }
    pub unsafe fn TypeLib(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TypeLib)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTypeLib<P0>(&self, bstrtypelib: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTypeLib)(::windows_core::Interface::as_raw(self), bstrtypelib.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventClass_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub EventClassID: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetEventClassID: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub EventClassName: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetEventClassName: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FiringInterfaceID: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetFiringInterfaceID: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CustomConfigCLSID: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetCustomConfigCLSID: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TypeLib: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTypeLib: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEventClass2, IEventClass2_Vtbl, 0xfb2b72a1_7a68_11d1_88f9_0080c7d771bf);
::windows_core::imp::interface_hierarchy!(IEventClass2, ::windows_core::IUnknown, super::IDispatch, IEventClass);
impl IEventClass2 {
    pub unsafe fn EventClassID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EventClassID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEventClassID<P0>(&self, bstreventclassid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetEventClassID)(::windows_core::Interface::as_raw(self), bstreventclassid.into_param().abi()).ok()
    }
    pub unsafe fn EventClassName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EventClassName)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEventClassName<P0>(&self, bstreventclassname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetEventClassName)(::windows_core::Interface::as_raw(self), bstreventclassname.into_param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.OwnerSID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOwnerSID<P0>(&self, bstrownersid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetOwnerSID)(::windows_core::Interface::as_raw(self), bstrownersid.into_param().abi()).ok()
    }
    pub unsafe fn FiringInterfaceID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FiringInterfaceID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFiringInterfaceID<P0>(&self, bstrfiringinterfaceid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetFiringInterfaceID)(::windows_core::Interface::as_raw(self), bstrfiringinterfaceid.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn CustomConfigCLSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CustomConfigCLSID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCustomConfigCLSID<P0>(&self, bstrcustomconfigclsid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetCustomConfigCLSID)(::windows_core::Interface::as_raw(self), bstrcustomconfigclsid.into_param().abi()).ok()
    }
    pub unsafe fn TypeLib(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TypeLib)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTypeLib<P0>(&self, bstrtypelib: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTypeLib)(::windows_core::Interface::as_raw(self), bstrtypelib.into_param().abi()).ok()
    }
    pub unsafe fn PublisherID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PublisherID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPublisherID<P0>(&self, bstrpublisherid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPublisherID)(::windows_core::Interface::as_raw(self), bstrpublisherid.into_param().abi()).ok()
    }
    pub unsafe fn MultiInterfacePublisherFilterCLSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MultiInterfacePublisherFilterCLSID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMultiInterfacePublisherFilterCLSID<P0>(&self, bstrpubfilclsid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetMultiInterfacePublisherFilterCLSID)(::windows_core::Interface::as_raw(self), bstrpubfilclsid.into_param().abi()).ok()
    }
    pub unsafe fn AllowInprocActivation(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowInprocActivation)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowInprocActivation<P0>(&self, fallowinprocactivation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowInprocActivation)(::windows_core::Interface::as_raw(self), fallowinprocactivation.into_param().abi()).ok()
    }
    pub unsafe fn FireInParallel(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FireInParallel)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFireInParallel<P0>(&self, ffireinparallel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetFireInParallel)(::windows_core::Interface::as_raw(self), ffireinparallel.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventClass2_Vtbl {
    pub base__: IEventClass_Vtbl,
    pub PublisherID: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub MultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetMultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AllowInprocActivation: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetAllowInprocActivation: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub FireInParallel: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetFireInParallel: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEventControl, IEventControl_Vtbl, 0x0343e2f4_86f6_11d1_b760_00c04fb926af);
::windows_core::imp::interface_hierarchy!(IEventControl, ::windows_core::IUnknown, super::IDispatch);
impl IEventControl {
    pub unsafe fn SetPublisherFilter<P0, P1>(&self, methodname: P0, ppublisherfilter: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<IPublisherFilter>,
    {
        (::windows_core::Interface::vtable(self).SetPublisherFilter)(::windows_core::Interface::as_raw(self), methodname.into_param().abi(), ppublisherfilter.into_param().abi()).ok()
    }
    pub unsafe fn AllowInprocActivation(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowInprocActivation)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowInprocActivation<P0>(&self, fallowinprocactivation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowInprocActivation)(::windows_core::Interface::as_raw(self), fallowinprocactivation.into_param().abi()).ok()
    }
    pub unsafe fn GetSubscriptions<P0, P1>(&self, methodname: P0, optionalcriteria: P1, optionalerrorindex: *const i32) -> ::windows_core::Result<IEventObjectCollection>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSubscriptions)(::windows_core::Interface::as_raw(self), methodname.into_param().abi(), optionalcriteria.into_param().abi(), optionalerrorindex, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDefaultQuery<P0, P1>(&self, methodname: P0, criteria: P1) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SetDefaultQuery)(::windows_core::Interface::as_raw(self), methodname.into_param().abi(), criteria.into_param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IEventControl_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SetPublisherFilter: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AllowInprocActivation: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetAllowInprocActivation: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetSubscriptions: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, *const i32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDefaultQuery: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, *mut i32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEventObjectChange, IEventObjectChange_Vtbl, 0xf4a07d70_2e25_11d1_9964_00c04fbbb345);
::windows_core::imp::interface_hierarchy!(IEventObjectChange, ::windows_core::IUnknown);
impl IEventObjectChange {
    pub unsafe fn ChangedSubscription<P0>(&self, changetype: EOC_ChangeType, bstrsubscriptionid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ChangedSubscription)(::windows_core::Interface::as_raw(self), changetype, bstrsubscriptionid.into_param().abi()).ok()
    }
    pub unsafe fn ChangedEventClass<P0>(&self, changetype: EOC_ChangeType, bstreventclassid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ChangedEventClass)(::windows_core::Interface::as_raw(self), changetype, bstreventclassid.into_param().abi()).ok()
    }
    pub unsafe fn ChangedPublisher<P0>(&self, changetype: EOC_ChangeType, bstrpublisherid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ChangedPublisher)(::windows_core::Interface::as_raw(self), changetype, bstrpublisherid.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventObjectChange_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ChangedSubscription: unsafe extern "system" fn(*mut ::core::ffi::c_void, EOC_ChangeType, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ChangedEventClass: unsafe extern "system" fn(*mut ::core::ffi::c_void, EOC_ChangeType, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ChangedPublisher: unsafe extern "system" fn(*mut ::core::ffi::c_void, EOC_ChangeType, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEventObjectChange2, IEventObjectChange2_Vtbl, 0x7701a9c3_bd68_438f_83e0_67bf4f53a422);
::windows_core::imp::interface_hierarchy!(IEventObjectChange2, ::windows_core::IUnknown);
impl IEventObjectChange2 {
    pub unsafe fn ChangedSubscription(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ChangedSubscription)(::windows_core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn ChangedEventClass(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ChangedEventClass)(::windows_core::Interface::as_raw(self), pinfo).ok()
    }
}
#[repr(C)]
pub struct IEventObjectChange2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ChangedSubscription: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const COMEVENTSYSCHANGEINFO) -> ::windows_core::HRESULT,
    pub ChangedEventClass: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const COMEVENTSYSCHANGEINFO) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEventObjectCollection, IEventObjectCollection_Vtbl, 0xf89ac270_d4eb_11d1_b682_00805fc79216);
::windows_core::imp::interface_hierarchy!(IEventObjectCollection, ::windows_core::IUnknown, super::IDispatch);
impl IEventObjectCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Item<P0>(&self, objectid: P0) -> ::windows_core::Result<::windows_core::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), objectid.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NewEnum(&self) -> ::windows_core::Result<IEnumEventObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Add<P0>(&self, item: *const ::windows_core::VARIANT, objectid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(item), objectid.into_param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, objectid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), objectid.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventObjectCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT,
    pub NewEnum: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut i32) -> ::windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::std::mem::MaybeUninit<::windows_core::VARIANT>, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEventProperty, IEventProperty_Vtbl, 0xda538ee2_f4de_11d1_b6bb_00805fc79216);
::windows_core::imp::interface_hierarchy!(IEventProperty, ::windows_core::IUnknown, super::IDispatch);
impl IEventProperty {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, propertyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), propertyname.into_param().abi()).ok()
    }
    pub unsafe fn Value(&self) -> ::windows_core::Result<::windows_core::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Value)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue(&self, propertyvalue: *const ::windows_core::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propertyvalue)).ok()
    }
}
#[repr(C)]
pub struct IEventProperty_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEventPublisher, IEventPublisher_Vtbl, 0xe341516b_2e32_11d1_9964_00c04fbbb345);
::windows_core::imp::interface_hierarchy!(IEventPublisher, ::windows_core::IUnknown, super::IDispatch);
impl IEventPublisher {
    pub unsafe fn PublisherID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PublisherID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPublisherID<P0>(&self, bstrpublisherid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPublisherID)(::windows_core::Interface::as_raw(self), bstrpublisherid.into_param().abi()).ok()
    }
    pub unsafe fn PublisherName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PublisherName)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPublisherName<P0>(&self, bstrpublishername: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPublisherName)(::windows_core::Interface::as_raw(self), bstrpublishername.into_param().abi()).ok()
    }
    pub unsafe fn PublisherType(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PublisherType)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPublisherType<P0>(&self, bstrpublishertype: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPublisherType)(::windows_core::Interface::as_raw(self), bstrpublishertype.into_param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OwnerSID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOwnerSID<P0>(&self, bstrownersid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOwnerSID)(::windows_core::Interface::as_raw(self), bstrownersid.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn GetDefaultProperty<P0>(&self, bstrpropertyname: P0) -> ::windows_core::Result<::windows_core::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PutDefaultProperty<P0>(&self, bstrpropertyname: P0, propertyvalue: *const ::windows_core::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).PutDefaultProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), ::core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn RemoveDefaultProperty<P0>(&self, bstrpropertyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RemoveDefaultProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi()).ok()
    }
    pub unsafe fn GetDefaultPropertyCollection(&self) -> ::windows_core::Result<IEventObjectCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultPropertyCollection)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEventPublisher_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub PublisherID: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PublisherName: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPublisherName: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PublisherType: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPublisherType: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetDefaultProperty: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT,
    pub PutDefaultProperty: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, *const ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT,
    pub RemoveDefaultProperty: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetDefaultPropertyCollection: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEventSubscription, IEventSubscription_Vtbl, 0x4a6b0e15_2e38_11d1_9965_00c04fbbb345);
::windows_core::imp::interface_hierarchy!(IEventSubscription, ::windows_core::IUnknown, super::IDispatch);
impl IEventSubscription {
    pub unsafe fn SubscriptionID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SubscriptionID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSubscriptionID<P0>(&self, bstrsubscriptionid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSubscriptionID)(::windows_core::Interface::as_raw(self), bstrsubscriptionid.into_param().abi()).ok()
    }
    pub unsafe fn SubscriptionName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SubscriptionName)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSubscriptionName<P0>(&self, bstrsubscriptionname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSubscriptionName)(::windows_core::Interface::as_raw(self), bstrsubscriptionname.into_param().abi()).ok()
    }
    pub unsafe fn PublisherID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PublisherID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPublisherID<P0>(&self, bstrpublisherid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPublisherID)(::windows_core::Interface::as_raw(self), bstrpublisherid.into_param().abi()).ok()
    }
    pub unsafe fn EventClassID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EventClassID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEventClassID<P0>(&self, bstreventclassid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetEventClassID)(::windows_core::Interface::as_raw(self), bstreventclassid.into_param().abi()).ok()
    }
    pub unsafe fn MethodName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MethodName)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMethodName<P0>(&self, bstrmethodname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetMethodName)(::windows_core::Interface::as_raw(self), bstrmethodname.into_param().abi()).ok()
    }
    pub unsafe fn SubscriberCLSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SubscriberCLSID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSubscriberCLSID<P0>(&self, bstrsubscriberclsid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSubscriberCLSID)(::windows_core::Interface::as_raw(self), bstrsubscriberclsid.into_param().abi()).ok()
    }
    pub unsafe fn SubscriberInterface(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SubscriberInterface)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSubscriberInterface<P0>(&self, psubscriberinterface: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).SetSubscriberInterface)(::windows_core::Interface::as_raw(self), psubscriberinterface.into_param().abi()).ok()
    }
    pub unsafe fn PerUser(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PerUser)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPerUser<P0>(&self, fperuser: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetPerUser)(::windows_core::Interface::as_raw(self), fperuser.into_param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OwnerSID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOwnerSID<P0>(&self, bstrownersid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOwnerSID)(::windows_core::Interface::as_raw(self), bstrownersid.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnabled<P0>(&self, fenabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), fenabled.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn MachineName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MachineName)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMachineName<P0>(&self, bstrmachinename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetMachineName)(::windows_core::Interface::as_raw(self), bstrmachinename.into_param().abi()).ok()
    }
    pub unsafe fn GetPublisherProperty<P0>(&self, bstrpropertyname: P0) -> ::windows_core::Result<::windows_core::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPublisherProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PutPublisherProperty<P0>(&self, bstrpropertyname: P0, propertyvalue: *const ::windows_core::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).PutPublisherProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), ::core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn RemovePublisherProperty<P0>(&self, bstrpropertyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RemovePublisherProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi()).ok()
    }
    pub unsafe fn GetPublisherPropertyCollection(&self) -> ::windows_core::Result<IEventObjectCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPublisherPropertyCollection)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSubscriberProperty<P0>(&self, bstrpropertyname: P0) -> ::windows_core::Result<::windows_core::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSubscriberProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PutSubscriberProperty<P0>(&self, bstrpropertyname: P0, propertyvalue: *const ::windows_core::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).PutSubscriberProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), ::core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn RemoveSubscriberProperty<P0>(&self, bstrpropertyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RemoveSubscriberProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi()).ok()
    }
    pub unsafe fn GetSubscriberPropertyCollection(&self) -> ::windows_core::Result<IEventObjectCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSubscriberPropertyCollection)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InterfaceID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InterfaceID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetInterfaceID<P0>(&self, bstrinterfaceid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetInterfaceID)(::windows_core::Interface::as_raw(self), bstrinterfaceid.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventSubscription_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SubscriptionID: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSubscriptionID: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SubscriptionName: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSubscriptionName: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PublisherID: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub EventClassID: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetEventClassID: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub MethodName: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetMethodName: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SubscriberCLSID: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSubscriberCLSID: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SubscriberInterface: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSubscriberInterface: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PerUser: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetPerUser: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub MachineName: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetMachineName: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetPublisherProperty: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT,
    pub PutPublisherProperty: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, *const ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT,
    pub RemovePublisherProperty: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetPublisherPropertyCollection: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSubscriberProperty: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT,
    pub PutSubscriberProperty: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, *const ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT,
    pub RemoveSubscriberProperty: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetSubscriberPropertyCollection: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InterfaceID: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetInterfaceID: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEventSystem, IEventSystem_Vtbl, 0x4e14fb9f_2e22_11d1_9964_00c04fbbb345);
::windows_core::imp::interface_hierarchy!(IEventSystem, ::windows_core::IUnknown, super::IDispatch);
impl IEventSystem {
    pub unsafe fn Query<P0, P1>(&self, progid: P0, querycriteria: P1, errorindex: *mut i32) -> ::windows_core::Result<::windows_core::IUnknown>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Query)(::windows_core::Interface::as_raw(self), progid.into_param().abi(), querycriteria.into_param().abi(), errorindex, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Store<P0, P1>(&self, progid: P0, pinterface: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).Store)(::windows_core::Interface::as_raw(self), progid.into_param().abi(), pinterface.into_param().abi()).ok()
    }
    pub unsafe fn Remove<P0, P1>(&self, progid: P0, querycriteria: P1) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), progid.into_param().abi(), querycriteria.into_param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn EventObjectChangeEventClassID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EventObjectChangeEventClassID)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryS<P0, P1>(&self, progid: P0, querycriteria: P1) -> ::windows_core::Result<::windows_core::IUnknown>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueryS)(::windows_core::Interface::as_raw(self), progid.into_param().abi(), querycriteria.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveS<P0, P1>(&self, progid: P0, querycriteria: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RemoveS)(::windows_core::Interface::as_raw(self), progid.into_param().abi(), querycriteria.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventSystem_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Query: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, *mut i32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Store: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, *mut i32) -> ::windows_core::HRESULT,
    pub EventObjectChangeEventClassID: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub QueryS: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveS: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IFiringControl, IFiringControl_Vtbl, 0xe0498c93_4efe_11d1_9971_00c04fbbb345);
::windows_core::imp::interface_hierarchy!(IFiringControl, ::windows_core::IUnknown, super::IDispatch);
impl IFiringControl {
    pub unsafe fn FireSubscription<P0>(&self, subscription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IEventSubscription>,
    {
        (::windows_core::Interface::vtable(self).FireSubscription)(::windows_core::Interface::as_raw(self), subscription.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IFiringControl_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub FireSubscription: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMultiInterfaceEventControl, IMultiInterfaceEventControl_Vtbl, 0x0343e2f5_86f6_11d1_b760_00c04fb926af);
::windows_core::imp::interface_hierarchy!(IMultiInterfaceEventControl, ::windows_core::IUnknown);
impl IMultiInterfaceEventControl {
    pub unsafe fn SetMultiInterfacePublisherFilter<P0>(&self, classfilter: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMultiInterfacePublisherFilter>,
    {
        (::windows_core::Interface::vtable(self).SetMultiInterfacePublisherFilter)(::windows_core::Interface::as_raw(self), classfilter.into_param().abi()).ok()
    }
    pub unsafe fn GetSubscriptions<P0, P1>(&self, eventiid: *const ::windows_core::GUID, bstrmethodname: P0, optionalcriteria: P1, optionalerrorindex: *const i32) -> ::windows_core::Result<IEventObjectCollection>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSubscriptions)(::windows_core::Interface::as_raw(self), eventiid, bstrmethodname.into_param().abi(), optionalcriteria.into_param().abi(), optionalerrorindex, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDefaultQuery<P0, P1>(&self, eventiid: *const ::windows_core::GUID, bstrmethodname: P0, bstrcriteria: P1) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SetDefaultQuery)(::windows_core::Interface::as_raw(self), eventiid, bstrmethodname.into_param().abi(), bstrcriteria.into_param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn AllowInprocActivation(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowInprocActivation)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowInprocActivation<P0>(&self, fallowinprocactivation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowInprocActivation)(::windows_core::Interface::as_raw(self), fallowinprocactivation.into_param().abi()).ok()
    }
    pub unsafe fn FireInParallel(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FireInParallel)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFireInParallel<P0>(&self, ffireinparallel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetFireInParallel)(::windows_core::Interface::as_raw(self), ffireinparallel.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMultiInterfaceEventControl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetMultiInterfacePublisherFilter: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSubscriptions: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::windows_core::GUID, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, *const i32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDefaultQuery: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::windows_core::GUID, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, *mut i32) -> ::windows_core::HRESULT,
    pub AllowInprocActivation: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetAllowInprocActivation: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub FireInParallel: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetFireInParallel: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMultiInterfacePublisherFilter, IMultiInterfacePublisherFilter_Vtbl, 0x465e5cc1_7b26_11d1_88fb_0080c7d771bf);
::windows_core::imp::interface_hierarchy!(IMultiInterfacePublisherFilter, ::windows_core::IUnknown);
impl IMultiInterfacePublisherFilter {
    pub unsafe fn Initialize<P0>(&self, peic: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMultiInterfaceEventControl>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), peic.into_param().abi()).ok()
    }
    pub unsafe fn PrepareToFire<P0, P1>(&self, iid: *const ::windows_core::GUID, methodname: P0, firingcontrol: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<IFiringControl>,
    {
        (::windows_core::Interface::vtable(self).PrepareToFire)(::windows_core::Interface::as_raw(self), iid, methodname.into_param().abi(), firingcontrol.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMultiInterfacePublisherFilter_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PrepareToFire: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::windows_core::GUID, ::std::mem::MaybeUninit<::windows_core::BSTR>, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IPublisherFilter, IPublisherFilter_Vtbl, 0x465e5cc0_7b26_11d1_88fb_0080c7d771bf);
::windows_core::imp::interface_hierarchy!(IPublisherFilter, ::windows_core::IUnknown);
impl IPublisherFilter {
    pub unsafe fn Initialize<P0, P1>(&self, methodname: P0, dispuserdefined: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), methodname.into_param().abi(), dispuserdefined.into_param().abi()).ok()
    }
    pub unsafe fn PrepareToFire<P0, P1>(&self, methodname: P0, firingcontrol: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<IFiringControl>,
    {
        (::windows_core::Interface::vtable(self).PrepareToFire)(::windows_core::Interface::as_raw(self), methodname.into_param().abi(), firingcontrol.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IPublisherFilter_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PrepareToFire: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
pub const CEventClass: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcdbec9c0_7a68_11d1_88f9_0080c7d771bf);
pub const CEventPublisher: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab944620_79c6_11d1_88f9_0080c7d771bf);
pub const CEventSubscription: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7542e960_79c7_11d1_88f9_0080c7d771bf);
pub const CEventSystem: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e14fba2_2e22_11d1_9964_00c04fbbb345);
pub const EOC_DeletedObject: EOC_ChangeType = EOC_ChangeType(2i32);
pub const EOC_ModifiedObject: EOC_ChangeType = EOC_ChangeType(1i32);
pub const EOC_NewObject: EOC_ChangeType = EOC_ChangeType(0i32);
pub const EventObjectChange: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0565000_9df4_11d1_a281_00c04fca0aa7);
pub const EventObjectChange2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb07bacd_cd56_4e63_a8ff_cbf0355fb9f4);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct EOC_ChangeType(pub i32);
impl ::windows_core::TypeKind for EOC_ChangeType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EOC_ChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EOC_ChangeType").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct COMEVENTSYSCHANGEINFO {
    pub cbSize: u32,
    pub changeType: EOC_ChangeType,
    pub objectId: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub partitionId: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub applicationId: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub reserved: [::windows_core::GUID; 10],
}
impl ::core::clone::Clone for COMEVENTSYSCHANGEINFO {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for COMEVENTSYSCHANGEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMEVENTSYSCHANGEINFO").field("cbSize", &self.cbSize).field("changeType", &self.changeType).field("objectId", &self.objectId).field("partitionId", &self.partitionId).field("applicationId", &self.applicationId).field("reserved", &self.reserved).finish()
    }
}
impl ::windows_core::TypeKind for COMEVENTSYSCHANGEINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for COMEVENTSYSCHANGEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.changeType == other.changeType && self.objectId == other.objectId && self.partitionId == other.partitionId && self.applicationId == other.applicationId && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for COMEVENTSYSCHANGEINFO {}
impl ::core::default::Default for COMEVENTSYSCHANGEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
