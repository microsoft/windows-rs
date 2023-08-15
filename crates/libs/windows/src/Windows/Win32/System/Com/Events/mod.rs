#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IDontSupportEventSubscription(::windows_core::IUnknown);
impl IDontSupportEventSubscription {}
::windows_core::imp::interface_hierarchy!(IDontSupportEventSubscription, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IDontSupportEventSubscription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDontSupportEventSubscription {}
impl ::core::fmt::Debug for IDontSupportEventSubscription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDontSupportEventSubscription").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDontSupportEventSubscription {
    type Vtable = IDontSupportEventSubscription_Vtbl;
}
impl ::core::clone::Clone for IDontSupportEventSubscription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDontSupportEventSubscription {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x784121f1_62a6_4b89_855f_d65f296de83a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDontSupportEventSubscription_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEnumEventObject(::windows_core::IUnknown);
impl IEnumEventObject {
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumEventObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, ppinterface: &mut [::core::option::Option<::windows_core::IUnknown>], cretelem: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppinterface.len() as _, ::core::mem::transmute(ppinterface.as_ptr()), cretelem).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, cskipelem: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), cskipelem).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IEnumEventObject, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IEnumEventObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumEventObject {}
impl ::core::fmt::Debug for IEnumEventObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumEventObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumEventObject {
    type Vtable = IEnumEventObject_Vtbl;
}
impl ::core::clone::Clone for IEnumEventObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEnumEventObject {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4a07d63_2e25_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumEventObject_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creqelem: u32, ppinterface: *mut *mut ::core::ffi::c_void, cretelem: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cskipelem: u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventClass(::windows_core::IUnknown);
impl IEventClass {
    pub unsafe fn EventClassID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EventClassID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEventClassID<P0>(&self, bstreventclassid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetEventClassID)(::windows_core::Interface::as_raw(self), bstreventclassid.into_param().abi()).ok()
    }
    pub unsafe fn EventClassName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EventClassName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEventClassName<P0>(&self, bstreventclassname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetEventClassName)(::windows_core::Interface::as_raw(self), bstreventclassname.into_param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OwnerSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOwnerSID<P0>(&self, bstrownersid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOwnerSID)(::windows_core::Interface::as_raw(self), bstrownersid.into_param().abi()).ok()
    }
    pub unsafe fn FiringInterfaceID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FiringInterfaceID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFiringInterfaceID<P0>(&self, bstrfiringinterfaceid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFiringInterfaceID)(::windows_core::Interface::as_raw(self), bstrfiringinterfaceid.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn CustomConfigCLSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CustomConfigCLSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCustomConfigCLSID<P0>(&self, bstrcustomconfigclsid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCustomConfigCLSID)(::windows_core::Interface::as_raw(self), bstrcustomconfigclsid.into_param().abi()).ok()
    }
    pub unsafe fn TypeLib(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TypeLib)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTypeLib<P0>(&self, bstrtypelib: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTypeLib)(::windows_core::Interface::as_raw(self), bstrtypelib.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IEventClass, ::windows_core::IUnknown, super::IDispatch);
impl ::core::cmp::PartialEq for IEventClass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventClass {}
impl ::core::fmt::Debug for IEventClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventClass").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventClass {
    type Vtable = IEventClass_Vtbl;
}
impl ::core::clone::Clone for IEventClass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEventClass {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb2b72a0_7a68_11d1_88f9_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventClass_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub EventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetEventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstreventclassid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub EventClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetEventClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstreventclassname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrownersid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrownersid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FiringInterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfiringinterfaceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetFiringInterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfiringinterfaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CustomConfigCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcustomconfigclsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetCustomConfigCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcustomconfigclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TypeLib: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtypelib: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTypeLib: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtypelib: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventClass2(::windows_core::IUnknown);
impl IEventClass2 {
    pub unsafe fn EventClassID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EventClassID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEventClassID<P0>(&self, bstreventclassid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetEventClassID)(::windows_core::Interface::as_raw(self), bstreventclassid.into_param().abi()).ok()
    }
    pub unsafe fn EventClassName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EventClassName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEventClassName<P0>(&self, bstreventclassname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetEventClassName)(::windows_core::Interface::as_raw(self), bstreventclassname.into_param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.OwnerSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOwnerSID<P0>(&self, bstrownersid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetOwnerSID)(::windows_core::Interface::as_raw(self), bstrownersid.into_param().abi()).ok()
    }
    pub unsafe fn FiringInterfaceID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FiringInterfaceID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFiringInterfaceID<P0>(&self, bstrfiringinterfaceid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetFiringInterfaceID)(::windows_core::Interface::as_raw(self), bstrfiringinterfaceid.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn CustomConfigCLSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CustomConfigCLSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCustomConfigCLSID<P0>(&self, bstrcustomconfigclsid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetCustomConfigCLSID)(::windows_core::Interface::as_raw(self), bstrcustomconfigclsid.into_param().abi()).ok()
    }
    pub unsafe fn TypeLib(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TypeLib)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTypeLib<P0>(&self, bstrtypelib: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTypeLib)(::windows_core::Interface::as_raw(self), bstrtypelib.into_param().abi()).ok()
    }
    pub unsafe fn PublisherID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PublisherID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPublisherID<P0>(&self, bstrpublisherid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPublisherID)(::windows_core::Interface::as_raw(self), bstrpublisherid.into_param().abi()).ok()
    }
    pub unsafe fn MultiInterfacePublisherFilterCLSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MultiInterfacePublisherFilterCLSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMultiInterfacePublisherFilterCLSID<P0>(&self, bstrpubfilclsid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetMultiInterfacePublisherFilterCLSID)(::windows_core::Interface::as_raw(self), bstrpubfilclsid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowInprocActivation(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowInprocActivation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowInprocActivation<P0>(&self, fallowinprocactivation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowInprocActivation)(::windows_core::Interface::as_raw(self), fallowinprocactivation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FireInParallel(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FireInParallel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFireInParallel<P0>(&self, ffireinparallel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetFireInParallel)(::windows_core::Interface::as_raw(self), ffireinparallel.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IEventClass2, ::windows_core::IUnknown, super::IDispatch, IEventClass);
impl ::core::cmp::PartialEq for IEventClass2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventClass2 {}
impl ::core::fmt::Debug for IEventClass2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventClass2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventClass2 {
    type Vtable = IEventClass2_Vtbl;
}
impl ::core::clone::Clone for IEventClass2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEventClass2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb2b72a1_7a68_11d1_88f9_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventClass2_Vtbl {
    pub base__: IEventClass_Vtbl,
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublisherid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub MultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpubfilclsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetMultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpubfilclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FireInParallel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFireInParallel: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventControl(::windows_core::IUnknown);
impl IEventControl {
    pub unsafe fn SetPublisherFilter<P0, P1>(&self, methodname: P0, ppublisherfilter: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<IPublisherFilter>,
    {
        (::windows_core::Interface::vtable(self).SetPublisherFilter)(::windows_core::Interface::as_raw(self), methodname.into_param().abi(), ppublisherfilter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowInprocActivation(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowInprocActivation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
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
        (::windows_core::Interface::vtable(self).GetSubscriptions)(::windows_core::Interface::as_raw(self), methodname.into_param().abi(), optionalcriteria.into_param().abi(), optionalerrorindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDefaultQuery<P0, P1>(&self, methodname: P0, criteria: P1) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SetDefaultQuery)(::windows_core::Interface::as_raw(self), methodname.into_param().abi(), criteria.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEventControl, ::windows_core::IUnknown, super::IDispatch);
impl ::core::cmp::PartialEq for IEventControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventControl {}
impl ::core::fmt::Debug for IEventControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventControl {
    type Vtable = IEventControl_Vtbl;
}
impl ::core::clone::Clone for IEventControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEventControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0343e2f4_86f6_11d1_b760_00c04fb926af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventControl_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SetPublisherFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppublisherfilter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInprocActivation: usize,
    pub GetSubscriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalcriteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDefaultQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, criteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, errorindex: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventObjectChange(::windows_core::IUnknown);
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
::windows_core::imp::interface_hierarchy!(IEventObjectChange, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IEventObjectChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventObjectChange {}
impl ::core::fmt::Debug for IEventObjectChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventObjectChange").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventObjectChange {
    type Vtable = IEventObjectChange_Vtbl;
}
impl ::core::clone::Clone for IEventObjectChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEventObjectChange {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4a07d70_2e25_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventObjectChange_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ChangedSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrsubscriptionid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ChangedEventClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstreventclassid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ChangedPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrpublisherid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventObjectChange2(::windows_core::IUnknown);
impl IEventObjectChange2 {
    pub unsafe fn ChangedSubscription(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ChangedSubscription)(::windows_core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn ChangedEventClass(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ChangedEventClass)(::windows_core::Interface::as_raw(self), pinfo).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IEventObjectChange2, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IEventObjectChange2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventObjectChange2 {}
impl ::core::fmt::Debug for IEventObjectChange2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventObjectChange2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventObjectChange2 {
    type Vtable = IEventObjectChange2_Vtbl;
}
impl ::core::clone::Clone for IEventObjectChange2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEventObjectChange2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7701a9c3_bd68_438f_83e0_67bf4f53a422);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventObjectChange2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ChangedSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows_core::HRESULT,
    pub ChangedEventClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventObjectCollection(::windows_core::IUnknown);
impl IEventObjectCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item<P0>(&self, objectid: P0) -> ::windows_core::Result<super::super::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), objectid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn NewEnum(&self) -> ::windows_core::Result<IEnumEventObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Add<P0>(&self, item: *const super::super::Variant::VARIANT, objectid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), item, objectid.into_param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, objectid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), objectid.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IEventObjectCollection, ::windows_core::IUnknown, super::IDispatch);
impl ::core::cmp::PartialEq for IEventObjectCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventObjectCollection {}
impl ::core::fmt::Debug for IEventObjectCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventObjectCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventObjectCollection {
    type Vtable = IEventObjectCollection_Vtbl;
}
impl ::core::clone::Clone for IEventObjectCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEventObjectCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf89ac270_d4eb_11d1_b682_00805fc79216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventObjectCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pitem: *mut super::super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *const super::super::Variant::VARIANT, objectid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventProperty(::windows_core::IUnknown);
impl IEventProperty {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, propertyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), propertyname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Value(&self) -> ::windows_core::Result<super::super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Value)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, propertyvalue: *const super::super::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), propertyvalue).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IEventProperty, ::windows_core::IUnknown, super::IDispatch);
impl ::core::cmp::PartialEq for IEventProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventProperty {}
impl ::core::fmt::Debug for IEventProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventProperty {
    type Vtable = IEventProperty_Vtbl;
}
impl ::core::clone::Clone for IEventProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEventProperty {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda538ee2_f4de_11d1_b6bb_00805fc79216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventProperty_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyvalue: *const super::super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventPublisher(::windows_core::IUnknown);
impl IEventPublisher {
    pub unsafe fn PublisherID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PublisherID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPublisherID<P0>(&self, bstrpublisherid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPublisherID)(::windows_core::Interface::as_raw(self), bstrpublisherid.into_param().abi()).ok()
    }
    pub unsafe fn PublisherName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PublisherName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPublisherName<P0>(&self, bstrpublishername: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPublisherName)(::windows_core::Interface::as_raw(self), bstrpublishername.into_param().abi()).ok()
    }
    pub unsafe fn PublisherType(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PublisherType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPublisherType<P0>(&self, bstrpublishertype: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPublisherType)(::windows_core::Interface::as_raw(self), bstrpublishertype.into_param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OwnerSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOwnerSID<P0>(&self, bstrownersid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOwnerSID)(::windows_core::Interface::as_raw(self), bstrownersid.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetDefaultProperty<P0>(&self, bstrpropertyname: P0) -> ::windows_core::Result<super::super::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutDefaultProperty<P0>(&self, bstrpropertyname: P0, propertyvalue: *const super::super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).PutDefaultProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), propertyvalue).ok()
    }
    pub unsafe fn RemoveDefaultProperty<P0>(&self, bstrpropertyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RemoveDefaultProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi()).ok()
    }
    pub unsafe fn GetDefaultPropertyCollection(&self) -> ::windows_core::Result<IEventObjectCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultPropertyCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEventPublisher, ::windows_core::IUnknown, super::IDispatch);
impl ::core::cmp::PartialEq for IEventPublisher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventPublisher {}
impl ::core::fmt::Debug for IEventPublisher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventPublisher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventPublisher {
    type Vtable = IEventPublisher_Vtbl;
}
impl ::core::clone::Clone for IEventPublisher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEventPublisher {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe341516b_2e32_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventPublisher_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublisherid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PublisherName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublishername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPublisherName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublishername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PublisherType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublishertype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPublisherType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublishertype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrownersid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrownersid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetDefaultProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *mut super::super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetDefaultProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PutDefaultProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *const super::super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PutDefaultProperty: usize,
    pub RemoveDefaultProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetDefaultPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventSubscription(::windows_core::IUnknown);
impl IEventSubscription {
    pub unsafe fn SubscriptionID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SubscriptionID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSubscriptionID<P0>(&self, bstrsubscriptionid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSubscriptionID)(::windows_core::Interface::as_raw(self), bstrsubscriptionid.into_param().abi()).ok()
    }
    pub unsafe fn SubscriptionName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SubscriptionName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSubscriptionName<P0>(&self, bstrsubscriptionname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSubscriptionName)(::windows_core::Interface::as_raw(self), bstrsubscriptionname.into_param().abi()).ok()
    }
    pub unsafe fn PublisherID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PublisherID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPublisherID<P0>(&self, bstrpublisherid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPublisherID)(::windows_core::Interface::as_raw(self), bstrpublisherid.into_param().abi()).ok()
    }
    pub unsafe fn EventClassID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EventClassID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEventClassID<P0>(&self, bstreventclassid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetEventClassID)(::windows_core::Interface::as_raw(self), bstreventclassid.into_param().abi()).ok()
    }
    pub unsafe fn MethodName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MethodName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMethodName<P0>(&self, bstrmethodname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetMethodName)(::windows_core::Interface::as_raw(self), bstrmethodname.into_param().abi()).ok()
    }
    pub unsafe fn SubscriberCLSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SubscriberCLSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSubscriberCLSID<P0>(&self, bstrsubscriberclsid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSubscriberCLSID)(::windows_core::Interface::as_raw(self), bstrsubscriberclsid.into_param().abi()).ok()
    }
    pub unsafe fn SubscriberInterface(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SubscriberInterface)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSubscriberInterface<P0>(&self, psubscriberinterface: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).SetSubscriberInterface)(::windows_core::Interface::as_raw(self), psubscriberinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PerUser(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PerUser)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPerUser<P0>(&self, fperuser: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetPerUser)(::windows_core::Interface::as_raw(self), fperuser.into_param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OwnerSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOwnerSID<P0>(&self, bstrownersid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOwnerSID)(::windows_core::Interface::as_raw(self), bstrownersid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, fenabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), fenabled.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn MachineName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MachineName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMachineName<P0>(&self, bstrmachinename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetMachineName)(::windows_core::Interface::as_raw(self), bstrmachinename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetPublisherProperty<P0>(&self, bstrpropertyname: P0) -> ::windows_core::Result<super::super::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPublisherProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutPublisherProperty<P0>(&self, bstrpropertyname: P0, propertyvalue: *const super::super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).PutPublisherProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), propertyvalue).ok()
    }
    pub unsafe fn RemovePublisherProperty<P0>(&self, bstrpropertyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RemovePublisherProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi()).ok()
    }
    pub unsafe fn GetPublisherPropertyCollection(&self) -> ::windows_core::Result<IEventObjectCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPublisherPropertyCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetSubscriberProperty<P0>(&self, bstrpropertyname: P0) -> ::windows_core::Result<super::super::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSubscriberProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutSubscriberProperty<P0>(&self, bstrpropertyname: P0, propertyvalue: *const super::super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).PutSubscriberProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), propertyvalue).ok()
    }
    pub unsafe fn RemoveSubscriberProperty<P0>(&self, bstrpropertyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RemoveSubscriberProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi()).ok()
    }
    pub unsafe fn GetSubscriberPropertyCollection(&self) -> ::windows_core::Result<IEventObjectCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSubscriberPropertyCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InterfaceID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InterfaceID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInterfaceID<P0>(&self, bstrinterfaceid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetInterfaceID)(::windows_core::Interface::as_raw(self), bstrinterfaceid.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IEventSubscription, ::windows_core::IUnknown, super::IDispatch);
impl ::core::cmp::PartialEq for IEventSubscription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventSubscription {}
impl ::core::fmt::Debug for IEventSubscription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventSubscription").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventSubscription {
    type Vtable = IEventSubscription_Vtbl;
}
impl ::core::clone::Clone for IEventSubscription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEventSubscription {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a6b0e15_2e38_11d1_9965_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventSubscription_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SubscriptionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubscriptionid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSubscriptionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubscriptionid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SubscriptionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubscriptionname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSubscriptionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubscriptionname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublisherid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub EventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetEventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstreventclassid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub MethodName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmethodname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetMethodName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmethodname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SubscriberCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubscriberclsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSubscriberCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubscriberclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SubscriberInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubscriberinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSubscriberInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psubscriberinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PerUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfperuser: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PerUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPerUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fperuser: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPerUser: usize,
    pub OwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrownersid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrownersid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabled: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub MachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmachinename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetMachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmachinename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetPublisherProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *mut super::super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetPublisherProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PutPublisherProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *const super::super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PutPublisherProperty: usize,
    pub RemovePublisherProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetPublisherPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetSubscriberProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *mut super::super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetSubscriberProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PutSubscriberProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *const super::super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PutSubscriberProperty: usize,
    pub RemoveSubscriberProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetSubscriberPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrinterfaceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetInterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinterfaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventSystem(::windows_core::IUnknown);
impl IEventSystem {
    pub unsafe fn Query<P0, P1>(&self, progid: P0, querycriteria: P1, errorindex: *mut i32, ppinterface: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Query)(::windows_core::Interface::as_raw(self), progid.into_param().abi(), querycriteria.into_param().abi(), errorindex, ::core::mem::transmute(ppinterface)).ok()
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
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), progid.into_param().abi(), querycriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EventObjectChangeEventClassID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EventObjectChangeEventClassID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryS<P0, P1>(&self, progid: P0, querycriteria: P1) -> ::windows_core::Result<::windows_core::IUnknown>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueryS)(::windows_core::Interface::as_raw(self), progid.into_param().abi(), querycriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveS<P0, P1>(&self, progid: P0, querycriteria: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RemoveS)(::windows_core::Interface::as_raw(self), progid.into_param().abi(), querycriteria.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IEventSystem, ::windows_core::IUnknown, super::IDispatch);
impl ::core::cmp::PartialEq for IEventSystem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventSystem {}
impl ::core::fmt::Debug for IEventSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventSystem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventSystem {
    type Vtable = IEventSystem_Vtbl;
}
impl ::core::clone::Clone for IEventSystem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEventSystem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e14fb9f_2e22_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventSystem_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::std::mem::MaybeUninit<::windows_core::BSTR>, querycriteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, errorindex: *mut i32, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Store: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::std::mem::MaybeUninit<::windows_core::BSTR>, querycriteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, errorindex: *mut i32) -> ::windows_core::HRESULT,
    pub EventObjectChangeEventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub QueryS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::std::mem::MaybeUninit<::windows_core::BSTR>, querycriteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::std::mem::MaybeUninit<::windows_core::BSTR>, querycriteria: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IFiringControl(::windows_core::IUnknown);
impl IFiringControl {
    pub unsafe fn FireSubscription<P0>(&self, subscription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IEventSubscription>,
    {
        (::windows_core::Interface::vtable(self).FireSubscription)(::windows_core::Interface::as_raw(self), subscription.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IFiringControl, ::windows_core::IUnknown, super::IDispatch);
impl ::core::cmp::PartialEq for IFiringControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFiringControl {}
impl ::core::fmt::Debug for IFiringControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFiringControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFiringControl {
    type Vtable = IFiringControl_Vtbl;
}
impl ::core::clone::Clone for IFiringControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFiringControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0498c93_4efe_11d1_9971_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFiringControl_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub FireSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscription: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IMultiInterfaceEventControl(::windows_core::IUnknown);
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
        (::windows_core::Interface::vtable(self).GetSubscriptions)(::windows_core::Interface::as_raw(self), eventiid, bstrmethodname.into_param().abi(), optionalcriteria.into_param().abi(), optionalerrorindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDefaultQuery<P0, P1>(&self, eventiid: *const ::windows_core::GUID, bstrmethodname: P0, bstrcriteria: P1) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SetDefaultQuery)(::windows_core::Interface::as_raw(self), eventiid, bstrmethodname.into_param().abi(), bstrcriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowInprocActivation(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowInprocActivation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowInprocActivation<P0>(&self, fallowinprocactivation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowInprocActivation)(::windows_core::Interface::as_raw(self), fallowinprocactivation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FireInParallel(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FireInParallel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFireInParallel<P0>(&self, ffireinparallel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetFireInParallel)(::windows_core::Interface::as_raw(self), ffireinparallel.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMultiInterfaceEventControl, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IMultiInterfaceEventControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiInterfaceEventControl {}
impl ::core::fmt::Debug for IMultiInterfaceEventControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiInterfaceEventControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMultiInterfaceEventControl {
    type Vtable = IMultiInterfaceEventControl_Vtbl;
}
impl ::core::clone::Clone for IMultiInterfaceEventControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMultiInterfaceEventControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0343e2f5_86f6_11d1_b760_00c04fb926af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiInterfaceEventControl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetMultiInterfacePublisherFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classfilter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSubscriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventiid: *const ::windows_core::GUID, bstrmethodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalcriteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDefaultQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventiid: *const ::windows_core::GUID, bstrmethodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrcriteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, errorindex: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FireInParallel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFireInParallel: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IMultiInterfacePublisherFilter(::windows_core::IUnknown);
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
::windows_core::imp::interface_hierarchy!(IMultiInterfacePublisherFilter, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IMultiInterfacePublisherFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiInterfacePublisherFilter {}
impl ::core::fmt::Debug for IMultiInterfacePublisherFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiInterfacePublisherFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMultiInterfacePublisherFilter {
    type Vtable = IMultiInterfacePublisherFilter_Vtbl;
}
impl ::core::clone::Clone for IMultiInterfacePublisherFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMultiInterfacePublisherFilter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x465e5cc1_7b26_11d1_88fb_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiInterfacePublisherFilter_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peic: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PrepareToFire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, methodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, firingcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IPublisherFilter(::windows_core::IUnknown);
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
::windows_core::imp::interface_hierarchy!(IPublisherFilter, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPublisherFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPublisherFilter {}
impl ::core::fmt::Debug for IPublisherFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPublisherFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPublisherFilter {
    type Vtable = IPublisherFilter_Vtbl;
}
impl ::core::clone::Clone for IPublisherFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPublisherFilter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x465e5cc0_7b26_11d1_88fb_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPublisherFilter_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, dispuserdefined: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PrepareToFire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::std::mem::MaybeUninit<::windows_core::BSTR>, firingcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const CEventClass: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcdbec9c0_7a68_11d1_88f9_0080c7d771bf);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const CEventPublisher: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab944620_79c6_11d1_88f9_0080c7d771bf);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const CEventSubscription: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7542e960_79c7_11d1_88f9_0080c7d771bf);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const CEventSystem: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e14fba2_2e22_11d1_9964_00c04fbbb345);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_DeletedObject: EOC_ChangeType = EOC_ChangeType(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_ModifiedObject: EOC_ChangeType = EOC_ChangeType(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_NewObject: EOC_ChangeType = EOC_ChangeType(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EventObjectChange: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0565000_9df4_11d1_a281_00c04fca0aa7);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EventObjectChange2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb07bacd_cd56_4e63_a8ff_cbf0355fb9f4);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EOC_ChangeType(pub i32);
impl ::core::marker::Copy for EOC_ChangeType {}
impl ::core::clone::Clone for EOC_ChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EOC_ChangeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EOC_ChangeType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EOC_ChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EOC_ChangeType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
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
