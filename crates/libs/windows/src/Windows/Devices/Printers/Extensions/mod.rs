#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DWorkflow(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrint3DWorkflow {
    type Vtable = IPrint3DWorkflow_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrint3DWorkflow {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc56f74bd_3669_4a66_ab42_c8151930cd34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflow_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPrintModelPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsPrintReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsPrintReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PrintRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrintRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrintRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrintRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DWorkflow2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrint3DWorkflow2 {
    type Vtable = IPrint3DWorkflow2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrint3DWorkflow2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2a6c54f_8ac1_4918_9741_e34f3004239e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflow2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PrinterChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrinterChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrinterChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrinterChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DWorkflowPrintRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrint3DWorkflowPrintRequestedEventArgs {
    type Vtable = IPrint3DWorkflowPrintRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrint3DWorkflowPrintRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19f8c858_5ac8_4b55_8a5f_e61567dafb4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflowPrintRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Print3DWorkflowStatus) -> ::windows::core::HRESULT,
    pub SetExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Print3DWorkflowDetail) -> ::windows::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSourceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DWorkflowPrinterChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrint3DWorkflowPrinterChangedEventArgs {
    type Vtable = IPrint3DWorkflowPrinterChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrint3DWorkflowPrinterChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45226402_95fc_4847_93b3_134dbf5c60f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflowPrinterChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NewDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintExtensionContextStatic(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrintExtensionContextStatic {
    type Vtable = IPrintExtensionContextStatic_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintExtensionContextStatic {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe70d9fc1_ff79_4aa4_8c9b_0c93aedfde8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintExtensionContextStatic_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FromDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintNotificationEventDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrintNotificationEventDetails {
    type Vtable = IPrintNotificationEventDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintNotificationEventDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe00e4c8a_4828_4da1_8bb8_8672df8515e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintNotificationEventDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PrinterName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EventData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetEventData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrintTaskConfiguration {
    type Vtable = IPrintTaskConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintTaskConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3c22451_3aa4_4885_9240_311f5f8fbe9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PrinterExtensionContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSaveRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSaveRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskConfigurationSaveRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrintTaskConfigurationSaveRequest {
    type Vtable = IPrintTaskConfigurationSaveRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintTaskConfigurationSaveRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeeaf2fcb_621e_4b62_ac77_b281cce08d60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskConfigurationSaveRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printerextensioncontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskConfigurationSaveRequestedDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrintTaskConfigurationSaveRequestedDeferral {
    type Vtable = IPrintTaskConfigurationSaveRequestedDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintTaskConfigurationSaveRequestedDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe959d568_f729_44a4_871d_bd0628696a33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskConfigurationSaveRequestedDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskConfigurationSaveRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrintTaskConfigurationSaveRequestedEventArgs {
    type Vtable = IPrintTaskConfigurationSaveRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintTaskConfigurationSaveRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06c2879_0d61_4938_91d0_96a45bee8479);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskConfigurationSaveRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
pub struct Print3DWorkflow(::windows::core::IUnknown);
impl Print3DWorkflow {
    pub fn DeviceID(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceID)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetPrintModelPackage(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPrintModelPackage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPrintReady(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPrintReady)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsPrintReady(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsPrintReady)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrintRequested(&self, eventhandler: &super::super::super::Foundation::TypedEventHandler<Print3DWorkflow, Print3DWorkflowPrintRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrintRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(eventhandler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrintRequested(&self, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePrintRequested)(::windows::core::Vtable::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrinterChanged(&self, eventhandler: &super::super::super::Foundation::TypedEventHandler<Print3DWorkflow, Print3DWorkflowPrinterChangedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IPrint3DWorkflow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrinterChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(eventhandler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrinterChanged(&self, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrint3DWorkflow2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePrinterChanged)(::windows::core::Vtable::as_raw(this), eventcookie).ok() }
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
impl ::core::fmt::Debug for Print3DWorkflow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DWorkflow").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Print3DWorkflow {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.Print3DWorkflow;{c56f74bd-3669-4a66-ab42-c8151930cd34})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Print3DWorkflow {
    type Vtable = IPrint3DWorkflow_Vtbl;
}
unsafe impl ::windows::core::Interface for Print3DWorkflow {
    const IID: ::windows::core::GUID = <IPrint3DWorkflow as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Print3DWorkflow {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.Print3DWorkflow";
}
::windows::core::interface_hierarchy!(Print3DWorkflow, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Print3DWorkflow {}
unsafe impl ::core::marker::Sync for Print3DWorkflow {}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
pub struct Print3DWorkflowPrintRequestedEventArgs(::windows::core::IUnknown);
impl Print3DWorkflowPrintRequestedEventArgs {
    pub fn Status(&self) -> ::windows::core::Result<Print3DWorkflowStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetExtendedStatus(&self, value: Print3DWorkflowDetail) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetExtendedStatus)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SetSource<P0>(&self, source: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSource)(::windows::core::Vtable::as_raw(this), source.into().abi()).ok() }
    }
    pub fn SetSourceChanged(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSourceChanged)(::windows::core::Vtable::as_raw(this), value).ok() }
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
impl ::core::fmt::Debug for Print3DWorkflowPrintRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DWorkflowPrintRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Print3DWorkflowPrintRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.Print3DWorkflowPrintRequestedEventArgs;{19f8c858-5ac8-4b55-8a5f-e61567dafb4d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Print3DWorkflowPrintRequestedEventArgs {
    type Vtable = IPrint3DWorkflowPrintRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for Print3DWorkflowPrintRequestedEventArgs {
    const IID: ::windows::core::GUID = <IPrint3DWorkflowPrintRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Print3DWorkflowPrintRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.Print3DWorkflowPrintRequestedEventArgs";
}
::windows::core::interface_hierarchy!(Print3DWorkflowPrintRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Print3DWorkflowPrintRequestedEventArgs {}
unsafe impl ::core::marker::Sync for Print3DWorkflowPrintRequestedEventArgs {}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
pub struct Print3DWorkflowPrinterChangedEventArgs(::windows::core::IUnknown);
impl Print3DWorkflowPrinterChangedEventArgs {
    pub fn NewDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewDeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
impl ::core::fmt::Debug for Print3DWorkflowPrinterChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DWorkflowPrinterChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Print3DWorkflowPrinterChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.Print3DWorkflowPrinterChangedEventArgs;{45226402-95fc-4847-93b3-134dbf5c60f7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Print3DWorkflowPrinterChangedEventArgs {
    type Vtable = IPrint3DWorkflowPrinterChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for Print3DWorkflowPrinterChangedEventArgs {
    const IID: ::windows::core::GUID = <IPrint3DWorkflowPrinterChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Print3DWorkflowPrinterChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.Print3DWorkflowPrinterChangedEventArgs";
}
::windows::core::interface_hierarchy!(Print3DWorkflowPrinterChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Print3DWorkflowPrinterChangedEventArgs {}
unsafe impl ::core::marker::Sync for Print3DWorkflowPrinterChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
pub struct PrintExtensionContext;
impl PrintExtensionContext {
    pub fn FromDeviceId(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPrintExtensionContextStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromDeviceId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrintExtensionContextStatic<R, F: FnOnce(&IPrintExtensionContextStatic) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PrintExtensionContext, IPrintExtensionContextStatic> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for PrintExtensionContext {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintExtensionContext";
}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
pub struct PrintNotificationEventDetails(::windows::core::IUnknown);
impl PrintNotificationEventDetails {
    pub fn PrinterName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrinterName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EventData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EventData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetEventData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetEventData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::core::fmt::Debug for PrintNotificationEventDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintNotificationEventDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintNotificationEventDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.PrintNotificationEventDetails;{e00e4c8a-4828-4da1-8bb8-8672df8515e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintNotificationEventDetails {
    type Vtable = IPrintNotificationEventDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintNotificationEventDetails {
    const IID: ::windows::core::GUID = <IPrintNotificationEventDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintNotificationEventDetails {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintNotificationEventDetails";
}
::windows::core::interface_hierarchy!(PrintNotificationEventDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintNotificationEventDetails {}
unsafe impl ::core::marker::Sync for PrintNotificationEventDetails {}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
pub struct PrintTaskConfiguration(::windows::core::IUnknown);
impl PrintTaskConfiguration {
    pub fn PrinterExtensionContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrinterExtensionContext)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveRequested(&self, eventhandler: &super::super::super::Foundation::TypedEventHandler<PrintTaskConfiguration, PrintTaskConfigurationSaveRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(eventhandler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSaveRequested(&self, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSaveRequested)(::windows::core::Vtable::as_raw(this), eventcookie).ok() }
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
impl ::core::fmt::Debug for PrintTaskConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.PrintTaskConfiguration;{e3c22451-3aa4-4885-9240-311f5f8fbe9d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintTaskConfiguration {
    type Vtable = IPrintTaskConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintTaskConfiguration {
    const IID: ::windows::core::GUID = <IPrintTaskConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTaskConfiguration {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintTaskConfiguration";
}
::windows::core::interface_hierarchy!(PrintTaskConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
pub struct PrintTaskConfigurationSaveRequest(::windows::core::IUnknown);
impl PrintTaskConfigurationSaveRequest {
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Cancel)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Save<P0>(&self, printerextensioncontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Save)(::windows::core::Vtable::as_raw(this), printerextensioncontext.into().abi()).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<PrintTaskConfigurationSaveRequestedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Deadline)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
impl ::core::fmt::Debug for PrintTaskConfigurationSaveRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskConfigurationSaveRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskConfigurationSaveRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequest;{eeaf2fcb-621e-4b62-ac77-b281cce08d60})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintTaskConfigurationSaveRequest {
    type Vtable = IPrintTaskConfigurationSaveRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintTaskConfigurationSaveRequest {
    const IID: ::windows::core::GUID = <IPrintTaskConfigurationSaveRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTaskConfigurationSaveRequest {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequest";
}
::windows::core::interface_hierarchy!(PrintTaskConfigurationSaveRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
pub struct PrintTaskConfigurationSaveRequestedDeferral(::windows::core::IUnknown);
impl PrintTaskConfigurationSaveRequestedDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Complete)(::windows::core::Vtable::as_raw(this)).ok() }
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
impl ::core::fmt::Debug for PrintTaskConfigurationSaveRequestedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskConfigurationSaveRequestedDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskConfigurationSaveRequestedDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedDeferral;{e959d568-f729-44a4-871d-bd0628696a33})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintTaskConfigurationSaveRequestedDeferral {
    type Vtable = IPrintTaskConfigurationSaveRequestedDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintTaskConfigurationSaveRequestedDeferral {
    const IID: ::windows::core::GUID = <IPrintTaskConfigurationSaveRequestedDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTaskConfigurationSaveRequestedDeferral {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedDeferral";
}
::windows::core::interface_hierarchy!(PrintTaskConfigurationSaveRequestedDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
pub struct PrintTaskConfigurationSaveRequestedEventArgs(::windows::core::IUnknown);
impl PrintTaskConfigurationSaveRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<PrintTaskConfigurationSaveRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
impl ::core::fmt::Debug for PrintTaskConfigurationSaveRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskConfigurationSaveRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskConfigurationSaveRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedEventArgs;{e06c2879-0d61-4938-91d0-96a45bee8479})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintTaskConfigurationSaveRequestedEventArgs {
    type Vtable = IPrintTaskConfigurationSaveRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintTaskConfigurationSaveRequestedEventArgs {
    const IID: ::windows::core::GUID = <IPrintTaskConfigurationSaveRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTaskConfigurationSaveRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedEventArgs";
}
::windows::core::interface_hierarchy!(PrintTaskConfigurationSaveRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for Print3DWorkflowDetail {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for Print3DWorkflowDetail {
    type Abi = Self;
}
impl ::core::fmt::Debug for Print3DWorkflowDetail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DWorkflowDetail").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Print3DWorkflowDetail {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.Extensions.Print3DWorkflowDetail;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for Print3DWorkflowStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for Print3DWorkflowStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for Print3DWorkflowStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DWorkflowStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Print3DWorkflowStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.Extensions.Print3DWorkflowStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
