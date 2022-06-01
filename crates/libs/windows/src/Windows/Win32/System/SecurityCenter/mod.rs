#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWSCDefaultProduct(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWSCDefaultProduct {
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDefaultProduct<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, etype: SECURITY_PRODUCT_TYPE, pguid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultProduct)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(etype), pguid.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWSCDefaultProduct> for ::windows::core::IUnknown {
    fn from(value: IWSCDefaultProduct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWSCDefaultProduct> for ::windows::core::IUnknown {
    fn from(value: &IWSCDefaultProduct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSCDefaultProduct {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSCDefaultProduct {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWSCDefaultProduct> for super::Com::IDispatch {
    fn from(value: IWSCDefaultProduct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWSCDefaultProduct> for super::Com::IDispatch {
    fn from(value: &IWSCDefaultProduct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IWSCDefaultProduct {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IWSCDefaultProduct {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWSCDefaultProduct {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSCDefaultProduct {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSCDefaultProduct {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSCDefaultProduct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSCDefaultProduct").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWSCDefaultProduct {
    type Vtable = IWSCDefaultProduct_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0476d69c_f21a_11e5_9ce9_5e5517507c66);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSCDefaultProduct_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDefaultProduct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, etype: SECURITY_PRODUCT_TYPE, pguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDefaultProduct: usize,
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWSCProductList(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWSCProductList {
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn Initialize(&self, provider: WSC_SECURITY_PROVIDER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(provider)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: u32) -> ::windows::core::Result<IWscProduct> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWscProduct>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWSCProductList> for ::windows::core::IUnknown {
    fn from(value: IWSCProductList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWSCProductList> for ::windows::core::IUnknown {
    fn from(value: &IWSCProductList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWSCProductList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWSCProductList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWSCProductList> for super::Com::IDispatch {
    fn from(value: IWSCProductList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWSCProductList> for super::Com::IDispatch {
    fn from(value: &IWSCProductList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IWSCProductList {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IWSCProductList {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWSCProductList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSCProductList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSCProductList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSCProductList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSCProductList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWSCProductList {
    type Vtable = IWSCProductList_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x722a338c_6e8e_4e72_ac27_1417fb0c81c2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSCProductList_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: WSC_SECURITY_PROVIDER) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWscProduct(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWscProduct {
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows::core::Interface::vtable(self).ProductName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn ProductState(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_STATE>::zeroed();
        (::windows::core::Interface::vtable(self).ProductState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_STATE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn SignatureStatus(&self) -> ::windows::core::Result<WSC_SECURITY_SIGNATURE_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_SIGNATURE_STATUS>::zeroed();
        (::windows::core::Interface::vtable(self).SignatureStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_SIGNATURE_STATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemediationPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows::core::Interface::vtable(self).RemediationPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductStateTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows::core::Interface::vtable(self).ProductStateTimestamp)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows::core::Interface::vtable(self).ProductGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductIsDefault(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows::core::Interface::vtable(self).ProductIsDefault)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct> for ::windows::core::IUnknown {
    fn from(value: IWscProduct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct> for ::windows::core::IUnknown {
    fn from(value: &IWscProduct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWscProduct {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWscProduct {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct> for super::Com::IDispatch {
    fn from(value: IWscProduct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct> for super::Com::IDispatch {
    fn from(value: &IWscProduct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IWscProduct {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IWscProduct {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWscProduct {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWscProduct {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWscProduct {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWscProduct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWscProduct").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWscProduct {
    type Vtable = IWscProduct_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c38232e_3a45_4a27_92b0_1a16a975f669);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWscProduct_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ProductName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProductName: usize,
    pub ProductState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_PRODUCT_STATE) -> ::windows::core::HRESULT,
    pub SignatureStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_SIGNATURE_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemediationPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemediationPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProductStateTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProductStateTimestamp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProductGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProductGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProductIsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProductIsDefault: usize,
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWscProduct2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWscProduct2 {
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows::core::Interface::vtable(self).base__.ProductName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn ProductState(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_STATE>::zeroed();
        (::windows::core::Interface::vtable(self).base__.ProductState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_STATE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn SignatureStatus(&self) -> ::windows::core::Result<WSC_SECURITY_SIGNATURE_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_SIGNATURE_STATUS>::zeroed();
        (::windows::core::Interface::vtable(self).base__.SignatureStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_SIGNATURE_STATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemediationPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows::core::Interface::vtable(self).base__.RemediationPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductStateTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows::core::Interface::vtable(self).base__.ProductStateTimestamp)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows::core::Interface::vtable(self).base__.ProductGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductIsDefault(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows::core::Interface::vtable(self).base__.ProductIsDefault)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn AntivirusScanSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows::core::Interface::vtable(self).AntivirusScanSubstatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn AntivirusSettingsSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows::core::Interface::vtable(self).AntivirusSettingsSubstatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn AntivirusProtectionUpdateSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows::core::Interface::vtable(self).AntivirusProtectionUpdateSubstatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn FirewallDomainProfileSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows::core::Interface::vtable(self).FirewallDomainProfileSubstatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn FirewallPrivateProfileSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows::core::Interface::vtable(self).FirewallPrivateProfileSubstatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn FirewallPublicProfileSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows::core::Interface::vtable(self).FirewallPublicProfileSubstatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct2> for ::windows::core::IUnknown {
    fn from(value: IWscProduct2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct2> for ::windows::core::IUnknown {
    fn from(value: &IWscProduct2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWscProduct2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWscProduct2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct2> for super::Com::IDispatch {
    fn from(value: IWscProduct2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct2> for super::Com::IDispatch {
    fn from(value: &IWscProduct2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IWscProduct2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IWscProduct2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct2> for IWscProduct {
    fn from(value: IWscProduct2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct2> for IWscProduct {
    fn from(value: &IWscProduct2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IWscProduct> for IWscProduct2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWscProduct> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IWscProduct> for &'a IWscProduct2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWscProduct> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWscProduct2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWscProduct2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWscProduct2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWscProduct2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWscProduct2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWscProduct2 {
    type Vtable = IWscProduct2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf896ca54_fe09_4403_86d4_23cb488d81d8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWscProduct2_Vtbl {
    pub base__: IWscProduct_Vtbl,
    pub AntivirusScanSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT,
    pub AntivirusSettingsSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT,
    pub AntivirusProtectionUpdateSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT,
    pub FirewallDomainProfileSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT,
    pub FirewallPrivateProfileSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT,
    pub FirewallPublicProfileSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWscProduct3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWscProduct3 {
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.ProductName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn ProductState(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_STATE>::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.ProductState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_STATE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn SignatureStatus(&self) -> ::windows::core::Result<WSC_SECURITY_SIGNATURE_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_SIGNATURE_STATUS>::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.SignatureStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_SIGNATURE_STATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemediationPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.RemediationPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductStateTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.ProductStateTimestamp)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.ProductGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductIsDefault(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.ProductIsDefault)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn AntivirusScanSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows::core::Interface::vtable(self).base__.AntivirusScanSubstatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn AntivirusSettingsSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows::core::Interface::vtable(self).base__.AntivirusSettingsSubstatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn AntivirusProtectionUpdateSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows::core::Interface::vtable(self).base__.AntivirusProtectionUpdateSubstatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn FirewallDomainProfileSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows::core::Interface::vtable(self).base__.FirewallDomainProfileSubstatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn FirewallPrivateProfileSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows::core::Interface::vtable(self).base__.FirewallPrivateProfileSubstatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn FirewallPublicProfileSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows::core::Interface::vtable(self).base__.FirewallPublicProfileSubstatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub unsafe fn AntivirusDaysUntilExpired(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).AntivirusDaysUntilExpired)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct3> for ::windows::core::IUnknown {
    fn from(value: IWscProduct3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct3> for ::windows::core::IUnknown {
    fn from(value: &IWscProduct3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWscProduct3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWscProduct3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct3> for super::Com::IDispatch {
    fn from(value: IWscProduct3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct3> for super::Com::IDispatch {
    fn from(value: &IWscProduct3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IWscProduct3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IWscProduct3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct3> for IWscProduct {
    fn from(value: IWscProduct3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct3> for IWscProduct {
    fn from(value: &IWscProduct3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IWscProduct> for IWscProduct3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWscProduct> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IWscProduct> for &'a IWscProduct3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWscProduct> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct3> for IWscProduct2 {
    fn from(value: IWscProduct3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct3> for IWscProduct2 {
    fn from(value: &IWscProduct3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IWscProduct2> for IWscProduct3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWscProduct2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IWscProduct2> for &'a IWscProduct3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWscProduct2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWscProduct3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWscProduct3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWscProduct3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWscProduct3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWscProduct3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWscProduct3 {
    type Vtable = IWscProduct3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55536524_d1d1_4726_8c7c_04996a1904e7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWscProduct3_Vtbl {
    pub base__: IWscProduct2_Vtbl,
    pub AntivirusDaysUntilExpired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwdays: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SECURITY_PRODUCT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const SECURITY_PRODUCT_TYPE_ANTIVIRUS: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const SECURITY_PRODUCT_TYPE_FIREWALL: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const SECURITY_PRODUCT_TYPE_ANTISPYWARE: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(2i32);
impl ::core::marker::Copy for SECURITY_PRODUCT_TYPE {}
impl ::core::clone::Clone for SECURITY_PRODUCT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SECURITY_PRODUCT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SECURITY_PRODUCT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SECURITY_PRODUCT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECURITY_PRODUCT_TYPE").field(&self.0).finish()
    }
}
pub const WSCDefaultProduct: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2981a36e_f22d_11e5_9ce9_5e5517507c66);
pub const WSCProductList: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17072f7b_9abe_4a74_a261_1eb76b55107a);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSC_SECURITY_PRODUCT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_STATE_ON: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_STATE_OFF: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_STATE_SNOOZED: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_STATE_EXPIRED: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(3i32);
impl ::core::marker::Copy for WSC_SECURITY_PRODUCT_STATE {}
impl ::core::clone::Clone for WSC_SECURITY_PRODUCT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSC_SECURITY_PRODUCT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WSC_SECURITY_PRODUCT_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSC_SECURITY_PRODUCT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_PRODUCT_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSC_SECURITY_PRODUCT_SUBSTATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NOT_SET: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(0i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NO_ACTION: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(1i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_RECOMMENDED: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(2i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_NEEDED: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(3i32);
impl ::core::marker::Copy for WSC_SECURITY_PRODUCT_SUBSTATUS {}
impl ::core::clone::Clone for WSC_SECURITY_PRODUCT_SUBSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSC_SECURITY_PRODUCT_SUBSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WSC_SECURITY_PRODUCT_SUBSTATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSC_SECURITY_PRODUCT_SUBSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_PRODUCT_SUBSTATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSC_SECURITY_PROVIDER(pub i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_FIREWALL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(1i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_AUTOUPDATE_SETTINGS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(2i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_ANTIVIRUS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(4i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_ANTISPYWARE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(8i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_INTERNET_SETTINGS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(16i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_USER_ACCOUNT_CONTROL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(32i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_SERVICE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(64i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_NONE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(0i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_ALL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(127i32);
impl ::core::marker::Copy for WSC_SECURITY_PROVIDER {}
impl ::core::clone::Clone for WSC_SECURITY_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSC_SECURITY_PROVIDER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WSC_SECURITY_PROVIDER {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSC_SECURITY_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_PROVIDER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSC_SECURITY_PROVIDER_HEALTH(pub i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_HEALTH_GOOD: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(0i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_HEALTH_NOTMONITORED: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(1i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_HEALTH_POOR: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(2i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_HEALTH_SNOOZE: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(3i32);
impl ::core::marker::Copy for WSC_SECURITY_PROVIDER_HEALTH {}
impl ::core::clone::Clone for WSC_SECURITY_PROVIDER_HEALTH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSC_SECURITY_PROVIDER_HEALTH {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WSC_SECURITY_PROVIDER_HEALTH {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSC_SECURITY_PROVIDER_HEALTH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_PROVIDER_HEALTH").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSC_SECURITY_SIGNATURE_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_OUT_OF_DATE: WSC_SECURITY_SIGNATURE_STATUS = WSC_SECURITY_SIGNATURE_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_UP_TO_DATE: WSC_SECURITY_SIGNATURE_STATUS = WSC_SECURITY_SIGNATURE_STATUS(1i32);
impl ::core::marker::Copy for WSC_SECURITY_SIGNATURE_STATUS {}
impl ::core::clone::Clone for WSC_SECURITY_SIGNATURE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSC_SECURITY_SIGNATURE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WSC_SECURITY_SIGNATURE_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSC_SECURITY_SIGNATURE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_SIGNATURE_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[inline]
pub unsafe fn WscGetAntiMalwareUri() -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WscGetAntiMalwareUri(ppszuri: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows::core::PWSTR>::zeroed();
        WscGetAntiMalwareUri(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[inline]
pub unsafe fn WscGetSecurityProviderHealth(providers: u32, phealth: *mut WSC_SECURITY_PROVIDER_HEALTH) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WscGetSecurityProviderHealth(providers: u32, phealth: *mut WSC_SECURITY_PROVIDER_HEALTH) -> ::windows::core::HRESULT;
        }
        WscGetSecurityProviderHealth(::core::mem::transmute(providers), ::core::mem::transmute(phealth)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[inline]
pub unsafe fn WscQueryAntiMalwareUri() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WscQueryAntiMalwareUri() -> ::windows::core::HRESULT;
        }
        WscQueryAntiMalwareUri().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
#[inline]
pub unsafe fn WscRegisterForChanges(reserved: *mut ::core::ffi::c_void, phcallbackregistration: *mut super::super::Foundation::HANDLE, lpcallbackaddress: super::Threading::LPTHREAD_START_ROUTINE, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WscRegisterForChanges(reserved: *mut ::core::ffi::c_void, phcallbackregistration: *mut super::super::Foundation::HANDLE, lpcallbackaddress: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        WscRegisterForChanges(::core::mem::transmute(reserved), ::core::mem::transmute(phcallbackregistration), ::core::mem::transmute(lpcallbackaddress), ::core::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[inline]
pub unsafe fn WscRegisterForUserNotifications() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WscRegisterForUserNotifications() -> ::windows::core::HRESULT;
        }
        WscRegisterForUserNotifications().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WscUnRegisterChanges<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hregistrationhandle: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WscUnRegisterChanges(hregistrationhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        WscUnRegisterChanges(hregistrationhandle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
