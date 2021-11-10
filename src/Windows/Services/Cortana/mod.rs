#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Services_Cortana`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CortanaActionableInsights(pub ::windows::runtime::IInspectable);
impl CortanaActionableInsights {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Services_Cortana`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Cortana`, `Foundation`*"]
    pub fn IsAvailableAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Services_Cortana`, `Foundation`, `Storage_Streams`*"]
    pub fn ShowInsightsForImageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, imagestream: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), imagestream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Services_Cortana`, `Foundation`, `Storage_Streams`*"]
    pub fn ShowInsightsForImageWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Param1: ::windows::runtime::IntoParam<'a, CortanaActionableInsightsOptions>>(&self, imagestream: Param0, options: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), imagestream.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Cortana`, `Foundation`*"]
    pub fn ShowInsightsForTextAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Cortana`, `Foundation`*"]
    pub fn ShowInsightsForTextWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, CortanaActionableInsightsOptions>>(&self, text: Param0, options: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), text.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation"))]
    #[doc = "*Required features: `Services_Cortana`, `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn ShowInsightsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::DataTransfer::DataPackage>>(&self, datapackage: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), datapackage.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation"))]
    #[doc = "*Required features: `Services_Cortana`, `ApplicationModel_DataTransfer`, `Foundation`*"]
    pub fn ShowInsightsWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::DataTransfer::DataPackage>, Param1: ::windows::runtime::IntoParam<'a, CortanaActionableInsightsOptions>>(&self, datapackage: Param0, options: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), datapackage.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Services_Cortana`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<CortanaActionableInsights> {
        Self::ICortanaActionableInsightsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CortanaActionableInsights>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Services_Cortana`, `System`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<CortanaActionableInsights> {
        Self::ICortanaActionableInsightsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<CortanaActionableInsights>(result__)
        })
    }
    pub fn ICortanaActionableInsightsStatics<R, F: FnOnce(&ICortanaActionableInsightsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CortanaActionableInsights, ICortanaActionableInsightsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CortanaActionableInsights {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Cortana.CortanaActionableInsights;{951ec6b1-fc83-586d-8b84-2452c8981625})");
}
unsafe impl ::windows::runtime::Interface for CortanaActionableInsights {
    type Vtable = ICortanaActionableInsights_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x951ec6b1_fc83_586d_8b84_2452c8981625);
}
impl ::windows::runtime::RuntimeName for CortanaActionableInsights {
    const NAME: &'static str = "Windows.Services.Cortana.CortanaActionableInsights";
}
impl ::core::convert::From<CortanaActionableInsights> for ::windows::runtime::IUnknown {
    fn from(value: CortanaActionableInsights) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CortanaActionableInsights> for ::windows::runtime::IUnknown {
    fn from(value: &CortanaActionableInsights) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CortanaActionableInsights {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CortanaActionableInsights {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CortanaActionableInsights> for ::windows::runtime::IInspectable {
    fn from(value: CortanaActionableInsights) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CortanaActionableInsights> for ::windows::runtime::IInspectable {
    fn from(value: &CortanaActionableInsights) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CortanaActionableInsights {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CortanaActionableInsights {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CortanaActionableInsights {}
unsafe impl ::core::marker::Sync for CortanaActionableInsights {}
#[doc = "*Required features: `Services_Cortana`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CortanaActionableInsightsOptions(pub ::windows::runtime::IInspectable);
impl CortanaActionableInsightsOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CortanaActionableInsightsOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Cortana`, `Foundation`*"]
    pub fn ContentSourceWebLink(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Cortana`, `Foundation`*"]
    pub fn SetContentSourceWebLink<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Services_Cortana`*"]
    pub fn SurroundingText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Services_Cortana`*"]
    pub fn SetSurroundingText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CortanaActionableInsightsOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Cortana.CortanaActionableInsightsOptions;{aac2bbcf-9782-5420-b81e-7ae56af31815})");
}
unsafe impl ::windows::runtime::Interface for CortanaActionableInsightsOptions {
    type Vtable = ICortanaActionableInsightsOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaac2bbcf_9782_5420_b81e_7ae56af31815);
}
impl ::windows::runtime::RuntimeName for CortanaActionableInsightsOptions {
    const NAME: &'static str = "Windows.Services.Cortana.CortanaActionableInsightsOptions";
}
impl ::core::convert::From<CortanaActionableInsightsOptions> for ::windows::runtime::IUnknown {
    fn from(value: CortanaActionableInsightsOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CortanaActionableInsightsOptions> for ::windows::runtime::IUnknown {
    fn from(value: &CortanaActionableInsightsOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CortanaActionableInsightsOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CortanaActionableInsightsOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CortanaActionableInsightsOptions> for ::windows::runtime::IInspectable {
    fn from(value: CortanaActionableInsightsOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CortanaActionableInsightsOptions> for ::windows::runtime::IInspectable {
    fn from(value: &CortanaActionableInsightsOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CortanaActionableInsightsOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CortanaActionableInsightsOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CortanaActionableInsightsOptions {}
unsafe impl ::core::marker::Sync for CortanaActionableInsightsOptions {}
#[doc = "*Required features: `Services_Cortana`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CortanaPermission(pub i32);
impl CortanaPermission {
    pub const BrowsingHistory: CortanaPermission = CortanaPermission(0i32);
    pub const Calendar: CortanaPermission = CortanaPermission(1i32);
    pub const CallHistory: CortanaPermission = CortanaPermission(2i32);
    pub const Contacts: CortanaPermission = CortanaPermission(3i32);
    pub const Email: CortanaPermission = CortanaPermission(4i32);
    pub const InputPersonalization: CortanaPermission = CortanaPermission(5i32);
    pub const Location: CortanaPermission = CortanaPermission(6i32);
    pub const Messaging: CortanaPermission = CortanaPermission(7i32);
    pub const Microphone: CortanaPermission = CortanaPermission(8i32);
    pub const Personalization: CortanaPermission = CortanaPermission(9i32);
    pub const PhoneCall: CortanaPermission = CortanaPermission(10i32);
}
impl ::core::convert::From<i32> for CortanaPermission {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CortanaPermission {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CortanaPermission {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Cortana.CortanaPermission;i4)");
}
impl ::windows::runtime::DefaultType for CortanaPermission {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_Cortana`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CortanaPermissionsChangeResult(pub i32);
impl CortanaPermissionsChangeResult {
    pub const Success: CortanaPermissionsChangeResult = CortanaPermissionsChangeResult(0i32);
    pub const Unavailable: CortanaPermissionsChangeResult = CortanaPermissionsChangeResult(1i32);
    pub const DisabledByPolicy: CortanaPermissionsChangeResult = CortanaPermissionsChangeResult(2i32);
}
impl ::core::convert::From<i32> for CortanaPermissionsChangeResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CortanaPermissionsChangeResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CortanaPermissionsChangeResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Cortana.CortanaPermissionsChangeResult;i4)");
}
impl ::windows::runtime::DefaultType for CortanaPermissionsChangeResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_Cortana`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CortanaPermissionsManager(pub ::windows::runtime::IInspectable);
impl CortanaPermissionsManager {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Services_Cortana`*"]
    pub fn IsSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Services_Cortana`, `Foundation`, `Foundation_Collections`*"]
    pub fn ArePermissionsGrantedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<CortanaPermission>>>(&self, permissions: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), permissions.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Services_Cortana`, `Foundation`, `Foundation_Collections`*"]
    pub fn GrantPermissionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<CortanaPermission>>>(&self, permissions: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<CortanaPermissionsChangeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), permissions.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CortanaPermissionsChangeResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Services_Cortana`, `Foundation`, `Foundation_Collections`*"]
    pub fn RevokePermissionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<CortanaPermission>>>(&self, permissions: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<CortanaPermissionsChangeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), permissions.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CortanaPermissionsChangeResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Services_Cortana`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<CortanaPermissionsManager> {
        Self::ICortanaPermissionsManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CortanaPermissionsManager>(result__)
        })
    }
    pub fn ICortanaPermissionsManagerStatics<R, F: FnOnce(&ICortanaPermissionsManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CortanaPermissionsManager, ICortanaPermissionsManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CortanaPermissionsManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Cortana.CortanaPermissionsManager;{191330e0-8695-438a-9545-3da4e822ddb4})");
}
unsafe impl ::windows::runtime::Interface for CortanaPermissionsManager {
    type Vtable = ICortanaPermissionsManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x191330e0_8695_438a_9545_3da4e822ddb4);
}
impl ::windows::runtime::RuntimeName for CortanaPermissionsManager {
    const NAME: &'static str = "Windows.Services.Cortana.CortanaPermissionsManager";
}
impl ::core::convert::From<CortanaPermissionsManager> for ::windows::runtime::IUnknown {
    fn from(value: CortanaPermissionsManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CortanaPermissionsManager> for ::windows::runtime::IUnknown {
    fn from(value: &CortanaPermissionsManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CortanaPermissionsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CortanaPermissionsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CortanaPermissionsManager> for ::windows::runtime::IInspectable {
    fn from(value: CortanaPermissionsManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CortanaPermissionsManager> for ::windows::runtime::IInspectable {
    fn from(value: &CortanaPermissionsManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CortanaPermissionsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CortanaPermissionsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CortanaPermissionsManager {}
unsafe impl ::core::marker::Sync for CortanaPermissionsManager {}
#[doc = "*Required features: `Services_Cortana`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CortanaSettings(pub ::windows::runtime::IInspectable);
impl CortanaSettings {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Services_Cortana`*"]
    pub fn HasUserConsentToVoiceActivation(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Services_Cortana`*"]
    pub fn IsVoiceActivationEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Services_Cortana`*"]
    pub fn SetIsVoiceActivationEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Services_Cortana`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::ICortanaSettingsStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Services_Cortana`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<CortanaSettings> {
        Self::ICortanaSettingsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CortanaSettings>(result__)
        })
    }
    pub fn ICortanaSettingsStatics<R, F: FnOnce(&ICortanaSettingsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CortanaSettings, ICortanaSettingsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CortanaSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Cortana.CortanaSettings;{54d571a7-8062-40f4-abe7-dedfd697b019})");
}
unsafe impl ::windows::runtime::Interface for CortanaSettings {
    type Vtable = ICortanaSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x54d571a7_8062_40f4_abe7_dedfd697b019);
}
impl ::windows::runtime::RuntimeName for CortanaSettings {
    const NAME: &'static str = "Windows.Services.Cortana.CortanaSettings";
}
impl ::core::convert::From<CortanaSettings> for ::windows::runtime::IUnknown {
    fn from(value: CortanaSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CortanaSettings> for ::windows::runtime::IUnknown {
    fn from(value: &CortanaSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CortanaSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CortanaSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CortanaSettings> for ::windows::runtime::IInspectable {
    fn from(value: CortanaSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CortanaSettings> for ::windows::runtime::IInspectable {
    fn from(value: &CortanaSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CortanaSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CortanaSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CortanaSettings {}
unsafe impl ::core::marker::Sync for CortanaSettings {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICortanaActionableInsights(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICortanaActionableInsights {
    type Vtable = ICortanaActionableInsights_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x951ec6b1_fc83_586d_8b84_2452c8981625);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaActionableInsights_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagestream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagestream: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datapackage: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation")))] usize,
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datapackage: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICortanaActionableInsightsOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICortanaActionableInsightsOptions {
    type Vtable = ICortanaActionableInsightsOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaac2bbcf_9782_5420_b81e_7ae56af31815);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaActionableInsightsOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICortanaActionableInsightsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICortanaActionableInsightsStatics {
    type Vtable = ICortanaActionableInsightsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb5ded412_9d2f_5cb5_9b05_356a0b836c10);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaActionableInsightsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICortanaPermissionsManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICortanaPermissionsManager {
    type Vtable = ICortanaPermissionsManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x191330e0_8695_438a_9545_3da4e822ddb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaPermissionsManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, permissions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, permissions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, permissions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICortanaPermissionsManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICortanaPermissionsManagerStatics {
    type Vtable = ICortanaPermissionsManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x76b1e67a_b045_4414_9d6d_2ad3a5fe3a7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaPermissionsManagerStatics_abi(
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
pub struct ICortanaSettings(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICortanaSettings {
    type Vtable = ICortanaSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x54d571a7_8062_40f4_abe7_dedfd697b019);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICortanaSettingsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICortanaSettingsStatics {
    type Vtable = ICortanaSettingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8b2ccd7e_2ec0_446d_9285_33f07ce8ac04);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
