#[doc(hidden)]
#[repr(transparent)]
pub struct IQuickLink(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IQuickLink {
    type Vtable = IQuickLink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x603e4308_f0be_4adc_acc9_8b27ab9cf556);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuickLink_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDataFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDataFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFileTypes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareOperation {
    type Vtable = IShareOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2246bab8_d0f8_41c1_a82a_4137db6504fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareOperation_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QuickLinkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoveThisQuickLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportDataRetrieved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportSubmittedBackgroundTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportCompletedWithQuickLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quicklink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareOperation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareOperation2 {
    type Vtable = IShareOperation2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ffb97c1_9778_4a09_8e5b_cb5e482d0555);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareOperation2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DismissUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareOperation3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareOperation3 {
    type Vtable = IShareOperation3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ef6b382_b7a7_4571_a2a6_994a034988b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareOperation3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub Contacts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections")))]
    Contacts: usize,
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_ShareTarget\"`*"]
#[repr(transparent)]
pub struct QuickLink(::windows::core::IUnknown);
impl QuickLink {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<QuickLink, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Thumbnail)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::RandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Storage::Streams::RandomAccessStreamReference>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetThumbnail)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedDataFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SupportedDataFormats)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFileTypes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SupportedFileTypes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for QuickLink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for QuickLink {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ShareTarget.QuickLink;{603e4308-f0be-4adc-acc9-8b27ab9cf556})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for QuickLink {
    type Vtable = IQuickLink_Vtbl;
    const IID: ::windows::core::GUID = <IQuickLink as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for QuickLink {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareTarget.QuickLink";
}
impl ::core::convert::From<QuickLink> for ::windows::core::IUnknown {
    fn from(value: QuickLink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QuickLink> for ::windows::core::IUnknown {
    fn from(value: &QuickLink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&QuickLink> for &::windows::core::IUnknown {
    fn from(value: &QuickLink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<QuickLink> for ::windows::core::IInspectable {
    fn from(value: QuickLink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QuickLink> for ::windows::core::IInspectable {
    fn from(value: &QuickLink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&QuickLink> for &::windows::core::IInspectable {
    fn from(value: &QuickLink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_ShareTarget\"`*"]
#[repr(transparent)]
pub struct ShareOperation(::windows::core::IUnknown);
impl ShareOperation {
    pub fn Data(&self) -> ::windows::core::Result<super::DataPackageView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DataPackageView>(result__)
        }
    }
    pub fn QuickLinkId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).QuickLinkId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn RemoveThisQuickLink(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveThisQuickLink)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportStarted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportStarted)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportDataRetrieved(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportDataRetrieved)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportSubmittedBackgroundTask(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportSubmittedBackgroundTask)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportCompletedWithQuickLink<'a, P0>(&self, quicklink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, QuickLink>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompletedWithQuickLink)(::windows::core::Interface::as_raw(this), quicklink.into().abi()).ok() }
    }
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportError(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportError)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DismissUI(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IShareOperation2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DismissUI)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub fn Contacts(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::Contacts::Contact>> {
        let this = &::windows::core::Interface::cast::<IShareOperation3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contacts)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::Contacts::Contact>>(result__)
        }
    }
}
impl ::core::clone::Clone for ShareOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for ShareOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ShareTarget.ShareOperation;{2246bab8-d0f8-41c1-a82a-4137db6504fb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ShareOperation {
    type Vtable = IShareOperation_Vtbl;
    const IID: ::windows::core::GUID = <IShareOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ShareOperation {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareTarget.ShareOperation";
}
impl ::core::convert::From<ShareOperation> for ::windows::core::IUnknown {
    fn from(value: ShareOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareOperation> for ::windows::core::IUnknown {
    fn from(value: &ShareOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ShareOperation> for &::windows::core::IUnknown {
    fn from(value: &ShareOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ShareOperation> for ::windows::core::IInspectable {
    fn from(value: ShareOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareOperation> for ::windows::core::IInspectable {
    fn from(value: &ShareOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ShareOperation> for &::windows::core::IInspectable {
    fn from(value: &ShareOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
