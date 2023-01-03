#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IDontSupportEventSubscription(::windows::core::IUnknown);
impl IDontSupportEventSubscription {}
::windows::core::interface_hierarchy!(IDontSupportEventSubscription, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDontSupportEventSubscription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IDontSupportEventSubscription {
    type Vtable = IDontSupportEventSubscription_Vtbl;
}
unsafe impl ::windows::core::Interface for IDontSupportEventSubscription {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x784121f1_62a6_4b89_855f_d65f296de83a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDontSupportEventSubscription_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEnumEventObject(::windows::core::IUnknown);
impl IEnumEventObject {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumEventObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Next(&self, ppinterface: &mut [::core::option::Option<::windows::core::IUnknown>], cretelem: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), ppinterface.len() as _, ::core::mem::transmute(ppinterface.as_ptr()), cretelem).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, cskipelem: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), cskipelem).ok()
    }
}
::windows::core::interface_hierarchy!(IEnumEventObject, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumEventObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IEnumEventObject {
    type Vtable = IEnumEventObject_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumEventObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4a07d63_2e25_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumEventObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creqelem: u32, ppinterface: *mut *mut ::core::ffi::c_void, cretelem: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cskipelem: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventClass(::windows::core::IUnknown);
impl IEventClass {
    pub unsafe fn EventClassID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EventClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEventClassID(&self, bstreventclassid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetEventClassID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstreventclassid)).ok()
    }
    pub unsafe fn EventClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EventClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEventClassName(&self, bstreventclassname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetEventClassName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstreventclassname)).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OwnerSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOwnerSID(&self, bstrownersid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOwnerSID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrownersid)).ok()
    }
    pub unsafe fn FiringInterfaceID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FiringInterfaceID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFiringInterfaceID(&self, bstrfiringinterfaceid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFiringInterfaceID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrfiringinterfaceid)).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    pub unsafe fn CustomConfigCLSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CustomConfigCLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCustomConfigCLSID(&self, bstrcustomconfigclsid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCustomConfigCLSID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrcustomconfigclsid)).ok()
    }
    pub unsafe fn TypeLib(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TypeLib)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTypeLib(&self, bstrtypelib: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTypeLib)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtypelib)).ok()
    }
}
::windows::core::interface_hierarchy!(IEventClass, ::windows::core::IUnknown, super::IDispatch);
impl ::core::clone::Clone for IEventClass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IEventClass {
    type Vtable = IEventClass_Vtbl;
}
unsafe impl ::windows::core::Interface for IEventClass {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb2b72a0_7a68_11d1_88f9_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventClass_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub EventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetEventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstreventclassid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EventClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetEventClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstreventclassname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrownersid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrownersid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FiringInterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfiringinterfaceid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFiringInterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfiringinterfaceid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CustomConfigCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcustomconfigclsid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCustomConfigCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcustomconfigclsid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TypeLib: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtypelib: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTypeLib: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtypelib: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventClass2(::windows::core::IUnknown);
impl IEventClass2 {
    pub unsafe fn EventClassID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EventClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEventClassID(&self, bstreventclassid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEventClassID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstreventclassid)).ok()
    }
    pub unsafe fn EventClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EventClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEventClassName(&self, bstreventclassname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEventClassName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstreventclassname)).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OwnerSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOwnerSID(&self, bstrownersid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOwnerSID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrownersid)).ok()
    }
    pub unsafe fn FiringInterfaceID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FiringInterfaceID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFiringInterfaceID(&self, bstrfiringinterfaceid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFiringInterfaceID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrfiringinterfaceid)).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    pub unsafe fn CustomConfigCLSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CustomConfigCLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCustomConfigCLSID(&self, bstrcustomconfigclsid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCustomConfigCLSID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrcustomconfigclsid)).ok()
    }
    pub unsafe fn TypeLib(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TypeLib)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTypeLib(&self, bstrtypelib: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTypeLib)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtypelib)).ok()
    }
    pub unsafe fn PublisherID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PublisherID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPublisherID(&self, bstrpublisherid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPublisherID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpublisherid)).ok()
    }
    pub unsafe fn MultiInterfacePublisherFilterCLSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MultiInterfacePublisherFilterCLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMultiInterfacePublisherFilterCLSID(&self, bstrpubfilclsid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMultiInterfacePublisherFilterCLSID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpubfilclsid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowInprocActivation(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AllowInprocActivation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowInprocActivation<P0>(&self, fallowinprocactivation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetAllowInprocActivation)(::windows::core::Vtable::as_raw(self), fallowinprocactivation.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FireInParallel(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FireInParallel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFireInParallel<P0>(&self, ffireinparallel: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetFireInParallel)(::windows::core::Vtable::as_raw(self), ffireinparallel.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IEventClass2, ::windows::core::IUnknown, super::IDispatch, IEventClass);
impl ::core::clone::Clone for IEventClass2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IEventClass2 {
    type Vtable = IEventClass2_Vtbl;
}
unsafe impl ::windows::core::Interface for IEventClass2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb2b72a1_7a68_11d1_88f9_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventClass2_Vtbl {
    pub base__: IEventClass_Vtbl,
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublisherid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpubfilclsid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpubfilclsid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FireInParallel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFireInParallel: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventControl(::windows::core::IUnknown);
impl IEventControl {
    pub unsafe fn SetPublisherFilter<P0>(&self, methodname: &::windows::core::BSTR, ppublisherfilter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPublisherFilter>>,
    {
        (::windows::core::Vtable::vtable(self).SetPublisherFilter)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(methodname), ppublisherfilter.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowInprocActivation(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AllowInprocActivation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowInprocActivation<P0>(&self, fallowinprocactivation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetAllowInprocActivation)(::windows::core::Vtable::as_raw(self), fallowinprocactivation.into()).ok()
    }
    pub unsafe fn GetSubscriptions(&self, methodname: &::windows::core::BSTR, optionalcriteria: &::windows::core::BSTR, optionalerrorindex: *const i32) -> ::windows::core::Result<IEventObjectCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSubscriptions)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(methodname), ::core::mem::transmute_copy(optionalcriteria), optionalerrorindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDefaultQuery(&self, methodname: &::windows::core::BSTR, criteria: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SetDefaultQuery)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(methodname), ::core::mem::transmute_copy(criteria), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEventControl, ::windows::core::IUnknown, super::IDispatch);
impl ::core::clone::Clone for IEventControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IEventControl {
    type Vtable = IEventControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IEventControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0343e2f4_86f6_11d1_b760_00c04fb926af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventControl_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SetPublisherFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: *mut ::core::ffi::c_void, ppublisherfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInprocActivation: usize,
    pub GetSubscriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: *mut ::core::ffi::c_void, optionalcriteria: *mut ::core::ffi::c_void, optionalerrorindex: *const i32, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDefaultQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: *mut ::core::ffi::c_void, criteria: *mut ::core::ffi::c_void, errorindex: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventObjectChange(::windows::core::IUnknown);
impl IEventObjectChange {
    pub unsafe fn ChangedSubscription(&self, changetype: EOC_ChangeType, bstrsubscriptionid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ChangedSubscription)(::windows::core::Vtable::as_raw(self), changetype, ::core::mem::transmute_copy(bstrsubscriptionid)).ok()
    }
    pub unsafe fn ChangedEventClass(&self, changetype: EOC_ChangeType, bstreventclassid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ChangedEventClass)(::windows::core::Vtable::as_raw(self), changetype, ::core::mem::transmute_copy(bstreventclassid)).ok()
    }
    pub unsafe fn ChangedPublisher(&self, changetype: EOC_ChangeType, bstrpublisherid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ChangedPublisher)(::windows::core::Vtable::as_raw(self), changetype, ::core::mem::transmute_copy(bstrpublisherid)).ok()
    }
}
::windows::core::interface_hierarchy!(IEventObjectChange, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEventObjectChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IEventObjectChange {
    type Vtable = IEventObjectChange_Vtbl;
}
unsafe impl ::windows::core::Interface for IEventObjectChange {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4a07d70_2e25_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventObjectChange_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ChangedSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrsubscriptionid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ChangedEventClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstreventclassid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ChangedPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrpublisherid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventObjectChange2(::windows::core::IUnknown);
impl IEventObjectChange2 {
    pub unsafe fn ChangedSubscription(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ChangedSubscription)(::windows::core::Vtable::as_raw(self), pinfo).ok()
    }
    pub unsafe fn ChangedEventClass(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ChangedEventClass)(::windows::core::Vtable::as_raw(self), pinfo).ok()
    }
}
::windows::core::interface_hierarchy!(IEventObjectChange2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEventObjectChange2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IEventObjectChange2 {
    type Vtable = IEventObjectChange2_Vtbl;
}
unsafe impl ::windows::core::Interface for IEventObjectChange2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7701a9c3_bd68_438f_83e0_67bf4f53a422);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventObjectChange2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ChangedSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::HRESULT,
    pub ChangedEventClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventObjectCollection(::windows::core::IUnknown);
impl IEventObjectCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, objectid: &::windows::core::BSTR) -> ::windows::core::Result<super::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(objectid), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NewEnum(&self) -> ::windows::core::Result<IEnumEventObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Add(&self, item: *const super::VARIANT, objectid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Add)(::windows::core::Vtable::as_raw(self), item, ::core::mem::transmute_copy(objectid)).ok()
    }
    pub unsafe fn Remove(&self, objectid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Remove)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(objectid)).ok()
    }
}
::windows::core::interface_hierarchy!(IEventObjectCollection, ::windows::core::IUnknown, super::IDispatch);
impl ::core::clone::Clone for IEventObjectCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IEventObjectCollection {
    type Vtable = IEventObjectCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IEventObjectCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf89ac270_d4eb_11d1_b682_00805fc79216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventObjectCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: *mut ::core::ffi::c_void, pitem: *mut super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *const super::VARIANT, objectid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventProperty(::windows::core::IUnknown);
impl IEventProperty {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, propertyname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(propertyname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Value)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetValue)(::windows::core::Vtable::as_raw(self), propertyvalue).ok()
    }
}
::windows::core::interface_hierarchy!(IEventProperty, ::windows::core::IUnknown, super::IDispatch);
impl ::core::clone::Clone for IEventProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IEventProperty {
    type Vtable = IEventProperty_Vtbl;
}
unsafe impl ::windows::core::Interface for IEventProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda538ee2_f4de_11d1_b6bb_00805fc79216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventProperty_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    SetValue: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventPublisher(::windows::core::IUnknown);
impl IEventPublisher {
    pub unsafe fn PublisherID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PublisherID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPublisherID(&self, bstrpublisherid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPublisherID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpublisherid)).ok()
    }
    pub unsafe fn PublisherName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PublisherName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPublisherName(&self, bstrpublishername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPublisherName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpublishername)).ok()
    }
    pub unsafe fn PublisherType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PublisherType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPublisherType(&self, bstrpublishertype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPublisherType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpublishertype)).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OwnerSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOwnerSID(&self, bstrownersid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOwnerSID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrownersid)).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetDefaultProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<super::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDefaultProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn PutDefaultProperty(&self, bstrpropertyname: &::windows::core::BSTR, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PutDefaultProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname), propertyvalue).ok()
    }
    pub unsafe fn RemoveDefaultProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveDefaultProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname)).ok()
    }
    pub unsafe fn GetDefaultPropertyCollection(&self) -> ::windows::core::Result<IEventObjectCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDefaultPropertyCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEventPublisher, ::windows::core::IUnknown, super::IDispatch);
impl ::core::clone::Clone for IEventPublisher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IEventPublisher {
    type Vtable = IEventPublisher_Vtbl;
}
unsafe impl ::windows::core::Interface for IEventPublisher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe341516b_2e32_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventPublisher_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublisherid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PublisherName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublishername: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPublisherName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublishername: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PublisherType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublishertype: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPublisherType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublishertype: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrownersid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrownersid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetDefaultProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: *mut ::core::ffi::c_void, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetDefaultProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub PutDefaultProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: *mut ::core::ffi::c_void, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    PutDefaultProperty: usize,
    pub RemoveDefaultProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDefaultPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventSubscription(::windows::core::IUnknown);
impl IEventSubscription {
    pub unsafe fn SubscriptionID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SubscriptionID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSubscriptionID(&self, bstrsubscriptionid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSubscriptionID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrsubscriptionid)).ok()
    }
    pub unsafe fn SubscriptionName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SubscriptionName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSubscriptionName(&self, bstrsubscriptionname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSubscriptionName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrsubscriptionname)).ok()
    }
    pub unsafe fn PublisherID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PublisherID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPublisherID(&self, bstrpublisherid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPublisherID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpublisherid)).ok()
    }
    pub unsafe fn EventClassID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EventClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEventClassID(&self, bstreventclassid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetEventClassID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstreventclassid)).ok()
    }
    pub unsafe fn MethodName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MethodName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMethodName(&self, bstrmethodname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMethodName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmethodname)).ok()
    }
    pub unsafe fn SubscriberCLSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SubscriberCLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSubscriberCLSID(&self, bstrsubscriberclsid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSubscriberCLSID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrsubscriberclsid)).ok()
    }
    pub unsafe fn SubscriberInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SubscriberInterface)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSubscriberInterface<P0>(&self, psubscriberinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetSubscriberInterface)(::windows::core::Vtable::as_raw(self), psubscriberinterface.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PerUser(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PerUser)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPerUser<P0>(&self, fperuser: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetPerUser)(::windows::core::Vtable::as_raw(self), fperuser.into()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OwnerSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOwnerSID(&self, bstrownersid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOwnerSID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrownersid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Enabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, fenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetEnabled)(::windows::core::Vtable::as_raw(self), fenabled.into()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    pub unsafe fn MachineName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MachineName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMachineName(&self, bstrmachinename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMachineName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmachinename)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPublisherProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<super::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPublisherProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn PutPublisherProperty(&self, bstrpropertyname: &::windows::core::BSTR, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PutPublisherProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname), propertyvalue).ok()
    }
    pub unsafe fn RemovePublisherProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemovePublisherProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname)).ok()
    }
    pub unsafe fn GetPublisherPropertyCollection(&self) -> ::windows::core::Result<IEventObjectCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPublisherPropertyCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetSubscriberProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<super::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSubscriberProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn PutSubscriberProperty(&self, bstrpropertyname: &::windows::core::BSTR, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PutSubscriberProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname), propertyvalue).ok()
    }
    pub unsafe fn RemoveSubscriberProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveSubscriberProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname)).ok()
    }
    pub unsafe fn GetSubscriberPropertyCollection(&self) -> ::windows::core::Result<IEventObjectCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSubscriberPropertyCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InterfaceID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InterfaceID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetInterfaceID(&self, bstrinterfaceid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInterfaceID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrinterfaceid)).ok()
    }
}
::windows::core::interface_hierarchy!(IEventSubscription, ::windows::core::IUnknown, super::IDispatch);
impl ::core::clone::Clone for IEventSubscription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IEventSubscription {
    type Vtable = IEventSubscription_Vtbl;
}
unsafe impl ::windows::core::Interface for IEventSubscription {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a6b0e15_2e38_11d1_9965_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventSubscription_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SubscriptionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubscriptionid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSubscriptionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubscriptionid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SubscriptionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubscriptionname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSubscriptionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubscriptionname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublisherid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetEventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstreventclassid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MethodName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmethodname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMethodName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmethodname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SubscriberCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubscriberclsid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSubscriberCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubscriberclsid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SubscriberInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubscriberinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSubscriberInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psubscriberinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PerUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfperuser: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PerUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPerUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fperuser: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPerUser: usize,
    pub OwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrownersid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrownersid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmachinename: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmachinename: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetPublisherProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: *mut ::core::ffi::c_void, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetPublisherProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub PutPublisherProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: *mut ::core::ffi::c_void, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    PutPublisherProperty: usize,
    pub RemovePublisherProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPublisherPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetSubscriberProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: *mut ::core::ffi::c_void, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetSubscriberProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub PutSubscriberProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: *mut ::core::ffi::c_void, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    PutSubscriberProperty: usize,
    pub RemoveSubscriberProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSubscriberPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrinterfaceid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetInterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinterfaceid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventSystem(::windows::core::IUnknown);
impl IEventSystem {
    pub unsafe fn Query(&self, progid: &::windows::core::BSTR, querycriteria: &::windows::core::BSTR, errorindex: *mut i32, ppinterface: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Query)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(progid), ::core::mem::transmute_copy(querycriteria), errorindex, ::core::mem::transmute(ppinterface)).ok()
    }
    pub unsafe fn Store<P0>(&self, progid: &::windows::core::BSTR, pinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).Store)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(progid), pinterface.into().abi()).ok()
    }
    pub unsafe fn Remove(&self, progid: &::windows::core::BSTR, querycriteria: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Remove)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(progid), ::core::mem::transmute_copy(querycriteria), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EventObjectChangeEventClassID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EventObjectChangeEventClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn QueryS(&self, progid: &::windows::core::BSTR, querycriteria: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QueryS)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(progid), ::core::mem::transmute_copy(querycriteria), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveS(&self, progid: &::windows::core::BSTR, querycriteria: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveS)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(progid), ::core::mem::transmute_copy(querycriteria)).ok()
    }
}
::windows::core::interface_hierarchy!(IEventSystem, ::windows::core::IUnknown, super::IDispatch);
impl ::core::clone::Clone for IEventSystem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IEventSystem {
    type Vtable = IEventSystem_Vtbl;
}
unsafe impl ::windows::core::Interface for IEventSystem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e14fb9f_2e22_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventSystem_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: *mut ::core::ffi::c_void, querycriteria: *mut ::core::ffi::c_void, errorindex: *mut i32, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Store: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: *mut ::core::ffi::c_void, pinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: *mut ::core::ffi::c_void, querycriteria: *mut ::core::ffi::c_void, errorindex: *mut i32) -> ::windows::core::HRESULT,
    pub EventObjectChangeEventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: *mut ::core::ffi::c_void, querycriteria: *mut ::core::ffi::c_void, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: *mut ::core::ffi::c_void, querycriteria: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IFiringControl(::windows::core::IUnknown);
impl IFiringControl {
    pub unsafe fn FireSubscription<P0>(&self, subscription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IEventSubscription>>,
    {
        (::windows::core::Vtable::vtable(self).FireSubscription)(::windows::core::Vtable::as_raw(self), subscription.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IFiringControl, ::windows::core::IUnknown, super::IDispatch);
impl ::core::clone::Clone for IFiringControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IFiringControl {
    type Vtable = IFiringControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IFiringControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0498c93_4efe_11d1_9971_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFiringControl_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub FireSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IMultiInterfaceEventControl(::windows::core::IUnknown);
impl IMultiInterfaceEventControl {
    pub unsafe fn SetMultiInterfacePublisherFilter<P0>(&self, classfilter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMultiInterfacePublisherFilter>>,
    {
        (::windows::core::Vtable::vtable(self).SetMultiInterfacePublisherFilter)(::windows::core::Vtable::as_raw(self), classfilter.into().abi()).ok()
    }
    pub unsafe fn GetSubscriptions(&self, eventiid: *const ::windows::core::GUID, bstrmethodname: &::windows::core::BSTR, optionalcriteria: &::windows::core::BSTR, optionalerrorindex: *const i32) -> ::windows::core::Result<IEventObjectCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSubscriptions)(::windows::core::Vtable::as_raw(self), eventiid, ::core::mem::transmute_copy(bstrmethodname), ::core::mem::transmute_copy(optionalcriteria), optionalerrorindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDefaultQuery(&self, eventiid: *const ::windows::core::GUID, bstrmethodname: &::windows::core::BSTR, bstrcriteria: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SetDefaultQuery)(::windows::core::Vtable::as_raw(self), eventiid, ::core::mem::transmute_copy(bstrmethodname), ::core::mem::transmute_copy(bstrcriteria), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowInprocActivation(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AllowInprocActivation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowInprocActivation<P0>(&self, fallowinprocactivation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetAllowInprocActivation)(::windows::core::Vtable::as_raw(self), fallowinprocactivation.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FireInParallel(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FireInParallel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFireInParallel<P0>(&self, ffireinparallel: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetFireInParallel)(::windows::core::Vtable::as_raw(self), ffireinparallel.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IMultiInterfaceEventControl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMultiInterfaceEventControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IMultiInterfaceEventControl {
    type Vtable = IMultiInterfaceEventControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IMultiInterfaceEventControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0343e2f5_86f6_11d1_b760_00c04fb926af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiInterfaceEventControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetMultiInterfacePublisherFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSubscriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventiid: *const ::windows::core::GUID, bstrmethodname: *mut ::core::ffi::c_void, optionalcriteria: *mut ::core::ffi::c_void, optionalerrorindex: *const i32, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDefaultQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventiid: *const ::windows::core::GUID, bstrmethodname: *mut ::core::ffi::c_void, bstrcriteria: *mut ::core::ffi::c_void, errorindex: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FireInParallel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFireInParallel: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IMultiInterfacePublisherFilter(::windows::core::IUnknown);
impl IMultiInterfacePublisherFilter {
    pub unsafe fn Initialize<P0>(&self, peic: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMultiInterfaceEventControl>>,
    {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), peic.into().abi()).ok()
    }
    pub unsafe fn PrepareToFire<P0>(&self, iid: *const ::windows::core::GUID, methodname: &::windows::core::BSTR, firingcontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFiringControl>>,
    {
        (::windows::core::Vtable::vtable(self).PrepareToFire)(::windows::core::Vtable::as_raw(self), iid, ::core::mem::transmute_copy(methodname), firingcontrol.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IMultiInterfacePublisherFilter, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMultiInterfacePublisherFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IMultiInterfacePublisherFilter {
    type Vtable = IMultiInterfacePublisherFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for IMultiInterfacePublisherFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x465e5cc1_7b26_11d1_88fb_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiInterfacePublisherFilter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peic: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PrepareToFire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, methodname: *mut ::core::ffi::c_void, firingcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IPublisherFilter(::windows::core::IUnknown);
impl IPublisherFilter {
    pub unsafe fn Initialize<P0>(&self, methodname: &::windows::core::BSTR, dispuserdefined: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(methodname), dispuserdefined.into().abi()).ok()
    }
    pub unsafe fn PrepareToFire<P0>(&self, methodname: &::windows::core::BSTR, firingcontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFiringControl>>,
    {
        (::windows::core::Vtable::vtable(self).PrepareToFire)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(methodname), firingcontrol.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IPublisherFilter, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPublisherFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IPublisherFilter {
    type Vtable = IPublisherFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for IPublisherFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x465e5cc0_7b26_11d1_88fb_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPublisherFilter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: *mut ::core::ffi::c_void, dispuserdefined: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PrepareToFire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: *mut ::core::ffi::c_void, firingcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const CEventClass: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdbec9c0_7a68_11d1_88f9_0080c7d771bf);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const CEventPublisher: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab944620_79c6_11d1_88f9_0080c7d771bf);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const CEventSubscription: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7542e960_79c7_11d1_88f9_0080c7d771bf);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const CEventSystem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e14fba2_2e22_11d1_9964_00c04fbbb345);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EventObjectChange: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0565000_9df4_11d1_a281_00c04fca0aa7);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EventObjectChange2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb07bacd_cd56_4e63_a8ff_cbf0355fb9f4);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EOC_ChangeType(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_NewObject: EOC_ChangeType = EOC_ChangeType(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_ModifiedObject: EOC_ChangeType = EOC_ChangeType(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_DeletedObject: EOC_ChangeType = EOC_ChangeType(2i32);
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
unsafe impl ::windows::core::Abi for EOC_ChangeType {
    type Abi = Self;
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
    pub objectId: ::windows::core::ManuallyDrop<::windows::core::BSTR>,
    pub partitionId: ::windows::core::ManuallyDrop<::windows::core::BSTR>,
    pub applicationId: ::windows::core::ManuallyDrop<::windows::core::BSTR>,
    pub reserved: [::windows::core::GUID; 10],
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
unsafe impl ::windows::core::Abi for COMEVENTSYSCHANGEINFO {
    type Abi = Self;
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
