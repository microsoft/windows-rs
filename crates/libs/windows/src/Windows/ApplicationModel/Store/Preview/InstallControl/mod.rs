#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallItem(::windows::core::IUnknown);
impl AppInstallItem {
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProductId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackageFamilyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn InstallType(&self) -> ::windows::core::Result<AppInstallType> {
        let this = self;
        unsafe {
            let mut result__: AppInstallType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InstallType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallType>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn IsUserInitiated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsUserInitiated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn GetCurrentStatus(&self) -> ::windows::core::Result<AppInstallStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Cancel)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Pause)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn Restart(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Restart)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Completed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn CancelWithTelemetry<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, correlationvector: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).CancelWithTelemetry)(::core::mem::transmute_copy(this), correlationvector.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn PauseWithTelemetry<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, correlationvector: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PauseWithTelemetry)(::core::mem::transmute_copy(this), correlationvector.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn RestartWithTelemetry<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, correlationvector: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RestartWithTelemetry)(::core::mem::transmute_copy(this), correlationvector.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Children)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn ItemOperationsMightAffectOtherItems(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemOperationsMightAffectOtherItems)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn LaunchAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchAfterInstall)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetLaunchAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLaunchAfterInstall)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn PinToDesktopAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PinToDesktopAfterInstall)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetPinToDesktopAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPinToDesktopAfterInstall)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn PinToStartAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PinToStartAfterInstall)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetPinToStartAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPinToStartAfterInstall)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn PinToTaskbarAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PinToTaskbarAfterInstall)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetPinToTaskbarAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPinToTaskbarAfterInstall)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn CompletedInstallToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__: AppInstallationToastNotificationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CompletedInstallToastNotificationMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallationToastNotificationMode>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCompletedInstallToastNotificationMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn InstallInProgressToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__: AppInstallationToastNotificationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InstallInProgressToastNotificationMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallationToastNotificationMode>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInstallInProgressToastNotificationMode)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for AppInstallItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallItem {}
impl ::core::fmt::Debug for AppInstallItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem;{49d3dfab-168a-4cbf-a93a-9e448c82737d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppInstallItem {
    type Vtable = IAppInstallItem_Vtbl;
    const IID: ::windows::core::GUID = <IAppInstallItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInstallItem {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem";
}
impl ::core::convert::From<AppInstallItem> for ::windows::core::IUnknown {
    fn from(value: AppInstallItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallItem> for ::windows::core::IUnknown {
    fn from(value: &AppInstallItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppInstallItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppInstallItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallItem> for ::windows::core::IInspectable {
    fn from(value: AppInstallItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallItem> for ::windows::core::IInspectable {
    fn from(value: &AppInstallItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppInstallItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppInstallItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallItem {}
unsafe impl ::core::marker::Sync for AppInstallItem {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallManager(::windows::core::IUnknown);
impl AppInstallManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppInstallManager, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppInstallItems(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppInstallItems)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn Cancel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Cancel)(::core::mem::transmute_copy(this), productid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn Pause<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Pause)(::core::mem::transmute_copy(this), productid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn Restart<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Restart)(::core::mem::transmute_copy(this), productid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveItemCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveItemStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn AutoUpdateSetting(&self) -> ::windows::core::Result<AutoUpdateSetting> {
        let this = self;
        unsafe {
            let mut result__: AutoUpdateSetting = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutoUpdateSetting)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutoUpdateSetting>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetAutoUpdateSetting(&self, value: AutoUpdateSetting) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAutoUpdateSetting)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn AcquisitionIdentity(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AcquisitionIdentity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetAcquisitionIdentity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAcquisitionIdentity)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsApplicableAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, skuid: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIsApplicableAsync)(::core::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAppInstallAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, skuid: Param1, repair: bool, forceuseofnonremovablestorage: bool) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartAppInstallAsync)(::core::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), repair, forceuseofnonremovablestorage, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateAppByPackageFamilyNameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateAppByPackageFamilyNameAsync)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SearchForUpdatesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, skuid: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SearchForUpdatesAsync)(::core::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SearchForAllUpdatesAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SearchForAllUpdatesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsStoreBlockedByPolicyAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, storeclientname: Param0, storeclientpublisher: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStoreBlockedByPolicyAsync)(::core::mem::transmute_copy(this), storeclientname.into_param().abi(), storeclientpublisher.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsAppAllowedToInstallAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIsAppAllowedToInstallAsync)(::core::mem::transmute_copy(this), productid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAppInstallWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param6: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, skuid: Param1, repair: bool, forceuseofnonremovablestorage: bool, catalogid: Param4, bundleid: Param5, correlationvector: Param6) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartAppInstallWithTelemetryAsync)(::core::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), repair, forceuseofnonremovablestorage, catalogid.into_param().abi(), bundleid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateAppByPackageFamilyNameWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefamilyname: Param0, correlationvector: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateAppByPackageFamilyNameWithTelemetryAsync)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SearchForUpdatesWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, skuid: Param1, catalogid: Param2, correlationvector: Param3) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SearchForUpdatesWithTelemetryAsync)(::core::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), catalogid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SearchForAllUpdatesWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, correlationvector: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SearchForAllUpdatesWithTelemetryAsync)(::core::mem::transmute_copy(this), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsAppAllowedToInstallWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, skuid: Param1, catalogid: Param2, correlationvector: Param3) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIsAppAllowedToInstallWithTelemetryAsync)(::core::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), catalogid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn CancelWithTelemetry<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, correlationvector: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).CancelWithTelemetry)(::core::mem::transmute_copy(this), productid.into_param().abi(), correlationvector.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn PauseWithTelemetry<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, correlationvector: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PauseWithTelemetry)(::core::mem::transmute_copy(this), productid.into_param().abi(), correlationvector.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn RestartWithTelemetry<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, correlationvector: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RestartWithTelemetry)(::core::mem::transmute_copy(this), productid.into_param().abi(), correlationvector.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation_Collections\"`, `\"Management_Deployment\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub fn StartProductInstallAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param6: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param7: ::windows::core::IntoParam<'a, super::super::super::super::Management::Deployment::PackageVolume>>(
        &self,
        productid: Param0,
        catalogid: Param1,
        flightid: Param2,
        clientid: Param3,
        repair: bool,
        forceuseofnonremovablestorage: bool,
        correlationvector: Param6,
        targetvolume: Param7,
    ) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartProductInstallAsync)(::core::mem::transmute_copy(this), productid.into_param().abi(), catalogid.into_param().abi(), flightid.into_param().abi(), clientid.into_param().abi(), repair, forceuseofnonremovablestorage, correlationvector.into_param().abi(), targetvolume.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation_Collections\"`, `\"Management_Deployment\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System"))]
    pub fn StartProductInstallForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param7: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param8: ::windows::core::IntoParam<'a, super::super::super::super::Management::Deployment::PackageVolume>>(
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
    ) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartProductInstallForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), catalogid.into_param().abi(), flightid.into_param().abi(), clientid.into_param().abi(), repair, forceuseofnonremovablestorage, correlationvector.into_param().abi(), targetvolume.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UpdateAppByPackageFamilyNameForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, user: Param0, packagefamilyname: Param1, correlationvector: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateAppByPackageFamilyNameForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), packagefamilyname.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn SearchForUpdatesForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, user: Param0, productid: Param1, skuid: Param2, catalogid: Param3, correlationvector: Param4) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SearchForUpdatesForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), skuid.into_param().abi(), catalogid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation_Collections\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub fn SearchForAllUpdatesForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, user: Param0, correlationvector: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SearchForAllUpdatesForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetIsAppAllowedToInstallForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, user: Param0, productid: Param1, skuid: Param2, catalogid: Param3, correlationvector: Param4) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIsAppAllowedToInstallForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), skuid.into_param().abi(), catalogid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetIsApplicableForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, user: Param0, productid: Param1, skuid: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIsApplicableForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), skuid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn MoveToFrontOfDownloadQueue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, correlationvector: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).MoveToFrontOfDownloadQueue)(::core::mem::transmute_copy(this), productid.into_param().abi(), correlationvector.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFreeUserEntitlementAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, storeid: Param0, campaignid: Param1, correlationvector: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFreeUserEntitlementAsync)(::core::mem::transmute_copy(this), storeid.into_param().abi(), campaignid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetFreeUserEntitlementForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, user: Param0, storeid: Param1, campaignid: Param2, correlationvector: Param3) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFreeUserEntitlementForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), storeid.into_param().abi(), campaignid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFreeDeviceEntitlementAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, storeid: Param0, campaignid: Param1, correlationvector: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFreeDeviceEntitlementAsync)(::core::mem::transmute_copy(this), storeid.into_param().abi(), campaignid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppInstallItemsWithGroupSupport(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppInstallItemsWithGroupSupport)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SearchForAllUpdatesWithUpdateOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, AppUpdateOptions>>(&self, correlationvector: Param0, clientid: Param1, updateoptions: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SearchForAllUpdatesWithUpdateOptionsAsync)(::core::mem::transmute_copy(this), correlationvector.into_param().abi(), clientid.into_param().abi(), updateoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation_Collections\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub fn SearchForAllUpdatesWithUpdateOptionsForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, AppUpdateOptions>>(&self, user: Param0, correlationvector: Param1, clientid: Param2, updateoptions: Param3) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SearchForAllUpdatesWithUpdateOptionsForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), correlationvector.into_param().abi(), clientid.into_param().abi(), updateoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SearchForUpdatesWithUpdateOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, AppUpdateOptions>>(&self, productid: Param0, skuid: Param1, correlationvector: Param2, clientid: Param3, updateoptions: Param4) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SearchForUpdatesWithUpdateOptionsAsync)(::core::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), correlationvector.into_param().abi(), clientid.into_param().abi(), updateoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn SearchForUpdatesWithUpdateOptionsForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, AppUpdateOptions>>(&self, user: Param0, productid: Param1, skuid: Param2, correlationvector: Param3, clientid: Param4, updateoptions: Param5) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SearchForUpdatesWithUpdateOptionsForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), skuid.into_param().abi(), correlationvector.into_param().abi(), clientid.into_param().abi(), updateoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartProductInstallWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, AppInstallOptions>>(&self, productid: Param0, flightid: Param1, clientid: Param2, correlationvector: Param3, installoptions: Param4) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartProductInstallWithOptionsAsync)(::core::mem::transmute_copy(this), productid.into_param().abi(), flightid.into_param().abi(), clientid.into_param().abi(), correlationvector.into_param().abi(), installoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation_Collections\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub fn StartProductInstallWithOptionsForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, AppInstallOptions>>(&self, user: Param0, productid: Param1, flightid: Param2, clientid: Param3, correlationvector: Param4, installoptions: Param5) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartProductInstallWithOptionsForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), flightid.into_param().abi(), clientid.into_param().abi(), correlationvector.into_param().abi(), installoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsPackageIdentityAllowedToInstallAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, correlationvector: Param0, packageidentityname: Param1, publishercertificatename: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIsPackageIdentityAllowedToInstallAsync)(::core::mem::transmute_copy(this), correlationvector.into_param().abi(), packageidentityname.into_param().abi(), publishercertificatename.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetIsPackageIdentityAllowedToInstallForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, user: Param0, correlationvector: Param1, packageidentityname: Param2, publishercertificatename: Param3) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIsPackageIdentityAllowedToInstallForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), correlationvector.into_param().abi(), packageidentityname.into_param().abi(), publishercertificatename.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn CanInstallForAllUsers(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager7>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanInstallForAllUsers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppInstallManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallManager {}
impl ::core::fmt::Debug for AppInstallManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager;{9353e170-8441-4b45-bd72-7c2fa925beee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppInstallManager {
    type Vtable = IAppInstallManager_Vtbl;
    const IID: ::windows::core::GUID = <IAppInstallManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInstallManager {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager";
}
impl ::core::convert::From<AppInstallManager> for ::windows::core::IUnknown {
    fn from(value: AppInstallManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallManager> for ::windows::core::IUnknown {
    fn from(value: &AppInstallManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppInstallManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppInstallManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallManager> for ::windows::core::IInspectable {
    fn from(value: AppInstallManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallManager> for ::windows::core::IInspectable {
    fn from(value: &AppInstallManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppInstallManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppInstallManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallManager {}
unsafe impl ::core::marker::Sync for AppInstallManager {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallManagerItemEventArgs(::windows::core::IUnknown);
impl AppInstallManagerItemEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn Item(&self) -> ::windows::core::Result<AppInstallItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Item)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallItem>(result__)
        }
    }
}
impl ::core::clone::Clone for AppInstallManagerItemEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallManagerItemEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallManagerItemEventArgs {}
impl ::core::fmt::Debug for AppInstallManagerItemEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallManagerItemEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallManagerItemEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs;{bc505743-4674-4dd1-957e-c25682086a14})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppInstallManagerItemEventArgs {
    type Vtable = IAppInstallManagerItemEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppInstallManagerItemEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInstallManagerItemEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs";
}
impl ::core::convert::From<AppInstallManagerItemEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppInstallManagerItemEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallManagerItemEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppInstallManagerItemEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppInstallManagerItemEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppInstallManagerItemEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallManagerItemEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppInstallManagerItemEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallManagerItemEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppInstallManagerItemEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppInstallManagerItemEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppInstallManagerItemEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallManagerItemEventArgs {}
unsafe impl ::core::marker::Sync for AppInstallManagerItemEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallOptions(::windows::core::IUnknown);
impl AppInstallOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppInstallOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn CatalogId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CatalogId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetCatalogId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCatalogId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn ForceUseOfNonRemovableStorage(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ForceUseOfNonRemovableStorage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetForceUseOfNonRemovableStorage(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForceUseOfNonRemovableStorage)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn AllowForcedAppRestart(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowForcedAppRestart)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowForcedAppRestart)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn Repair(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Repair)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetRepair(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRepair)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Management_Deployment\"`*"]
    #[cfg(feature = "Management_Deployment")]
    pub fn TargetVolume(&self) -> ::windows::core::Result<super::super::super::super::Management::Deployment::PackageVolume> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetVolume)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Management::Deployment::PackageVolume>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"Management_Deployment\"`*"]
    #[cfg(feature = "Management_Deployment")]
    pub fn SetTargetVolume<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Management::Deployment::PackageVolume>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTargetVolume)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn LaunchAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchAfterInstall)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetLaunchAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLaunchAfterInstall)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn PinToDesktopAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PinToDesktopAfterInstall)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetPinToDesktopAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPinToDesktopAfterInstall)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn PinToStartAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PinToStartAfterInstall)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetPinToStartAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPinToStartAfterInstall)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn PinToTaskbarAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PinToTaskbarAfterInstall)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetPinToTaskbarAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPinToTaskbarAfterInstall)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn CompletedInstallToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: AppInstallationToastNotificationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CompletedInstallToastNotificationMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallationToastNotificationMode>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCompletedInstallToastNotificationMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn InstallInProgressToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: AppInstallationToastNotificationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InstallInProgressToastNotificationMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallationToastNotificationMode>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInstallInProgressToastNotificationMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn InstallForAllUsers(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InstallForAllUsers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetInstallForAllUsers(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInstallForAllUsers)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn StageButDoNotInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StageButDoNotInstall)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetStageButDoNotInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStageButDoNotInstall)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn CampaignId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CampaignId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetCampaignId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCampaignId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn ExtendedCampaignId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedCampaignId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetExtendedCampaignId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetExtendedCampaignId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppInstallOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallOptions {}
impl ::core::fmt::Debug for AppInstallOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions;{c9808300-1cb8-4eb6-8c9f-6a30c64a5b51})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppInstallOptions {
    type Vtable = IAppInstallOptions_Vtbl;
    const IID: ::windows::core::GUID = <IAppInstallOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInstallOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions";
}
impl ::core::convert::From<AppInstallOptions> for ::windows::core::IUnknown {
    fn from(value: AppInstallOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallOptions> for ::windows::core::IUnknown {
    fn from(value: &AppInstallOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppInstallOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppInstallOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallOptions> for ::windows::core::IInspectable {
    fn from(value: AppInstallOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallOptions> for ::windows::core::IInspectable {
    fn from(value: &AppInstallOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppInstallOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppInstallOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallOptions {}
unsafe impl ::core::marker::Sync for AppInstallOptions {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppInstallState(pub i32);
impl AppInstallState {
    pub const Pending: Self = Self(0i32);
    pub const Starting: Self = Self(1i32);
    pub const AcquiringLicense: Self = Self(2i32);
    pub const Downloading: Self = Self(3i32);
    pub const RestoringData: Self = Self(4i32);
    pub const Installing: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
    pub const Canceled: Self = Self(7i32);
    pub const Paused: Self = Self(8i32);
    pub const Error: Self = Self(9i32);
    pub const PausedLowBattery: Self = Self(10i32);
    pub const PausedWiFiRecommended: Self = Self(11i32);
    pub const PausedWiFiRequired: Self = Self(12i32);
    pub const ReadyToDownload: Self = Self(13i32);
}
impl ::core::marker::Copy for AppInstallState {}
impl ::core::clone::Clone for AppInstallState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppInstallState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppInstallState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppInstallState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallStatus(::windows::core::IUnknown);
impl AppInstallStatus {
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn InstallState(&self) -> ::windows::core::Result<AppInstallState> {
        let this = self;
        unsafe {
            let mut result__: AppInstallState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InstallState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn DownloadSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DownloadSizeInBytes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn BytesDownloaded(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BytesDownloaded)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn PercentComplete(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PercentComplete)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IAppInstallStatus2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn ReadyForLaunch(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallStatus2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadyForLaunch)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn IsStaged(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallStatus3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStaged)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppInstallStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallStatus {}
impl ::core::fmt::Debug for AppInstallStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus;{936dccfa-2450-4126-88b1-6127a644dd5c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppInstallStatus {
    type Vtable = IAppInstallStatus_Vtbl;
    const IID: ::windows::core::GUID = <IAppInstallStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInstallStatus {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus";
}
impl ::core::convert::From<AppInstallStatus> for ::windows::core::IUnknown {
    fn from(value: AppInstallStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallStatus> for ::windows::core::IUnknown {
    fn from(value: &AppInstallStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppInstallStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppInstallStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallStatus> for ::windows::core::IInspectable {
    fn from(value: AppInstallStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallStatus> for ::windows::core::IInspectable {
    fn from(value: &AppInstallStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppInstallStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppInstallStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallStatus {}
unsafe impl ::core::marker::Sync for AppInstallStatus {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppInstallType(pub i32);
impl AppInstallType {
    pub const Install: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Repair: Self = Self(2i32);
}
impl ::core::marker::Copy for AppInstallType {}
impl ::core::clone::Clone for AppInstallType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppInstallType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppInstallType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppInstallType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppInstallationToastNotificationMode(pub i32);
impl AppInstallationToastNotificationMode {
    pub const Default: Self = Self(0i32);
    pub const Toast: Self = Self(1i32);
    pub const ToastWithoutPopup: Self = Self(2i32);
    pub const NoToast: Self = Self(3i32);
}
impl ::core::marker::Copy for AppInstallationToastNotificationMode {}
impl ::core::clone::Clone for AppInstallationToastNotificationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppInstallationToastNotificationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppInstallationToastNotificationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppInstallationToastNotificationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallationToastNotificationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallationToastNotificationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallationToastNotificationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppUpdateOptions(::windows::core::IUnknown);
impl AppUpdateOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppUpdateOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn CatalogId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CatalogId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetCatalogId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCatalogId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn AllowForcedAppRestart(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowForcedAppRestart)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowForcedAppRestart)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn AutomaticallyDownloadAndInstallUpdateIfFound(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppUpdateOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutomaticallyDownloadAndInstallUpdateIfFound)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn SetAutomaticallyDownloadAndInstallUpdateIfFound(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppUpdateOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAutomaticallyDownloadAndInstallUpdateIfFound)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for AppUpdateOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppUpdateOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppUpdateOptions {}
impl ::core::fmt::Debug for AppUpdateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppUpdateOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppUpdateOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions;{26f0b02f-c2f3-4aea-af8c-6308dd9db85f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppUpdateOptions {
    type Vtable = IAppUpdateOptions_Vtbl;
    const IID: ::windows::core::GUID = <IAppUpdateOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppUpdateOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions";
}
impl ::core::convert::From<AppUpdateOptions> for ::windows::core::IUnknown {
    fn from(value: AppUpdateOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUpdateOptions> for ::windows::core::IUnknown {
    fn from(value: &AppUpdateOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppUpdateOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppUpdateOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppUpdateOptions> for ::windows::core::IInspectable {
    fn from(value: AppUpdateOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUpdateOptions> for ::windows::core::IInspectable {
    fn from(value: &AppUpdateOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppUpdateOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppUpdateOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppUpdateOptions {}
unsafe impl ::core::marker::Sync for AppUpdateOptions {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutoUpdateSetting(pub i32);
impl AutoUpdateSetting {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const EnabledByPolicy: Self = Self(3i32);
}
impl ::core::marker::Copy for AutoUpdateSetting {}
impl ::core::clone::Clone for AutoUpdateSetting {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoUpdateSetting {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AutoUpdateSetting {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutoUpdateSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoUpdateSetting").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutoUpdateSetting {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AutoUpdateSetting;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct GetEntitlementResult(::windows::core::IUnknown);
impl GetEntitlementResult {
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<GetEntitlementStatus> {
        let this = self;
        unsafe {
            let mut result__: GetEntitlementStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GetEntitlementStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for GetEntitlementResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GetEntitlementResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GetEntitlementResult {}
impl ::core::fmt::Debug for GetEntitlementResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetEntitlementResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GetEntitlementResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult;{74fc843f-1a9e-4609-8e4d-819086d08a3d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GetEntitlementResult {
    type Vtable = IGetEntitlementResult_Vtbl;
    const IID: ::windows::core::GUID = <IGetEntitlementResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GetEntitlementResult {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult";
}
impl ::core::convert::From<GetEntitlementResult> for ::windows::core::IUnknown {
    fn from(value: GetEntitlementResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GetEntitlementResult> for ::windows::core::IUnknown {
    fn from(value: &GetEntitlementResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GetEntitlementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GetEntitlementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GetEntitlementResult> for ::windows::core::IInspectable {
    fn from(value: GetEntitlementResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GetEntitlementResult> for ::windows::core::IInspectable {
    fn from(value: &GetEntitlementResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GetEntitlementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GetEntitlementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GetEntitlementResult {}
unsafe impl ::core::marker::Sync for GetEntitlementResult {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GetEntitlementStatus(pub i32);
impl GetEntitlementStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NoStoreAccount: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
}
impl ::core::marker::Copy for GetEntitlementStatus {}
impl ::core::clone::Clone for GetEntitlementStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GetEntitlementStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GetEntitlementStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GetEntitlementStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetEntitlementStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GetEntitlementStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallItem {
    type Vtable = IAppInstallItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49d3dfab_168a_4cbf_a93a_9e448c82737d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub InstallType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallType) -> ::windows::core::HRESULT,
    pub IsUserInitiated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetCurrentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Restart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallItem2 {
    type Vtable = IAppInstallItem2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3972af8_40c0_4fd7_aa6c_0aa13ca6188c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CancelWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PauseWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RestartWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallItem3 {
    type Vtable = IAppInstallItem3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f3dc998_dd47_433c_9234_560172d67a45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    pub ItemOperationsMightAffectOtherItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallItem4 {
    type Vtable = IAppInstallItem4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2d1ce12_71ff_4fc8_b540_453d4b37e1d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LaunchAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetLaunchAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallItem5 {
    type Vtable = IAppInstallItem5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5510e7cc_4076_4a0b_9472_c21d9d380e55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PinToStartAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPinToStartAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub SetCompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub InstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub SetInstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallManager {
    type Vtable = IAppInstallManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9353e170_8441_4b45_bd72_7c2fa925beee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppInstallItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppInstallItems: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Restart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ItemCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ItemStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemStatusChanged: usize,
    pub AutoUpdateSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AutoUpdateSetting) -> ::windows::core::HRESULT,
    pub SetAutoUpdateSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AutoUpdateSetting) -> ::windows::core::HRESULT,
    pub AcquisitionIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAcquisitionIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetIsApplicableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsApplicableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartAppInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAppInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAppByPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAppByPackageFamilyNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SearchForUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SearchForUpdatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SearchForAllUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SearchForAllUpdatesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub IsStoreBlockedByPolicyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeclientname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, storeclientpublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsStoreBlockedByPolicyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsAppAllowedToInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsAppAllowedToInstallAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallManager2 {
    type Vtable = IAppInstallManager2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16937851_ed37_480d_8314_52e27c03f04a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub StartAppInstallWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, bundleid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAppInstallWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAppByPackageFamilyNameWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAppByPackageFamilyNameWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SearchForUpdatesWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SearchForUpdatesWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SearchForAllUpdatesWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SearchForAllUpdatesWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsAppAllowedToInstallWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsAppAllowedToInstallWithTelemetryAsync: usize,
    pub CancelWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PauseWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RestartWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallManager3 {
    type Vtable = IAppInstallManager3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95b24b17_e96a_4d0e_84e1_c8cb417a0178);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub StartProductInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Management_Deployment")))]
    StartProductInstallAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System"))]
    pub StartProductInstallForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System")))]
    StartProductInstallForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub UpdateAppByPackageFamilyNameForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    UpdateAppByPackageFamilyNameForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub SearchForUpdatesForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    SearchForUpdatesForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub SearchForAllUpdatesForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    SearchForAllUpdatesForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetIsAppAllowedToInstallForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetIsAppAllowedToInstallForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetIsApplicableForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetIsApplicableForUserAsync: usize,
    pub MoveToFrontOfDownloadQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallManager4 {
    type Vtable = IAppInstallManager4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x260a2a16_5a9e_4ebd_b944_f2ba75c31159);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetFreeUserEntitlementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, campaignid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFreeUserEntitlementAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetFreeUserEntitlementForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, campaignid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetFreeUserEntitlementForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFreeDeviceEntitlementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, campaignid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFreeDeviceEntitlementAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallManager5 {
    type Vtable = IAppInstallManager5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cd7be4c_1be9_4f7f_b675_aa1d64a529b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppInstallItemsWithGroupSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppInstallItemsWithGroupSupport: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallManager6 {
    type Vtable = IAppInstallManager6_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9e7d408_f27a_4471_b2f4_e76efcbebcca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager6_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SearchForAllUpdatesWithUpdateOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, updateoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SearchForAllUpdatesWithUpdateOptionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub SearchForAllUpdatesWithUpdateOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, updateoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    SearchForAllUpdatesWithUpdateOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SearchForUpdatesWithUpdateOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, updateoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SearchForUpdatesWithUpdateOptionsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub SearchForUpdatesWithUpdateOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, updateoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    SearchForUpdatesWithUpdateOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StartProductInstallWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, installoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartProductInstallWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub StartProductInstallWithOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, installoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    StartProductInstallWithOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsPackageIdentityAllowedToInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packageidentityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, publishercertificatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsPackageIdentityAllowedToInstallAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetIsPackageIdentityAllowedToInstallForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packageidentityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, publishercertificatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetIsPackageIdentityAllowedToInstallForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager7(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallManager7 {
    type Vtable = IAppInstallManager7_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5ee7b30_d5e4_49a3_9853_3db03203321d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager7_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CanInstallForAllUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManagerItemEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallManagerItemEventArgs {
    type Vtable = IAppInstallManagerItemEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc505743_4674_4dd1_957e_c25682086a14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManagerItemEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallOptions {
    type Vtable = IAppInstallOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9808300_1cb8_4eb6_8c9f_6a30c64a5b51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CatalogId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCatalogId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ForceUseOfNonRemovableStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetForceUseOfNonRemovableStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Repair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetRepair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Management_Deployment")]
    pub TargetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    TargetVolume: usize,
    #[cfg(feature = "Management_Deployment")]
    pub SetTargetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    SetTargetVolume: usize,
    pub LaunchAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetLaunchAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallOptions2 {
    type Vtable = IAppInstallOptions2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a04c0d7_c94b_425e_95b4_bf27faeaee89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallOptions2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PinToStartAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPinToStartAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub SetCompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub InstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub SetInstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub InstallForAllUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetInstallForAllUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub StageButDoNotInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStageButDoNotInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ExtendedCampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetExtendedCampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallStatus {
    type Vtable = IAppInstallStatus_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x936dccfa_2450_4126_88b1_6127a644dd5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub InstallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallState) -> ::windows::core::HRESULT,
    pub DownloadSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub BytesDownloaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub PercentComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallStatus2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallStatus2 {
    type Vtable = IAppInstallStatus2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96e7818a_5e92_4aa9_8edc_58fed4b87e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub ReadyForLaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallStatus3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallStatus3 {
    type Vtable = IAppInstallStatus3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb880c56_837b_4b4c_9ebb_6d44a0a96307);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsStaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUpdateOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppUpdateOptions {
    type Vtable = IAppUpdateOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26f0b02f_c2f3_4aea_af8c_6308dd9db85f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUpdateOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CatalogId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCatalogId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUpdateOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppUpdateOptions2 {
    type Vtable = IAppUpdateOptions2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4646e08_ed26_4bf9_9679_48f628e53df8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUpdateOptions2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AutomaticallyDownloadAndInstallUpdateIfFound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAutomaticallyDownloadAndInstallUpdateIfFound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGetEntitlementResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGetEntitlementResult {
    type Vtable = IGetEntitlementResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74fc843f_1a9e_4609_8e4d_819086d08a3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetEntitlementResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GetEntitlementStatus) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
