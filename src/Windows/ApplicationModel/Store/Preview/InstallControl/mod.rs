#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppInstallItem(::windows::runtime::IInspectable);
impl AppInstallItem {
    pub fn ProductId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn PackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn InstallType(&self) -> ::windows::runtime::Result<AppInstallType> {
        let this = self;
        unsafe {
            let mut result__: AppInstallType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallType>(result__)
        }
    }
    pub fn IsUserInitiated(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn GetCurrentStatus(&self) -> ::windows::runtime::Result<AppInstallStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallStatus>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn Restart(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Completed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn CancelWithTelemetry<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, correlationvector: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), correlationvector.into_param().abi()).ok() }
    }
    pub fn PauseWithTelemetry<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, correlationvector: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), correlationvector.into_param().abi()).ok() }
    }
    pub fn RestartWithTelemetry<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, correlationvector: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), correlationvector.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>(result__)
        }
    }
    pub fn ItemOperationsMightAffectOtherItems(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn LaunchAfterInstall(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem4>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetLaunchAfterInstall(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn PinToDesktopAfterInstall(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToDesktopAfterInstall(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn PinToStartAfterInstall(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToStartAfterInstall(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn PinToTaskbarAfterInstall(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToTaskbarAfterInstall(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn CompletedInstallToastNotificationMode(&self) -> ::windows::runtime::Result<AppInstallationToastNotificationMode> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__: AppInstallationToastNotificationMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallationToastNotificationMode>(result__)
        }
    }
    pub fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn InstallInProgressToastNotificationMode(&self) -> ::windows::runtime::Result<AppInstallationToastNotificationMode> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__: AppInstallationToastNotificationMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallationToastNotificationMode>(result__)
        }
    }
    pub fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppInstallItem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem;{49d3dfab-168a-4cbf-a93a-9e448c82737d})");
}
unsafe impl ::windows::runtime::Interface for AppInstallItem {
    type Vtable = IAppInstallItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1238622123, 5770, 19647, [169, 58, 158, 68, 140, 130, 115, 125]);
}
impl ::windows::runtime::RuntimeName for AppInstallItem {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem";
}
impl ::std::convert::From<AppInstallItem> for ::windows::runtime::IUnknown {
    fn from(value: AppInstallItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppInstallItem> for ::windows::runtime::IUnknown {
    fn from(value: &AppInstallItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppInstallItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppInstallItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AppInstallItem> for ::windows::runtime::IInspectable {
    fn from(value: AppInstallItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppInstallItem> for ::windows::runtime::IInspectable {
    fn from(value: &AppInstallItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppInstallItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppInstallItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppInstallItem {}
unsafe impl ::std::marker::Sync for AppInstallItem {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppInstallManager(::windows::runtime::IInspectable);
impl AppInstallManager {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppInstallManager, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppInstallItems(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>(result__)
        }
    }
    pub fn Cancel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, productid: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), productid.into_param().abi()).ok() }
    }
    pub fn Pause<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, productid: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), productid.into_param().abi()).ok() }
    }
    pub fn Restart<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, productid: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), productid.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ItemCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ItemStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn AutoUpdateSetting(&self) -> ::windows::runtime::Result<AutoUpdateSetting> {
        let this = self;
        unsafe {
            let mut result__: AutoUpdateSetting = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AutoUpdateSetting>(result__)
        }
    }
    pub fn SetAutoUpdateSetting(&self, value: AutoUpdateSetting) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn AcquisitionIdentity(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAcquisitionIdentity<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetIsApplicableAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, productid: Param0, skuid: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartAppInstallAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, productid: Param0, skuid: Param1, repair: bool, forceuseofnonremovablestorage: bool) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), repair, forceuseofnonremovablestorage, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateAppByPackageFamilyNameAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SearchForUpdatesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, productid: Param0, skuid: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SearchForAllUpdatesAsync(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn IsStoreBlockedByPolicyAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, storeclientname: Param0, storeclientpublisher: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), storeclientname.into_param().abi(), storeclientpublisher.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetIsAppAllowedToInstallAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, productid: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), productid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartAppInstallWithTelemetryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param6: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        productid: Param0,
        skuid: Param1,
        repair: bool,
        forceuseofnonremovablestorage: bool,
        catalogid: Param4,
        bundleid: Param5,
        correlationvector: Param6,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), repair, forceuseofnonremovablestorage, catalogid.into_param().abi(), bundleid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateAppByPackageFamilyNameWithTelemetryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefamilyname: Param0, correlationvector: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), packagefamilyname.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SearchForUpdatesWithTelemetryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        productid: Param0,
        skuid: Param1,
        catalogid: Param2,
        correlationvector: Param3,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), catalogid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SearchForAllUpdatesWithTelemetryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, correlationvector: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetIsAppAllowedToInstallWithTelemetryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        productid: Param0,
        skuid: Param1,
        catalogid: Param2,
        correlationvector: Param3,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), catalogid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn CancelWithTelemetry<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, productid: Param0, correlationvector: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), productid.into_param().abi(), correlationvector.into_param().abi()).ok() }
    }
    pub fn PauseWithTelemetry<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, productid: Param0, correlationvector: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), productid.into_param().abi(), correlationvector.into_param().abi()).ok() }
    }
    pub fn RestartWithTelemetry<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, productid: Param0, correlationvector: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), productid.into_param().abi(), correlationvector.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub fn StartProductInstallAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param6: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param7: ::windows::runtime::IntoParam<'a, super::super::super::super::Management::Deployment::PackageVolume>,
    >(
        &self,
        productid: Param0,
        catalogid: Param1,
        flightid: Param2,
        clientid: Param3,
        repair: bool,
        forceuseofnonremovablestorage: bool,
        correlationvector: Param6,
        targetvolume: Param7,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), productid.into_param().abi(), catalogid.into_param().abi(), flightid.into_param().abi(), clientid.into_param().abi(), repair, forceuseofnonremovablestorage, correlationvector.into_param().abi(), targetvolume.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System"))]
    pub fn StartProductInstallForUserAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::System::User>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param7: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param8: ::windows::runtime::IntoParam<'a, super::super::super::super::Management::Deployment::PackageVolume>,
    >(
        &self,
        user: Param0,
        productid: Param1,
        catalogid: Param2,
        flightid: Param3,
        clientid: Param4,
        repair: bool,
        forceuseofnonremovablestorage: bool,
        correlationvector: Param7,
        targetvolume: Param8,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), catalogid.into_param().abi(), flightid.into_param().abi(), clientid.into_param().abi(), repair, forceuseofnonremovablestorage, correlationvector.into_param().abi(), targetvolume.into_param().abi(), &mut result__)
                .from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UpdateAppByPackageFamilyNameForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, user: Param0, packagefamilyname: Param1, correlationvector: Param2) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), user.into_param().abi(), packagefamilyname.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn SearchForUpdatesForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        user: Param0,
        productid: Param1,
        skuid: Param2,
        catalogid: Param3,
        correlationvector: Param4,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), skuid.into_param().abi(), catalogid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))]
    pub fn SearchForAllUpdatesForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, user: Param0, correlationvector: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), user.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetIsAppAllowedToInstallForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        user: Param0,
        productid: Param1,
        skuid: Param2,
        catalogid: Param3,
        correlationvector: Param4,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), skuid.into_param().abi(), catalogid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetIsApplicableForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, user: Param0, productid: Param1, skuid: Param2) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), skuid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn MoveToFrontOfDownloadQueue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, productid: Param0, correlationvector: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), productid.into_param().abi(), correlationvector.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFreeUserEntitlementAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, storeid: Param0, campaignid: Param1, correlationvector: Param2) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), storeid.into_param().abi(), campaignid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetFreeUserEntitlementForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        user: Param0,
        storeid: Param1,
        campaignid: Param2,
        correlationvector: Param3,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), storeid.into_param().abi(), campaignid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFreeDeviceEntitlementAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, storeid: Param0, campaignid: Param1, correlationvector: Param2) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), storeid.into_param().abi(), campaignid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppInstallItemsWithGroupSupport(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SearchForAllUpdatesWithUpdateOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, AppUpdateOptions>>(
        &self,
        correlationvector: Param0,
        clientid: Param1,
        updateoptions: Param2,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), correlationvector.into_param().abi(), clientid.into_param().abi(), updateoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))]
    pub fn SearchForAllUpdatesWithUpdateOptionsForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, AppUpdateOptions>>(
        &self,
        user: Param0,
        correlationvector: Param1,
        clientid: Param2,
        updateoptions: Param3,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), correlationvector.into_param().abi(), clientid.into_param().abi(), updateoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SearchForUpdatesWithUpdateOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, AppUpdateOptions>>(
        &self,
        productid: Param0,
        skuid: Param1,
        correlationvector: Param2,
        clientid: Param3,
        updateoptions: Param4,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), correlationvector.into_param().abi(), clientid.into_param().abi(), updateoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn SearchForUpdatesWithUpdateOptionsForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param5: ::windows::runtime::IntoParam<'a, AppUpdateOptions>>(
        &self,
        user: Param0,
        productid: Param1,
        skuid: Param2,
        correlationvector: Param3,
        clientid: Param4,
        updateoptions: Param5,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), skuid.into_param().abi(), correlationvector.into_param().abi(), clientid.into_param().abi(), updateoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn StartProductInstallWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, AppInstallOptions>>(
        &self,
        productid: Param0,
        flightid: Param1,
        clientid: Param2,
        correlationvector: Param3,
        installoptions: Param4,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), productid.into_param().abi(), flightid.into_param().abi(), clientid.into_param().abi(), correlationvector.into_param().abi(), installoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))]
    pub fn StartProductInstallWithOptionsForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param5: ::windows::runtime::IntoParam<'a, AppInstallOptions>>(
        &self,
        user: Param0,
        productid: Param1,
        flightid: Param2,
        clientid: Param3,
        correlationvector: Param4,
        installoptions: Param5,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), flightid.into_param().abi(), clientid.into_param().abi(), correlationvector.into_param().abi(), installoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetIsPackageIdentityAllowedToInstallAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, correlationvector: Param0, packageidentityname: Param1, publishercertificatename: Param2) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), correlationvector.into_param().abi(), packageidentityname.into_param().abi(), publishercertificatename.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetIsPackageIdentityAllowedToInstallForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        user: Param0,
        correlationvector: Param1,
        packageidentityname: Param2,
        publishercertificatename: Param3,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), user.into_param().abi(), correlationvector.into_param().abi(), packageidentityname.into_param().abi(), publishercertificatename.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn CanInstallForAllUsers(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallManager7>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppInstallManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager;{9353e170-8441-4b45-bd72-7c2fa925beee})");
}
unsafe impl ::windows::runtime::Interface for AppInstallManager {
    type Vtable = IAppInstallManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2471747952, 33857, 19269, [189, 114, 124, 47, 169, 37, 190, 238]);
}
impl ::windows::runtime::RuntimeName for AppInstallManager {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager";
}
impl ::std::convert::From<AppInstallManager> for ::windows::runtime::IUnknown {
    fn from(value: AppInstallManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppInstallManager> for ::windows::runtime::IUnknown {
    fn from(value: &AppInstallManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppInstallManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppInstallManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AppInstallManager> for ::windows::runtime::IInspectable {
    fn from(value: AppInstallManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppInstallManager> for ::windows::runtime::IInspectable {
    fn from(value: &AppInstallManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppInstallManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppInstallManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppInstallManager {}
unsafe impl ::std::marker::Sync for AppInstallManager {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppInstallManagerItemEventArgs(::windows::runtime::IInspectable);
impl AppInstallManagerItemEventArgs {
    pub fn Item(&self) -> ::windows::runtime::Result<AppInstallItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallItem>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppInstallManagerItemEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs;{bc505743-4674-4dd1-957e-c25682086a14})");
}
unsafe impl ::windows::runtime::Interface for AppInstallManagerItemEventArgs {
    type Vtable = IAppInstallManagerItemEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3159381827, 18036, 19921, [149, 126, 194, 86, 130, 8, 106, 20]);
}
impl ::windows::runtime::RuntimeName for AppInstallManagerItemEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs";
}
impl ::std::convert::From<AppInstallManagerItemEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppInstallManagerItemEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppInstallManagerItemEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppInstallManagerItemEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppInstallManagerItemEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppInstallManagerItemEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AppInstallManagerItemEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppInstallManagerItemEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppInstallManagerItemEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppInstallManagerItemEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppInstallManagerItemEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppInstallManagerItemEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppInstallManagerItemEventArgs {}
unsafe impl ::std::marker::Sync for AppInstallManagerItemEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppInstallOptions(::windows::runtime::IInspectable);
impl AppInstallOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppInstallOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CatalogId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetCatalogId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ForceUseOfNonRemovableStorage(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceUseOfNonRemovableStorage(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn AllowForcedAppRestart(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Repair(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetRepair(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Management_Deployment")]
    pub fn TargetVolume(&self) -> ::windows::runtime::Result<super::super::super::super::Management::Deployment::PackageVolume> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Management::Deployment::PackageVolume>(result__)
        }
    }
    #[cfg(feature = "Management_Deployment")]
    pub fn SetTargetVolume<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Management::Deployment::PackageVolume>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn LaunchAfterInstall(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetLaunchAfterInstall(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn PinToDesktopAfterInstall(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToDesktopAfterInstall(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn PinToStartAfterInstall(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToStartAfterInstall(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn PinToTaskbarAfterInstall(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToTaskbarAfterInstall(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn CompletedInstallToastNotificationMode(&self) -> ::windows::runtime::Result<AppInstallationToastNotificationMode> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: AppInstallationToastNotificationMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallationToastNotificationMode>(result__)
        }
    }
    pub fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn InstallInProgressToastNotificationMode(&self) -> ::windows::runtime::Result<AppInstallationToastNotificationMode> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: AppInstallationToastNotificationMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallationToastNotificationMode>(result__)
        }
    }
    pub fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn InstallForAllUsers(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetInstallForAllUsers(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn StageButDoNotInstall(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetStageButDoNotInstall(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn CampaignId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetCampaignId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ExtendedCampaignId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetExtendedCampaignId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppInstallOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions;{c9808300-1cb8-4eb6-8c9f-6a30c64a5b51})");
}
unsafe impl ::windows::runtime::Interface for AppInstallOptions {
    type Vtable = IAppInstallOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3380642560, 7352, 20150, [140, 159, 106, 48, 198, 74, 91, 81]);
}
impl ::windows::runtime::RuntimeName for AppInstallOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions";
}
impl ::std::convert::From<AppInstallOptions> for ::windows::runtime::IUnknown {
    fn from(value: AppInstallOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppInstallOptions> for ::windows::runtime::IUnknown {
    fn from(value: &AppInstallOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppInstallOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppInstallOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AppInstallOptions> for ::windows::runtime::IInspectable {
    fn from(value: AppInstallOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppInstallOptions> for ::windows::runtime::IInspectable {
    fn from(value: &AppInstallOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppInstallOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppInstallOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppInstallOptions {}
unsafe impl ::std::marker::Sync for AppInstallOptions {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppInstallState(pub i32);
impl AppInstallState {
    pub const Pending: AppInstallState = AppInstallState(0i32);
    pub const Starting: AppInstallState = AppInstallState(1i32);
    pub const AcquiringLicense: AppInstallState = AppInstallState(2i32);
    pub const Downloading: AppInstallState = AppInstallState(3i32);
    pub const RestoringData: AppInstallState = AppInstallState(4i32);
    pub const Installing: AppInstallState = AppInstallState(5i32);
    pub const Completed: AppInstallState = AppInstallState(6i32);
    pub const Canceled: AppInstallState = AppInstallState(7i32);
    pub const Paused: AppInstallState = AppInstallState(8i32);
    pub const Error: AppInstallState = AppInstallState(9i32);
    pub const PausedLowBattery: AppInstallState = AppInstallState(10i32);
    pub const PausedWiFiRecommended: AppInstallState = AppInstallState(11i32);
    pub const PausedWiFiRequired: AppInstallState = AppInstallState(12i32);
    pub const ReadyToDownload: AppInstallState = AppInstallState(13i32);
}
impl ::std::convert::From<i32> for AppInstallState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppInstallState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppInstallState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallState;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppInstallStatus(::windows::runtime::IInspectable);
impl AppInstallStatus {
    pub fn InstallState(&self) -> ::windows::runtime::Result<AppInstallState> {
        let this = self;
        unsafe {
            let mut result__: AppInstallState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallState>(result__)
        }
    }
    pub fn DownloadSizeInBytes(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn BytesDownloaded(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn PercentComplete(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallStatus2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::System::User>(result__)
        }
    }
    pub fn ReadyForLaunch(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallStatus2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsStaged(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppInstallStatus3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppInstallStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus;{936dccfa-2450-4126-88b1-6127a644dd5c})");
}
unsafe impl ::windows::runtime::Interface for AppInstallStatus {
    type Vtable = IAppInstallStatus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2473446650, 9296, 16678, [136, 177, 97, 39, 166, 68, 221, 92]);
}
impl ::windows::runtime::RuntimeName for AppInstallStatus {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus";
}
impl ::std::convert::From<AppInstallStatus> for ::windows::runtime::IUnknown {
    fn from(value: AppInstallStatus) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppInstallStatus> for ::windows::runtime::IUnknown {
    fn from(value: &AppInstallStatus) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppInstallStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppInstallStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AppInstallStatus> for ::windows::runtime::IInspectable {
    fn from(value: AppInstallStatus) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppInstallStatus> for ::windows::runtime::IInspectable {
    fn from(value: &AppInstallStatus) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppInstallStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppInstallStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppInstallStatus {}
unsafe impl ::std::marker::Sync for AppInstallStatus {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppInstallType(pub i32);
impl AppInstallType {
    pub const Install: AppInstallType = AppInstallType(0i32);
    pub const Update: AppInstallType = AppInstallType(1i32);
    pub const Repair: AppInstallType = AppInstallType(2i32);
}
impl ::std::convert::From<i32> for AppInstallType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppInstallType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppInstallType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallType;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppInstallationToastNotificationMode(pub i32);
impl AppInstallationToastNotificationMode {
    pub const Default: AppInstallationToastNotificationMode = AppInstallationToastNotificationMode(0i32);
    pub const Toast: AppInstallationToastNotificationMode = AppInstallationToastNotificationMode(1i32);
    pub const ToastWithoutPopup: AppInstallationToastNotificationMode = AppInstallationToastNotificationMode(2i32);
    pub const NoToast: AppInstallationToastNotificationMode = AppInstallationToastNotificationMode(3i32);
}
impl ::std::convert::From<i32> for AppInstallationToastNotificationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppInstallationToastNotificationMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppInstallationToastNotificationMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallationToastNotificationMode;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppUpdateOptions(::windows::runtime::IInspectable);
impl AppUpdateOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppUpdateOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CatalogId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetCatalogId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn AllowForcedAppRestart(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn AutomaticallyDownloadAndInstallUpdateIfFound(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppUpdateOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutomaticallyDownloadAndInstallUpdateIfFound(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppUpdateOptions2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppUpdateOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions;{26f0b02f-c2f3-4aea-af8c-6308dd9db85f})");
}
unsafe impl ::windows::runtime::Interface for AppUpdateOptions {
    type Vtable = IAppUpdateOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(653307951, 49907, 19178, [175, 140, 99, 8, 221, 157, 184, 95]);
}
impl ::windows::runtime::RuntimeName for AppUpdateOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions";
}
impl ::std::convert::From<AppUpdateOptions> for ::windows::runtime::IUnknown {
    fn from(value: AppUpdateOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppUpdateOptions> for ::windows::runtime::IUnknown {
    fn from(value: &AppUpdateOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppUpdateOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppUpdateOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AppUpdateOptions> for ::windows::runtime::IInspectable {
    fn from(value: AppUpdateOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppUpdateOptions> for ::windows::runtime::IInspectable {
    fn from(value: &AppUpdateOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppUpdateOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppUpdateOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppUpdateOptions {}
unsafe impl ::std::marker::Sync for AppUpdateOptions {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutoUpdateSetting(pub i32);
impl AutoUpdateSetting {
    pub const Disabled: AutoUpdateSetting = AutoUpdateSetting(0i32);
    pub const Enabled: AutoUpdateSetting = AutoUpdateSetting(1i32);
    pub const DisabledByPolicy: AutoUpdateSetting = AutoUpdateSetting(2i32);
    pub const EnabledByPolicy: AutoUpdateSetting = AutoUpdateSetting(3i32);
}
impl ::std::convert::From<i32> for AutoUpdateSetting {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AutoUpdateSetting {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AutoUpdateSetting {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AutoUpdateSetting;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GetEntitlementResult(::windows::runtime::IInspectable);
impl GetEntitlementResult {
    pub fn Status(&self) -> ::windows::runtime::Result<GetEntitlementStatus> {
        let this = self;
        unsafe {
            let mut result__: GetEntitlementStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GetEntitlementStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GetEntitlementResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult;{74fc843f-1a9e-4609-8e4d-819086d08a3d})");
}
unsafe impl ::windows::runtime::Interface for GetEntitlementResult {
    type Vtable = IGetEntitlementResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1962705983, 6814, 17929, [142, 77, 129, 144, 134, 208, 138, 61]);
}
impl ::windows::runtime::RuntimeName for GetEntitlementResult {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult";
}
impl ::std::convert::From<GetEntitlementResult> for ::windows::runtime::IUnknown {
    fn from(value: GetEntitlementResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GetEntitlementResult> for ::windows::runtime::IUnknown {
    fn from(value: &GetEntitlementResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GetEntitlementResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GetEntitlementResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GetEntitlementResult> for ::windows::runtime::IInspectable {
    fn from(value: GetEntitlementResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GetEntitlementResult> for ::windows::runtime::IInspectable {
    fn from(value: &GetEntitlementResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GetEntitlementResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GetEntitlementResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GetEntitlementResult {}
unsafe impl ::std::marker::Sync for GetEntitlementResult {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GetEntitlementStatus(pub i32);
impl GetEntitlementStatus {
    pub const Succeeded: GetEntitlementStatus = GetEntitlementStatus(0i32);
    pub const NoStoreAccount: GetEntitlementStatus = GetEntitlementStatus(1i32);
    pub const NetworkError: GetEntitlementStatus = GetEntitlementStatus(2i32);
    pub const ServerError: GetEntitlementStatus = GetEntitlementStatus(3i32);
}
impl ::std::convert::From<i32> for GetEntitlementStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GetEntitlementStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GetEntitlementStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementStatus;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallItem(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallItem {
    type Vtable = IAppInstallItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1238622123, 5770, 19647, [169, 58, 158, 68, 140, 130, 115, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppInstallType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallItem2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallItem2 {
    type Vtable = IAppInstallItem2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3549899512, 16576, 20439, [170, 108, 10, 161, 60, 166, 24, 140]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallItem3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallItem3 {
    type Vtable = IAppInstallItem3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1866320280, 56647, 17212, [146, 52, 86, 1, 114, 214, 122, 69]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallItem4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallItem4 {
    type Vtable = IAppInstallItem4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3268529682, 29183, 20424, [181, 64, 69, 61, 75, 55, 225, 209]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallItem5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallItem5 {
    type Vtable = IAppInstallItem5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1427171276, 16502, 18955, [148, 114, 194, 29, 157, 56, 14, 85]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppInstallationToastNotificationMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: AppInstallationToastNotificationMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppInstallationToastNotificationMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: AppInstallationToastNotificationMode) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallManager {
    type Vtable = IAppInstallManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2471747952, 33857, 19269, [189, 114, 124, 47, 169, 37, 190, 238]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AutoUpdateSetting) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: AutoUpdateSetting) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, skuid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, skuid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, skuid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storeclientname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, storeclientpublisher: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallManager2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallManager2 {
    type Vtable = IAppInstallManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(378763345, 60727, 18445, [131, 20, 82, 226, 124, 3, 240, 74]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        skuid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        repair: bool,
        forceuseofnonremovablestorage: bool,
        catalogid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        bundleid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, skuid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, catalogid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, skuid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, catalogid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallManager3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallManager3 {
    type Vtable = IAppInstallManager3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2511489815, 59754, 19726, [132, 225, 200, 203, 65, 122, 1, 120]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        catalogid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        flightid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        clientid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        repair: bool,
        forceuseofnonremovablestorage: bool,
        correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        targetvolume: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        user: ::windows::runtime::RawPtr,
        productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        catalogid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        flightid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        clientid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        repair: bool,
        forceuseofnonremovablestorage: bool,
        correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        targetvolume: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, skuid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, catalogid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, skuid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, catalogid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, skuid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallManager4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallManager4 {
    type Vtable = IAppInstallManager4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(638200342, 23198, 20157, [185, 68, 242, 186, 117, 195, 17, 89]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storeid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, campaignid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, storeid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, campaignid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storeid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, campaignid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallManager5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallManager5 {
    type Vtable = IAppInstallManager5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1020771916, 7145, 20351, [182, 117, 170, 29, 100, 165, 41, 178]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallManager6(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallManager6 {
    type Vtable = IAppInstallManager6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3387413512, 62074, 17521, [178, 244, 231, 110, 252, 190, 188, 202]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager6_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, clientid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, updateoptions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, clientid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, updateoptions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, skuid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, clientid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, updateoptions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, skuid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, clientid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, updateoptions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, flightid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, clientid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, installoptions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, flightid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, clientid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, installoptions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packageidentityname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, publishercertificatename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, correlationvector: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packageidentityname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, publishercertificatename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallManager7(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallManager7 {
    type Vtable = IAppInstallManager7_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2783869744, 54756, 18851, [152, 83, 61, 176, 50, 3, 50, 29]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager7_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallManagerItemEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallManagerItemEventArgs {
    type Vtable = IAppInstallManagerItemEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3159381827, 18036, 19921, [149, 126, 194, 86, 130, 8, 106, 20]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManagerItemEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallOptions {
    type Vtable = IAppInstallOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3380642560, 7352, 20150, [140, 159, 106, 48, 198, 74, 91, 81]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Management_Deployment")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))] usize,
    #[cfg(feature = "Management_Deployment")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallOptions2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallOptions2 {
    type Vtable = IAppInstallOptions2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2315567319, 51531, 16990, [149, 180, 191, 39, 250, 234, 238, 137]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallOptions2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppInstallationToastNotificationMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: AppInstallationToastNotificationMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppInstallationToastNotificationMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: AppInstallationToastNotificationMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallStatus(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallStatus {
    type Vtable = IAppInstallStatus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2473446650, 9296, 16678, [136, 177, 97, 39, 166, 68, 221, 92]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppInstallState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallStatus2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallStatus2 {
    type Vtable = IAppInstallStatus2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2531754378, 24210, 19113, [142, 220, 88, 254, 212, 184, 126, 0]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppInstallStatus3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallStatus3 {
    type Vtable = IAppInstallStatus3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3414690902, 33659, 19276, [158, 187, 109, 68, 160, 169, 99, 7]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppUpdateOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppUpdateOptions {
    type Vtable = IAppUpdateOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(653307951, 49907, 19178, [175, 140, 99, 8, 221, 157, 184, 95]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUpdateOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppUpdateOptions2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppUpdateOptions2 {
    type Vtable = IAppUpdateOptions2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4100222472, 60710, 19449, [150, 121, 72, 246, 40, 229, 61, 248]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUpdateOptions2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGetEntitlementResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGetEntitlementResult {
    type Vtable = IGetEntitlementResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1962705983, 6814, 17929, [142, 77, 129, 144, 134, 208, 138, 61]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetEntitlementResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GetEntitlementStatus) -> ::windows::runtime::HRESULT,
);
