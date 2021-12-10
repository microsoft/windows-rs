#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DWorkflow(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrint3DWorkflow {
    type Vtable = IPrint3DWorkflowVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc56f74bd_3669_4a66_ab42_c8151930cd34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflowVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DWorkflow2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrint3DWorkflow2 {
    type Vtable = IPrint3DWorkflow2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2a6c54f_8ac1_4918_9741_e34f3004239e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflow2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DWorkflowPrintRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrint3DWorkflowPrintRequestedEventArgs {
    type Vtable = IPrint3DWorkflowPrintRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19f8c858_5ac8_4b55_8a5f_e61567dafb4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflowPrintRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Print3DWorkflowStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Print3DWorkflowDetail) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DWorkflowPrinterChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrint3DWorkflowPrinterChangedEventArgs {
    type Vtable = IPrint3DWorkflowPrinterChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45226402_95fc_4847_93b3_134dbf5c60f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflowPrinterChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintExtensionContextStatic(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintExtensionContextStatic {
    type Vtable = IPrintExtensionContextStaticVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe70d9fc1_ff79_4aa4_8c9b_0c93aedfde8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintExtensionContextStaticVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintNotificationEventDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintNotificationEventDetails {
    type Vtable = IPrintNotificationEventDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe00e4c8a_4828_4da1_8bb8_8672df8515e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintNotificationEventDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskConfiguration {
    type Vtable = IPrintTaskConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3c22451_3aa4_4885_9240_311f5f8fbe9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskConfigurationSaveRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskConfigurationSaveRequest {
    type Vtable = IPrintTaskConfigurationSaveRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeeaf2fcb_621e_4b62_ac77_b281cce08d60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskConfigurationSaveRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printerextensioncontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskConfigurationSaveRequestedDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskConfigurationSaveRequestedDeferral {
    type Vtable = IPrintTaskConfigurationSaveRequestedDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe959d568_f729_44a4_871d_bd0628696a33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskConfigurationSaveRequestedDeferralVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskConfigurationSaveRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskConfigurationSaveRequestedEventArgs {
    type Vtable = IPrintTaskConfigurationSaveRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06c2879_0d61_4938_91d0_96a45bee8479);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskConfigurationSaveRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct Print3DWorkflow(::windows::core::IUnknown);
impl Print3DWorkflow {
    pub fn DeviceID(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetPrintModelPackage(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn IsPrintReady(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPrintReady(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PrintRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<Print3DWorkflow, Print3DWorkflowPrintRequestedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePrintRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PrinterChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<Print3DWorkflow, Print3DWorkflowPrinterChangedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IPrint3DWorkflow2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePrinterChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrint3DWorkflow2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for Print3DWorkflow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Print3DWorkflow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DWorkflow {}
unsafe impl ::windows::core::RuntimeType for Print3DWorkflow {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.Print3DWorkflow;{c56f74bd-3669-4a66-ab42-c8151930cd34})");
}
unsafe impl ::windows::core::Interface for Print3DWorkflow {
    type Vtable = IPrint3DWorkflowVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc56f74bd_3669_4a66_ab42_c8151930cd34);
}
impl ::windows::core::RuntimeName for Print3DWorkflow {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.Print3DWorkflow";
}
impl ::core::convert::From<Print3DWorkflow> for ::windows::core::IUnknown {
    fn from(value: Print3DWorkflow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflow> for ::windows::core::IUnknown {
    fn from(value: &Print3DWorkflow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Print3DWorkflow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Print3DWorkflow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Print3DWorkflow> for ::windows::core::IInspectable {
    fn from(value: Print3DWorkflow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflow> for ::windows::core::IInspectable {
    fn from(value: &Print3DWorkflow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Print3DWorkflow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Print3DWorkflow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Print3DWorkflow {}
unsafe impl ::core::marker::Sync for Print3DWorkflow {}
#[repr(transparent)]
pub struct Print3DWorkflowDetail(pub i32);
impl Print3DWorkflowDetail {
    pub const Unknown: Self = Self(0i32);
    pub const ModelExceedsPrintBed: Self = Self(1i32);
    pub const UploadFailed: Self = Self(2i32);
    pub const InvalidMaterialSelection: Self = Self(3i32);
    pub const InvalidModel: Self = Self(4i32);
    pub const ModelNotManifold: Self = Self(5i32);
    pub const InvalidPrintTicket: Self = Self(6i32);
}
impl ::core::marker::Copy for Print3DWorkflowDetail {}
impl ::core::clone::Clone for Print3DWorkflowDetail {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for Print3DWorkflowDetail {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for Print3DWorkflowDetail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DWorkflowDetail {}
unsafe impl ::windows::core::RuntimeType for Print3DWorkflowDetail {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.Extensions.Print3DWorkflowDetail;i4)");
}
impl ::windows::core::DefaultType for Print3DWorkflowDetail {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct Print3DWorkflowPrintRequestedEventArgs(::windows::core::IUnknown);
impl Print3DWorkflowPrintRequestedEventArgs {
    pub fn Status(&self) -> ::windows::core::Result<Print3DWorkflowStatus> {
        let this = self;
        unsafe {
            let mut result__: Print3DWorkflowStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Print3DWorkflowStatus>(result__)
        }
    }
    pub fn SetExtendedStatus(&self, value: Print3DWorkflowDetail) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SetSource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, source: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), source.into_param().abi()).ok() }
    }
    pub fn SetSourceChanged(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for Print3DWorkflowPrintRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Print3DWorkflowPrintRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DWorkflowPrintRequestedEventArgs {}
unsafe impl ::windows::core::RuntimeType for Print3DWorkflowPrintRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.Print3DWorkflowPrintRequestedEventArgs;{19f8c858-5ac8-4b55-8a5f-e61567dafb4d})");
}
unsafe impl ::windows::core::Interface for Print3DWorkflowPrintRequestedEventArgs {
    type Vtable = IPrint3DWorkflowPrintRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19f8c858_5ac8_4b55_8a5f_e61567dafb4d);
}
impl ::windows::core::RuntimeName for Print3DWorkflowPrintRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.Print3DWorkflowPrintRequestedEventArgs";
}
impl ::core::convert::From<Print3DWorkflowPrintRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: Print3DWorkflowPrintRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflowPrintRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &Print3DWorkflowPrintRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Print3DWorkflowPrintRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Print3DWorkflowPrintRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Print3DWorkflowPrintRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: Print3DWorkflowPrintRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflowPrintRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &Print3DWorkflowPrintRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Print3DWorkflowPrintRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Print3DWorkflowPrintRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Print3DWorkflowPrintRequestedEventArgs {}
unsafe impl ::core::marker::Sync for Print3DWorkflowPrintRequestedEventArgs {}
#[repr(transparent)]
pub struct Print3DWorkflowPrinterChangedEventArgs(::windows::core::IUnknown);
impl Print3DWorkflowPrinterChangedEventArgs {
    pub fn NewDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for Print3DWorkflowPrinterChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Print3DWorkflowPrinterChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DWorkflowPrinterChangedEventArgs {}
unsafe impl ::windows::core::RuntimeType for Print3DWorkflowPrinterChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.Print3DWorkflowPrinterChangedEventArgs;{45226402-95fc-4847-93b3-134dbf5c60f7})");
}
unsafe impl ::windows::core::Interface for Print3DWorkflowPrinterChangedEventArgs {
    type Vtable = IPrint3DWorkflowPrinterChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45226402_95fc_4847_93b3_134dbf5c60f7);
}
impl ::windows::core::RuntimeName for Print3DWorkflowPrinterChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.Print3DWorkflowPrinterChangedEventArgs";
}
impl ::core::convert::From<Print3DWorkflowPrinterChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: Print3DWorkflowPrinterChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflowPrinterChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &Print3DWorkflowPrinterChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Print3DWorkflowPrinterChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Print3DWorkflowPrinterChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Print3DWorkflowPrinterChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: Print3DWorkflowPrinterChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflowPrinterChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &Print3DWorkflowPrinterChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Print3DWorkflowPrinterChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Print3DWorkflowPrinterChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Print3DWorkflowPrinterChangedEventArgs {}
unsafe impl ::core::marker::Sync for Print3DWorkflowPrinterChangedEventArgs {}
#[repr(transparent)]
pub struct Print3DWorkflowStatus(pub i32);
impl Print3DWorkflowStatus {
    pub const Abandoned: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
    pub const Slicing: Self = Self(3i32);
    pub const Submitted: Self = Self(4i32);
}
impl ::core::marker::Copy for Print3DWorkflowStatus {}
impl ::core::clone::Clone for Print3DWorkflowStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for Print3DWorkflowStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for Print3DWorkflowStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DWorkflowStatus {}
unsafe impl ::windows::core::RuntimeType for Print3DWorkflowStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.Extensions.Print3DWorkflowStatus;i4)");
}
impl ::windows::core::DefaultType for Print3DWorkflowStatus {
    type DefaultType = Self;
}
pub struct PrintExtensionContext {}
impl PrintExtensionContext {
    pub fn FromDeviceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPrintExtensionContextStatic(|this| unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn IPrintExtensionContextStatic<R, F: FnOnce(&IPrintExtensionContextStatic) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PrintExtensionContext, IPrintExtensionContextStatic> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PrintExtensionContext {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintExtensionContext";
}
#[repr(transparent)]
pub struct PrintNotificationEventDetails(::windows::core::IUnknown);
impl PrintNotificationEventDetails {
    pub fn PrinterName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn EventData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetEventData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for PrintNotificationEventDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintNotificationEventDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintNotificationEventDetails {}
unsafe impl ::windows::core::RuntimeType for PrintNotificationEventDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.PrintNotificationEventDetails;{e00e4c8a-4828-4da1-8bb8-8672df8515e7})");
}
unsafe impl ::windows::core::Interface for PrintNotificationEventDetails {
    type Vtable = IPrintNotificationEventDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe00e4c8a_4828_4da1_8bb8_8672df8515e7);
}
impl ::windows::core::RuntimeName for PrintNotificationEventDetails {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintNotificationEventDetails";
}
impl ::core::convert::From<PrintNotificationEventDetails> for ::windows::core::IUnknown {
    fn from(value: PrintNotificationEventDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintNotificationEventDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintNotificationEventDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintNotificationEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PrintNotificationEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintNotificationEventDetails> for ::windows::core::IInspectable {
    fn from(value: PrintNotificationEventDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintNotificationEventDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintNotificationEventDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintNotificationEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PrintNotificationEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintNotificationEventDetails {}
unsafe impl ::core::marker::Sync for PrintNotificationEventDetails {}
#[repr(transparent)]
pub struct PrintTaskConfiguration(::windows::core::IUnknown);
impl PrintTaskConfiguration {
    pub fn PrinterExtensionContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SaveRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PrintTaskConfiguration, PrintTaskConfigurationSaveRequestedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSaveRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for PrintTaskConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskConfiguration {}
unsafe impl ::windows::core::RuntimeType for PrintTaskConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.PrintTaskConfiguration;{e3c22451-3aa4-4885-9240-311f5f8fbe9d})");
}
unsafe impl ::windows::core::Interface for PrintTaskConfiguration {
    type Vtable = IPrintTaskConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3c22451_3aa4_4885_9240_311f5f8fbe9d);
}
impl ::windows::core::RuntimeName for PrintTaskConfiguration {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintTaskConfiguration";
}
impl ::core::convert::From<PrintTaskConfiguration> for ::windows::core::IUnknown {
    fn from(value: PrintTaskConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskConfiguration> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTaskConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PrintTaskConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintTaskConfiguration> for ::windows::core::IInspectable {
    fn from(value: PrintTaskConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskConfiguration> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTaskConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PrintTaskConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct PrintTaskConfigurationSaveRequest(::windows::core::IUnknown);
impl PrintTaskConfigurationSaveRequest {
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Save<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, printerextensioncontext: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), printerextensioncontext.into_param().abi()).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<PrintTaskConfigurationSaveRequestedDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTaskConfigurationSaveRequestedDeferral>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTaskConfigurationSaveRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskConfigurationSaveRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskConfigurationSaveRequest {}
unsafe impl ::windows::core::RuntimeType for PrintTaskConfigurationSaveRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequest;{eeaf2fcb-621e-4b62-ac77-b281cce08d60})");
}
unsafe impl ::windows::core::Interface for PrintTaskConfigurationSaveRequest {
    type Vtable = IPrintTaskConfigurationSaveRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeeaf2fcb_621e_4b62_ac77_b281cce08d60);
}
impl ::windows::core::RuntimeName for PrintTaskConfigurationSaveRequest {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequest";
}
impl ::core::convert::From<PrintTaskConfigurationSaveRequest> for ::windows::core::IUnknown {
    fn from(value: PrintTaskConfigurationSaveRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskConfigurationSaveRequest> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskConfigurationSaveRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTaskConfigurationSaveRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PrintTaskConfigurationSaveRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintTaskConfigurationSaveRequest> for ::windows::core::IInspectable {
    fn from(value: PrintTaskConfigurationSaveRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskConfigurationSaveRequest> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskConfigurationSaveRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTaskConfigurationSaveRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PrintTaskConfigurationSaveRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct PrintTaskConfigurationSaveRequestedDeferral(::windows::core::IUnknown);
impl PrintTaskConfigurationSaveRequestedDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for PrintTaskConfigurationSaveRequestedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskConfigurationSaveRequestedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskConfigurationSaveRequestedDeferral {}
unsafe impl ::windows::core::RuntimeType for PrintTaskConfigurationSaveRequestedDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedDeferral;{e959d568-f729-44a4-871d-bd0628696a33})");
}
unsafe impl ::windows::core::Interface for PrintTaskConfigurationSaveRequestedDeferral {
    type Vtable = IPrintTaskConfigurationSaveRequestedDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe959d568_f729_44a4_871d_bd0628696a33);
}
impl ::windows::core::RuntimeName for PrintTaskConfigurationSaveRequestedDeferral {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedDeferral";
}
impl ::core::convert::From<PrintTaskConfigurationSaveRequestedDeferral> for ::windows::core::IUnknown {
    fn from(value: PrintTaskConfigurationSaveRequestedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskConfigurationSaveRequestedDeferral> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskConfigurationSaveRequestedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTaskConfigurationSaveRequestedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PrintTaskConfigurationSaveRequestedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintTaskConfigurationSaveRequestedDeferral> for ::windows::core::IInspectable {
    fn from(value: PrintTaskConfigurationSaveRequestedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskConfigurationSaveRequestedDeferral> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskConfigurationSaveRequestedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTaskConfigurationSaveRequestedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PrintTaskConfigurationSaveRequestedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct PrintTaskConfigurationSaveRequestedEventArgs(::windows::core::IUnknown);
impl PrintTaskConfigurationSaveRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<PrintTaskConfigurationSaveRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTaskConfigurationSaveRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTaskConfigurationSaveRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskConfigurationSaveRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskConfigurationSaveRequestedEventArgs {}
unsafe impl ::windows::core::RuntimeType for PrintTaskConfigurationSaveRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedEventArgs;{e06c2879-0d61-4938-91d0-96a45bee8479})");
}
unsafe impl ::windows::core::Interface for PrintTaskConfigurationSaveRequestedEventArgs {
    type Vtable = IPrintTaskConfigurationSaveRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06c2879_0d61_4938_91d0_96a45bee8479);
}
impl ::windows::core::RuntimeName for PrintTaskConfigurationSaveRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedEventArgs";
}
impl ::core::convert::From<PrintTaskConfigurationSaveRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintTaskConfigurationSaveRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskConfigurationSaveRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskConfigurationSaveRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTaskConfigurationSaveRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PrintTaskConfigurationSaveRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintTaskConfigurationSaveRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintTaskConfigurationSaveRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskConfigurationSaveRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskConfigurationSaveRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTaskConfigurationSaveRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PrintTaskConfigurationSaveRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
