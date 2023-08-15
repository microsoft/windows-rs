#[doc(hidden)]
#[repr(transparent)]
pub struct IQuickLink(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQuickLink {
    type Vtable = IQuickLink_Vtbl;
}
impl ::core::clone::Clone for IQuickLink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IQuickLink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x603e4308_f0be_4adc_acc9_8b27ab9cf556);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuickLink_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDataFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDataFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFileTypes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShareOperation {
    type Vtable = IShareOperation_Vtbl;
}
impl ::core::clone::Clone for IShareOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IShareOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2246bab8_d0f8_41c1_a82a_4137db6504fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub QuickLinkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoveThisQuickLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportDataRetrieved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportSubmittedBackgroundTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportCompletedWithQuickLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quicklink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareOperation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShareOperation2 {
    type Vtable = IShareOperation2_Vtbl;
}
impl ::core::clone::Clone for IShareOperation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IShareOperation2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ffb97c1_9778_4a09_8e5b_cb5e482d0555);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareOperation2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DismissUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareOperation3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShareOperation3 {
    type Vtable = IShareOperation3_Vtbl;
}
impl ::core::clone::Clone for IShareOperation3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IShareOperation3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ef6b382_b7a7_4571_a2a6_994a034988b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareOperation3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub Contacts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections")))]
    Contacts: usize,
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_ShareTarget\"`*"]
#[repr(transparent)]
pub struct QuickLink(::windows_core::IUnknown);
impl QuickLink {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<QuickLink, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::super::super::Storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Storage::Streams::RandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnail)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedDataFormats(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedDataFormats)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFileTypes(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedFileTypes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for QuickLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for QuickLink {}
impl ::core::fmt::Debug for QuickLink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QuickLink").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for QuickLink {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ShareTarget.QuickLink;{603e4308-f0be-4adc-acc9-8b27ab9cf556})");
}
impl ::core::clone::Clone for QuickLink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for QuickLink {
    type Vtable = IQuickLink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for QuickLink {
    const IID: ::windows_core::GUID = <IQuickLink as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for QuickLink {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareTarget.QuickLink";
}
::windows_core::imp::interface_hierarchy!(QuickLink, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_ShareTarget\"`*"]
#[repr(transparent)]
pub struct ShareOperation(::windows_core::IUnknown);
impl ShareOperation {
    pub fn Data(&self) -> ::windows_core::Result<super::DataPackageView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn QuickLinkId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QuickLinkId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveThisQuickLink(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveThisQuickLink)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportStarted(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportStarted)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportDataRetrieved(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportDataRetrieved)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportSubmittedBackgroundTask(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportSubmittedBackgroundTask)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportCompletedWithQuickLink<P0>(&self, quicklink: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<QuickLink>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompletedWithQuickLink)(::windows_core::Interface::as_raw(this), quicklink.into_param().abi()).ok() }
    }
    pub fn ReportCompleted(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportError(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportError)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DismissUI(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IShareOperation2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DismissUI)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub fn Contacts(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::Contacts::Contact>> {
        let this = &::windows_core::ComInterface::cast::<IShareOperation3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contacts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ShareOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareOperation {}
impl ::core::fmt::Debug for ShareOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ShareOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ShareTarget.ShareOperation;{2246bab8-d0f8-41c1-a82a-4137db6504fb})");
}
impl ::core::clone::Clone for ShareOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ShareOperation {
    type Vtable = IShareOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ShareOperation {
    const IID: ::windows_core::GUID = <IShareOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ShareOperation {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareTarget.ShareOperation";
}
::windows_core::imp::interface_hierarchy!(ShareOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
