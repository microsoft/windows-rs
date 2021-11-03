#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreDragDropManager(pub ::windows::runtime::IInspectable);
impl CoreDragDropManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`, `Foundation`*"]
    pub fn TargetRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreDragDropManager, CoreDropOperationTargetRequestedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`, `Foundation`*"]
    pub fn RemoveTargetRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn AreConcurrentOperationsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn SetAreConcurrentOperationsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<CoreDragDropManager> {
        Self::ICoreDragDropManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreDragDropManager>(result__)
        })
    }
    pub fn ICoreDragDropManagerStatics<R, F: FnOnce(&ICoreDragDropManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreDragDropManager, ICoreDragDropManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreDragDropManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager;{7d56d344-8464-4faf-aa49-37ea6e2d7bd1})");
}
unsafe impl ::windows::runtime::Interface for CoreDragDropManager {
    type Vtable = ICoreDragDropManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2102842180, 33892, 20399, [170, 73, 55, 234, 110, 45, 123, 209]);
}
impl ::windows::runtime::RuntimeName for CoreDragDropManager {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager";
}
impl ::std::convert::From<CoreDragDropManager> for ::windows::runtime::IUnknown {
    fn from(value: CoreDragDropManager) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CoreDragDropManager> for ::windows::runtime::IUnknown {
    fn from(value: &CoreDragDropManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreDragDropManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CoreDragDropManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CoreDragDropManager> for ::windows::runtime::IInspectable {
    fn from(value: CoreDragDropManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreDragDropManager> for ::windows::runtime::IInspectable {
    fn from(value: &CoreDragDropManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreDragDropManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreDragDropManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreDragDropManager {}
unsafe impl ::std::marker::Sync for CoreDragDropManager {}
#[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreDragInfo(pub ::windows::runtime::IInspectable);
impl CoreDragInfo {
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::DataPackageView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DataPackageView>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn Modifiers(&self) -> ::windows::runtime::Result<super::DragDropModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::DragDropModifiers = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::DragDropModifiers>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn AllowedOperations(&self) -> ::windows::runtime::Result<super::super::DataPackageOperation> {
        let this = &::windows::runtime::Interface::cast::<ICoreDragInfo2>(self)?;
        unsafe {
            let mut result__: super::super::DataPackageOperation = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DataPackageOperation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreDragInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo;{48353a8b-cb50-464e-9575-cd4e3a7ab028})");
}
unsafe impl ::windows::runtime::Interface for CoreDragInfo {
    type Vtable = ICoreDragInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1211447947, 52048, 17998, [149, 117, 205, 78, 58, 122, 176, 40]);
}
impl ::windows::runtime::RuntimeName for CoreDragInfo {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo";
}
impl ::std::convert::From<CoreDragInfo> for ::windows::runtime::IUnknown {
    fn from(value: CoreDragInfo) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CoreDragInfo> for ::windows::runtime::IUnknown {
    fn from(value: &CoreDragInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreDragInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CoreDragInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CoreDragInfo> for ::windows::runtime::IInspectable {
    fn from(value: CoreDragInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreDragInfo> for ::windows::runtime::IInspectable {
    fn from(value: &CoreDragInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreDragInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreDragInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreDragInfo {}
unsafe impl ::std::marker::Sync for CoreDragInfo {}
#[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreDragOperation(pub ::windows::runtime::IInspectable);
impl CoreDragOperation {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreDragOperation, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::DataPackage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DataPackage>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn SetPointerId(&self, pointerid: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), pointerid).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`, `Graphics_Imaging`*"]
    pub fn SetDragUIContentFromSoftwareBitmap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Graphics::Imaging::SoftwareBitmap>>(&self, softwarebitmap: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), softwarebitmap.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`, `Foundation`, `Graphics_Imaging`*"]
    pub fn SetDragUIContentFromSoftwareBitmapWithAnchorPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Graphics::Imaging::SoftwareBitmap>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, softwarebitmap: Param0, anchorpoint: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), softwarebitmap.into_param().abi(), anchorpoint.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn DragUIContentMode(&self) -> ::windows::runtime::Result<CoreDragUIContentMode> {
        let this = self;
        unsafe {
            let mut result__: CoreDragUIContentMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreDragUIContentMode>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn SetDragUIContentMode(&self, value: CoreDragUIContentMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`, `Foundation`*"]
    pub fn StartAsync(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn AllowedOperations(&self) -> ::windows::runtime::Result<super::super::DataPackageOperation> {
        let this = &::windows::runtime::Interface::cast::<ICoreDragOperation2>(self)?;
        unsafe {
            let mut result__: super::super::DataPackageOperation = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DataPackageOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn SetAllowedOperations(&self, value: super::super::DataPackageOperation) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreDragOperation2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreDragOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation;{cc06de4f-6db0-4e62-ab1b-a74a02dc6d85})");
}
unsafe impl ::windows::runtime::Interface for CoreDragOperation {
    type Vtable = ICoreDragOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3423002191, 28080, 20066, [171, 27, 167, 74, 2, 220, 109, 133]);
}
impl ::windows::runtime::RuntimeName for CoreDragOperation {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation";
}
impl ::std::convert::From<CoreDragOperation> for ::windows::runtime::IUnknown {
    fn from(value: CoreDragOperation) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CoreDragOperation> for ::windows::runtime::IUnknown {
    fn from(value: &CoreDragOperation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreDragOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CoreDragOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CoreDragOperation> for ::windows::runtime::IInspectable {
    fn from(value: CoreDragOperation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreDragOperation> for ::windows::runtime::IInspectable {
    fn from(value: &CoreDragOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreDragOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreDragOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreDragOperation {}
unsafe impl ::std::marker::Sync for CoreDragOperation {}
#[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreDragUIContentMode(pub u32);
impl CoreDragUIContentMode {
    pub const Auto: CoreDragUIContentMode = CoreDragUIContentMode(0u32);
    pub const Deferred: CoreDragUIContentMode = CoreDragUIContentMode(1u32);
}
impl ::std::convert::From<u32> for CoreDragUIContentMode {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreDragUIContentMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreDragUIContentMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIContentMode;u4)");
}
impl ::windows::runtime::DefaultType for CoreDragUIContentMode {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CoreDragUIContentMode {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CoreDragUIContentMode {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CoreDragUIContentMode {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CoreDragUIContentMode {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CoreDragUIContentMode {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreDragUIOverride(pub ::windows::runtime::IInspectable);
impl CoreDragUIOverride {
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`, `Graphics_Imaging`*"]
    pub fn SetContentFromSoftwareBitmap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Graphics::Imaging::SoftwareBitmap>>(&self, softwarebitmap: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), softwarebitmap.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`, `Foundation`, `Graphics_Imaging`*"]
    pub fn SetContentFromSoftwareBitmapWithAnchorPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Graphics::Imaging::SoftwareBitmap>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, softwarebitmap: Param0, anchorpoint: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), softwarebitmap.into_param().abi(), anchorpoint.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn IsContentVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn SetIsContentVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn Caption(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn SetCaption<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn IsCaptionVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn SetIsCaptionVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn IsGlyphVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn SetIsGlyphVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreDragUIOverride {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIOverride;{89a85064-3389-4f4f-8897-7e8a3ffb3c93})");
}
unsafe impl ::windows::runtime::Interface for CoreDragUIOverride {
    type Vtable = ICoreDragUIOverride_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2309509220, 13193, 20303, [136, 151, 126, 138, 63, 251, 60, 147]);
}
impl ::windows::runtime::RuntimeName for CoreDragUIOverride {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIOverride";
}
impl ::std::convert::From<CoreDragUIOverride> for ::windows::runtime::IUnknown {
    fn from(value: CoreDragUIOverride) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CoreDragUIOverride> for ::windows::runtime::IUnknown {
    fn from(value: &CoreDragUIOverride) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreDragUIOverride {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CoreDragUIOverride {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CoreDragUIOverride> for ::windows::runtime::IInspectable {
    fn from(value: CoreDragUIOverride) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreDragUIOverride> for ::windows::runtime::IInspectable {
    fn from(value: &CoreDragUIOverride) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreDragUIOverride {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreDragUIOverride {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreDragUIOverride {}
unsafe impl ::std::marker::Sync for CoreDragUIOverride {}
#[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreDropOperationTargetRequestedEventArgs(pub ::windows::runtime::IInspectable);
impl CoreDropOperationTargetRequestedEventArgs {
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
    pub fn SetTarget<'a, Param0: ::windows::runtime::IntoParam<'a, ICoreDropOperationTarget>>(&self, target: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), target.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreDropOperationTargetRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDropOperationTargetRequestedEventArgs;{2aca929a-5e28-4ea6-829e-29134e665d6d})");
}
unsafe impl ::windows::runtime::Interface for CoreDropOperationTargetRequestedEventArgs {
    type Vtable = ICoreDropOperationTargetRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(717918874, 24104, 20134, [130, 158, 41, 19, 78, 102, 93, 109]);
}
impl ::windows::runtime::RuntimeName for CoreDropOperationTargetRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDropOperationTargetRequestedEventArgs";
}
impl ::std::convert::From<CoreDropOperationTargetRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CoreDropOperationTargetRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CoreDropOperationTargetRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CoreDropOperationTargetRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreDropOperationTargetRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CoreDropOperationTargetRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CoreDropOperationTargetRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CoreDropOperationTargetRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreDropOperationTargetRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CoreDropOperationTargetRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreDropOperationTargetRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreDropOperationTargetRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreDropOperationTargetRequestedEventArgs {}
unsafe impl ::std::marker::Sync for CoreDropOperationTargetRequestedEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreDragDropManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreDragDropManager {
    type Vtable = ICoreDragDropManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2102842180, 33892, 20399, [170, 73, 55, 234, 110, 45, 123, 209]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragDropManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreDragDropManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreDragDropManagerStatics {
    type Vtable = ICoreDragDropManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2504195530, 55826, 19484, [141, 6, 4, 29, 178, 151, 51, 195]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragDropManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreDragInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreDragInfo {
    type Vtable = ICoreDragInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1211447947, 52048, 17998, [149, 117, 205, 78, 58, 122, 176, 40]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::DragDropModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreDragInfo2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreDragInfo2 {
    type Vtable = ICoreDragInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3309736421, 59131, 19828, [180, 177, 138, 60, 23, 242, 94, 158]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::DataPackageOperation) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreDragOperation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreDragOperation {
    type Vtable = ICoreDragOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3423002191, 28080, 20066, [171, 27, 167, 74, 2, 220, 109, 133]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, softwarebitmap: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, softwarebitmap: ::windows::runtime::RawPtr, anchorpoint: super::super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CoreDragUIContentMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CoreDragUIContentMode) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreDragOperation2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreDragOperation2 {
    type Vtable = ICoreDragOperation2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2185961004, 55706, 20419, [133, 7, 108, 24, 47, 51, 180, 106]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragOperation2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::DataPackageOperation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::DataPackageOperation) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreDragUIOverride(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreDragUIOverride {
    type Vtable = ICoreDragUIOverride_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2309509220, 13193, 20303, [136, 151, 126, 138, 63, 251, 60, 147]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragUIOverride_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, softwarebitmap: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, softwarebitmap: ::windows::runtime::RawPtr, anchorpoint: super::super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`*"]
pub struct ICoreDropOperationTarget(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreDropOperationTarget {
    type Vtable = ICoreDropOperationTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3641860502, 19547, 16765, [187, 55, 118, 56, 29, 239, 141, 180]);
}
impl ICoreDropOperationTarget {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`, `Foundation`*"]
    pub fn EnterAsync<'a, Param0: ::windows::runtime::IntoParam<'a, CoreDragInfo>, Param1: ::windows::runtime::IntoParam<'a, CoreDragUIOverride>>(&self, draginfo: Param0, draguioverride: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), draginfo.into_param().abi(), draguioverride.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`, `Foundation`*"]
    pub fn OverAsync<'a, Param0: ::windows::runtime::IntoParam<'a, CoreDragInfo>, Param1: ::windows::runtime::IntoParam<'a, CoreDragUIOverride>>(&self, draginfo: Param0, draguioverride: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), draginfo.into_param().abi(), draguioverride.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`, `Foundation`*"]
    pub fn LeaveAsync<'a, Param0: ::windows::runtime::IntoParam<'a, CoreDragInfo>>(&self, draginfo: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), draginfo.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_DataTransfer_DragDrop_Core`, `Foundation`*"]
    pub fn DropAsync<'a, Param0: ::windows::runtime::IntoParam<'a, CoreDragInfo>>(&self, draginfo: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), draginfo.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ICoreDropOperationTarget {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{d9126196-4c5b-417d-bb37-76381def8db4}");
}
impl ::std::convert::From<ICoreDropOperationTarget> for ::windows::runtime::IUnknown {
    fn from(value: ICoreDropOperationTarget) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ICoreDropOperationTarget> for ::windows::runtime::IUnknown {
    fn from(value: &ICoreDropOperationTarget) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICoreDropOperationTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICoreDropOperationTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ICoreDropOperationTarget> for ::windows::runtime::IInspectable {
    fn from(value: ICoreDropOperationTarget) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ICoreDropOperationTarget> for ::windows::runtime::IInspectable {
    fn from(value: &ICoreDropOperationTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ICoreDropOperationTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ICoreDropOperationTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDropOperationTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, draginfo: ::windows::runtime::RawPtr, draguioverride: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, draginfo: ::windows::runtime::RawPtr, draguioverride: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, draginfo: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, draginfo: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreDropOperationTargetRequestedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreDropOperationTargetRequestedEventArgs {
    type Vtable = ICoreDropOperationTargetRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(717918874, 24104, 20134, [130, 158, 41, 19, 78, 102, 93, 109]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDropOperationTargetRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
