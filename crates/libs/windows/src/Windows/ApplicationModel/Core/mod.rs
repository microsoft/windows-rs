#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct AppListEntry(::windows::core::IUnknown);
impl AppListEntry {
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn DisplayInfo(&self) -> ::windows::core::Result<super::AppDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AppDisplayInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppListEntry2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppUserModelId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn LaunchForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(&self, user: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppListEntry3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn AppInfo(&self) -> ::windows::core::Result<super::AppInfo> {
        let this = &::windows::core::Interface::cast::<IAppListEntry4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AppInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for AppListEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppListEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppListEntry {}
impl ::core::fmt::Debug for AppListEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppListEntry").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppListEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.AppListEntry;{ef00f07f-2108-490a-877a-8a9f17c25fad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppListEntry {
    type Vtable = IAppListEntry_Vtbl;
    const IID: ::windows::core::GUID = <IAppListEntry as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppListEntry {
    const NAME: &'static str = "Windows.ApplicationModel.Core.AppListEntry";
}
impl ::core::convert::From<AppListEntry> for ::windows::core::IUnknown {
    fn from(value: AppListEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppListEntry> for ::windows::core::IUnknown {
    fn from(value: &AppListEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppListEntry> for ::windows::core::IInspectable {
    fn from(value: AppListEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppListEntry> for ::windows::core::IInspectable {
    fn from(value: &AppListEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppListEntry {}
unsafe impl ::core::marker::Sync for AppListEntry {}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppRestartFailureReason(pub i32);
impl AppRestartFailureReason {
    pub const RestartPending: Self = Self(0i32);
    pub const NotInForeground: Self = Self(1i32);
    pub const InvalidUser: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for AppRestartFailureReason {}
impl ::core::clone::Clone for AppRestartFailureReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppRestartFailureReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppRestartFailureReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppRestartFailureReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRestartFailureReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppRestartFailureReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Core.AppRestartFailureReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
pub struct CoreApplication {}
impl CoreApplication {
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn Id() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Suspending<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<super::SuspendingEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Suspending)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSuspending<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveSuspending)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Resuming<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Resuming)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResuming<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveResuming)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties() -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn GetCurrentView() -> ::windows::core::Result<CoreApplicationView> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreApplicationView>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn Run<'a, Param0: ::windows::core::IntoParam<'a, IFrameworkViewSource>>(viewsource: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows::core::Interface::vtable(this).Run)(::core::mem::transmute_copy(this), viewsource.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RunWithActivationFactories<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IGetActivationFactory>>(activationfactorycallback: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows::core::Interface::vtable(this).RunWithActivationFactories)(::core::mem::transmute_copy(this), activationfactorycallback.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"ApplicationModel_Activation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn BackgroundActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<super::Activation::BackgroundActivatedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundActivated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBackgroundActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveBackgroundActivated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LeavingBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<super::LeavingBackgroundEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LeavingBackground)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLeavingBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveLeavingBackground)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnteredBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<super::EnteredBackgroundEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnteredBackground)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnteredBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveEnteredBackground)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn EnablePrelaunch(value: bool) -> ::windows::core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows::core::Interface::vtable(this).EnablePrelaunch)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestRestartAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(launcharguments: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>> {
        Self::ICoreApplication3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestRestartAsync)(::core::mem::transmute_copy(this), launcharguments.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn RequestRestartForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, launcharguments: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>> {
        Self::ICoreApplication3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestRestartForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), launcharguments.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn Exit() -> ::windows::core::Result<()> {
        Self::ICoreApplicationExit(|this| unsafe { (::windows::core::Interface::vtable(this).Exit)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Exiting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplicationExit(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Exiting)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveExiting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplicationExit(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveExiting)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnhandledErrorDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplicationUnhandledError(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnhandledErrorDetected)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnhandledErrorDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplicationUnhandledError(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveUnhandledErrorDetected)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn IncrementApplicationUseCount() -> ::windows::core::Result<()> {
        Self::ICoreApplicationUseCount(|this| unsafe { (::windows::core::Interface::vtable(this).IncrementApplicationUseCount)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn DecrementApplicationUseCount() -> ::windows::core::Result<()> {
        Self::ICoreApplicationUseCount(|this| unsafe { (::windows::core::Interface::vtable(this).DecrementApplicationUseCount)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Views() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<CoreApplicationView>> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Views)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<CoreApplicationView>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn CreateNewView<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(runtimetype: Param0, entrypoint: Param1) -> ::windows::core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateNewView)(::core::mem::transmute_copy(this), runtimetype.into_param().abi(), entrypoint.into_param().abi(), &mut result__).from_abi::<CoreApplicationView>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn MainView() -> ::windows::core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MainView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreApplicationView>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn CreateNewViewFromMainView() -> ::windows::core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateNewViewFromMainView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreApplicationView>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn CreateNewViewWithViewSource<'a, Param0: ::windows::core::IntoParam<'a, IFrameworkViewSource>>(viewsource: Param0) -> ::windows::core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateNewViewWithViewSource)(::core::mem::transmute_copy(this), viewsource.into_param().abi(), &mut result__).from_abi::<CoreApplicationView>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreApplication<R, F: FnOnce(&ICoreApplication) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplication> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ICoreApplication2<R, F: FnOnce(&ICoreApplication2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplication2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ICoreApplication3<R, F: FnOnce(&ICoreApplication3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplication3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ICoreApplicationExit<R, F: FnOnce(&ICoreApplicationExit) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplicationExit> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ICoreApplicationUnhandledError<R, F: FnOnce(&ICoreApplicationUnhandledError) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplicationUnhandledError> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ICoreApplicationUseCount<R, F: FnOnce(&ICoreApplicationUseCount) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplicationUseCount> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ICoreImmersiveApplication<R, F: FnOnce(&ICoreImmersiveApplication) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreImmersiveApplication> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ICoreImmersiveApplication2<R, F: FnOnce(&ICoreImmersiveApplication2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreImmersiveApplication2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ICoreImmersiveApplication3<R, F: FnOnce(&ICoreImmersiveApplication3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreImmersiveApplication3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CoreApplication {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplication";
}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct CoreApplicationView(::windows::core::IUnknown);
impl CoreApplicationView {
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn CoreWindow(&self) -> ::windows::core::Result<super::super::UI::Core::CoreWindow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CoreWindow)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Core::CoreWindow>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"ApplicationModel_Activation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Activated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreApplicationView, super::Activation::IActivatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Activated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveActivated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn IsMain(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsMain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn IsHosted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHosted)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn IsComponent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsComponent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn TitleBar(&self) -> ::windows::core::Result<CoreApplicationViewTitleBar> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TitleBar)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreApplicationViewTitleBar>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HostedViewClosing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreApplicationView, HostedViewClosingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HostedViewClosing)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHostedViewClosing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHostedViewClosing)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::DispatcherQueue>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreApplicationView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreApplicationView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreApplicationView {}
impl ::core::fmt::Debug for CoreApplicationView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreApplicationView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreApplicationView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.CoreApplicationView;{638bb2db-451d-4661-b099-414f34ffb9f1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreApplicationView {
    type Vtable = ICoreApplicationView_Vtbl;
    const IID: ::windows::core::GUID = <ICoreApplicationView as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreApplicationView {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplicationView";
}
impl ::core::convert::From<CoreApplicationView> for ::windows::core::IUnknown {
    fn from(value: CoreApplicationView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreApplicationView> for ::windows::core::IUnknown {
    fn from(value: &CoreApplicationView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreApplicationView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreApplicationView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreApplicationView> for ::windows::core::IInspectable {
    fn from(value: CoreApplicationView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreApplicationView> for ::windows::core::IInspectable {
    fn from(value: &CoreApplicationView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreApplicationView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreApplicationView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct CoreApplicationViewTitleBar(::windows::core::IUnknown);
impl CoreApplicationViewTitleBar {
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn SetExtendViewIntoTitleBar(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExtendViewIntoTitleBar)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn ExtendViewIntoTitleBar(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendViewIntoTitleBar)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn SystemOverlayLeftInset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemOverlayLeftInset)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn SystemOverlayRightInset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemOverlayRightInset)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn Height(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Height)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LayoutMetricsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LayoutMetricsChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLayoutMetricsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLayoutMetricsChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVisible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsVisibleChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVisibleChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsVisibleChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveIsVisibleChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for CoreApplicationViewTitleBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreApplicationViewTitleBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreApplicationViewTitleBar {}
impl ::core::fmt::Debug for CoreApplicationViewTitleBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreApplicationViewTitleBar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreApplicationViewTitleBar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.CoreApplicationViewTitleBar;{006d35e3-e1f1-431b-9508-29b96926ac53})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreApplicationViewTitleBar {
    type Vtable = ICoreApplicationViewTitleBar_Vtbl;
    const IID: ::windows::core::GUID = <ICoreApplicationViewTitleBar as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreApplicationViewTitleBar {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplicationViewTitleBar";
}
impl ::core::convert::From<CoreApplicationViewTitleBar> for ::windows::core::IUnknown {
    fn from(value: CoreApplicationViewTitleBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreApplicationViewTitleBar> for ::windows::core::IUnknown {
    fn from(value: &CoreApplicationViewTitleBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreApplicationViewTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreApplicationViewTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreApplicationViewTitleBar> for ::windows::core::IInspectable {
    fn from(value: CoreApplicationViewTitleBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreApplicationViewTitleBar> for ::windows::core::IInspectable {
    fn from(value: &CoreApplicationViewTitleBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreApplicationViewTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreApplicationViewTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct HostedViewClosingEventArgs(::windows::core::IUnknown);
impl HostedViewClosingEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for HostedViewClosingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HostedViewClosingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HostedViewClosingEventArgs {}
impl ::core::fmt::Debug for HostedViewClosingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostedViewClosingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HostedViewClosingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.HostedViewClosingEventArgs;{d238943c-b24e-4790-acb5-3e4243c4ff87})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HostedViewClosingEventArgs {
    type Vtable = IHostedViewClosingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IHostedViewClosingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HostedViewClosingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Core.HostedViewClosingEventArgs";
}
impl ::core::convert::From<HostedViewClosingEventArgs> for ::windows::core::IUnknown {
    fn from(value: HostedViewClosingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HostedViewClosingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HostedViewClosingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HostedViewClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HostedViewClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HostedViewClosingEventArgs> for ::windows::core::IInspectable {
    fn from(value: HostedViewClosingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HostedViewClosingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HostedViewClosingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HostedViewClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HostedViewClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HostedViewClosingEventArgs {}
unsafe impl ::core::marker::Sync for HostedViewClosingEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppListEntry(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppListEntry {
    type Vtable = IAppListEntry_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef00f07f_2108_490a_877a_8a9f17c25fad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DisplayInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LaunchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppListEntry2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppListEntry2 {
    type Vtable = IAppListEntry2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0a618ad_bf35_42ac_ac06_86eeeb41d04b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppListEntry3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppListEntry3 {
    type Vtable = IAppListEntry3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6099f28d_fc32_470a_bc69_4b061a76ef2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub LaunchForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    LaunchForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppListEntry4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppListEntry4 {
    type Vtable = IAppListEntry4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a131ed2_56f5_487c_8697_5166f3b33da0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplication(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreApplication {
    type Vtable = ICoreApplication_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0aacf7a4_5e1d_49df_8034_fb6a68bc5ed1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplication_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Suspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Suspending: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSuspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSuspending: usize,
    #[cfg(feature = "Foundation")]
    pub Resuming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Resuming: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResuming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResuming: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub GetCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewsource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RunWithActivationFactories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activationfactorycallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunWithActivationFactories: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplication2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreApplication2 {
    type Vtable = ICoreApplication2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x998681fb_1ab6_4b7f_be4a_9a0645224c04);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplication2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub BackgroundActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    BackgroundActivated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBackgroundActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBackgroundActivated: usize,
    #[cfg(feature = "Foundation")]
    pub LeavingBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LeavingBackground: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLeavingBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLeavingBackground: usize,
    #[cfg(feature = "Foundation")]
    pub EnteredBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnteredBackground: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnteredBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnteredBackground: usize,
    pub EnablePrelaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplication3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreApplication3 {
    type Vtable = ICoreApplication3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfeec0d39_598b_4507_8a67_772632580a57);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplication3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestRestartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, launcharguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestRestartAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub RequestRestartForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, launcharguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    RequestRestartForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationExit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreApplicationExit {
    type Vtable = ICoreApplicationExit_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf86461d_261e_4b72_9acd_44ed2ace6a29);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationExit_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Exit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Exiting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Exiting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveExiting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveExiting: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct ICoreApplicationUnhandledError(::windows::core::IUnknown);
impl ICoreApplicationUnhandledError {
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnhandledErrorDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnhandledErrorDetected)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnhandledErrorDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUnhandledErrorDetected)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<ICoreApplicationUnhandledError> for ::windows::core::IUnknown {
    fn from(value: ICoreApplicationUnhandledError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreApplicationUnhandledError> for ::windows::core::IUnknown {
    fn from(value: &ICoreApplicationUnhandledError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICoreApplicationUnhandledError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICoreApplicationUnhandledError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICoreApplicationUnhandledError> for ::windows::core::IInspectable {
    fn from(value: ICoreApplicationUnhandledError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreApplicationUnhandledError> for ::windows::core::IInspectable {
    fn from(value: &ICoreApplicationUnhandledError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICoreApplicationUnhandledError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICoreApplicationUnhandledError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoreApplicationUnhandledError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreApplicationUnhandledError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreApplicationUnhandledError {}
impl ::core::fmt::Debug for ICoreApplicationUnhandledError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreApplicationUnhandledError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICoreApplicationUnhandledError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f0e24ab0-dd09-42e1-b0bc-e0e131f78d7e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICoreApplicationUnhandledError {
    type Vtable = ICoreApplicationUnhandledError_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0e24ab0_dd09_42e1_b0bc_e0e131f78d7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationUnhandledError_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub UnhandledErrorDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnhandledErrorDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnhandledErrorDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnhandledErrorDetected: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationUseCount(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreApplicationUseCount {
    type Vtable = ICoreApplicationUseCount_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x518dc408_c077_475b_809e_0bc0c57e4b74);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationUseCount_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IncrementApplicationUseCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DecrementApplicationUseCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationView(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreApplicationView {
    type Vtable = ICoreApplicationView_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x638bb2db_451d_4661_b099_414f34ffb9f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Core")]
    pub CoreWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    CoreWindow: usize,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub Activated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    Activated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivated: usize,
    pub IsMain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsHosted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationView2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreApplicationView2 {
    type Vtable = ICoreApplicationView2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68eb7adf_917f_48eb_9aeb_7de53e086ab1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Core")]
    pub Dispatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    Dispatcher: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationView3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreApplicationView3 {
    type Vtable = ICoreApplicationView3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07ebe1b3_a4cf_4550_ab70_b07e85330bc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TitleBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HostedViewClosing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HostedViewClosing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHostedViewClosing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHostedViewClosing: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationView5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreApplicationView5 {
    type Vtable = ICoreApplicationView5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bc095a8_8ef0_446d_9e60_3a3e0428c671);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationView6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreApplicationView6 {
    type Vtable = ICoreApplicationView6_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc119d49a_0679_49ba_803f_b79c5cf34cca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView6_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationViewTitleBar(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreApplicationViewTitleBar {
    type Vtable = ICoreApplicationViewTitleBar_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x006d35e3_e1f1_431b_9508_29b96926ac53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationViewTitleBar_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetExtendViewIntoTitleBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ExtendViewIntoTitleBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SystemOverlayLeftInset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SystemOverlayRightInset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LayoutMetricsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LayoutMetricsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLayoutMetricsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLayoutMetricsChanged: usize,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsVisibleChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsVisibleChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsVisibleChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsVisibleChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreImmersiveApplication(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreImmersiveApplication {
    type Vtable = ICoreImmersiveApplication_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ada0e3e_e4a2_4123_b451_dc96bf800419);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Views: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Views: usize,
    pub CreateNewView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runtimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, entrypoint: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub MainView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreImmersiveApplication2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreImmersiveApplication2 {
    type Vtable = ICoreImmersiveApplication2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x828e1e36_e9e3_4cfc_9b66_48b78ea9bb2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateNewViewFromMainView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreImmersiveApplication3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreImmersiveApplication3 {
    type Vtable = ICoreImmersiveApplication3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34a05b2f_ee0d_41e5_8314_cf10c91bf0af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateNewViewWithViewSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewsource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct IFrameworkView(::windows::core::IUnknown);
impl IFrameworkView {
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, CoreApplicationView>>(&self, applicationview: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Initialize)(::core::mem::transmute_copy(this), applicationview.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn SetWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Core::CoreWindow>>(&self, window: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWindow)(::core::mem::transmute_copy(this), window.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn Load<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, entrypoint: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Load)(::core::mem::transmute_copy(this), entrypoint.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn Run(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Run)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn Uninitialize(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Uninitialize)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IFrameworkView> for ::windows::core::IUnknown {
    fn from(value: IFrameworkView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFrameworkView> for ::windows::core::IUnknown {
    fn from(value: &IFrameworkView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFrameworkView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFrameworkView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFrameworkView> for ::windows::core::IInspectable {
    fn from(value: IFrameworkView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFrameworkView> for ::windows::core::IInspectable {
    fn from(value: &IFrameworkView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFrameworkView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IFrameworkView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFrameworkView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFrameworkView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFrameworkView {}
impl ::core::fmt::Debug for IFrameworkView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFrameworkView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IFrameworkView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{faab5cd0-8924-45ac-ad0f-a08fae5d0324}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IFrameworkView {
    type Vtable = IFrameworkView_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaab5cd0_8924_45ac_ad0f_a08fae5d0324);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkView_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationview: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub SetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    SetWindow: usize,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entrypoint: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct IFrameworkViewSource(::windows::core::IUnknown);
impl IFrameworkViewSource {
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn CreateView(&self) -> ::windows::core::Result<IFrameworkView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IFrameworkView>(result__)
        }
    }
}
impl ::core::convert::From<IFrameworkViewSource> for ::windows::core::IUnknown {
    fn from(value: IFrameworkViewSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFrameworkViewSource> for ::windows::core::IUnknown {
    fn from(value: &IFrameworkViewSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFrameworkViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFrameworkViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFrameworkViewSource> for ::windows::core::IInspectable {
    fn from(value: IFrameworkViewSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFrameworkViewSource> for ::windows::core::IInspectable {
    fn from(value: &IFrameworkViewSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFrameworkViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IFrameworkViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFrameworkViewSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFrameworkViewSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFrameworkViewSource {}
impl ::core::fmt::Debug for IFrameworkViewSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFrameworkViewSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IFrameworkViewSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cd770614-65c4-426c-9494-34fc43554862}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IFrameworkViewSource {
    type Vtable = IFrameworkViewSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd770614_65c4_426c_9494_34fc43554862);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkViewSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHostedViewClosingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHostedViewClosingEventArgs {
    type Vtable = IHostedViewClosingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd238943c_b24e_4790_acb5_3e4243c4ff87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostedViewClosingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnhandledError(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUnhandledError {
    type Vtable = IUnhandledError_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9459b726_53b5_4686_9eaf_fa8162dc3980);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnhandledError_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Propagate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnhandledErrorDetectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUnhandledErrorDetectedEventArgs {
    type Vtable = IUnhandledErrorDetectedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x679ab78b_b336_4822_ac40_0d750f0b7a2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnhandledErrorDetectedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub UnhandledError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct UnhandledError(::windows::core::IUnknown);
impl UnhandledError {
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn Propagate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Propagate)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for UnhandledError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UnhandledError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UnhandledError {}
impl ::core::fmt::Debug for UnhandledError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnhandledError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnhandledError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.UnhandledError;{9459b726-53b5-4686-9eaf-fa8162dc3980})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UnhandledError {
    type Vtable = IUnhandledError_Vtbl;
    const IID: ::windows::core::GUID = <IUnhandledError as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UnhandledError {
    const NAME: &'static str = "Windows.ApplicationModel.Core.UnhandledError";
}
impl ::core::convert::From<UnhandledError> for ::windows::core::IUnknown {
    fn from(value: UnhandledError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnhandledError> for ::windows::core::IUnknown {
    fn from(value: &UnhandledError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UnhandledError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UnhandledError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UnhandledError> for ::windows::core::IInspectable {
    fn from(value: UnhandledError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnhandledError> for ::windows::core::IInspectable {
    fn from(value: &UnhandledError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UnhandledError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UnhandledError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UnhandledError {}
unsafe impl ::core::marker::Sync for UnhandledError {}
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct UnhandledErrorDetectedEventArgs(::windows::core::IUnknown);
impl UnhandledErrorDetectedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    pub fn UnhandledError(&self) -> ::windows::core::Result<UnhandledError> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnhandledError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnhandledError>(result__)
        }
    }
}
impl ::core::clone::Clone for UnhandledErrorDetectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UnhandledErrorDetectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UnhandledErrorDetectedEventArgs {}
impl ::core::fmt::Debug for UnhandledErrorDetectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnhandledErrorDetectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnhandledErrorDetectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs;{679ab78b-b336-4822-ac40-0d750f0b7a2b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UnhandledErrorDetectedEventArgs {
    type Vtable = IUnhandledErrorDetectedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IUnhandledErrorDetectedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UnhandledErrorDetectedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs";
}
impl ::core::convert::From<UnhandledErrorDetectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: UnhandledErrorDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnhandledErrorDetectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UnhandledErrorDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UnhandledErrorDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UnhandledErrorDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UnhandledErrorDetectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: UnhandledErrorDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnhandledErrorDetectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UnhandledErrorDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UnhandledErrorDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UnhandledErrorDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UnhandledErrorDetectedEventArgs {}
unsafe impl ::core::marker::Sync for UnhandledErrorDetectedEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
