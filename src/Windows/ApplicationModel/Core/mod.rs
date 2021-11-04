#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `ApplicationModel_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppListEntry(pub ::windows::runtime::IInspectable);
impl AppListEntry {
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn DisplayInfo(&self) -> ::windows::runtime::Result<super::AppDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::AppDisplayInfo>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn LaunchAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn AppUserModelId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppListEntry2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`, `System`*"]
    pub fn LaunchForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(&self, user: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IAppListEntry3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn AppInfo(&self) -> ::windows::runtime::Result<super::AppInfo> {
        let this = &::windows::runtime::Interface::cast::<IAppListEntry4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::AppInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppListEntry {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.AppListEntry;{ef00f07f-2108-490a-877a-8a9f17c25fad})");
}
unsafe impl ::windows::runtime::Interface for AppListEntry {
    type Vtable = IAppListEntry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4009816191, 8456, 18698, [135, 122, 138, 159, 23, 194, 95, 173]);
}
impl ::windows::runtime::RuntimeName for AppListEntry {
    const NAME: &'static str = "Windows.ApplicationModel.Core.AppListEntry";
}
impl ::std::convert::From<AppListEntry> for ::windows::runtime::IUnknown {
    fn from(value: AppListEntry) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AppListEntry> for ::windows::runtime::IUnknown {
    fn from(value: &AppListEntry) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppListEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppListEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AppListEntry> for ::windows::runtime::IInspectable {
    fn from(value: AppListEntry) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppListEntry> for ::windows::runtime::IInspectable {
    fn from(value: &AppListEntry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppListEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppListEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppListEntry {}
unsafe impl ::std::marker::Sync for AppListEntry {}
#[doc = "*Required features: `ApplicationModel_Core`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppRestartFailureReason(pub i32);
impl AppRestartFailureReason {
    pub const RestartPending: AppRestartFailureReason = AppRestartFailureReason(0i32);
    pub const NotInForeground: AppRestartFailureReason = AppRestartFailureReason(1i32);
    pub const InvalidUser: AppRestartFailureReason = AppRestartFailureReason(2i32);
    pub const Other: AppRestartFailureReason = AppRestartFailureReason(3i32);
}
impl ::std::convert::From<i32> for AppRestartFailureReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppRestartFailureReason {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppRestartFailureReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Core.AppRestartFailureReason;i4)");
}
impl ::windows::runtime::DefaultType for AppRestartFailureReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Core`*"]
pub struct CoreApplication {}
impl CoreApplication {
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn Id() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn Suspending<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<super::SuspendingEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn RemoveSuspending<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn Resuming<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn RemoveResuming<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation_Collections`*"]
    pub fn Properties() -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn GetCurrentView() -> ::windows::runtime::Result<CoreApplicationView> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreApplicationView>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn Run<'a, Param0: ::windows::runtime::IntoParam<'a, IFrameworkViewSource>>(viewsource: Param0) -> ::windows::runtime::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), viewsource.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn RunWithActivationFactories<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IGetActivationFactory>>(activationfactorycallback: Param0) -> ::windows::runtime::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), activationfactorycallback.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn Exit() -> ::windows::runtime::Result<()> {
        Self::ICoreApplicationExit(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn Exiting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplicationExit(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn RemoveExiting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::ICoreApplicationExit(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn UnhandledErrorDetected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplicationUnhandledError(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn RemoveUnhandledErrorDetected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::ICoreApplicationUnhandledError(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn IncrementApplicationUseCount() -> ::windows::runtime::Result<()> {
        Self::ICoreApplicationUseCount(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn DecrementApplicationUseCount() -> ::windows::runtime::Result<()> {
        Self::ICoreApplicationUseCount(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation_Collections`*"]
    pub fn Views() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<CoreApplicationView>> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<CoreApplicationView>>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn CreateNewView<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(runtimetype: Param0, entrypoint: Param1) -> ::windows::runtime::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), runtimetype.into_param().abi(), entrypoint.into_param().abi(), &mut result__).from_abi::<CoreApplicationView>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn MainView() -> ::windows::runtime::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreApplicationView>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn CreateNewViewFromMainView() -> ::windows::runtime::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreApplicationView>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn CreateNewViewWithViewSource<'a, Param0: ::windows::runtime::IntoParam<'a, IFrameworkViewSource>>(viewsource: Param0) -> ::windows::runtime::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), viewsource.into_param().abi(), &mut result__).from_abi::<CoreApplicationView>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    #[doc = "*Required features: `ApplicationModel_Core`, `ApplicationModel_Activation`, `Foundation`*"]
    pub fn BackgroundActivated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<super::Activation::BackgroundActivatedEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn RemoveBackgroundActivated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn LeavingBackground<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<super::LeavingBackgroundEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn RemoveLeavingBackground<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn EnteredBackground<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<super::EnteredBackgroundEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn RemoveEnteredBackground<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn EnablePrelaunch(value: bool) -> ::windows::runtime::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn RequestRestartAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(launcharguments: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>> {
        Self::ICoreApplication3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), launcharguments.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`, `System`*"]
    pub fn RequestRestartForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(user: Param0, launcharguments: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>> {
        Self::ICoreApplication3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), launcharguments.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>>(result__)
        })
    }
    pub fn ICoreApplication<R, F: FnOnce(&ICoreApplication) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreApplication, ICoreApplication> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreApplicationExit<R, F: FnOnce(&ICoreApplicationExit) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreApplication, ICoreApplicationExit> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreApplicationUnhandledError<R, F: FnOnce(&ICoreApplicationUnhandledError) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreApplication, ICoreApplicationUnhandledError> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreApplicationUseCount<R, F: FnOnce(&ICoreApplicationUseCount) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreApplication, ICoreApplicationUseCount> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreImmersiveApplication<R, F: FnOnce(&ICoreImmersiveApplication) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreApplication, ICoreImmersiveApplication> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreImmersiveApplication2<R, F: FnOnce(&ICoreImmersiveApplication2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreApplication, ICoreImmersiveApplication2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreImmersiveApplication3<R, F: FnOnce(&ICoreImmersiveApplication3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreApplication, ICoreImmersiveApplication3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreApplication2<R, F: FnOnce(&ICoreApplication2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreApplication, ICoreApplication2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreApplication3<R, F: FnOnce(&ICoreApplication3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreApplication, ICoreApplication3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for CoreApplication {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplication";
}
#[doc = "*Required features: `ApplicationModel_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreApplicationView(pub ::windows::runtime::IInspectable);
impl CoreApplicationView {
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `ApplicationModel_Core`, `UI_Core`*"]
    pub fn CoreWindow(&self) -> ::windows::runtime::Result<super::super::UI::Core::CoreWindow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Core::CoreWindow>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    #[doc = "*Required features: `ApplicationModel_Core`, `ApplicationModel_Activation`, `Foundation`*"]
    pub fn Activated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreApplicationView, super::Activation::IActivatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn RemoveActivated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn IsMain(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn IsHosted(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `ApplicationModel_Core`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<ICoreApplicationView2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn IsComponent(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn TitleBar(&self) -> ::windows::runtime::Result<CoreApplicationViewTitleBar> {
        let this = &::windows::runtime::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreApplicationViewTitleBar>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn HostedViewClosing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreApplicationView, HostedViewClosingEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn RemoveHostedViewClosing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::runtime::Interface::cast::<ICoreApplicationView5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Core`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<ICoreApplicationView6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::DispatcherQueue>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreApplicationView {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.CoreApplicationView;{638bb2db-451d-4661-b099-414f34ffb9f1})");
}
unsafe impl ::windows::runtime::Interface for CoreApplicationView {
    type Vtable = ICoreApplicationView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1670099675, 17693, 18017, [176, 153, 65, 79, 52, 255, 185, 241]);
}
impl ::windows::runtime::RuntimeName for CoreApplicationView {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplicationView";
}
impl ::std::convert::From<CoreApplicationView> for ::windows::runtime::IUnknown {
    fn from(value: CoreApplicationView) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CoreApplicationView> for ::windows::runtime::IUnknown {
    fn from(value: &CoreApplicationView) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreApplicationView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CoreApplicationView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CoreApplicationView> for ::windows::runtime::IInspectable {
    fn from(value: CoreApplicationView) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreApplicationView> for ::windows::runtime::IInspectable {
    fn from(value: &CoreApplicationView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreApplicationView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreApplicationView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `ApplicationModel_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreApplicationViewTitleBar(pub ::windows::runtime::IInspectable);
impl CoreApplicationViewTitleBar {
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn SetExtendViewIntoTitleBar(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn ExtendViewIntoTitleBar(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn SystemOverlayLeftInset(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn SystemOverlayRightInset(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn Height(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn LayoutMetricsChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn RemoveLayoutMetricsChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn IsVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn IsVisibleChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn RemoveIsVisibleChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreApplicationViewTitleBar {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.CoreApplicationViewTitleBar;{006d35e3-e1f1-431b-9508-29b96926ac53})");
}
unsafe impl ::windows::runtime::Interface for CoreApplicationViewTitleBar {
    type Vtable = ICoreApplicationViewTitleBar_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(7157219, 57841, 17179, [149, 8, 41, 185, 105, 38, 172, 83]);
}
impl ::windows::runtime::RuntimeName for CoreApplicationViewTitleBar {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplicationViewTitleBar";
}
impl ::std::convert::From<CoreApplicationViewTitleBar> for ::windows::runtime::IUnknown {
    fn from(value: CoreApplicationViewTitleBar) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CoreApplicationViewTitleBar> for ::windows::runtime::IUnknown {
    fn from(value: &CoreApplicationViewTitleBar) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreApplicationViewTitleBar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CoreApplicationViewTitleBar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CoreApplicationViewTitleBar> for ::windows::runtime::IInspectable {
    fn from(value: CoreApplicationViewTitleBar) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreApplicationViewTitleBar> for ::windows::runtime::IInspectable {
    fn from(value: &CoreApplicationViewTitleBar) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreApplicationViewTitleBar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreApplicationViewTitleBar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `ApplicationModel_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HostedViewClosingEventArgs(pub ::windows::runtime::IInspectable);
impl HostedViewClosingEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HostedViewClosingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.HostedViewClosingEventArgs;{d238943c-b24e-4790-acb5-3e4243c4ff87})");
}
unsafe impl ::windows::runtime::Interface for HostedViewClosingEventArgs {
    type Vtable = IHostedViewClosingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3526923324, 45646, 18320, [172, 181, 62, 66, 67, 196, 255, 135]);
}
impl ::windows::runtime::RuntimeName for HostedViewClosingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Core.HostedViewClosingEventArgs";
}
impl ::std::convert::From<HostedViewClosingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: HostedViewClosingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&HostedViewClosingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &HostedViewClosingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HostedViewClosingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HostedViewClosingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<HostedViewClosingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: HostedViewClosingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HostedViewClosingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &HostedViewClosingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HostedViewClosingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HostedViewClosingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HostedViewClosingEventArgs {}
unsafe impl ::std::marker::Sync for HostedViewClosingEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppListEntry(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppListEntry {
    type Vtable = IAppListEntry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4009816191, 8456, 18698, [135, 122, 138, 159, 23, 194, 95, 173]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppListEntry2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppListEntry2 {
    type Vtable = IAppListEntry2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3500546221, 48949, 17068, [172, 6, 134, 238, 235, 65, 208, 75]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppListEntry3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppListEntry3 {
    type Vtable = IAppListEntry3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1620701837, 64562, 18186, [188, 105, 75, 6, 26, 118, 239, 46]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppListEntry4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppListEntry4 {
    type Vtable = IAppListEntry4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(705896146, 22261, 18556, [134, 151, 81, 102, 243, 179, 61, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry4_abi(
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
pub struct ICoreApplication(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreApplication {
    type Vtable = ICoreApplication_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(179107748, 24093, 18911, [128, 52, 251, 106, 104, 188, 94, 209]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplication_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewsource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activationfactorycallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplication2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreApplication2 {
    type Vtable = ICoreApplication2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2575729147, 6838, 19327, [190, 74, 154, 6, 69, 34, 76, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplication2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplication3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreApplication3 {
    type Vtable = ICoreApplication3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4276882745, 22923, 17671, [138, 103, 119, 38, 50, 88, 10, 87]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplication3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, launcharguments: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, launcharguments: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplicationExit(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreApplicationExit {
    type Vtable = ICoreApplicationExit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3481683485, 9758, 19314, [154, 205, 68, 237, 42, 206, 106, 41]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationExit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Core`*"]
pub struct ICoreApplicationUnhandledError(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreApplicationUnhandledError {
    type Vtable = ICoreApplicationUnhandledError_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4041362096, 56585, 17121, [176, 188, 224, 225, 49, 247, 141, 126]);
}
impl ICoreApplicationUnhandledError {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn UnhandledErrorDetected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Core`, `Foundation`*"]
    pub fn RemoveUnhandledErrorDetected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ICoreApplicationUnhandledError {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{f0e24ab0-dd09-42e1-b0bc-e0e131f78d7e}");
}
impl ::std::convert::From<ICoreApplicationUnhandledError> for ::windows::runtime::IUnknown {
    fn from(value: ICoreApplicationUnhandledError) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ICoreApplicationUnhandledError> for ::windows::runtime::IUnknown {
    fn from(value: &ICoreApplicationUnhandledError) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICoreApplicationUnhandledError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICoreApplicationUnhandledError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ICoreApplicationUnhandledError> for ::windows::runtime::IInspectable {
    fn from(value: ICoreApplicationUnhandledError) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ICoreApplicationUnhandledError> for ::windows::runtime::IInspectable {
    fn from(value: &ICoreApplicationUnhandledError) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ICoreApplicationUnhandledError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ICoreApplicationUnhandledError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationUnhandledError_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplicationUseCount(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreApplicationUseCount {
    type Vtable = ICoreApplicationUseCount_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1368245256, 49271, 18267, [128, 158, 11, 192, 197, 126, 75, 116]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationUseCount_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplicationView(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreApplicationView {
    type Vtable = ICoreApplicationView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1670099675, 17693, 18017, [176, 153, 65, 79, 52, 255, 185, 241]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplicationView2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreApplicationView2 {
    type Vtable = ICoreApplicationView2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1760262879, 37247, 18667, [154, 235, 125, 229, 62, 8, 106, 177]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplicationView3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreApplicationView3 {
    type Vtable = ICoreApplicationView3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(132899251, 42191, 17744, [171, 112, 176, 126, 133, 51, 11, 200]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplicationView5(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreApplicationView5 {
    type Vtable = ICoreApplicationView5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(734041512, 36592, 17517, [158, 96, 58, 62, 4, 40, 198, 113]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView5_abi(
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
#[doc(hidden)]
pub struct ICoreApplicationView6(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreApplicationView6 {
    type Vtable = ICoreApplicationView6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3239695514, 1657, 18874, [128, 63, 183, 156, 92, 243, 76, 202]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView6_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplicationViewTitleBar(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreApplicationViewTitleBar {
    type Vtable = ICoreApplicationViewTitleBar_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(7157219, 57841, 17179, [149, 8, 41, 185, 105, 38, 172, 83]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationViewTitleBar_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreImmersiveApplication {
    type Vtable = ICoreImmersiveApplication_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(450498110, 58530, 16675, [180, 81, 220, 150, 191, 128, 4, 25]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, runtimetype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, entrypoint: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreImmersiveApplication2 {
    type Vtable = ICoreImmersiveApplication2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2190351926, 59875, 19708, [155, 102, 72, 183, 142, 169, 187, 44]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication2_abi(
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
pub struct ICoreImmersiveApplication3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreImmersiveApplication3 {
    type Vtable = ICoreImmersiveApplication3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(882924335, 60941, 16869, [131, 20, 207, 16, 201, 27, 240, 175]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewsource: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Core`*"]
pub struct IFrameworkView(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFrameworkView {
    type Vtable = IFrameworkView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4205534416, 35108, 17836, [173, 15, 160, 143, 174, 93, 3, 36]);
}
impl IFrameworkView {
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, CoreApplicationView>>(&self, applicationview: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), applicationview.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `ApplicationModel_Core`, `UI_Core`*"]
    pub fn SetWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Core::CoreWindow>>(&self, window: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), window.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, entrypoint: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), entrypoint.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn Run(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn Uninitialize(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IFrameworkView {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{faab5cd0-8924-45ac-ad0f-a08fae5d0324}");
}
impl ::std::convert::From<IFrameworkView> for ::windows::runtime::IUnknown {
    fn from(value: IFrameworkView) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&IFrameworkView> for ::windows::runtime::IUnknown {
    fn from(value: &IFrameworkView) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFrameworkView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFrameworkView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<IFrameworkView> for ::windows::runtime::IInspectable {
    fn from(value: IFrameworkView) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IFrameworkView> for ::windows::runtime::IInspectable {
    fn from(value: &IFrameworkView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IFrameworkView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IFrameworkView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkView_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, applicationview: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, entrypoint: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Core`*"]
pub struct IFrameworkViewSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFrameworkViewSource {
    type Vtable = IFrameworkViewSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3447129620, 26052, 17004, [148, 148, 52, 252, 67, 85, 72, 98]);
}
impl IFrameworkViewSource {
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn CreateView(&self) -> ::windows::runtime::Result<IFrameworkView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IFrameworkView>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IFrameworkViewSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{cd770614-65c4-426c-9494-34fc43554862}");
}
impl ::std::convert::From<IFrameworkViewSource> for ::windows::runtime::IUnknown {
    fn from(value: IFrameworkViewSource) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&IFrameworkViewSource> for ::windows::runtime::IUnknown {
    fn from(value: &IFrameworkViewSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFrameworkViewSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFrameworkViewSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<IFrameworkViewSource> for ::windows::runtime::IInspectable {
    fn from(value: IFrameworkViewSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IFrameworkViewSource> for ::windows::runtime::IInspectable {
    fn from(value: &IFrameworkViewSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IFrameworkViewSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IFrameworkViewSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkViewSource_abi(
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
pub struct IHostedViewClosingEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHostedViewClosingEventArgs {
    type Vtable = IHostedViewClosingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3526923324, 45646, 18320, [172, 181, 62, 66, 67, 196, 255, 135]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostedViewClosingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUnhandledError(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUnhandledError {
    type Vtable = IUnhandledError_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2488907558, 21429, 18054, [158, 175, 250, 129, 98, 220, 57, 128]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnhandledError_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUnhandledErrorDetectedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUnhandledErrorDetectedEventArgs {
    type Vtable = IUnhandledErrorDetectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1738192779, 45878, 18466, [172, 64, 13, 117, 15, 11, 122, 43]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnhandledErrorDetectedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `ApplicationModel_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UnhandledError(pub ::windows::runtime::IInspectable);
impl UnhandledError {
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn Propagate(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UnhandledError {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.UnhandledError;{9459b726-53b5-4686-9eaf-fa8162dc3980})");
}
unsafe impl ::windows::runtime::Interface for UnhandledError {
    type Vtable = IUnhandledError_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2488907558, 21429, 18054, [158, 175, 250, 129, 98, 220, 57, 128]);
}
impl ::windows::runtime::RuntimeName for UnhandledError {
    const NAME: &'static str = "Windows.ApplicationModel.Core.UnhandledError";
}
impl ::std::convert::From<UnhandledError> for ::windows::runtime::IUnknown {
    fn from(value: UnhandledError) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&UnhandledError> for ::windows::runtime::IUnknown {
    fn from(value: &UnhandledError) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UnhandledError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UnhandledError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<UnhandledError> for ::windows::runtime::IInspectable {
    fn from(value: UnhandledError) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UnhandledError> for ::windows::runtime::IInspectable {
    fn from(value: &UnhandledError) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UnhandledError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UnhandledError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UnhandledError {}
unsafe impl ::std::marker::Sync for UnhandledError {}
#[doc = "*Required features: `ApplicationModel_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UnhandledErrorDetectedEventArgs(pub ::windows::runtime::IInspectable);
impl UnhandledErrorDetectedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Core`*"]
    pub fn UnhandledError(&self) -> ::windows::runtime::Result<UnhandledError> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UnhandledError>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UnhandledErrorDetectedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs;{679ab78b-b336-4822-ac40-0d750f0b7a2b})");
}
unsafe impl ::windows::runtime::Interface for UnhandledErrorDetectedEventArgs {
    type Vtable = IUnhandledErrorDetectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1738192779, 45878, 18466, [172, 64, 13, 117, 15, 11, 122, 43]);
}
impl ::windows::runtime::RuntimeName for UnhandledErrorDetectedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs";
}
impl ::std::convert::From<UnhandledErrorDetectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: UnhandledErrorDetectedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&UnhandledErrorDetectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &UnhandledErrorDetectedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UnhandledErrorDetectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UnhandledErrorDetectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<UnhandledErrorDetectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: UnhandledErrorDetectedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UnhandledErrorDetectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &UnhandledErrorDetectedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UnhandledErrorDetectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UnhandledErrorDetectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UnhandledErrorDetectedEventArgs {}
unsafe impl ::std::marker::Sync for UnhandledErrorDetectedEventArgs {}
