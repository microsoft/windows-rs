#[cfg(feature = "Graphics_Printing_OptionDetails")]
pub mod OptionDetails;
#[cfg(feature = "Graphics_Printing_PrintSupport")]
pub mod PrintSupport;
#[cfg(feature = "Graphics_Printing_PrintTicket")]
pub mod PrintTicket;
#[cfg(feature = "Graphics_Printing_Workflow")]
pub mod Workflow;
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintDocumentSource(::windows::core::IUnknown);
impl IPrintDocumentSource {}
impl ::core::convert::From<IPrintDocumentSource> for ::windows::core::IUnknown {
    fn from(value: IPrintDocumentSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintDocumentSource> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrintDocumentSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintDocumentSource> for ::windows::core::IUnknown {
    fn from(value: &IPrintDocumentSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IPrintDocumentSource> for ::windows::core::IInspectable {
    fn from(value: IPrintDocumentSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintDocumentSource> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IPrintDocumentSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintDocumentSource> for ::windows::core::IInspectable {
    fn from(value: &IPrintDocumentSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPrintDocumentSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintDocumentSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintDocumentSource {}
impl ::core::fmt::Debug for IPrintDocumentSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintDocumentSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPrintDocumentSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{dedc0c30-f1eb-47df-aae6-ed5427511f01}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IPrintDocumentSource {
    type Vtable = IPrintDocumentSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdedc0c30_f1eb_47df_aae6_ed5427511f01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentSource_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintManager {
    type Vtable = IPrintManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff2a9694_8c99_44fd_ae4a_19d9aa9a0f0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManager_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PrintTaskRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrintTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrintTaskRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrintTaskRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintManagerStatic(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintManagerStatic {
    type Vtable = IPrintManagerStatic_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58185dcd_e634_4654_84f0_e0152a8217ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManagerStatic_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShowPrintUIAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowPrintUIAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintManagerStatic2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintManagerStatic2 {
    type Vtable = IPrintManagerStatic2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35a99955_e6ab_4139_9abd_b86a729b3598);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManagerStatic2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintPageInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintPageInfo {
    type Vtable = IPrintPageInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd4be9c9_a6a1_4ada_930e_da872a4f23d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPageInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SetMediaSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintMediaSize) -> ::windows::core::HRESULT,
    pub MediaSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintMediaSize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPageSize: usize,
    #[cfg(feature = "Foundation")]
    pub PageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PageSize: usize,
    pub SetDpiX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub DpiX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetDpiY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub DpiY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintOrientation) -> ::windows::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintOrientation) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintPageRange(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintPageRange {
    type Vtable = IPrintPageRange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8a06c54_6e7c_51c5_57fd_0660c2d71513);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPageRange_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub FirstPageNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub LastPageNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintPageRangeFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintPageRangeFactory {
    type Vtable = IPrintPageRangeFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x408fd45f_e047_5f85_7129_fb085a4fad14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPageRangeFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, firstpage: i32, lastpage: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithSinglePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintPageRangeOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintPageRangeOptions {
    type Vtable = IPrintPageRangeOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce6db728_1357_46b2_a923_79f995f448fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPageRangeOptions_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SetAllowAllPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowAllPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowCurrentPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowCurrentPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowCustomSetOfPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowCustomSetOfPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTask(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTask {
    type Vtable = IPrintTask_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61d80247_6cf6_4fad_84e2_a5e82e2d4ceb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTask_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    Properties: usize,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Previewing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Previewing: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePreviewing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePreviewing: usize,
    #[cfg(feature = "Foundation")]
    pub Submitting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Submitting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSubmitting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSubmitting: usize,
    #[cfg(feature = "Foundation")]
    pub Progressing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Progressing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProgressing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProgressing: usize,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTask2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTask2 {
    type Vtable = IPrintTask2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36234877_3e53_4d9d_8f5e_316ac8dedae1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTask2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SetIsPreviewEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsPreviewEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskCompletedEventArgs {
    type Vtable = IPrintTaskCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bcd34af_24e9_4c10_8d07_14c346ba3fce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Completion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintTaskCompletion) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskOptions {
    type Vtable = IPrintTaskOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a0a66bb_d289_41bb_96dd_57e28338ae3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptions_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SetBordering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintBordering) -> ::windows::core::HRESULT,
    pub Bordering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintBordering) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetPagePrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printpageinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetPagePrintTicket: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskOptions2 {
    type Vtable = IPrintTaskOptions2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb9b1606_9a36_4b59_8617_b217849262e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptions2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PageRangeOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CustomPageRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CustomPageRanges: usize,
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintTaskOptionsCore(::windows::core::IUnknown);
impl IPrintTaskOptionsCore {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPageDescription(&self, jobpagenumber: u32) -> ::windows::core::Result<PrintPageDescription> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetPageDescription)(::windows::core::Interface::as_raw(this), jobpagenumber, result__.as_mut_ptr()).from_abi::<PrintPageDescription>(result__)
        }
    }
}
impl ::core::convert::From<IPrintTaskOptionsCore> for ::windows::core::IUnknown {
    fn from(value: IPrintTaskOptionsCore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintTaskOptionsCore> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrintTaskOptionsCore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCore> for ::windows::core::IUnknown {
    fn from(value: &IPrintTaskOptionsCore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IPrintTaskOptionsCore> for ::windows::core::IInspectable {
    fn from(value: IPrintTaskOptionsCore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintTaskOptionsCore> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IPrintTaskOptionsCore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCore> for ::windows::core::IInspectable {
    fn from(value: &IPrintTaskOptionsCore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPrintTaskOptionsCore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintTaskOptionsCore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintTaskOptionsCore {}
impl ::core::fmt::Debug for IPrintTaskOptionsCore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintTaskOptionsCore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPrintTaskOptionsCore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1bdbb474-4ed1-41eb-be3c-72d18ed67337}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IPrintTaskOptionsCore {
    type Vtable = IPrintTaskOptionsCore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bdbb474_4ed1_41eb_be3c_72d18ed67337);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionsCore_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetPageDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobpagenumber: u32, result__: *mut PrintPageDescription) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPageDescription: usize,
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintTaskOptionsCoreProperties(::windows::core::IUnknown);
impl IPrintTaskOptionsCoreProperties {
    pub fn SetMediaSize(&self, value: PrintMediaSize) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMediaSize)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaSize(&self) -> ::windows::core::Result<PrintMediaSize> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MediaSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintMediaSize>(result__)
        }
    }
    pub fn SetMediaType(&self, value: PrintMediaType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMediaType)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaType(&self) -> ::windows::core::Result<PrintMediaType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MediaType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintMediaType>(result__)
        }
    }
    pub fn SetOrientation(&self, value: PrintOrientation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOrientation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Orientation(&self) -> ::windows::core::Result<PrintOrientation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Orientation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintOrientation>(result__)
        }
    }
    pub fn SetPrintQuality(&self, value: PrintQuality) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPrintQuality)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn PrintQuality(&self) -> ::windows::core::Result<PrintQuality> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrintQuality)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintQuality>(result__)
        }
    }
    pub fn SetColorMode(&self, value: PrintColorMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetColorMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ColorMode(&self) -> ::windows::core::Result<PrintColorMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ColorMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintColorMode>(result__)
        }
    }
    pub fn SetDuplex(&self, value: PrintDuplex) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDuplex)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Duplex(&self) -> ::windows::core::Result<PrintDuplex> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Duplex)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintDuplex>(result__)
        }
    }
    pub fn SetCollation(&self, value: PrintCollation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCollation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Collation(&self) -> ::windows::core::Result<PrintCollation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Collation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintCollation>(result__)
        }
    }
    pub fn SetStaple(&self, value: PrintStaple) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStaple)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Staple(&self) -> ::windows::core::Result<PrintStaple> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Staple)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintStaple>(result__)
        }
    }
    pub fn SetHolePunch(&self, value: PrintHolePunch) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHolePunch)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn HolePunch(&self) -> ::windows::core::Result<PrintHolePunch> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HolePunch)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintHolePunch>(result__)
        }
    }
    pub fn SetBinding(&self, value: PrintBinding) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBinding)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Binding(&self) -> ::windows::core::Result<PrintBinding> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Binding)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintBinding>(result__)
        }
    }
    pub fn MinCopies(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MinCopies)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MaxCopies(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxCopies)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetNumberOfCopies(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNumberOfCopies)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumberOfCopies(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NumberOfCopies)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::convert::From<IPrintTaskOptionsCoreProperties> for ::windows::core::IUnknown {
    fn from(value: IPrintTaskOptionsCoreProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintTaskOptionsCoreProperties> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrintTaskOptionsCoreProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCoreProperties> for ::windows::core::IUnknown {
    fn from(value: &IPrintTaskOptionsCoreProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IPrintTaskOptionsCoreProperties> for ::windows::core::IInspectable {
    fn from(value: IPrintTaskOptionsCoreProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintTaskOptionsCoreProperties> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IPrintTaskOptionsCoreProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCoreProperties> for ::windows::core::IInspectable {
    fn from(value: &IPrintTaskOptionsCoreProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPrintTaskOptionsCoreProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintTaskOptionsCoreProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintTaskOptionsCoreProperties {}
impl ::core::fmt::Debug for IPrintTaskOptionsCoreProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintTaskOptionsCoreProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPrintTaskOptionsCoreProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c1b71832-9e93-4e55-814b-3326a59efce1}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IPrintTaskOptionsCoreProperties {
    type Vtable = IPrintTaskOptionsCoreProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1b71832_9e93_4e55_814b_3326a59efce1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionsCoreProperties_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SetMediaSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintMediaSize) -> ::windows::core::HRESULT,
    pub MediaSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintMediaSize) -> ::windows::core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintMediaType) -> ::windows::core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintMediaType) -> ::windows::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintOrientation) -> ::windows::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintOrientation) -> ::windows::core::HRESULT,
    pub SetPrintQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintQuality) -> ::windows::core::HRESULT,
    pub PrintQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintQuality) -> ::windows::core::HRESULT,
    pub SetColorMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintColorMode) -> ::windows::core::HRESULT,
    pub ColorMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintColorMode) -> ::windows::core::HRESULT,
    pub SetDuplex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintDuplex) -> ::windows::core::HRESULT,
    pub Duplex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintDuplex) -> ::windows::core::HRESULT,
    pub SetCollation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintCollation) -> ::windows::core::HRESULT,
    pub Collation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintCollation) -> ::windows::core::HRESULT,
    pub SetStaple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintStaple) -> ::windows::core::HRESULT,
    pub Staple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintStaple) -> ::windows::core::HRESULT,
    pub SetHolePunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintHolePunch) -> ::windows::core::HRESULT,
    pub HolePunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintHolePunch) -> ::windows::core::HRESULT,
    pub SetBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintBinding) -> ::windows::core::HRESULT,
    pub Binding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintBinding) -> ::windows::core::HRESULT,
    pub MinCopies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MaxCopies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetNumberOfCopies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub NumberOfCopies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintTaskOptionsCoreUIConfiguration(::windows::core::IUnknown);
impl IPrintTaskOptionsCoreUIConfiguration {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DisplayedOptions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayedOptions)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
impl ::core::convert::From<IPrintTaskOptionsCoreUIConfiguration> for ::windows::core::IUnknown {
    fn from(value: IPrintTaskOptionsCoreUIConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintTaskOptionsCoreUIConfiguration> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrintTaskOptionsCoreUIConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCoreUIConfiguration> for ::windows::core::IUnknown {
    fn from(value: &IPrintTaskOptionsCoreUIConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IPrintTaskOptionsCoreUIConfiguration> for ::windows::core::IInspectable {
    fn from(value: IPrintTaskOptionsCoreUIConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintTaskOptionsCoreUIConfiguration> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IPrintTaskOptionsCoreUIConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCoreUIConfiguration> for ::windows::core::IInspectable {
    fn from(value: &IPrintTaskOptionsCoreUIConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPrintTaskOptionsCoreUIConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintTaskOptionsCoreUIConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintTaskOptionsCoreUIConfiguration {}
impl ::core::fmt::Debug for IPrintTaskOptionsCoreUIConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintTaskOptionsCoreUIConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPrintTaskOptionsCoreUIConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{62e69e23-9a1e-4336-b74f-3cc7f4cff709}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IPrintTaskOptionsCoreUIConfiguration {
    type Vtable = IPrintTaskOptionsCoreUIConfiguration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62e69e23_9a1e_4336_b74f_3cc7f4cff709);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionsCoreUIConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub DisplayedOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DisplayedOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskProgressingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskProgressingEventArgs {
    type Vtable = IPrintTaskProgressingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x810cd3cb_b410_4282_a073_5ac378234174);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskProgressingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DocumentPageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskRequest {
    type Vtable = IPrintTaskRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ff61e2e_2722_4240_a67c_f364849a17f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskRequest_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    pub CreatePrintTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskRequestedDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskRequestedDeferral {
    type Vtable = IPrintTaskRequestedDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfefb3f0_ce3e_42c7_9496_64800c622c44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskRequestedDeferral_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskRequestedEventArgs {
    type Vtable = IPrintTaskRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0aff924_a31b_454c_a7b6_5d0cc522fc16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskSourceRequestedArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskSourceRequestedArgs {
    type Vtable = IPrintTaskSourceRequestedArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9f067be_f456_41f0_9c98_5ce73e851410);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskSourceRequestedArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskSourceRequestedDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskSourceRequestedDeferral {
    type Vtable = IPrintTaskSourceRequestedDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a1560d1_6992_4d9d_8555_4ca4563fb166);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskSourceRequestedDeferral_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskTargetDeviceSupport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskTargetDeviceSupport {
    type Vtable = IPrintTaskTargetDeviceSupport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x295d70c0_c2cb_4b7d_b0ea_93095091a220);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskTargetDeviceSupport_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SetIsPrinterTargetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsPrinterTargetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIs3DManufacturingTargetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Is3DManufacturingTargetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStandardPrintTaskOptionsStatic(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStandardPrintTaskOptionsStatic {
    type Vtable = IStandardPrintTaskOptionsStatic_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4483d26_0dd0_4cd4_baff_930fc7d6a574);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardPrintTaskOptionsStatic_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub MediaSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PrintQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ColorMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Duplex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Collation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Staple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HolePunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Binding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Copies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub InputBin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStandardPrintTaskOptionsStatic2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStandardPrintTaskOptionsStatic2 {
    type Vtable = IStandardPrintTaskOptionsStatic2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3be38bf4_7a44_4269_9a52_81261e289ee9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardPrintTaskOptionsStatic2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Bordering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStandardPrintTaskOptionsStatic3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStandardPrintTaskOptionsStatic3 {
    type Vtable = IStandardPrintTaskOptionsStatic3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbf68e86_3858_41b3_a799_55dd9888d475);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardPrintTaskOptionsStatic3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CustomPageRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintBinding(pub i32);
impl PrintBinding {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const None: Self = Self(3i32);
    pub const Bale: Self = Self(4i32);
    pub const BindBottom: Self = Self(5i32);
    pub const BindLeft: Self = Self(6i32);
    pub const BindRight: Self = Self(7i32);
    pub const BindTop: Self = Self(8i32);
    pub const Booklet: Self = Self(9i32);
    pub const EdgeStitchBottom: Self = Self(10i32);
    pub const EdgeStitchLeft: Self = Self(11i32);
    pub const EdgeStitchRight: Self = Self(12i32);
    pub const EdgeStitchTop: Self = Self(13i32);
    pub const Fold: Self = Self(14i32);
    pub const JogOffset: Self = Self(15i32);
    pub const Trim: Self = Self(16i32);
}
impl ::core::marker::Copy for PrintBinding {}
impl ::core::clone::Clone for PrintBinding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintBinding {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PrintBinding {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintBinding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintBinding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintBinding;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintBordering(pub i32);
impl PrintBordering {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Bordered: Self = Self(3i32);
    pub const Borderless: Self = Self(4i32);
}
impl ::core::marker::Copy for PrintBordering {}
impl ::core::clone::Clone for PrintBordering {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintBordering {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PrintBordering {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintBordering {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintBordering").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintBordering {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintBordering;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintCollation(pub i32);
impl PrintCollation {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Collated: Self = Self(3i32);
    pub const Uncollated: Self = Self(4i32);
}
impl ::core::marker::Copy for PrintCollation {}
impl ::core::clone::Clone for PrintCollation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintCollation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PrintCollation {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintCollation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintCollation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintCollation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintCollation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintColorMode(pub i32);
impl PrintColorMode {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Color: Self = Self(3i32);
    pub const Grayscale: Self = Self(4i32);
    pub const Monochrome: Self = Self(5i32);
}
impl ::core::marker::Copy for PrintColorMode {}
impl ::core::clone::Clone for PrintColorMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintColorMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PrintColorMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintColorMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintColorMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintColorMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintColorMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintDuplex(pub i32);
impl PrintDuplex {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const OneSided: Self = Self(3i32);
    pub const TwoSidedShortEdge: Self = Self(4i32);
    pub const TwoSidedLongEdge: Self = Self(5i32);
}
impl ::core::marker::Copy for PrintDuplex {}
impl ::core::clone::Clone for PrintDuplex {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintDuplex {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PrintDuplex {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintDuplex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintDuplex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintDuplex {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintDuplex;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintHolePunch(pub i32);
impl PrintHolePunch {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const None: Self = Self(3i32);
    pub const LeftEdge: Self = Self(4i32);
    pub const RightEdge: Self = Self(5i32);
    pub const TopEdge: Self = Self(6i32);
    pub const BottomEdge: Self = Self(7i32);
}
impl ::core::marker::Copy for PrintHolePunch {}
impl ::core::clone::Clone for PrintHolePunch {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintHolePunch {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PrintHolePunch {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintHolePunch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintHolePunch").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintHolePunch {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintHolePunch;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintManager(::windows::core::IUnknown);
impl PrintManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrintTaskRequested<'a, P0>(&self, eventhandler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PrintManager, PrintTaskRequestedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrintTaskRequested)(::windows::core::Interface::as_raw(this), eventhandler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrintTaskRequested(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePrintTaskRequested)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<PrintManager> {
        Self::IPrintManagerStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintManager>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowPrintUIAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IPrintManagerStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ShowPrintUIAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IPrintManagerStatic2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrintManagerStatic<R, F: FnOnce(&IPrintManagerStatic) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PrintManager, IPrintManagerStatic> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPrintManagerStatic2<R, F: FnOnce(&IPrintManagerStatic2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PrintManager, IPrintManagerStatic2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PrintManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintManager {}
impl ::core::fmt::Debug for PrintManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintManager;{ff2a9694-8c99-44fd-ae4a-19d9aa9a0f0a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintManager {
    type Vtable = IPrintManager_Vtbl;
    const IID: ::windows::core::GUID = <IPrintManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintManager {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintManager";
}
impl ::core::convert::From<PrintManager> for ::windows::core::IUnknown {
    fn from(value: PrintManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintManager> for ::windows::core::IUnknown {
    fn from(value: &PrintManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintManager> for &::windows::core::IUnknown {
    fn from(value: &PrintManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintManager> for ::windows::core::IInspectable {
    fn from(value: PrintManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintManager> for ::windows::core::IInspectable {
    fn from(value: &PrintManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintManager> for &::windows::core::IInspectable {
    fn from(value: &PrintManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PrintManager {}
unsafe impl ::core::marker::Sync for PrintManager {}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintMediaSize(pub i32);
impl PrintMediaSize {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const BusinessCard: Self = Self(3i32);
    pub const CreditCard: Self = Self(4i32);
    pub const IsoA0: Self = Self(5i32);
    pub const IsoA1: Self = Self(6i32);
    pub const IsoA10: Self = Self(7i32);
    pub const IsoA2: Self = Self(8i32);
    pub const IsoA3: Self = Self(9i32);
    pub const IsoA3Extra: Self = Self(10i32);
    pub const IsoA3Rotated: Self = Self(11i32);
    pub const IsoA4: Self = Self(12i32);
    pub const IsoA4Extra: Self = Self(13i32);
    pub const IsoA4Rotated: Self = Self(14i32);
    pub const IsoA5: Self = Self(15i32);
    pub const IsoA5Extra: Self = Self(16i32);
    pub const IsoA5Rotated: Self = Self(17i32);
    pub const IsoA6: Self = Self(18i32);
    pub const IsoA6Rotated: Self = Self(19i32);
    pub const IsoA7: Self = Self(20i32);
    pub const IsoA8: Self = Self(21i32);
    pub const IsoA9: Self = Self(22i32);
    pub const IsoB0: Self = Self(23i32);
    pub const IsoB1: Self = Self(24i32);
    pub const IsoB10: Self = Self(25i32);
    pub const IsoB2: Self = Self(26i32);
    pub const IsoB3: Self = Self(27i32);
    pub const IsoB4: Self = Self(28i32);
    pub const IsoB4Envelope: Self = Self(29i32);
    pub const IsoB5Envelope: Self = Self(30i32);
    pub const IsoB5Extra: Self = Self(31i32);
    pub const IsoB7: Self = Self(32i32);
    pub const IsoB8: Self = Self(33i32);
    pub const IsoB9: Self = Self(34i32);
    pub const IsoC0: Self = Self(35i32);
    pub const IsoC1: Self = Self(36i32);
    pub const IsoC10: Self = Self(37i32);
    pub const IsoC2: Self = Self(38i32);
    pub const IsoC3: Self = Self(39i32);
    pub const IsoC3Envelope: Self = Self(40i32);
    pub const IsoC4: Self = Self(41i32);
    pub const IsoC4Envelope: Self = Self(42i32);
    pub const IsoC5: Self = Self(43i32);
    pub const IsoC5Envelope: Self = Self(44i32);
    pub const IsoC6: Self = Self(45i32);
    pub const IsoC6C5Envelope: Self = Self(46i32);
    pub const IsoC6Envelope: Self = Self(47i32);
    pub const IsoC7: Self = Self(48i32);
    pub const IsoC8: Self = Self(49i32);
    pub const IsoC9: Self = Self(50i32);
    pub const IsoDLEnvelope: Self = Self(51i32);
    pub const IsoDLEnvelopeRotated: Self = Self(52i32);
    pub const IsoSRA3: Self = Self(53i32);
    pub const Japan2LPhoto: Self = Self(54i32);
    pub const JapanChou3Envelope: Self = Self(55i32);
    pub const JapanChou3EnvelopeRotated: Self = Self(56i32);
    pub const JapanChou4Envelope: Self = Self(57i32);
    pub const JapanChou4EnvelopeRotated: Self = Self(58i32);
    pub const JapanDoubleHagakiPostcard: Self = Self(59i32);
    pub const JapanDoubleHagakiPostcardRotated: Self = Self(60i32);
    pub const JapanHagakiPostcard: Self = Self(61i32);
    pub const JapanHagakiPostcardRotated: Self = Self(62i32);
    pub const JapanKaku2Envelope: Self = Self(63i32);
    pub const JapanKaku2EnvelopeRotated: Self = Self(64i32);
    pub const JapanKaku3Envelope: Self = Self(65i32);
    pub const JapanKaku3EnvelopeRotated: Self = Self(66i32);
    pub const JapanLPhoto: Self = Self(67i32);
    pub const JapanQuadrupleHagakiPostcard: Self = Self(68i32);
    pub const JapanYou1Envelope: Self = Self(69i32);
    pub const JapanYou2Envelope: Self = Self(70i32);
    pub const JapanYou3Envelope: Self = Self(71i32);
    pub const JapanYou4Envelope: Self = Self(72i32);
    pub const JapanYou4EnvelopeRotated: Self = Self(73i32);
    pub const JapanYou6Envelope: Self = Self(74i32);
    pub const JapanYou6EnvelopeRotated: Self = Self(75i32);
    pub const JisB0: Self = Self(76i32);
    pub const JisB1: Self = Self(77i32);
    pub const JisB10: Self = Self(78i32);
    pub const JisB2: Self = Self(79i32);
    pub const JisB3: Self = Self(80i32);
    pub const JisB4: Self = Self(81i32);
    pub const JisB4Rotated: Self = Self(82i32);
    pub const JisB5: Self = Self(83i32);
    pub const JisB5Rotated: Self = Self(84i32);
    pub const JisB6: Self = Self(85i32);
    pub const JisB6Rotated: Self = Self(86i32);
    pub const JisB7: Self = Self(87i32);
    pub const JisB8: Self = Self(88i32);
    pub const JisB9: Self = Self(89i32);
    pub const NorthAmerica10x11: Self = Self(90i32);
    pub const NorthAmerica10x12: Self = Self(91i32);
    pub const NorthAmerica10x14: Self = Self(92i32);
    pub const NorthAmerica11x17: Self = Self(93i32);
    pub const NorthAmerica14x17: Self = Self(94i32);
    pub const NorthAmerica4x6: Self = Self(95i32);
    pub const NorthAmerica4x8: Self = Self(96i32);
    pub const NorthAmerica5x7: Self = Self(97i32);
    pub const NorthAmerica8x10: Self = Self(98i32);
    pub const NorthAmerica9x11: Self = Self(99i32);
    pub const NorthAmericaArchitectureASheet: Self = Self(100i32);
    pub const NorthAmericaArchitectureBSheet: Self = Self(101i32);
    pub const NorthAmericaArchitectureCSheet: Self = Self(102i32);
    pub const NorthAmericaArchitectureDSheet: Self = Self(103i32);
    pub const NorthAmericaArchitectureESheet: Self = Self(104i32);
    pub const NorthAmericaCSheet: Self = Self(105i32);
    pub const NorthAmericaDSheet: Self = Self(106i32);
    pub const NorthAmericaESheet: Self = Self(107i32);
    pub const NorthAmericaExecutive: Self = Self(108i32);
    pub const NorthAmericaGermanLegalFanfold: Self = Self(109i32);
    pub const NorthAmericaGermanStandardFanfold: Self = Self(110i32);
    pub const NorthAmericaLegal: Self = Self(111i32);
    pub const NorthAmericaLegalExtra: Self = Self(112i32);
    pub const NorthAmericaLetter: Self = Self(113i32);
    pub const NorthAmericaLetterExtra: Self = Self(114i32);
    pub const NorthAmericaLetterPlus: Self = Self(115i32);
    pub const NorthAmericaLetterRotated: Self = Self(116i32);
    pub const NorthAmericaMonarchEnvelope: Self = Self(117i32);
    pub const NorthAmericaNote: Self = Self(118i32);
    pub const NorthAmericaNumber10Envelope: Self = Self(119i32);
    pub const NorthAmericaNumber10EnvelopeRotated: Self = Self(120i32);
    pub const NorthAmericaNumber11Envelope: Self = Self(121i32);
    pub const NorthAmericaNumber12Envelope: Self = Self(122i32);
    pub const NorthAmericaNumber14Envelope: Self = Self(123i32);
    pub const NorthAmericaNumber9Envelope: Self = Self(124i32);
    pub const NorthAmericaPersonalEnvelope: Self = Self(125i32);
    pub const NorthAmericaQuarto: Self = Self(126i32);
    pub const NorthAmericaStatement: Self = Self(127i32);
    pub const NorthAmericaSuperA: Self = Self(128i32);
    pub const NorthAmericaSuperB: Self = Self(129i32);
    pub const NorthAmericaTabloid: Self = Self(130i32);
    pub const NorthAmericaTabloidExtra: Self = Self(131i32);
    pub const OtherMetricA3Plus: Self = Self(132i32);
    pub const OtherMetricA4Plus: Self = Self(133i32);
    pub const OtherMetricFolio: Self = Self(134i32);
    pub const OtherMetricInviteEnvelope: Self = Self(135i32);
    pub const OtherMetricItalianEnvelope: Self = Self(136i32);
    pub const Prc10Envelope: Self = Self(137i32);
    pub const Prc10EnvelopeRotated: Self = Self(138i32);
    pub const Prc16K: Self = Self(139i32);
    pub const Prc16KRotated: Self = Self(140i32);
    pub const Prc1Envelope: Self = Self(141i32);
    pub const Prc1EnvelopeRotated: Self = Self(142i32);
    pub const Prc2Envelope: Self = Self(143i32);
    pub const Prc2EnvelopeRotated: Self = Self(144i32);
    pub const Prc32K: Self = Self(145i32);
    pub const Prc32KBig: Self = Self(146i32);
    pub const Prc32KRotated: Self = Self(147i32);
    pub const Prc3Envelope: Self = Self(148i32);
    pub const Prc3EnvelopeRotated: Self = Self(149i32);
    pub const Prc4Envelope: Self = Self(150i32);
    pub const Prc4EnvelopeRotated: Self = Self(151i32);
    pub const Prc5Envelope: Self = Self(152i32);
    pub const Prc5EnvelopeRotated: Self = Self(153i32);
    pub const Prc6Envelope: Self = Self(154i32);
    pub const Prc6EnvelopeRotated: Self = Self(155i32);
    pub const Prc7Envelope: Self = Self(156i32);
    pub const Prc7EnvelopeRotated: Self = Self(157i32);
    pub const Prc8Envelope: Self = Self(158i32);
    pub const Prc8EnvelopeRotated: Self = Self(159i32);
    pub const Prc9Envelope: Self = Self(160i32);
    pub const Prc9EnvelopeRotated: Self = Self(161i32);
    pub const Roll04Inch: Self = Self(162i32);
    pub const Roll06Inch: Self = Self(163i32);
    pub const Roll08Inch: Self = Self(164i32);
    pub const Roll12Inch: Self = Self(165i32);
    pub const Roll15Inch: Self = Self(166i32);
    pub const Roll18Inch: Self = Self(167i32);
    pub const Roll22Inch: Self = Self(168i32);
    pub const Roll24Inch: Self = Self(169i32);
    pub const Roll30Inch: Self = Self(170i32);
    pub const Roll36Inch: Self = Self(171i32);
    pub const Roll54Inch: Self = Self(172i32);
}
impl ::core::marker::Copy for PrintMediaSize {}
impl ::core::clone::Clone for PrintMediaSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintMediaSize {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PrintMediaSize {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintMediaSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintMediaSize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintMediaSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintMediaSize;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintMediaType(pub i32);
impl PrintMediaType {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const AutoSelect: Self = Self(3i32);
    pub const Archival: Self = Self(4i32);
    pub const BackPrintFilm: Self = Self(5i32);
    pub const Bond: Self = Self(6i32);
    pub const CardStock: Self = Self(7i32);
    pub const Continuous: Self = Self(8i32);
    pub const EnvelopePlain: Self = Self(9i32);
    pub const EnvelopeWindow: Self = Self(10i32);
    pub const Fabric: Self = Self(11i32);
    pub const HighResolution: Self = Self(12i32);
    pub const Label: Self = Self(13i32);
    pub const MultiLayerForm: Self = Self(14i32);
    pub const MultiPartForm: Self = Self(15i32);
    pub const Photographic: Self = Self(16i32);
    pub const PhotographicFilm: Self = Self(17i32);
    pub const PhotographicGlossy: Self = Self(18i32);
    pub const PhotographicHighGloss: Self = Self(19i32);
    pub const PhotographicMatte: Self = Self(20i32);
    pub const PhotographicSatin: Self = Self(21i32);
    pub const PhotographicSemiGloss: Self = Self(22i32);
    pub const Plain: Self = Self(23i32);
    pub const Screen: Self = Self(24i32);
    pub const ScreenPaged: Self = Self(25i32);
    pub const Stationery: Self = Self(26i32);
    pub const TabStockFull: Self = Self(27i32);
    pub const TabStockPreCut: Self = Self(28i32);
    pub const Transparency: Self = Self(29i32);
    pub const TShirtTransfer: Self = Self(30i32);
    pub const None: Self = Self(31i32);
}
impl ::core::marker::Copy for PrintMediaType {}
impl ::core::clone::Clone for PrintMediaType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintMediaType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PrintMediaType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintMediaType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintMediaType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintMediaType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintMediaType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintOrientation(pub i32);
impl PrintOrientation {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Portrait: Self = Self(3i32);
    pub const PortraitFlipped: Self = Self(4i32);
    pub const Landscape: Self = Self(5i32);
    pub const LandscapeFlipped: Self = Self(6i32);
}
impl ::core::marker::Copy for PrintOrientation {}
impl ::core::clone::Clone for PrintOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintOrientation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PrintOrientation {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintOrientation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintOrientation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintOrientation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Printing\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct PrintPageDescription {
    pub PageSize: super::super::Foundation::Size,
    pub ImageableRect: super::super::Foundation::Rect,
    pub DpiX: u32,
    pub DpiY: u32,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for PrintPageDescription {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for PrintPageDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for PrintPageDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PrintPageDescription").field("PageSize", &self.PageSize).field("ImageableRect", &self.ImageableRect).field("DpiX", &self.DpiX).field("DpiY", &self.DpiY).finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for PrintPageDescription {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for PrintPageDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Printing.PrintPageDescription;struct(Windows.Foundation.Size;f4;f4);struct(Windows.Foundation.Rect;f4;f4;f4;f4);u4;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for PrintPageDescription {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PrintPageDescription>()) == 0 }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for PrintPageDescription {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for PrintPageDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintPageInfo(::windows::core::IUnknown);
impl PrintPageInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PrintPageInfo, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetMediaSize(&self, value: PrintMediaSize) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMediaSize)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaSize(&self) -> ::windows::core::Result<PrintMediaSize> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MediaSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintMediaSize>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPageSize(&self, value: super::super::Foundation::Size) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPageSize)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PageSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PageSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    pub fn SetDpiX(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDpiX)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DpiX(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DpiX)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetDpiY(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDpiY)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DpiY(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DpiY)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetOrientation(&self, value: PrintOrientation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOrientation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Orientation(&self) -> ::windows::core::Result<PrintOrientation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Orientation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintOrientation>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintPageInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintPageInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintPageInfo {}
impl ::core::fmt::Debug for PrintPageInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintPageInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintPageInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintPageInfo;{dd4be9c9-a6a1-4ada-930e-da872a4f23d3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintPageInfo {
    type Vtable = IPrintPageInfo_Vtbl;
    const IID: ::windows::core::GUID = <IPrintPageInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintPageInfo {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintPageInfo";
}
impl ::core::convert::From<PrintPageInfo> for ::windows::core::IUnknown {
    fn from(value: PrintPageInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintPageInfo> for ::windows::core::IUnknown {
    fn from(value: &PrintPageInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintPageInfo> for &::windows::core::IUnknown {
    fn from(value: &PrintPageInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintPageInfo> for ::windows::core::IInspectable {
    fn from(value: PrintPageInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintPageInfo> for ::windows::core::IInspectable {
    fn from(value: &PrintPageInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintPageInfo> for &::windows::core::IInspectable {
    fn from(value: &PrintPageInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PrintPageInfo {}
unsafe impl ::core::marker::Sync for PrintPageInfo {}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintPageRange(::windows::core::IUnknown);
impl PrintPageRange {
    pub fn FirstPageNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirstPageNumber)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn LastPageNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LastPageNumber)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Create(firstpage: i32, lastpage: i32) -> ::windows::core::Result<PrintPageRange> {
        Self::IPrintPageRangeFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), firstpage, lastpage, result__.as_mut_ptr()).from_abi::<PrintPageRange>(result__)
        })
    }
    pub fn CreateWithSinglePage(page: i32) -> ::windows::core::Result<PrintPageRange> {
        Self::IPrintPageRangeFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithSinglePage)(::windows::core::Interface::as_raw(this), page, result__.as_mut_ptr()).from_abi::<PrintPageRange>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrintPageRangeFactory<R, F: FnOnce(&IPrintPageRangeFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PrintPageRange, IPrintPageRangeFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PrintPageRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintPageRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintPageRange {}
impl ::core::fmt::Debug for PrintPageRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintPageRange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintPageRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintPageRange;{f8a06c54-6e7c-51c5-57fd-0660c2d71513})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintPageRange {
    type Vtable = IPrintPageRange_Vtbl;
    const IID: ::windows::core::GUID = <IPrintPageRange as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintPageRange {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintPageRange";
}
impl ::core::convert::From<PrintPageRange> for ::windows::core::IUnknown {
    fn from(value: PrintPageRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintPageRange> for ::windows::core::IUnknown {
    fn from(value: &PrintPageRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintPageRange> for &::windows::core::IUnknown {
    fn from(value: &PrintPageRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintPageRange> for ::windows::core::IInspectable {
    fn from(value: PrintPageRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintPageRange> for ::windows::core::IInspectable {
    fn from(value: &PrintPageRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintPageRange> for &::windows::core::IInspectable {
    fn from(value: &PrintPageRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PrintPageRange {}
unsafe impl ::core::marker::Sync for PrintPageRange {}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintPageRangeOptions(::windows::core::IUnknown);
impl PrintPageRangeOptions {
    pub fn SetAllowAllPages(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowAllPages)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowAllPages(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AllowAllPages)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowCurrentPage(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowCurrentPage)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowCurrentPage(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AllowCurrentPage)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowCustomSetOfPages(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowCustomSetOfPages)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowCustomSetOfPages(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AllowCustomSetOfPages)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintPageRangeOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintPageRangeOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintPageRangeOptions {}
impl ::core::fmt::Debug for PrintPageRangeOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintPageRangeOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintPageRangeOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintPageRangeOptions;{ce6db728-1357-46b2-a923-79f995f448fc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintPageRangeOptions {
    type Vtable = IPrintPageRangeOptions_Vtbl;
    const IID: ::windows::core::GUID = <IPrintPageRangeOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintPageRangeOptions {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintPageRangeOptions";
}
impl ::core::convert::From<PrintPageRangeOptions> for ::windows::core::IUnknown {
    fn from(value: PrintPageRangeOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintPageRangeOptions> for ::windows::core::IUnknown {
    fn from(value: &PrintPageRangeOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintPageRangeOptions> for &::windows::core::IUnknown {
    fn from(value: &PrintPageRangeOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintPageRangeOptions> for ::windows::core::IInspectable {
    fn from(value: PrintPageRangeOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintPageRangeOptions> for ::windows::core::IInspectable {
    fn from(value: &PrintPageRangeOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintPageRangeOptions> for &::windows::core::IInspectable {
    fn from(value: &PrintPageRangeOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PrintPageRangeOptions {}
unsafe impl ::core::marker::Sync for PrintPageRangeOptions {}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintQuality(pub i32);
impl PrintQuality {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Automatic: Self = Self(3i32);
    pub const Draft: Self = Self(4i32);
    pub const Fax: Self = Self(5i32);
    pub const High: Self = Self(6i32);
    pub const Normal: Self = Self(7i32);
    pub const Photographic: Self = Self(8i32);
    pub const Text: Self = Self(9i32);
}
impl ::core::marker::Copy for PrintQuality {}
impl ::core::clone::Clone for PrintQuality {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintQuality {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PrintQuality {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintQuality {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintQuality").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintQuality {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintQuality;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintStaple(pub i32);
impl PrintStaple {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const None: Self = Self(3i32);
    pub const StapleTopLeft: Self = Self(4i32);
    pub const StapleTopRight: Self = Self(5i32);
    pub const StapleBottomLeft: Self = Self(6i32);
    pub const StapleBottomRight: Self = Self(7i32);
    pub const StapleDualLeft: Self = Self(8i32);
    pub const StapleDualRight: Self = Self(9i32);
    pub const StapleDualTop: Self = Self(10i32);
    pub const StapleDualBottom: Self = Self(11i32);
    pub const SaddleStitch: Self = Self(12i32);
}
impl ::core::marker::Copy for PrintStaple {}
impl ::core::clone::Clone for PrintStaple {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintStaple {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PrintStaple {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintStaple {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintStaple").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintStaple {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintStaple;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintTask(::windows::core::IUnknown);
impl PrintTask {
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackagePropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::DataTransfer::DataPackagePropertySet>(result__)
        }
    }
    pub fn Source(&self) -> ::windows::core::Result<IPrintDocumentSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Source)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPrintDocumentSource>(result__)
        }
    }
    pub fn Options(&self) -> ::windows::core::Result<PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Options)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintTaskOptions>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Previewing<'a, P0>(&self, eventhandler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PrintTask, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Previewing)(::windows::core::Interface::as_raw(this), eventhandler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePreviewing(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePreviewing)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Submitting<'a, P0>(&self, eventhandler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PrintTask, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Submitting)(::windows::core::Interface::as_raw(this), eventhandler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSubmitting(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSubmitting)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Progressing<'a, P0>(&self, eventhandler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskProgressingEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Progressing)(::windows::core::Interface::as_raw(this), eventhandler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgressing(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveProgressing)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<'a, P0>(&self, eventhandler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskCompletedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Completed)(::windows::core::Interface::as_raw(this), eventhandler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCompleted)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn SetIsPreviewEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTask2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsPreviewEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPreviewEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPrintTask2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsPreviewEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPrinterTargetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskTargetDeviceSupport>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsPrinterTargetEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPrinterTargetEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPrintTaskTargetDeviceSupport>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsPrinterTargetEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIs3DManufacturingTargetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskTargetDeviceSupport>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIs3DManufacturingTargetEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Is3DManufacturingTargetEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPrintTaskTargetDeviceSupport>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Is3DManufacturingTargetEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTask {}
impl ::core::fmt::Debug for PrintTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTask").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTask {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTask;{61d80247-6cf6-4fad-84e2-a5e82e2d4ceb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintTask {
    type Vtable = IPrintTask_Vtbl;
    const IID: ::windows::core::GUID = <IPrintTask as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTask {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTask";
}
impl ::core::convert::From<PrintTask> for ::windows::core::IUnknown {
    fn from(value: PrintTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTask> for ::windows::core::IUnknown {
    fn from(value: &PrintTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTask> for &::windows::core::IUnknown {
    fn from(value: &PrintTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintTask> for ::windows::core::IInspectable {
    fn from(value: PrintTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTask> for ::windows::core::IInspectable {
    fn from(value: &PrintTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTask> for &::windows::core::IInspectable {
    fn from(value: &PrintTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PrintTask {}
unsafe impl ::core::marker::Sync for PrintTask {}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintTaskCompletedEventArgs(::windows::core::IUnknown);
impl PrintTaskCompletedEventArgs {
    pub fn Completion(&self) -> ::windows::core::Result<PrintTaskCompletion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Completion)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintTaskCompletion>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTaskCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskCompletedEventArgs {}
impl ::core::fmt::Debug for PrintTaskCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskCompletedEventArgs;{5bcd34af-24e9-4c10-8d07-14c346ba3fce})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintTaskCompletedEventArgs {
    type Vtable = IPrintTaskCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintTaskCompletedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskCompletedEventArgs";
}
impl ::core::convert::From<PrintTaskCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintTaskCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskCompletedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PrintTaskCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintTaskCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintTaskCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskCompletedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PrintTaskCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PrintTaskCompletedEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskCompletedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintTaskCompletion(pub i32);
impl PrintTaskCompletion {
    pub const Abandoned: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
    pub const Submitted: Self = Self(3i32);
}
impl ::core::marker::Copy for PrintTaskCompletion {}
impl ::core::clone::Clone for PrintTaskCompletion {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintTaskCompletion {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PrintTaskCompletion {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintTaskCompletion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskCompletion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskCompletion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintTaskCompletion;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintTaskOptions(::windows::core::IUnknown);
impl PrintTaskOptions {
    pub fn SetBordering(&self, value: PrintBordering) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBordering)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bordering(&self) -> ::windows::core::Result<PrintBordering> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Bordering)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintBordering>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetPagePrintTicket<'a, P0>(&self, printpageinfo: P0) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PrintPageInfo>>,
    {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetPagePrintTicket)(::windows::core::Interface::as_raw(this), printpageinfo.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    pub fn PageRangeOptions(&self) -> ::windows::core::Result<PrintPageRangeOptions> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PageRangeOptions)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintPageRangeOptions>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomPageRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<PrintPageRange>> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CustomPageRanges)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<PrintPageRange>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPageDescription(&self, jobpagenumber: u32) -> ::windows::core::Result<PrintPageDescription> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetPageDescription)(::windows::core::Interface::as_raw(this), jobpagenumber, result__.as_mut_ptr()).from_abi::<PrintPageDescription>(result__)
        }
    }
    pub fn SetMediaSize(&self, value: PrintMediaSize) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMediaSize)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaSize(&self) -> ::windows::core::Result<PrintMediaSize> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MediaSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintMediaSize>(result__)
        }
    }
    pub fn SetMediaType(&self, value: PrintMediaType) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMediaType)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaType(&self) -> ::windows::core::Result<PrintMediaType> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MediaType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintMediaType>(result__)
        }
    }
    pub fn SetOrientation(&self, value: PrintOrientation) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOrientation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Orientation(&self) -> ::windows::core::Result<PrintOrientation> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Orientation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintOrientation>(result__)
        }
    }
    pub fn SetPrintQuality(&self, value: PrintQuality) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrintQuality)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn PrintQuality(&self) -> ::windows::core::Result<PrintQuality> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrintQuality)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintQuality>(result__)
        }
    }
    pub fn SetColorMode(&self, value: PrintColorMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetColorMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ColorMode(&self) -> ::windows::core::Result<PrintColorMode> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ColorMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintColorMode>(result__)
        }
    }
    pub fn SetDuplex(&self, value: PrintDuplex) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDuplex)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Duplex(&self) -> ::windows::core::Result<PrintDuplex> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Duplex)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintDuplex>(result__)
        }
    }
    pub fn SetCollation(&self, value: PrintCollation) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCollation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Collation(&self) -> ::windows::core::Result<PrintCollation> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Collation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintCollation>(result__)
        }
    }
    pub fn SetStaple(&self, value: PrintStaple) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStaple)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Staple(&self) -> ::windows::core::Result<PrintStaple> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Staple)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintStaple>(result__)
        }
    }
    pub fn SetHolePunch(&self, value: PrintHolePunch) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetHolePunch)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn HolePunch(&self) -> ::windows::core::Result<PrintHolePunch> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HolePunch)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintHolePunch>(result__)
        }
    }
    pub fn SetBinding(&self, value: PrintBinding) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBinding)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Binding(&self) -> ::windows::core::Result<PrintBinding> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Binding)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintBinding>(result__)
        }
    }
    pub fn MinCopies(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MinCopies)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MaxCopies(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxCopies)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetNumberOfCopies(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNumberOfCopies)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumberOfCopies(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NumberOfCopies)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DisplayedOptions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreUIConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayedOptions)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTaskOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskOptions {}
impl ::core::fmt::Debug for PrintTaskOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskOptions;{1bdbb474-4ed1-41eb-be3c-72d18ed67337})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintTaskOptions {
    type Vtable = IPrintTaskOptionsCore_Vtbl;
    const IID: ::windows::core::GUID = <IPrintTaskOptionsCore as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTaskOptions {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskOptions";
}
impl ::core::convert::From<PrintTaskOptions> for ::windows::core::IUnknown {
    fn from(value: PrintTaskOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskOptions> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskOptions> for &::windows::core::IUnknown {
    fn from(value: &PrintTaskOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintTaskOptions> for ::windows::core::IInspectable {
    fn from(value: PrintTaskOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskOptions> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskOptions> for &::windows::core::IInspectable {
    fn from(value: &PrintTaskOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<PrintTaskOptions> for IPrintTaskOptionsCore {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintTaskOptions) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskOptions> for IPrintTaskOptionsCore {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskOptions) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&PrintTaskOptions> for ::windows::core::InParam<'a, IPrintTaskOptionsCore> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskOptions) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<PrintTaskOptions> for IPrintTaskOptionsCoreProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintTaskOptions) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskOptions> for IPrintTaskOptionsCoreProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskOptions) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&PrintTaskOptions> for ::windows::core::InParam<'a, IPrintTaskOptionsCoreProperties> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskOptions) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<PrintTaskOptions> for IPrintTaskOptionsCoreUIConfiguration {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintTaskOptions) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskOptions> for IPrintTaskOptionsCoreUIConfiguration {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskOptions) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&PrintTaskOptions> for ::windows::core::InParam<'a, IPrintTaskOptionsCoreUIConfiguration> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskOptions) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for PrintTaskOptions {}
unsafe impl ::core::marker::Sync for PrintTaskOptions {}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintTaskProgressingEventArgs(::windows::core::IUnknown);
impl PrintTaskProgressingEventArgs {
    pub fn DocumentPageCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DocumentPageCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTaskProgressingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskProgressingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskProgressingEventArgs {}
impl ::core::fmt::Debug for PrintTaskProgressingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskProgressingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskProgressingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskProgressingEventArgs;{810cd3cb-b410-4282-a073-5ac378234174})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintTaskProgressingEventArgs {
    type Vtable = IPrintTaskProgressingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintTaskProgressingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTaskProgressingEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskProgressingEventArgs";
}
impl ::core::convert::From<PrintTaskProgressingEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintTaskProgressingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskProgressingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskProgressingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskProgressingEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PrintTaskProgressingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintTaskProgressingEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintTaskProgressingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskProgressingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskProgressingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskProgressingEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PrintTaskProgressingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PrintTaskProgressingEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskProgressingEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintTaskRequest(::windows::core::IUnknown);
impl PrintTaskRequest {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Deadline)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn CreatePrintTask<'a, P0>(&self, title: &::windows::core::HSTRING, handler: P0) -> ::windows::core::Result<PrintTask>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PrintTaskSourceRequestedHandler>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreatePrintTask)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(title), handler.into().abi(), result__.as_mut_ptr()).from_abi::<PrintTask>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<PrintTaskRequestedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintTaskRequestedDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTaskRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskRequest {}
impl ::core::fmt::Debug for PrintTaskRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskRequest;{6ff61e2e-2722-4240-a67c-f364849a17f3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintTaskRequest {
    type Vtable = IPrintTaskRequest_Vtbl;
    const IID: ::windows::core::GUID = <IPrintTaskRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTaskRequest {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskRequest";
}
impl ::core::convert::From<PrintTaskRequest> for ::windows::core::IUnknown {
    fn from(value: PrintTaskRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskRequest> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskRequest> for &::windows::core::IUnknown {
    fn from(value: &PrintTaskRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintTaskRequest> for ::windows::core::IInspectable {
    fn from(value: PrintTaskRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskRequest> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskRequest> for &::windows::core::IInspectable {
    fn from(value: &PrintTaskRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PrintTaskRequest {}
unsafe impl ::core::marker::Sync for PrintTaskRequest {}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintTaskRequestedDeferral(::windows::core::IUnknown);
impl PrintTaskRequestedDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for PrintTaskRequestedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskRequestedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskRequestedDeferral {}
impl ::core::fmt::Debug for PrintTaskRequestedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskRequestedDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskRequestedDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskRequestedDeferral;{cfefb3f0-ce3e-42c7-9496-64800c622c44})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintTaskRequestedDeferral {
    type Vtable = IPrintTaskRequestedDeferral_Vtbl;
    const IID: ::windows::core::GUID = <IPrintTaskRequestedDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTaskRequestedDeferral {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskRequestedDeferral";
}
impl ::core::convert::From<PrintTaskRequestedDeferral> for ::windows::core::IUnknown {
    fn from(value: PrintTaskRequestedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskRequestedDeferral> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskRequestedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskRequestedDeferral> for &::windows::core::IUnknown {
    fn from(value: &PrintTaskRequestedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintTaskRequestedDeferral> for ::windows::core::IInspectable {
    fn from(value: PrintTaskRequestedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskRequestedDeferral> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskRequestedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskRequestedDeferral> for &::windows::core::IInspectable {
    fn from(value: &PrintTaskRequestedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PrintTaskRequestedDeferral {}
unsafe impl ::core::marker::Sync for PrintTaskRequestedDeferral {}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintTaskRequestedEventArgs(::windows::core::IUnknown);
impl PrintTaskRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<PrintTaskRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintTaskRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTaskRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskRequestedEventArgs {}
impl ::core::fmt::Debug for PrintTaskRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskRequestedEventArgs;{d0aff924-a31b-454c-a7b6-5d0cc522fc16})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintTaskRequestedEventArgs {
    type Vtable = IPrintTaskRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintTaskRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTaskRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskRequestedEventArgs";
}
impl ::core::convert::From<PrintTaskRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintTaskRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskRequestedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PrintTaskRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintTaskRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintTaskRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskRequestedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PrintTaskRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PrintTaskRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskRequestedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintTaskSourceRequestedArgs(::windows::core::IUnknown);
impl PrintTaskSourceRequestedArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Deadline)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn SetSource<'a, P0, E0>(&self, source: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IPrintDocumentSource>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSource)(::windows::core::Interface::as_raw(this), source.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<PrintTaskSourceRequestedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintTaskSourceRequestedDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTaskSourceRequestedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskSourceRequestedArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskSourceRequestedArgs {}
impl ::core::fmt::Debug for PrintTaskSourceRequestedArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskSourceRequestedArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskSourceRequestedArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskSourceRequestedArgs;{f9f067be-f456-41f0-9c98-5ce73e851410})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintTaskSourceRequestedArgs {
    type Vtable = IPrintTaskSourceRequestedArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintTaskSourceRequestedArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTaskSourceRequestedArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskSourceRequestedArgs";
}
impl ::core::convert::From<PrintTaskSourceRequestedArgs> for ::windows::core::IUnknown {
    fn from(value: PrintTaskSourceRequestedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskSourceRequestedArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskSourceRequestedArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskSourceRequestedArgs> for &::windows::core::IUnknown {
    fn from(value: &PrintTaskSourceRequestedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintTaskSourceRequestedArgs> for ::windows::core::IInspectable {
    fn from(value: PrintTaskSourceRequestedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskSourceRequestedArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskSourceRequestedArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskSourceRequestedArgs> for &::windows::core::IInspectable {
    fn from(value: &PrintTaskSourceRequestedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PrintTaskSourceRequestedArgs {}
unsafe impl ::core::marker::Sync for PrintTaskSourceRequestedArgs {}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintTaskSourceRequestedDeferral(::windows::core::IUnknown);
impl PrintTaskSourceRequestedDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for PrintTaskSourceRequestedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskSourceRequestedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskSourceRequestedDeferral {}
impl ::core::fmt::Debug for PrintTaskSourceRequestedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskSourceRequestedDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskSourceRequestedDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskSourceRequestedDeferral;{4a1560d1-6992-4d9d-8555-4ca4563fb166})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintTaskSourceRequestedDeferral {
    type Vtable = IPrintTaskSourceRequestedDeferral_Vtbl;
    const IID: ::windows::core::GUID = <IPrintTaskSourceRequestedDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTaskSourceRequestedDeferral {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskSourceRequestedDeferral";
}
impl ::core::convert::From<PrintTaskSourceRequestedDeferral> for ::windows::core::IUnknown {
    fn from(value: PrintTaskSourceRequestedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskSourceRequestedDeferral> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskSourceRequestedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskSourceRequestedDeferral> for &::windows::core::IUnknown {
    fn from(value: &PrintTaskSourceRequestedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintTaskSourceRequestedDeferral> for ::windows::core::IInspectable {
    fn from(value: PrintTaskSourceRequestedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskSourceRequestedDeferral> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskSourceRequestedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskSourceRequestedDeferral> for &::windows::core::IInspectable {
    fn from(value: &PrintTaskSourceRequestedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PrintTaskSourceRequestedDeferral {}
unsafe impl ::core::marker::Sync for PrintTaskSourceRequestedDeferral {}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintTaskSourceRequestedHandler(pub ::windows::core::IUnknown);
impl PrintTaskSourceRequestedHandler {
    pub fn new<F: FnMut(&::core::option::Option<PrintTaskSourceRequestedArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = PrintTaskSourceRequestedHandlerBox::<F> { vtable: &PrintTaskSourceRequestedHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, P0>(&self, args: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PrintTaskSourceRequestedArgs>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), args.into().abi()).ok() }
    }
}
#[repr(C)]
struct PrintTaskSourceRequestedHandlerBox<F: FnMut(&::core::option::Option<PrintTaskSourceRequestedArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const PrintTaskSourceRequestedHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<PrintTaskSourceRequestedArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> PrintTaskSourceRequestedHandlerBox<F> {
    const VTABLE: PrintTaskSourceRequestedHandler_Vtbl = PrintTaskSourceRequestedHandler_Vtbl {
        base__: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<PrintTaskSourceRequestedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&args)).into()
    }
}
impl ::core::clone::Clone for PrintTaskSourceRequestedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskSourceRequestedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskSourceRequestedHandler {}
impl ::core::fmt::Debug for PrintTaskSourceRequestedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskSourceRequestedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for PrintTaskSourceRequestedHandler {
    type Vtable = PrintTaskSourceRequestedHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c109fa8_5cb6_4b3a_8663_f39cb02dc9b4);
}
unsafe impl ::windows::core::RuntimeType for PrintTaskSourceRequestedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6c109fa8-5cb6-4b3a-8663-f39cb02dc9b4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct PrintTaskSourceRequestedHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
pub struct StandardPrintTaskOptions;
impl StandardPrintTaskOptions {
    pub fn MediaSize() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MediaSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MediaType() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MediaType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Orientation() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Orientation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn PrintQuality() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrintQuality)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ColorMode() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ColorMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Duplex() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Duplex)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Collation() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Collation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Staple() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Staple)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HolePunch() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HolePunch)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Binding() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Binding)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Copies() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Copies)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn NUp() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NUp)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn InputBin() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InputBin)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Bordering() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Bordering)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CustomPageRanges() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CustomPageRanges)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStandardPrintTaskOptionsStatic<R, F: FnOnce(&IStandardPrintTaskOptionsStatic) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StandardPrintTaskOptions, IStandardPrintTaskOptionsStatic> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStandardPrintTaskOptionsStatic2<R, F: FnOnce(&IStandardPrintTaskOptionsStatic2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StandardPrintTaskOptions, IStandardPrintTaskOptionsStatic2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStandardPrintTaskOptionsStatic3<R, F: FnOnce(&IStandardPrintTaskOptionsStatic3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StandardPrintTaskOptions, IStandardPrintTaskOptionsStatic3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for StandardPrintTaskOptions {
    const NAME: &'static str = "Windows.Graphics.Printing.StandardPrintTaskOptions";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
