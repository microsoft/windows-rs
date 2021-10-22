#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ActivitySensorTrigger(::windows::runtime::IInspectable);
impl ActivitySensorTrigger {
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub fn SubscribedActivities(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<
            super::super::Devices::Sensors::ActivityType,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::super::Devices::Sensors::ActivityType,
            >>(result__)
        }
    }
    pub fn ReportInterval(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub fn SupportedActivities(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<
            super::super::Devices::Sensors::ActivityType,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<
                super::super::Devices::Sensors::ActivityType,
            >>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn Create(
        reportintervalinmilliseconds: u32,
    ) -> ::windows::runtime::Result<ActivitySensorTrigger> {
        Self::IActivitySensorTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                reportintervalinmilliseconds,
                &mut result__,
            )
            .from_abi::<ActivitySensorTrigger>(result__)
        })
    }
    pub fn IActivitySensorTriggerFactory<
        R,
        F: FnOnce(&IActivitySensorTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ActivitySensorTrigger,
            IActivitySensorTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ActivitySensorTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.ActivitySensorTrigger;{d0dd4342-e37b-4823-a5fe-6b31dfefdeb0})" ) ;
}
unsafe impl ::windows::runtime::Interface for ActivitySensorTrigger {
    type Vtable = IActivitySensorTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3504161602,
        58235,
        18467,
        [165, 254, 107, 49, 223, 239, 222, 176],
    );
}
impl ::windows::runtime::RuntimeName for ActivitySensorTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ActivitySensorTrigger";
}
impl ::std::convert::From<ActivitySensorTrigger> for ::windows::runtime::IUnknown {
    fn from(value: ActivitySensorTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ActivitySensorTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &ActivitySensorTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ActivitySensorTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ActivitySensorTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ActivitySensorTrigger> for ::windows::runtime::IInspectable {
    fn from(value: ActivitySensorTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ActivitySensorTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &ActivitySensorTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ActivitySensorTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ActivitySensorTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ActivitySensorTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ActivitySensorTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ActivitySensorTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ActivitySensorTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for ActivitySensorTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &ActivitySensorTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ActivitySensorTrigger {}
unsafe impl ::std::marker::Sync for ActivitySensorTrigger {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AlarmAccessStatus(pub i32);
impl AlarmAccessStatus {
    pub const Unspecified: AlarmAccessStatus = AlarmAccessStatus(0i32);
    pub const AllowedWithWakeupCapability: AlarmAccessStatus = AlarmAccessStatus(1i32);
    pub const AllowedWithoutWakeupCapability: AlarmAccessStatus = AlarmAccessStatus(2i32);
    pub const Denied: AlarmAccessStatus = AlarmAccessStatus(3i32);
}
impl ::std::convert::From<i32> for AlarmAccessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AlarmAccessStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AlarmAccessStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.Background.AlarmAccessStatus;i4)",
    );
}
pub struct AlarmApplicationManager {}
impl AlarmApplicationManager {
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync(
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<AlarmAccessStatus>>
    {
        Self::IAlarmApplicationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<AlarmAccessStatus>>(result__)
        })
    }
    pub fn GetAccessStatus() -> ::windows::runtime::Result<AlarmAccessStatus> {
        Self::IAlarmApplicationManagerStatics(|this| unsafe {
            let mut result__: AlarmAccessStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AlarmAccessStatus>(result__)
        })
    }
    pub fn IAlarmApplicationManagerStatics<
        R,
        F: FnOnce(&IAlarmApplicationManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AlarmApplicationManager,
            IAlarmApplicationManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for AlarmApplicationManager {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AlarmApplicationManager";
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AppBroadcastTrigger(::windows::runtime::IInspectable);
impl AppBroadcastTrigger {
    pub fn SetProviderInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, AppBroadcastTriggerProviderInfo>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ProviderInfo(&self) -> ::windows::runtime::Result<AppBroadcastTriggerProviderInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppBroadcastTriggerProviderInfo>(result__)
        }
    }
    pub fn CreateAppBroadcastTrigger<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        providerkey: Param0,
    ) -> ::windows::runtime::Result<AppBroadcastTrigger> {
        Self::IAppBroadcastTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                providerkey.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AppBroadcastTrigger>(result__)
        })
    }
    pub fn IAppBroadcastTriggerFactory<
        R,
        F: FnOnce(&IAppBroadcastTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AppBroadcastTrigger,
            IAppBroadcastTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.AppBroadcastTrigger;{74d4f496-8d37-44ec-9481-2a0b9854eb48})" ) ;
}
unsafe impl ::windows::runtime::Interface for AppBroadcastTrigger {
    type Vtable = IAppBroadcastTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1960113302,
        36151,
        17644,
        [148, 129, 42, 11, 152, 84, 235, 72],
    );
}
impl ::windows::runtime::RuntimeName for AppBroadcastTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AppBroadcastTrigger";
}
impl ::std::convert::From<AppBroadcastTrigger> for ::windows::runtime::IUnknown {
    fn from(value: AppBroadcastTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppBroadcastTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &AppBroadcastTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppBroadcastTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppBroadcastTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AppBroadcastTrigger> for ::windows::runtime::IInspectable {
    fn from(value: AppBroadcastTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppBroadcastTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &AppBroadcastTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AppBroadcastTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AppBroadcastTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<AppBroadcastTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppBroadcastTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AppBroadcastTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppBroadcastTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for AppBroadcastTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &AppBroadcastTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AppBroadcastTrigger {}
unsafe impl ::std::marker::Sync for AppBroadcastTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AppBroadcastTriggerProviderInfo(::windows::runtime::IInspectable);
impl AppBroadcastTriggerProviderInfo {
    pub fn SetDisplayNameResource<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DisplayNameResource(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetLogoResource<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn LogoResource(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetVideoKeyFrameInterval<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn VideoKeyFrameInterval(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn SetMaxVideoBitrate(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxVideoBitrate(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetMaxVideoWidth(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxVideoWidth(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetMaxVideoHeight(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxVideoHeight(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastTriggerProviderInfo {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.AppBroadcastTriggerProviderInfo;{f219352d-9de8-4420-9ce2-5eff8f17376b})" ) ;
}
unsafe impl ::windows::runtime::Interface for AppBroadcastTriggerProviderInfo {
    type Vtable = IAppBroadcastTriggerProviderInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4061738285,
        40424,
        17440,
        [156, 226, 94, 255, 143, 23, 55, 107],
    );
}
impl ::windows::runtime::RuntimeName for AppBroadcastTriggerProviderInfo {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.AppBroadcastTriggerProviderInfo";
}
impl ::std::convert::From<AppBroadcastTriggerProviderInfo> for ::windows::runtime::IUnknown {
    fn from(value: AppBroadcastTriggerProviderInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppBroadcastTriggerProviderInfo> for ::windows::runtime::IUnknown {
    fn from(value: &AppBroadcastTriggerProviderInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AppBroadcastTriggerProviderInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AppBroadcastTriggerProviderInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AppBroadcastTriggerProviderInfo> for ::windows::runtime::IInspectable {
    fn from(value: AppBroadcastTriggerProviderInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppBroadcastTriggerProviderInfo> for ::windows::runtime::IInspectable {
    fn from(value: &AppBroadcastTriggerProviderInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AppBroadcastTriggerProviderInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AppBroadcastTriggerProviderInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppBroadcastTriggerProviderInfo {}
unsafe impl ::std::marker::Sync for AppBroadcastTriggerProviderInfo {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ApplicationTrigger(::windows::runtime::IInspectable);
impl ApplicationTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ApplicationTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAsync(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>>(
                result__,
            )
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RequestAsyncWithArguments<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::ValueSet>,
    >(
        &self,
        arguments: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                arguments.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>>(
                result__,
            )
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ApplicationTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.ApplicationTrigger;{0b468630-9574-492c-9e93-1a3ae6335fe9})" ) ;
}
unsafe impl ::windows::runtime::Interface for ApplicationTrigger {
    type Vtable = IApplicationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        189171248,
        38260,
        18732,
        [158, 147, 26, 58, 230, 51, 95, 233],
    );
}
impl ::windows::runtime::RuntimeName for ApplicationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ApplicationTrigger";
}
impl ::std::convert::From<ApplicationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: ApplicationTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ApplicationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &ApplicationTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ApplicationTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ApplicationTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ApplicationTrigger> for ::windows::runtime::IInspectable {
    fn from(value: ApplicationTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ApplicationTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &ApplicationTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ApplicationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ApplicationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ApplicationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ApplicationTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ApplicationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ApplicationTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for ApplicationTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &ApplicationTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ApplicationTrigger {}
unsafe impl ::std::marker::Sync for ApplicationTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ApplicationTriggerDetails(::windows::runtime::IInspectable);
impl ApplicationTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Arguments(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ApplicationTriggerDetails {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.ApplicationTriggerDetails;{97dc6ab2-2219-4a9e-9c5e-41d047f76e82})" ) ;
}
unsafe impl ::windows::runtime::Interface for ApplicationTriggerDetails {
    type Vtable = IApplicationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2547804850,
        8729,
        19102,
        [156, 94, 65, 208, 71, 247, 110, 130],
    );
}
impl ::windows::runtime::RuntimeName for ApplicationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ApplicationTriggerDetails";
}
impl ::std::convert::From<ApplicationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: ApplicationTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ApplicationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &ApplicationTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ApplicationTriggerDetails
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ApplicationTriggerDetails
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ApplicationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: ApplicationTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ApplicationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &ApplicationTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ApplicationTriggerDetails
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ApplicationTriggerDetails
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ApplicationTriggerDetails {}
unsafe impl ::std::marker::Sync for ApplicationTriggerDetails {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ApplicationTriggerResult(pub i32);
impl ApplicationTriggerResult {
    pub const Allowed: ApplicationTriggerResult = ApplicationTriggerResult(0i32);
    pub const CurrentlyRunning: ApplicationTriggerResult = ApplicationTriggerResult(1i32);
    pub const DisabledByPolicy: ApplicationTriggerResult = ApplicationTriggerResult(2i32);
    pub const UnknownError: ApplicationTriggerResult = ApplicationTriggerResult(3i32);
}
impl ::std::convert::From<i32> for ApplicationTriggerResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ApplicationTriggerResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ApplicationTriggerResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.Background.ApplicationTriggerResult;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AppointmentStoreNotificationTrigger(::windows::runtime::IInspectable);
impl AppointmentStoreNotificationTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AppointmentStoreNotificationTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentStoreNotificationTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.AppointmentStoreNotificationTrigger;{64d4040c-c201-42ad-aa2a-e21ba3425b6d})" ) ;
}
unsafe impl ::windows::runtime::Interface for AppointmentStoreNotificationTrigger {
    type Vtable = IAppointmentStoreNotificationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1691616268,
        49665,
        17069,
        [170, 42, 226, 27, 163, 66, 91, 109],
    );
}
impl ::windows::runtime::RuntimeName for AppointmentStoreNotificationTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.AppointmentStoreNotificationTrigger";
}
impl ::std::convert::From<AppointmentStoreNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentStoreNotificationTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppointmentStoreNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentStoreNotificationTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AppointmentStoreNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AppointmentStoreNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AppointmentStoreNotificationTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: AppointmentStoreNotificationTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppointmentStoreNotificationTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &AppointmentStoreNotificationTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AppointmentStoreNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AppointmentStoreNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<AppointmentStoreNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppointmentStoreNotificationTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AppointmentStoreNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppointmentStoreNotificationTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for AppointmentStoreNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &AppointmentStoreNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AppointmentStoreNotificationTrigger {}
unsafe impl ::std::marker::Sync for AppointmentStoreNotificationTrigger {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct BackgroundAccessRequestKind(pub i32);
impl BackgroundAccessRequestKind {
    pub const AlwaysAllowed: BackgroundAccessRequestKind = BackgroundAccessRequestKind(0i32);
    pub const AllowedSubjectToSystemPolicy: BackgroundAccessRequestKind =
        BackgroundAccessRequestKind(1i32);
}
impl ::std::convert::From<i32> for BackgroundAccessRequestKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BackgroundAccessRequestKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundAccessRequestKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.Background.BackgroundAccessRequestKind;i4)",
    );
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct BackgroundAccessStatus(pub i32);
impl BackgroundAccessStatus {
    pub const Unspecified: BackgroundAccessStatus = BackgroundAccessStatus(0i32);
    pub const AllowedWithAlwaysOnRealTimeConnectivity: BackgroundAccessStatus =
        BackgroundAccessStatus(1i32);
    pub const AllowedMayUseActiveRealTimeConnectivity: BackgroundAccessStatus =
        BackgroundAccessStatus(2i32);
    pub const Denied: BackgroundAccessStatus = BackgroundAccessStatus(3i32);
    pub const AlwaysAllowed: BackgroundAccessStatus = BackgroundAccessStatus(4i32);
    pub const AllowedSubjectToSystemPolicy: BackgroundAccessStatus = BackgroundAccessStatus(5i32);
    pub const DeniedBySystemPolicy: BackgroundAccessStatus = BackgroundAccessStatus(6i32);
    pub const DeniedByUser: BackgroundAccessStatus = BackgroundAccessStatus(7i32);
}
impl ::std::convert::From<i32> for BackgroundAccessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BackgroundAccessStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundAccessStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.Background.BackgroundAccessStatus;i4)",
    );
}
#[repr(C)]
#[derive(
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
)]
pub struct BackgroundAlarmApplicationContract(pub u8);
pub struct BackgroundExecutionManager {}
impl BackgroundExecutionManager {
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync(
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>>
    {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessForApplicationAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        applicationid: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>>
    {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                applicationid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>>(result__)
        })
    }
    pub fn RemoveAccess() -> ::windows::runtime::Result<()> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok()
        })
    }
    pub fn RemoveAccessForApplication<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        applicationid: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                applicationid.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn GetAccessStatus() -> ::windows::runtime::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__: BackgroundAccessStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BackgroundAccessStatus>(result__)
        })
    }
    pub fn GetAccessStatusForApplication<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        applicationid: Param0,
    ) -> ::windows::runtime::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__: BackgroundAccessStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                applicationid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<BackgroundAccessStatus>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessKindAsync<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        requestedaccess: BackgroundAccessRequestKind,
        reason: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IBackgroundExecutionManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                requestedaccess,
                reason.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessKindForModernStandbyAsync<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        requestedaccess: BackgroundAccessRequestKind,
        reason: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IBackgroundExecutionManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                requestedaccess,
                reason.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn GetAccessStatusForModernStandby() -> ::windows::runtime::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics3(|this| unsafe {
            let mut result__: BackgroundAccessStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BackgroundAccessStatus>(result__)
        })
    }
    pub fn GetAccessStatusForModernStandbyForApplication<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        applicationid: Param0,
    ) -> ::windows::runtime::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics3(|this| unsafe {
            let mut result__: BackgroundAccessStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                applicationid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<BackgroundAccessStatus>(result__)
        })
    }
    pub fn IBackgroundExecutionManagerStatics<
        R,
        F: FnOnce(&IBackgroundExecutionManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            BackgroundExecutionManager,
            IBackgroundExecutionManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBackgroundExecutionManagerStatics2<
        R,
        F: FnOnce(&IBackgroundExecutionManagerStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            BackgroundExecutionManager,
            IBackgroundExecutionManagerStatics2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBackgroundExecutionManagerStatics3<
        R,
        F: FnOnce(&IBackgroundExecutionManagerStatics3) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            BackgroundExecutionManager,
            IBackgroundExecutionManagerStatics3,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for BackgroundExecutionManager {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundExecutionManager";
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BackgroundTaskBuilder(::windows::runtime::IInspectable);
impl BackgroundTaskBuilder {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            BackgroundTaskBuilder,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetTaskEntryPoint<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TaskEntryPoint(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetTrigger<'a, Param0: ::windows::runtime::IntoParam<'a, IBackgroundTrigger>>(
        &self,
        trigger: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                trigger.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AddCondition<'a, Param0: ::windows::runtime::IntoParam<'a, IBackgroundCondition>>(
        &self,
        condition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                condition.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Register(&self) -> ::windows::runtime::Result<BackgroundTaskRegistration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BackgroundTaskRegistration>(result__)
        }
    }
    pub fn SetCancelOnConditionLoss(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskBuilder2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CancelOnConditionLoss(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskBuilder2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsNetworkRequested(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskBuilder3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsNetworkRequested(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskBuilder3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn TaskGroup(&self) -> ::windows::runtime::Result<BackgroundTaskRegistrationGroup> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskBuilder4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BackgroundTaskRegistrationGroup>(result__)
        }
    }
    pub fn SetTaskGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, BackgroundTaskRegistrationGroup>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskBuilder4>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetTaskEntryPointClsid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        taskentrypoint: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskBuilder5>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                taskentrypoint.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTaskBuilder {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.BackgroundTaskBuilder;{0351550e-3e64-4572-a93a-84075a37c917})" ) ;
}
unsafe impl ::windows::runtime::Interface for BackgroundTaskBuilder {
    type Vtable = IBackgroundTaskBuilder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        55661838,
        15972,
        17778,
        [169, 58, 132, 7, 90, 55, 201, 23],
    );
}
impl ::windows::runtime::RuntimeName for BackgroundTaskBuilder {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskBuilder";
}
impl ::std::convert::From<BackgroundTaskBuilder> for ::windows::runtime::IUnknown {
    fn from(value: BackgroundTaskBuilder) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BackgroundTaskBuilder> for ::windows::runtime::IUnknown {
    fn from(value: &BackgroundTaskBuilder) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BackgroundTaskBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &BackgroundTaskBuilder
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<BackgroundTaskBuilder> for ::windows::runtime::IInspectable {
    fn from(value: BackgroundTaskBuilder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BackgroundTaskBuilder> for ::windows::runtime::IInspectable {
    fn from(value: &BackgroundTaskBuilder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for BackgroundTaskBuilder
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a BackgroundTaskBuilder
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BackgroundTaskCanceledEventHandler(::windows::runtime::IUnknown);
impl BackgroundTaskCanceledEventHandler {
    pub fn new<
        F: FnMut(
                &::std::option::Option<IBackgroundTaskInstance>,
                BackgroundTaskCancellationReason,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = BackgroundTaskCanceledEventHandler_box::<F> {
            vtable: &BackgroundTaskCanceledEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, IBackgroundTaskInstance>>(
        &self,
        sender: Param0,
        reason: BackgroundTaskCancellationReason,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                reason,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTaskCanceledEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({a6c4bac0-51f8-4c57-ac3f-156dd1680c4f})",
    );
}
unsafe impl ::windows::runtime::Interface for BackgroundTaskCanceledEventHandler {
    type Vtable = BackgroundTaskCanceledEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2797910720,
        20984,
        19543,
        [172, 63, 21, 109, 209, 104, 12, 79],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundTaskCanceledEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        reason: BackgroundTaskCancellationReason,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct BackgroundTaskCanceledEventHandler_box<
    F: FnMut(
            &::std::option::Option<IBackgroundTaskInstance>,
            BackgroundTaskCancellationReason,
        ) -> ::windows::runtime::Result<()>
        + 'static,
> {
    vtable: *const BackgroundTaskCanceledEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<
        F: FnMut(
                &::std::option::Option<IBackgroundTaskInstance>,
                BackgroundTaskCancellationReason,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    > BackgroundTaskCanceledEventHandler_box<F>
{
    const VTABLE: BackgroundTaskCanceledEventHandler_abi = BackgroundTaskCanceledEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid
            == &<BackgroundTaskCanceledEventHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        reason: BackgroundTaskCancellationReason,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <IBackgroundTaskInstance as ::windows::runtime::Abi>::Abi
                as *const <IBackgroundTaskInstance as ::windows::runtime::Abi>::DefaultType),
            reason,
        )
        .into()
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct BackgroundTaskCancellationReason(pub i32);
impl BackgroundTaskCancellationReason {
    pub const Abort: BackgroundTaskCancellationReason = BackgroundTaskCancellationReason(0i32);
    pub const Terminating: BackgroundTaskCancellationReason =
        BackgroundTaskCancellationReason(1i32);
    pub const LoggingOff: BackgroundTaskCancellationReason = BackgroundTaskCancellationReason(2i32);
    pub const ServicingUpdate: BackgroundTaskCancellationReason =
        BackgroundTaskCancellationReason(3i32);
    pub const IdleTask: BackgroundTaskCancellationReason = BackgroundTaskCancellationReason(4i32);
    pub const Uninstall: BackgroundTaskCancellationReason = BackgroundTaskCancellationReason(5i32);
    pub const ConditionLoss: BackgroundTaskCancellationReason =
        BackgroundTaskCancellationReason(6i32);
    pub const SystemPolicy: BackgroundTaskCancellationReason =
        BackgroundTaskCancellationReason(7i32);
    pub const QuietHoursEntered: BackgroundTaskCancellationReason =
        BackgroundTaskCancellationReason(8i32);
    pub const ExecutionTimeExceeded: BackgroundTaskCancellationReason =
        BackgroundTaskCancellationReason(9i32);
    pub const ResourceRevocation: BackgroundTaskCancellationReason =
        BackgroundTaskCancellationReason(10i32);
    pub const EnergySaver: BackgroundTaskCancellationReason =
        BackgroundTaskCancellationReason(11i32);
}
impl ::std::convert::From<i32> for BackgroundTaskCancellationReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BackgroundTaskCancellationReason {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTaskCancellationReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.Background.BackgroundTaskCancellationReason;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BackgroundTaskCompletedEventArgs(::windows::runtime::IInspectable);
impl BackgroundTaskCompletedEventArgs {
    pub fn InstanceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    pub fn CheckResult(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTaskCompletedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.BackgroundTaskCompletedEventArgs;{565d25cf-f209-48f4-9967-2b184f7bfbf0})" ) ;
}
unsafe impl ::windows::runtime::Interface for BackgroundTaskCompletedEventArgs {
    type Vtable = IBackgroundTaskCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1448945103,
        61961,
        18676,
        [153, 103, 43, 24, 79, 123, 251, 240],
    );
}
impl ::windows::runtime::RuntimeName for BackgroundTaskCompletedEventArgs {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.BackgroundTaskCompletedEventArgs";
}
impl ::std::convert::From<BackgroundTaskCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: BackgroundTaskCompletedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BackgroundTaskCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &BackgroundTaskCompletedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for BackgroundTaskCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &BackgroundTaskCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<BackgroundTaskCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: BackgroundTaskCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BackgroundTaskCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &BackgroundTaskCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for BackgroundTaskCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a BackgroundTaskCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BackgroundTaskCompletedEventArgs {}
unsafe impl ::std::marker::Sync for BackgroundTaskCompletedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BackgroundTaskCompletedEventHandler(::windows::runtime::IUnknown);
impl BackgroundTaskCompletedEventHandler {
    pub fn new<
        F: FnMut(
                &::std::option::Option<BackgroundTaskRegistration>,
                &::std::option::Option<BackgroundTaskCompletedEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = BackgroundTaskCompletedEventHandler_box::<F> {
            vtable: &BackgroundTaskCompletedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, BackgroundTaskRegistration>,
        Param1: ::windows::runtime::IntoParam<'a, BackgroundTaskCompletedEventArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTaskCompletedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({5b38e929-a086-46a7-a678-439135822bcf})",
    );
}
unsafe impl ::windows::runtime::Interface for BackgroundTaskCompletedEventHandler {
    type Vtable = BackgroundTaskCompletedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1530456361,
        41094,
        18087,
        [166, 120, 67, 145, 53, 130, 43, 207],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundTaskCompletedEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct BackgroundTaskCompletedEventHandler_box<
    F: FnMut(
            &::std::option::Option<BackgroundTaskRegistration>,
            &::std::option::Option<BackgroundTaskCompletedEventArgs>,
        ) -> ::windows::runtime::Result<()>
        + 'static,
> {
    vtable: *const BackgroundTaskCompletedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<
        F: FnMut(
                &::std::option::Option<BackgroundTaskRegistration>,
                &::std::option::Option<BackgroundTaskCompletedEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    > BackgroundTaskCompletedEventHandler_box<F>
{
    const VTABLE: BackgroundTaskCompletedEventHandler_abi = BackgroundTaskCompletedEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid
            == &<BackgroundTaskCompletedEventHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < BackgroundTaskRegistration as :: windows :: runtime :: Abi > :: Abi as * const < BackgroundTaskRegistration as :: windows :: runtime :: Abi > :: DefaultType ) , & * ( & args as * const < BackgroundTaskCompletedEventArgs as :: windows :: runtime :: Abi > :: Abi as * const < BackgroundTaskCompletedEventArgs as :: windows :: runtime :: Abi > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BackgroundTaskDeferral(::windows::runtime::IInspectable);
impl BackgroundTaskDeferral {
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTaskDeferral {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.BackgroundTaskDeferral;{93cc156d-af27-4dd3-846e-24ee40cadd25})" ) ;
}
unsafe impl ::windows::runtime::Interface for BackgroundTaskDeferral {
    type Vtable = IBackgroundTaskDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2479625581,
        44839,
        19923,
        [132, 110, 36, 238, 64, 202, 221, 37],
    );
}
impl ::windows::runtime::RuntimeName for BackgroundTaskDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskDeferral";
}
impl ::std::convert::From<BackgroundTaskDeferral> for ::windows::runtime::IUnknown {
    fn from(value: BackgroundTaskDeferral) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BackgroundTaskDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &BackgroundTaskDeferral) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for BackgroundTaskDeferral
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &BackgroundTaskDeferral
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<BackgroundTaskDeferral> for ::windows::runtime::IInspectable {
    fn from(value: BackgroundTaskDeferral) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BackgroundTaskDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &BackgroundTaskDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for BackgroundTaskDeferral
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a BackgroundTaskDeferral
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BackgroundTaskDeferral {}
unsafe impl ::std::marker::Sync for BackgroundTaskDeferral {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BackgroundTaskProgressEventArgs(::windows::runtime::IInspectable);
impl BackgroundTaskProgressEventArgs {
    pub fn InstanceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    pub fn Progress(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTaskProgressEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.BackgroundTaskProgressEventArgs;{fb1468ac-8332-4d0a-9532-03eae684da31})" ) ;
}
unsafe impl ::windows::runtime::Interface for BackgroundTaskProgressEventArgs {
    type Vtable = IBackgroundTaskProgressEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4212418732,
        33586,
        19722,
        [149, 50, 3, 234, 230, 132, 218, 49],
    );
}
impl ::windows::runtime::RuntimeName for BackgroundTaskProgressEventArgs {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.BackgroundTaskProgressEventArgs";
}
impl ::std::convert::From<BackgroundTaskProgressEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: BackgroundTaskProgressEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BackgroundTaskProgressEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &BackgroundTaskProgressEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for BackgroundTaskProgressEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &BackgroundTaskProgressEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<BackgroundTaskProgressEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: BackgroundTaskProgressEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BackgroundTaskProgressEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &BackgroundTaskProgressEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for BackgroundTaskProgressEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a BackgroundTaskProgressEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BackgroundTaskProgressEventArgs {}
unsafe impl ::std::marker::Sync for BackgroundTaskProgressEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BackgroundTaskProgressEventHandler(::windows::runtime::IUnknown);
impl BackgroundTaskProgressEventHandler {
    pub fn new<
        F: FnMut(
                &::std::option::Option<BackgroundTaskRegistration>,
                &::std::option::Option<BackgroundTaskProgressEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = BackgroundTaskProgressEventHandler_box::<F> {
            vtable: &BackgroundTaskProgressEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, BackgroundTaskRegistration>,
        Param1: ::windows::runtime::IntoParam<'a, BackgroundTaskProgressEventArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTaskProgressEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({46e0683c-8a88-4c99-804c-76897f6277a6})",
    );
}
unsafe impl ::windows::runtime::Interface for BackgroundTaskProgressEventHandler {
    type Vtable = BackgroundTaskProgressEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1189111868,
        35464,
        19609,
        [128, 76, 118, 137, 127, 98, 119, 166],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundTaskProgressEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct BackgroundTaskProgressEventHandler_box<
    F: FnMut(
            &::std::option::Option<BackgroundTaskRegistration>,
            &::std::option::Option<BackgroundTaskProgressEventArgs>,
        ) -> ::windows::runtime::Result<()>
        + 'static,
> {
    vtable: *const BackgroundTaskProgressEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<
        F: FnMut(
                &::std::option::Option<BackgroundTaskRegistration>,
                &::std::option::Option<BackgroundTaskProgressEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    > BackgroundTaskProgressEventHandler_box<F>
{
    const VTABLE: BackgroundTaskProgressEventHandler_abi = BackgroundTaskProgressEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid
            == &<BackgroundTaskProgressEventHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < BackgroundTaskRegistration as :: windows :: runtime :: Abi > :: Abi as * const < BackgroundTaskRegistration as :: windows :: runtime :: Abi > :: DefaultType ) , & * ( & args as * const < BackgroundTaskProgressEventArgs as :: windows :: runtime :: Abi > :: Abi as * const < BackgroundTaskProgressEventArgs as :: windows :: runtime :: Abi > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BackgroundTaskRegistration(::windows::runtime::IInspectable);
impl BackgroundTaskRegistration {
    pub fn TaskId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Progress<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, BackgroundTaskProgressEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgress<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Completed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, BackgroundTaskCompletedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Unregister(&self, canceltask: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                canceltask,
            )
            .ok()
        }
    }
    pub fn Trigger(&self) -> ::windows::runtime::Result<IBackgroundTrigger> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskRegistration2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IBackgroundTrigger>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllTasks() -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IMapView<
            ::windows::runtime::GUID,
            IBackgroundTaskRegistration,
        >,
    > {
        Self::IBackgroundTaskRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IMapView<
                ::windows::runtime::GUID,
                IBackgroundTaskRegistration,
            >>(result__)
        })
    }
    pub fn TaskGroup(&self) -> ::windows::runtime::Result<BackgroundTaskRegistrationGroup> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskRegistration3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BackgroundTaskRegistrationGroup>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllTaskGroups() -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IMapView<
            ::windows::runtime::HSTRING,
            BackgroundTaskRegistrationGroup,
        >,
    > {
        Self::IBackgroundTaskRegistrationStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IMapView<
                ::windows::runtime::HSTRING,
                BackgroundTaskRegistrationGroup,
            >>(result__)
        })
    }
    pub fn GetTaskGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        groupid: Param0,
    ) -> ::windows::runtime::Result<BackgroundTaskRegistrationGroup> {
        Self::IBackgroundTaskRegistrationStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                groupid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<BackgroundTaskRegistrationGroup>(result__)
        })
    }
    pub fn IBackgroundTaskRegistrationStatics<
        R,
        F: FnOnce(&IBackgroundTaskRegistrationStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            BackgroundTaskRegistration,
            IBackgroundTaskRegistrationStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBackgroundTaskRegistrationStatics2<
        R,
        F: FnOnce(&IBackgroundTaskRegistrationStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            BackgroundTaskRegistration,
            IBackgroundTaskRegistrationStatics2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTaskRegistration {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.BackgroundTaskRegistration;{10654cc2-a26e-43bf-8c12-1fb40dbfbfa0})" ) ;
}
unsafe impl ::windows::runtime::Interface for BackgroundTaskRegistration {
    type Vtable = IBackgroundTaskRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        275074242,
        41582,
        17343,
        [140, 18, 31, 180, 13, 191, 191, 160],
    );
}
impl ::windows::runtime::RuntimeName for BackgroundTaskRegistration {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskRegistration";
}
impl ::std::convert::From<BackgroundTaskRegistration> for ::windows::runtime::IUnknown {
    fn from(value: BackgroundTaskRegistration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BackgroundTaskRegistration> for ::windows::runtime::IUnknown {
    fn from(value: &BackgroundTaskRegistration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for BackgroundTaskRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &BackgroundTaskRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<BackgroundTaskRegistration> for ::windows::runtime::IInspectable {
    fn from(value: BackgroundTaskRegistration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BackgroundTaskRegistration> for ::windows::runtime::IInspectable {
    fn from(value: &BackgroundTaskRegistration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for BackgroundTaskRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a BackgroundTaskRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<BackgroundTaskRegistration> for IBackgroundTaskRegistration {
    fn from(value: BackgroundTaskRegistration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BackgroundTaskRegistration> for IBackgroundTaskRegistration {
    fn from(value: &BackgroundTaskRegistration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTaskRegistration>
    for BackgroundTaskRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTaskRegistration> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTaskRegistration>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTaskRegistration>
    for &BackgroundTaskRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTaskRegistration> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTaskRegistration>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::TryFrom<BackgroundTaskRegistration> for IBackgroundTaskRegistration2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: BackgroundTaskRegistration) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&BackgroundTaskRegistration> for IBackgroundTaskRegistration2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &BackgroundTaskRegistration) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTaskRegistration2>
    for BackgroundTaskRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTaskRegistration2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTaskRegistration2>
    for &BackgroundTaskRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTaskRegistration2> {
        ::std::convert::TryInto::<IBackgroundTaskRegistration2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<BackgroundTaskRegistration> for IBackgroundTaskRegistration3 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: BackgroundTaskRegistration) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&BackgroundTaskRegistration> for IBackgroundTaskRegistration3 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &BackgroundTaskRegistration) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTaskRegistration3>
    for BackgroundTaskRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTaskRegistration3> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTaskRegistration3>
    for &BackgroundTaskRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTaskRegistration3> {
        ::std::convert::TryInto::<IBackgroundTaskRegistration3>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for BackgroundTaskRegistration {}
unsafe impl ::std::marker::Sync for BackgroundTaskRegistration {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BackgroundTaskRegistrationGroup(::windows::runtime::IInspectable);
impl BackgroundTaskRegistrationGroup {
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn BackgroundActivated<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                BackgroundTaskRegistrationGroup,
                super::Activation::BackgroundActivatedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveBackgroundActivated<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllTasks(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IMapView<
            ::windows::runtime::GUID,
            BackgroundTaskRegistration,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IMapView<
                ::windows::runtime::GUID,
                BackgroundTaskRegistration,
            >>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        id: Param0,
    ) -> ::windows::runtime::Result<BackgroundTaskRegistrationGroup> {
        Self::IBackgroundTaskRegistrationGroupFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                id.into_param().abi(),
                &mut result__,
            )
            .from_abi::<BackgroundTaskRegistrationGroup>(result__)
        })
    }
    pub fn CreateWithName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        id: Param0,
        name: Param1,
    ) -> ::windows::runtime::Result<BackgroundTaskRegistrationGroup> {
        Self::IBackgroundTaskRegistrationGroupFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                id.into_param().abi(),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<BackgroundTaskRegistrationGroup>(result__)
        })
    }
    pub fn IBackgroundTaskRegistrationGroupFactory<
        R,
        F: FnOnce(&IBackgroundTaskRegistrationGroupFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            BackgroundTaskRegistrationGroup,
            IBackgroundTaskRegistrationGroupFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTaskRegistrationGroup {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.BackgroundTaskRegistrationGroup;{2ab1919a-871b-4167-8a76-055cd67b5b23})" ) ;
}
unsafe impl ::windows::runtime::Interface for BackgroundTaskRegistrationGroup {
    type Vtable = IBackgroundTaskRegistrationGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        716280218,
        34587,
        16743,
        [138, 118, 5, 92, 214, 123, 91, 35],
    );
}
impl ::windows::runtime::RuntimeName for BackgroundTaskRegistrationGroup {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.BackgroundTaskRegistrationGroup";
}
impl ::std::convert::From<BackgroundTaskRegistrationGroup> for ::windows::runtime::IUnknown {
    fn from(value: BackgroundTaskRegistrationGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BackgroundTaskRegistrationGroup> for ::windows::runtime::IUnknown {
    fn from(value: &BackgroundTaskRegistrationGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for BackgroundTaskRegistrationGroup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &BackgroundTaskRegistrationGroup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<BackgroundTaskRegistrationGroup> for ::windows::runtime::IInspectable {
    fn from(value: BackgroundTaskRegistrationGroup) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BackgroundTaskRegistrationGroup> for ::windows::runtime::IInspectable {
    fn from(value: &BackgroundTaskRegistrationGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for BackgroundTaskRegistrationGroup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a BackgroundTaskRegistrationGroup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BackgroundTaskRegistrationGroup {}
unsafe impl ::std::marker::Sync for BackgroundTaskRegistrationGroup {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct BackgroundTaskThrottleCounter(pub i32);
impl BackgroundTaskThrottleCounter {
    pub const All: BackgroundTaskThrottleCounter = BackgroundTaskThrottleCounter(0i32);
    pub const Cpu: BackgroundTaskThrottleCounter = BackgroundTaskThrottleCounter(1i32);
    pub const Network: BackgroundTaskThrottleCounter = BackgroundTaskThrottleCounter(2i32);
}
impl ::std::convert::From<i32> for BackgroundTaskThrottleCounter {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BackgroundTaskThrottleCounter {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTaskThrottleCounter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.Background.BackgroundTaskThrottleCounter;i4)",
    );
}
pub struct BackgroundWorkCost {}
impl BackgroundWorkCost {
    pub fn CurrentBackgroundWorkCost() -> ::windows::runtime::Result<BackgroundWorkCostValue> {
        Self::IBackgroundWorkCostStatics(|this| unsafe {
            let mut result__: BackgroundWorkCostValue = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BackgroundWorkCostValue>(result__)
        })
    }
    pub fn IBackgroundWorkCostStatics<
        R,
        F: FnOnce(&IBackgroundWorkCostStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            BackgroundWorkCost,
            IBackgroundWorkCostStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for BackgroundWorkCost {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundWorkCost";
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct BackgroundWorkCostValue(pub i32);
impl BackgroundWorkCostValue {
    pub const Low: BackgroundWorkCostValue = BackgroundWorkCostValue(0i32);
    pub const Medium: BackgroundWorkCostValue = BackgroundWorkCostValue(1i32);
    pub const High: BackgroundWorkCostValue = BackgroundWorkCostValue(2i32);
}
impl ::std::convert::From<i32> for BackgroundWorkCostValue {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BackgroundWorkCostValue {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundWorkCostValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.Background.BackgroundWorkCostValue;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BluetoothLEAdvertisementPublisherTrigger(::windows::runtime::IInspectable);
impl BluetoothLEAdvertisementPublisherTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            BluetoothLEAdvertisementPublisherTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn Advertisement(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisement,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisement>(
                result__,
            )
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PreferredTransmitPowerLevelInDBm(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::IReference<i16>> {
        let this = &::windows::runtime::Interface::cast::<
            IBluetoothLEAdvertisementPublisherTrigger2,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<i16>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPreferredTransmitPowerLevelInDBm<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i16>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            IBluetoothLEAdvertisementPublisherTrigger2,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn UseExtendedFormat(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<
            IBluetoothLEAdvertisementPublisherTrigger2,
        >(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetUseExtendedFormat(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            IBluetoothLEAdvertisementPublisherTrigger2,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsAnonymous(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<
            IBluetoothLEAdvertisementPublisherTrigger2,
        >(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAnonymous(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            IBluetoothLEAdvertisementPublisherTrigger2,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IncludeTransmitPowerLevel(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<
            IBluetoothLEAdvertisementPublisherTrigger2,
        >(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIncludeTransmitPowerLevel(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            IBluetoothLEAdvertisementPublisherTrigger2,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisementPublisherTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.BluetoothLEAdvertisementPublisherTrigger;{ab3e2612-25d3-48ae-8724-d81877ae6129})" ) ;
}
unsafe impl ::windows::runtime::Interface for BluetoothLEAdvertisementPublisherTrigger {
    type Vtable = IBluetoothLEAdvertisementPublisherTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2872976914,
        9683,
        18606,
        [135, 36, 216, 24, 119, 174, 97, 41],
    );
}
impl ::windows::runtime::RuntimeName for BluetoothLEAdvertisementPublisherTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.BluetoothLEAdvertisementPublisherTrigger";
}
impl ::std::convert::From<BluetoothLEAdvertisementPublisherTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: BluetoothLEAdvertisementPublisherTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BluetoothLEAdvertisementPublisherTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: &BluetoothLEAdvertisementPublisherTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for BluetoothLEAdvertisementPublisherTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &BluetoothLEAdvertisementPublisherTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<BluetoothLEAdvertisementPublisherTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: BluetoothLEAdvertisementPublisherTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BluetoothLEAdvertisementPublisherTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &BluetoothLEAdvertisementPublisherTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for BluetoothLEAdvertisementPublisherTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a BluetoothLEAdvertisementPublisherTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<BluetoothLEAdvertisementPublisherTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: BluetoothLEAdvertisementPublisherTrigger,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&BluetoothLEAdvertisementPublisherTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &BluetoothLEAdvertisementPublisherTrigger,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for BluetoothLEAdvertisementPublisherTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &BluetoothLEAdvertisementPublisherTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for BluetoothLEAdvertisementPublisherTrigger {}
unsafe impl ::std::marker::Sync for BluetoothLEAdvertisementPublisherTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BluetoothLEAdvertisementWatcherTrigger(::windows::runtime::IInspectable);
impl BluetoothLEAdvertisementWatcherTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            BluetoothLEAdvertisementWatcherTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn MinSamplingInterval(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MaxSamplingInterval(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MinOutOfRangeTimeout(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MaxOutOfRangeTimeout(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Devices_Bluetooth")]
    pub fn SignalStrengthFilter(
        &self,
    ) -> ::windows::runtime::Result<super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter>(result__)
        }
    }
    #[cfg(feature = "Devices_Bluetooth")]
    pub fn SetSignalStrengthFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn AdvertisementFilter(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .12 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::Devices::Bluetooth::Advertisement:: BluetoothLEAdvertisementFilter > ( result__ )
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn SetAdvertisementFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AllowExtendedAdvertisements(&self) -> ::windows::runtime::Result<bool> {
        let this =
            &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementWatcherTrigger2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowExtendedAdvertisements(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementWatcherTrigger2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisementWatcherTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.BluetoothLEAdvertisementWatcherTrigger;{1aab1819-bce1-48eb-a827-59fb7cee52a6})" ) ;
}
unsafe impl ::windows::runtime::Interface for BluetoothLEAdvertisementWatcherTrigger {
    type Vtable = IBluetoothLEAdvertisementWatcherTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        447420441,
        48353,
        18667,
        [168, 39, 89, 251, 124, 238, 82, 166],
    );
}
impl ::windows::runtime::RuntimeName for BluetoothLEAdvertisementWatcherTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.BluetoothLEAdvertisementWatcherTrigger";
}
impl ::std::convert::From<BluetoothLEAdvertisementWatcherTrigger> for ::windows::runtime::IUnknown {
    fn from(value: BluetoothLEAdvertisementWatcherTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BluetoothLEAdvertisementWatcherTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: &BluetoothLEAdvertisementWatcherTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for BluetoothLEAdvertisementWatcherTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &BluetoothLEAdvertisementWatcherTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<BluetoothLEAdvertisementWatcherTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: BluetoothLEAdvertisementWatcherTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BluetoothLEAdvertisementWatcherTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &BluetoothLEAdvertisementWatcherTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for BluetoothLEAdvertisementWatcherTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a BluetoothLEAdvertisementWatcherTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<BluetoothLEAdvertisementWatcherTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: BluetoothLEAdvertisementWatcherTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&BluetoothLEAdvertisementWatcherTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &BluetoothLEAdvertisementWatcherTrigger,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for BluetoothLEAdvertisementWatcherTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &BluetoothLEAdvertisementWatcherTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for BluetoothLEAdvertisementWatcherTrigger {}
unsafe impl ::std::marker::Sync for BluetoothLEAdvertisementWatcherTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CachedFileUpdaterTrigger(::windows::runtime::IInspectable);
impl CachedFileUpdaterTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            CachedFileUpdaterTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CachedFileUpdaterTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.CachedFileUpdaterTrigger;{e21caeeb-32f2-4d31-b553-b9e01bde37e0})" ) ;
}
unsafe impl ::windows::runtime::Interface for CachedFileUpdaterTrigger {
    type Vtable = ICachedFileUpdaterTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3793530603,
        13042,
        19761,
        [181, 83, 185, 224, 27, 222, 55, 224],
    );
}
impl ::windows::runtime::RuntimeName for CachedFileUpdaterTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CachedFileUpdaterTrigger";
}
impl ::std::convert::From<CachedFileUpdaterTrigger> for ::windows::runtime::IUnknown {
    fn from(value: CachedFileUpdaterTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CachedFileUpdaterTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &CachedFileUpdaterTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CachedFileUpdaterTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CachedFileUpdaterTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CachedFileUpdaterTrigger> for ::windows::runtime::IInspectable {
    fn from(value: CachedFileUpdaterTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CachedFileUpdaterTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &CachedFileUpdaterTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CachedFileUpdaterTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CachedFileUpdaterTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<CachedFileUpdaterTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CachedFileUpdaterTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CachedFileUpdaterTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CachedFileUpdaterTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for CachedFileUpdaterTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &CachedFileUpdaterTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for CachedFileUpdaterTrigger {}
unsafe impl ::std::marker::Sync for CachedFileUpdaterTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CachedFileUpdaterTriggerDetails(::windows::runtime::IInspectable);
impl CachedFileUpdaterTriggerDetails {
    #[cfg(feature = "Storage_Provider")]
    pub fn UpdateTarget(
        &self,
    ) -> ::windows::runtime::Result<super::super::Storage::Provider::CachedFileTarget> {
        let this = self;
        unsafe {
            let mut result__: super::super::Storage::Provider::CachedFileTarget =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Storage::Provider::CachedFileTarget>(result__)
        }
    }
    #[cfg(feature = "Storage_Provider")]
    pub fn UpdateRequest(
        &self,
    ) -> ::windows::runtime::Result<super::super::Storage::Provider::FileUpdateRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Storage::Provider::FileUpdateRequest>(result__)
        }
    }
    pub fn CanRequestUserInput(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CachedFileUpdaterTriggerDetails {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.CachedFileUpdaterTriggerDetails;{71838c13-1314-47b4-9597-dc7e248c17cc})" ) ;
}
unsafe impl ::windows::runtime::Interface for CachedFileUpdaterTriggerDetails {
    type Vtable = ICachedFileUpdaterTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1904446483,
        4884,
        18356,
        [149, 151, 220, 126, 36, 140, 23, 204],
    );
}
impl ::windows::runtime::RuntimeName for CachedFileUpdaterTriggerDetails {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.CachedFileUpdaterTriggerDetails";
}
impl ::std::convert::From<CachedFileUpdaterTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: CachedFileUpdaterTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CachedFileUpdaterTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &CachedFileUpdaterTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CachedFileUpdaterTriggerDetails
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CachedFileUpdaterTriggerDetails
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CachedFileUpdaterTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: CachedFileUpdaterTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CachedFileUpdaterTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &CachedFileUpdaterTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CachedFileUpdaterTriggerDetails
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CachedFileUpdaterTriggerDetails
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CachedFileUpdaterTriggerDetails {}
unsafe impl ::std::marker::Sync for CachedFileUpdaterTriggerDetails {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ChatMessageNotificationTrigger(::windows::runtime::IInspectable);
impl ChatMessageNotificationTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ChatMessageNotificationTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageNotificationTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.ChatMessageNotificationTrigger;{513b43bf-1d40-5c5d-78f5-c923fee3739e})" ) ;
}
unsafe impl ::windows::runtime::Interface for ChatMessageNotificationTrigger {
    type Vtable = IChatMessageNotificationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1362838463,
        7488,
        23645,
        [120, 245, 201, 35, 254, 227, 115, 158],
    );
}
impl ::windows::runtime::RuntimeName for ChatMessageNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ChatMessageNotificationTrigger";
}
impl ::std::convert::From<ChatMessageNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: ChatMessageNotificationTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ChatMessageNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &ChatMessageNotificationTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ChatMessageNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ChatMessageNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ChatMessageNotificationTrigger> for ::windows::runtime::IInspectable {
    fn from(value: ChatMessageNotificationTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ChatMessageNotificationTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &ChatMessageNotificationTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ChatMessageNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ChatMessageNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ChatMessageNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ChatMessageNotificationTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ChatMessageNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ChatMessageNotificationTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for ChatMessageNotificationTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &ChatMessageNotificationTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ChatMessageNotificationTrigger {}
unsafe impl ::std::marker::Sync for ChatMessageNotificationTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ChatMessageReceivedNotificationTrigger(::windows::runtime::IInspectable);
impl ChatMessageReceivedNotificationTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ChatMessageReceivedNotificationTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageReceivedNotificationTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.ChatMessageReceivedNotificationTrigger;{3ea3760e-baf5-4077-88e9-060cf6f0c6d5})" ) ;
}
unsafe impl ::windows::runtime::Interface for ChatMessageReceivedNotificationTrigger {
    type Vtable = IChatMessageReceivedNotificationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1050899982,
        47861,
        16503,
        [136, 233, 6, 12, 246, 240, 198, 213],
    );
}
impl ::windows::runtime::RuntimeName for ChatMessageReceivedNotificationTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.ChatMessageReceivedNotificationTrigger";
}
impl ::std::convert::From<ChatMessageReceivedNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: ChatMessageReceivedNotificationTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ChatMessageReceivedNotificationTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: &ChatMessageReceivedNotificationTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ChatMessageReceivedNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ChatMessageReceivedNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ChatMessageReceivedNotificationTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: ChatMessageReceivedNotificationTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ChatMessageReceivedNotificationTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &ChatMessageReceivedNotificationTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ChatMessageReceivedNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ChatMessageReceivedNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ChatMessageReceivedNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ChatMessageReceivedNotificationTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ChatMessageReceivedNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &ChatMessageReceivedNotificationTrigger,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for ChatMessageReceivedNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &ChatMessageReceivedNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ChatMessageReceivedNotificationTrigger {}
unsafe impl ::std::marker::Sync for ChatMessageReceivedNotificationTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CommunicationBlockingAppSetAsActiveTrigger(::windows::runtime::IInspectable);
impl CommunicationBlockingAppSetAsActiveTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            CommunicationBlockingAppSetAsActiveTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CommunicationBlockingAppSetAsActiveTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.CommunicationBlockingAppSetAsActiveTrigger;{fb91f28a-16a5-486d-974c-7835a8477be2})" ) ;
}
unsafe impl ::windows::runtime::Interface for CommunicationBlockingAppSetAsActiveTrigger {
    type Vtable = ICommunicationBlockingAppSetAsActiveTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4220646026,
        5797,
        18541,
        [151, 76, 120, 53, 168, 71, 123, 226],
    );
}
impl ::windows::runtime::RuntimeName for CommunicationBlockingAppSetAsActiveTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.CommunicationBlockingAppSetAsActiveTrigger";
}
impl ::std::convert::From<CommunicationBlockingAppSetAsActiveTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: CommunicationBlockingAppSetAsActiveTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CommunicationBlockingAppSetAsActiveTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CommunicationBlockingAppSetAsActiveTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CommunicationBlockingAppSetAsActiveTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CommunicationBlockingAppSetAsActiveTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CommunicationBlockingAppSetAsActiveTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: CommunicationBlockingAppSetAsActiveTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CommunicationBlockingAppSetAsActiveTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CommunicationBlockingAppSetAsActiveTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CommunicationBlockingAppSetAsActiveTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CommunicationBlockingAppSetAsActiveTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<CommunicationBlockingAppSetAsActiveTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: CommunicationBlockingAppSetAsActiveTrigger,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CommunicationBlockingAppSetAsActiveTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &CommunicationBlockingAppSetAsActiveTrigger,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for CommunicationBlockingAppSetAsActiveTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &CommunicationBlockingAppSetAsActiveTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for CommunicationBlockingAppSetAsActiveTrigger {}
unsafe impl ::std::marker::Sync for CommunicationBlockingAppSetAsActiveTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ContactStoreNotificationTrigger(::windows::runtime::IInspectable);
impl ContactStoreNotificationTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ContactStoreNotificationTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactStoreNotificationTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.ContactStoreNotificationTrigger;{c833419b-4705-4571-9a16-06b997bf9c96})" ) ;
}
unsafe impl ::windows::runtime::Interface for ContactStoreNotificationTrigger {
    type Vtable = IContactStoreNotificationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3358802331,
        18181,
        17777,
        [154, 22, 6, 185, 151, 191, 156, 150],
    );
}
impl ::windows::runtime::RuntimeName for ContactStoreNotificationTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.ContactStoreNotificationTrigger";
}
impl ::std::convert::From<ContactStoreNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: ContactStoreNotificationTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactStoreNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &ContactStoreNotificationTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ContactStoreNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ContactStoreNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ContactStoreNotificationTrigger> for ::windows::runtime::IInspectable {
    fn from(value: ContactStoreNotificationTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactStoreNotificationTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &ContactStoreNotificationTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ContactStoreNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ContactStoreNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ContactStoreNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactStoreNotificationTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ContactStoreNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactStoreNotificationTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for ContactStoreNotificationTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &ContactStoreNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ContactStoreNotificationTrigger {}
unsafe impl ::std::marker::Sync for ContactStoreNotificationTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ContentPrefetchTrigger(::windows::runtime::IInspectable);
impl ContentPrefetchTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ContentPrefetchTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn WaitInterval(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Create<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>,
    >(
        waitinterval: Param0,
    ) -> ::windows::runtime::Result<ContentPrefetchTrigger> {
        Self::IContentPrefetchTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                waitinterval.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ContentPrefetchTrigger>(result__)
        })
    }
    pub fn IContentPrefetchTriggerFactory<
        R,
        F: FnOnce(&IContentPrefetchTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ContentPrefetchTrigger,
            IContentPrefetchTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContentPrefetchTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.ContentPrefetchTrigger;{710627ee-04fa-440b-80c0-173202199e5d})" ) ;
}
unsafe impl ::windows::runtime::Interface for ContentPrefetchTrigger {
    type Vtable = IContentPrefetchTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1896228846,
        1274,
        17419,
        [128, 192, 23, 50, 2, 25, 158, 93],
    );
}
impl ::windows::runtime::RuntimeName for ContentPrefetchTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ContentPrefetchTrigger";
}
impl ::std::convert::From<ContentPrefetchTrigger> for ::windows::runtime::IUnknown {
    fn from(value: ContentPrefetchTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContentPrefetchTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &ContentPrefetchTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ContentPrefetchTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ContentPrefetchTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ContentPrefetchTrigger> for ::windows::runtime::IInspectable {
    fn from(value: ContentPrefetchTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContentPrefetchTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &ContentPrefetchTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ContentPrefetchTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ContentPrefetchTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ContentPrefetchTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContentPrefetchTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ContentPrefetchTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContentPrefetchTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for ContentPrefetchTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &ContentPrefetchTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ConversationalAgentTrigger(::windows::runtime::IInspectable);
impl ConversationalAgentTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ConversationalAgentTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ConversationalAgentTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.ConversationalAgentTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})" ) ;
}
unsafe impl ::windows::runtime::Interface for ConversationalAgentTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl ::windows::runtime::RuntimeName for ConversationalAgentTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ConversationalAgentTrigger";
}
impl ::std::convert::From<ConversationalAgentTrigger> for ::windows::runtime::IUnknown {
    fn from(value: ConversationalAgentTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ConversationalAgentTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &ConversationalAgentTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ConversationalAgentTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ConversationalAgentTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ConversationalAgentTrigger> for ::windows::runtime::IInspectable {
    fn from(value: ConversationalAgentTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ConversationalAgentTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &ConversationalAgentTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ConversationalAgentTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ConversationalAgentTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ConversationalAgentTrigger> for IBackgroundTrigger {
    fn from(value: ConversationalAgentTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ConversationalAgentTrigger> for IBackgroundTrigger {
    fn from(value: &ConversationalAgentTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for ConversationalAgentTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &ConversationalAgentTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CustomSystemEventTrigger(::windows::runtime::IInspectable);
impl CustomSystemEventTrigger {
    pub fn TriggerId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Recurrence(&self) -> ::windows::runtime::Result<CustomSystemEventTriggerRecurrence> {
        let this = self;
        unsafe {
            let mut result__: CustomSystemEventTriggerRecurrence = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CustomSystemEventTriggerRecurrence>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        triggerid: Param0,
        recurrence: CustomSystemEventTriggerRecurrence,
    ) -> ::windows::runtime::Result<CustomSystemEventTrigger> {
        Self::ICustomSystemEventTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                triggerid.into_param().abi(),
                recurrence,
                &mut result__,
            )
            .from_abi::<CustomSystemEventTrigger>(result__)
        })
    }
    pub fn ICustomSystemEventTriggerFactory<
        R,
        F: FnOnce(&ICustomSystemEventTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            CustomSystemEventTrigger,
            ICustomSystemEventTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CustomSystemEventTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.CustomSystemEventTrigger;{f3596798-cf6b-4ef4-a0ca-29cf4a278c87})" ) ;
}
unsafe impl ::windows::runtime::Interface for CustomSystemEventTrigger {
    type Vtable = ICustomSystemEventTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4082722712,
        53099,
        20212,
        [160, 202, 41, 207, 74, 39, 140, 135],
    );
}
impl ::windows::runtime::RuntimeName for CustomSystemEventTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CustomSystemEventTrigger";
}
impl ::std::convert::From<CustomSystemEventTrigger> for ::windows::runtime::IUnknown {
    fn from(value: CustomSystemEventTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CustomSystemEventTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &CustomSystemEventTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CustomSystemEventTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CustomSystemEventTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CustomSystemEventTrigger> for ::windows::runtime::IInspectable {
    fn from(value: CustomSystemEventTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomSystemEventTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &CustomSystemEventTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CustomSystemEventTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CustomSystemEventTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<CustomSystemEventTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CustomSystemEventTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CustomSystemEventTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CustomSystemEventTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for CustomSystemEventTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &CustomSystemEventTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CustomSystemEventTriggerRecurrence(pub i32);
impl CustomSystemEventTriggerRecurrence {
    pub const Once: CustomSystemEventTriggerRecurrence = CustomSystemEventTriggerRecurrence(0i32);
    pub const Always: CustomSystemEventTriggerRecurrence = CustomSystemEventTriggerRecurrence(1i32);
}
impl ::std::convert::From<i32> for CustomSystemEventTriggerRecurrence {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CustomSystemEventTriggerRecurrence {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CustomSystemEventTriggerRecurrence {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.Background.CustomSystemEventTriggerRecurrence;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DeviceConnectionChangeTrigger(::windows::runtime::IInspectable);
impl DeviceConnectionChangeTrigger {
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn CanMaintainConnection(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn MaintainConnection(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetMaintainConnection(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        deviceid: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<DeviceConnectionChangeTrigger>,
    > {
        Self::IDeviceConnectionChangeTriggerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                deviceid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<DeviceConnectionChangeTrigger>>(
                result__,
            )
        })
    }
    pub fn IDeviceConnectionChangeTriggerStatics<
        R,
        F: FnOnce(&IDeviceConnectionChangeTriggerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            DeviceConnectionChangeTrigger,
            IDeviceConnectionChangeTriggerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceConnectionChangeTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.DeviceConnectionChangeTrigger;{90875e64-3cdd-4efb-ab1c-5b3b6a60ce34})" ) ;
}
unsafe impl ::windows::runtime::Interface for DeviceConnectionChangeTrigger {
    type Vtable = IDeviceConnectionChangeTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2424790628,
        15581,
        20219,
        [171, 28, 91, 59, 106, 96, 206, 52],
    );
}
impl ::windows::runtime::RuntimeName for DeviceConnectionChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceConnectionChangeTrigger";
}
impl ::std::convert::From<DeviceConnectionChangeTrigger> for ::windows::runtime::IUnknown {
    fn from(value: DeviceConnectionChangeTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DeviceConnectionChangeTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceConnectionChangeTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for DeviceConnectionChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &DeviceConnectionChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DeviceConnectionChangeTrigger> for ::windows::runtime::IInspectable {
    fn from(value: DeviceConnectionChangeTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DeviceConnectionChangeTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceConnectionChangeTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for DeviceConnectionChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DeviceConnectionChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<DeviceConnectionChangeTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeviceConnectionChangeTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DeviceConnectionChangeTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeviceConnectionChangeTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for DeviceConnectionChangeTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &DeviceConnectionChangeTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for DeviceConnectionChangeTrigger {}
unsafe impl ::std::marker::Sync for DeviceConnectionChangeTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DeviceManufacturerNotificationTrigger(::windows::runtime::IInspectable);
impl DeviceManufacturerNotificationTrigger {
    #[cfg(feature = "deprecated")]
    pub fn TriggerQualifier(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn OneShot(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        triggerqualifier: Param0,
        oneshot: bool,
    ) -> ::windows::runtime::Result<DeviceManufacturerNotificationTrigger> {
        Self::IDeviceManufacturerNotificationTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                triggerqualifier.into_param().abi(),
                oneshot,
                &mut result__,
            )
            .from_abi::<DeviceManufacturerNotificationTrigger>(result__)
        })
    }
    pub fn IDeviceManufacturerNotificationTriggerFactory<
        R,
        F: FnOnce(&IDeviceManufacturerNotificationTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            DeviceManufacturerNotificationTrigger,
            IDeviceManufacturerNotificationTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceManufacturerNotificationTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.DeviceManufacturerNotificationTrigger;{81278ab5-41ab-16da-86c2-7f7bf0912f5b})" ) ;
}
unsafe impl ::windows::runtime::Interface for DeviceManufacturerNotificationTrigger {
    type Vtable = IDeviceManufacturerNotificationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2166852277,
        16811,
        5850,
        [134, 194, 127, 123, 240, 145, 47, 91],
    );
}
impl ::windows::runtime::RuntimeName for DeviceManufacturerNotificationTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.DeviceManufacturerNotificationTrigger";
}
impl ::std::convert::From<DeviceManufacturerNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: DeviceManufacturerNotificationTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DeviceManufacturerNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceManufacturerNotificationTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for DeviceManufacturerNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &DeviceManufacturerNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DeviceManufacturerNotificationTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: DeviceManufacturerNotificationTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DeviceManufacturerNotificationTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &DeviceManufacturerNotificationTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for DeviceManufacturerNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DeviceManufacturerNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<DeviceManufacturerNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeviceManufacturerNotificationTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DeviceManufacturerNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeviceManufacturerNotificationTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for DeviceManufacturerNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &DeviceManufacturerNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DeviceServicingTrigger(::windows::runtime::IInspectable);
impl DeviceServicingTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            DeviceServicingTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAsyncSimple<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>,
    >(
        &self,
        deviceid: Param0,
        expectedduration: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                deviceid.into_param().abi(),
                expectedduration.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAsyncWithArguments<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        deviceid: Param0,
        expectedduration: Param1,
        arguments: Param2,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                deviceid.into_param().abi(),
                expectedduration.into_param().abi(),
                arguments.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceServicingTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.DeviceServicingTrigger;{1ab217ad-6e34-49d3-9e6f-17f1b6dfa881})" ) ;
}
unsafe impl ::windows::runtime::Interface for DeviceServicingTrigger {
    type Vtable = IDeviceServicingTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        447879085,
        28212,
        18899,
        [158, 111, 23, 241, 182, 223, 168, 129],
    );
}
impl ::windows::runtime::RuntimeName for DeviceServicingTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceServicingTrigger";
}
impl ::std::convert::From<DeviceServicingTrigger> for ::windows::runtime::IUnknown {
    fn from(value: DeviceServicingTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DeviceServicingTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceServicingTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for DeviceServicingTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &DeviceServicingTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DeviceServicingTrigger> for ::windows::runtime::IInspectable {
    fn from(value: DeviceServicingTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DeviceServicingTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceServicingTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for DeviceServicingTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DeviceServicingTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<DeviceServicingTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeviceServicingTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DeviceServicingTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeviceServicingTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for DeviceServicingTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &DeviceServicingTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for DeviceServicingTrigger {}
unsafe impl ::std::marker::Sync for DeviceServicingTrigger {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DeviceTriggerResult(pub i32);
impl DeviceTriggerResult {
    pub const Allowed: DeviceTriggerResult = DeviceTriggerResult(0i32);
    pub const DeniedByUser: DeviceTriggerResult = DeviceTriggerResult(1i32);
    pub const DeniedBySystem: DeviceTriggerResult = DeviceTriggerResult(2i32);
    pub const LowBattery: DeviceTriggerResult = DeviceTriggerResult(3i32);
}
impl ::std::convert::From<i32> for DeviceTriggerResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeviceTriggerResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeviceTriggerResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.Background.DeviceTriggerResult;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DeviceUseTrigger(::windows::runtime::IInspectable);
impl DeviceUseTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            DeviceUseTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAsyncSimple<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        deviceid: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                deviceid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAsyncWithArguments<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        deviceid: Param0,
        arguments: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                deviceid.into_param().abi(),
                arguments.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceUseTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.DeviceUseTrigger;{0da68011-334f-4d57-b6ec-6dca64b412e4})" ) ;
}
unsafe impl ::windows::runtime::Interface for DeviceUseTrigger {
    type Vtable = IDeviceUseTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        229015569,
        13135,
        19799,
        [182, 236, 109, 202, 100, 180, 18, 228],
    );
}
impl ::windows::runtime::RuntimeName for DeviceUseTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceUseTrigger";
}
impl ::std::convert::From<DeviceUseTrigger> for ::windows::runtime::IUnknown {
    fn from(value: DeviceUseTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DeviceUseTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceUseTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceUseTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DeviceUseTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DeviceUseTrigger> for ::windows::runtime::IInspectable {
    fn from(value: DeviceUseTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DeviceUseTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceUseTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceUseTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DeviceUseTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<DeviceUseTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeviceUseTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DeviceUseTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeviceUseTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for DeviceUseTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &DeviceUseTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for DeviceUseTrigger {}
unsafe impl ::std::marker::Sync for DeviceUseTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DeviceWatcherTrigger(::windows::runtime::IInspectable);
impl DeviceWatcherTrigger {}
unsafe impl ::windows::runtime::RuntimeType for DeviceWatcherTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.DeviceWatcherTrigger;{a4617fdd-8573-4260-befc-5bec89cb693d})" ) ;
}
unsafe impl ::windows::runtime::Interface for DeviceWatcherTrigger {
    type Vtable = IDeviceWatcherTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2757853149,
        34163,
        16992,
        [190, 252, 91, 236, 137, 203, 105, 61],
    );
}
impl ::windows::runtime::RuntimeName for DeviceWatcherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceWatcherTrigger";
}
impl ::std::convert::From<DeviceWatcherTrigger> for ::windows::runtime::IUnknown {
    fn from(value: DeviceWatcherTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DeviceWatcherTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceWatcherTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceWatcherTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DeviceWatcherTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DeviceWatcherTrigger> for ::windows::runtime::IInspectable {
    fn from(value: DeviceWatcherTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DeviceWatcherTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceWatcherTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for DeviceWatcherTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DeviceWatcherTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<DeviceWatcherTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeviceWatcherTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DeviceWatcherTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeviceWatcherTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for DeviceWatcherTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &DeviceWatcherTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct EmailStoreNotificationTrigger(::windows::runtime::IInspectable);
impl EmailStoreNotificationTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            EmailStoreNotificationTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailStoreNotificationTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.EmailStoreNotificationTrigger;{986d06da-47eb-4268-a4f2-f3f77188388a})" ) ;
}
unsafe impl ::windows::runtime::Interface for EmailStoreNotificationTrigger {
    type Vtable = IEmailStoreNotificationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2557282010,
        18411,
        17000,
        [164, 242, 243, 247, 113, 136, 56, 138],
    );
}
impl ::windows::runtime::RuntimeName for EmailStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.EmailStoreNotificationTrigger";
}
impl ::std::convert::From<EmailStoreNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: EmailStoreNotificationTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailStoreNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &EmailStoreNotificationTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for EmailStoreNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &EmailStoreNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<EmailStoreNotificationTrigger> for ::windows::runtime::IInspectable {
    fn from(value: EmailStoreNotificationTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailStoreNotificationTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &EmailStoreNotificationTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for EmailStoreNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a EmailStoreNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<EmailStoreNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: EmailStoreNotificationTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&EmailStoreNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &EmailStoreNotificationTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for EmailStoreNotificationTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &EmailStoreNotificationTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for EmailStoreNotificationTrigger {}
unsafe impl ::std::marker::Sync for EmailStoreNotificationTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct GattCharacteristicNotificationTrigger(::windows::runtime::IInspectable);
impl GattCharacteristicNotificationTrigger {
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Characteristic(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::Devices::Bluetooth::GenericAttributeProfile:: GattCharacteristic > ( result__ )
        }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Create<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic,
        >,
    >(
        characteristic: Param0,
    ) -> ::windows::runtime::Result<GattCharacteristicNotificationTrigger> {
        Self::IGattCharacteristicNotificationTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                characteristic.into_param().abi(),
                &mut result__,
            )
            .from_abi::<GattCharacteristicNotificationTrigger>(result__)
        })
    }
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub fn EventTriggeringMode(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode,
    > {
        let this =
            &::windows::runtime::Interface::cast::<IGattCharacteristicNotificationTrigger2>(self)?;
        unsafe {
            let mut result__ : super::super::Devices::Bluetooth::Background:: BluetoothEventTriggeringMode = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode>(
                result__,
            )
        }
    }
    #[cfg(all(
        feature = "Devices_Bluetooth_Background",
        feature = "Devices_Bluetooth_GenericAttributeProfile"
    ))]
    pub fn CreateWithEventTriggeringMode<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic,
        >,
    >(
        characteristic: Param0,
        eventtriggeringmode : super::super::Devices::Bluetooth::Background:: BluetoothEventTriggeringMode,
    ) -> ::windows::runtime::Result<GattCharacteristicNotificationTrigger> {
        Self::IGattCharacteristicNotificationTriggerFactory2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                characteristic.into_param().abi(),
                eventtriggeringmode,
                &mut result__,
            )
            .from_abi::<GattCharacteristicNotificationTrigger>(result__)
        })
    }
    pub fn IGattCharacteristicNotificationTriggerFactory<
        R,
        F: FnOnce(&IGattCharacteristicNotificationTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            GattCharacteristicNotificationTrigger,
            IGattCharacteristicNotificationTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGattCharacteristicNotificationTriggerFactory2<
        R,
        F: FnOnce(&IGattCharacteristicNotificationTriggerFactory2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            GattCharacteristicNotificationTrigger,
            IGattCharacteristicNotificationTriggerFactory2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GattCharacteristicNotificationTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.GattCharacteristicNotificationTrigger;{e25f8fc8-0696-474f-a732-f292b0cebc5d})" ) ;
}
unsafe impl ::windows::runtime::Interface for GattCharacteristicNotificationTrigger {
    type Vtable = IGattCharacteristicNotificationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3797913544,
        1686,
        18255,
        [167, 50, 242, 146, 176, 206, 188, 93],
    );
}
impl ::windows::runtime::RuntimeName for GattCharacteristicNotificationTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.GattCharacteristicNotificationTrigger";
}
impl ::std::convert::From<GattCharacteristicNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: GattCharacteristicNotificationTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GattCharacteristicNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &GattCharacteristicNotificationTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for GattCharacteristicNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &GattCharacteristicNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<GattCharacteristicNotificationTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: GattCharacteristicNotificationTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GattCharacteristicNotificationTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &GattCharacteristicNotificationTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for GattCharacteristicNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a GattCharacteristicNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<GattCharacteristicNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: GattCharacteristicNotificationTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&GattCharacteristicNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &GattCharacteristicNotificationTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for GattCharacteristicNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &GattCharacteristicNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for GattCharacteristicNotificationTrigger {}
unsafe impl ::std::marker::Sync for GattCharacteristicNotificationTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct GattServiceProviderTrigger(::windows::runtime::IInspectable);
impl GattServiceProviderTrigger {
    pub fn TriggerId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Service(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Devices::Bluetooth::GenericAttributeProfile::GattLocalService,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .7 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::Devices::Bluetooth::GenericAttributeProfile:: GattLocalService > ( result__ )
        }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]    pub fn SetAdvertisingParameters < 'a , Param0 : :: windows :: runtime :: IntoParam < 'a , super::super::Devices::Bluetooth::GenericAttributeProfile:: GattServiceProviderAdvertisingParameters > , > ( & self , value : Param0 , ) -> :: windows :: runtime :: Result < ( ) >{
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]    pub fn AdvertisingParameters < > ( & self , ) -> :: windows :: runtime :: Result < super::super::Devices::Bluetooth::GenericAttributeProfile:: GattServiceProviderAdvertisingParameters >{
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .9 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::Devices::Bluetooth::GenericAttributeProfile:: GattServiceProviderAdvertisingParameters > ( result__ )
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        triggerid: Param0,
        serviceuuid: Param1,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<GattServiceProviderTriggerResult>,
    > {
        Self::IGattServiceProviderTriggerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , triggerid . into_param ( ) . abi ( ) , serviceuuid . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < GattServiceProviderTriggerResult > > ( result__ )
        })
    }
    pub fn IGattServiceProviderTriggerStatics<
        R,
        F: FnOnce(&IGattServiceProviderTriggerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            GattServiceProviderTrigger,
            IGattServiceProviderTriggerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GattServiceProviderTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.GattServiceProviderTrigger;{ddc6a3e9-1557-4bd8-8542-468aa0c696f6})" ) ;
}
unsafe impl ::windows::runtime::Interface for GattServiceProviderTrigger {
    type Vtable = IGattServiceProviderTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3720782825,
        5463,
        19416,
        [133, 66, 70, 138, 160, 198, 150, 246],
    );
}
impl ::windows::runtime::RuntimeName for GattServiceProviderTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GattServiceProviderTrigger";
}
impl ::std::convert::From<GattServiceProviderTrigger> for ::windows::runtime::IUnknown {
    fn from(value: GattServiceProviderTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GattServiceProviderTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &GattServiceProviderTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for GattServiceProviderTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &GattServiceProviderTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<GattServiceProviderTrigger> for ::windows::runtime::IInspectable {
    fn from(value: GattServiceProviderTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GattServiceProviderTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &GattServiceProviderTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for GattServiceProviderTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a GattServiceProviderTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<GattServiceProviderTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: GattServiceProviderTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&GattServiceProviderTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &GattServiceProviderTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for GattServiceProviderTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &GattServiceProviderTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for GattServiceProviderTrigger {}
unsafe impl ::std::marker::Sync for GattServiceProviderTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct GattServiceProviderTriggerResult(::windows::runtime::IInspectable);
impl GattServiceProviderTriggerResult {
    pub fn Trigger(&self) -> ::windows::runtime::Result<GattServiceProviderTrigger> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<GattServiceProviderTrigger>(result__)
        }
    }
    #[cfg(feature = "Devices_Bluetooth")]
    pub fn Error(
        &self,
    ) -> ::windows::runtime::Result<super::super::Devices::Bluetooth::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Bluetooth::BluetoothError =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Devices::Bluetooth::BluetoothError>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GattServiceProviderTriggerResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.GattServiceProviderTriggerResult;{3c4691b1-b198-4e84-bad4-cf4ad299ed3a})" ) ;
}
unsafe impl ::windows::runtime::Interface for GattServiceProviderTriggerResult {
    type Vtable = IGattServiceProviderTriggerResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1011257777,
        45464,
        20100,
        [186, 212, 207, 74, 210, 153, 237, 58],
    );
}
impl ::windows::runtime::RuntimeName for GattServiceProviderTriggerResult {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.GattServiceProviderTriggerResult";
}
impl ::std::convert::From<GattServiceProviderTriggerResult> for ::windows::runtime::IUnknown {
    fn from(value: GattServiceProviderTriggerResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GattServiceProviderTriggerResult> for ::windows::runtime::IUnknown {
    fn from(value: &GattServiceProviderTriggerResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for GattServiceProviderTriggerResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &GattServiceProviderTriggerResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<GattServiceProviderTriggerResult> for ::windows::runtime::IInspectable {
    fn from(value: GattServiceProviderTriggerResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GattServiceProviderTriggerResult> for ::windows::runtime::IInspectable {
    fn from(value: &GattServiceProviderTriggerResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for GattServiceProviderTriggerResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a GattServiceProviderTriggerResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GattServiceProviderTriggerResult {}
unsafe impl ::std::marker::Sync for GattServiceProviderTriggerResult {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct GeovisitTrigger(::windows::runtime::IInspectable);
impl GeovisitTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            GeovisitTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Devices_Geolocation")]
    pub fn MonitoringScope(
        &self,
    ) -> ::windows::runtime::Result<super::super::Devices::Geolocation::VisitMonitoringScope> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Geolocation::VisitMonitoringScope =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Devices::Geolocation::VisitMonitoringScope>(result__)
        }
    }
    #[cfg(feature = "Devices_Geolocation")]
    pub fn SetMonitoringScope(
        &self,
        value: super::super::Devices::Geolocation::VisitMonitoringScope,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GeovisitTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.GeovisitTrigger;{4818edaa-04e1-4127-9a4c-19351b8a80a4})" ) ;
}
unsafe impl ::windows::runtime::Interface for GeovisitTrigger {
    type Vtable = IGeovisitTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1209593258,
        1249,
        16679,
        [154, 76, 25, 53, 27, 138, 128, 164],
    );
}
impl ::windows::runtime::RuntimeName for GeovisitTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GeovisitTrigger";
}
impl ::std::convert::From<GeovisitTrigger> for ::windows::runtime::IUnknown {
    fn from(value: GeovisitTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GeovisitTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &GeovisitTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GeovisitTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GeovisitTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<GeovisitTrigger> for ::windows::runtime::IInspectable {
    fn from(value: GeovisitTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GeovisitTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &GeovisitTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GeovisitTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a GeovisitTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<GeovisitTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: GeovisitTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&GeovisitTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &GeovisitTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for GeovisitTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &GeovisitTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for GeovisitTrigger {}
unsafe impl ::std::marker::Sync for GeovisitTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IActivitySensorTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IActivitySensorTrigger {
    type Vtable = IActivitySensorTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3504161602,
        58235,
        18467,
        [165, 254, 107, 49, 223, 239, 222, 176],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Sensors", feature = "Foundation_Collections")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Sensors", feature = "Foundation_Collections")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IActivitySensorTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IActivitySensorTriggerFactory {
    type Vtable = IActivitySensorTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2804322755,
        14391,
        17655,
        [131, 27, 1, 50, 204, 135, 43, 195],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reportintervalinmilliseconds: u32,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAlarmApplicationManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAlarmApplicationManagerStatics {
    type Vtable = IAlarmApplicationManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3389258299,
        52454,
        19938,
        [176, 155, 150, 40, 189, 51, 187, 190],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlarmApplicationManagerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut AlarmAccessStatus,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAppBroadcastTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastTrigger {
    type Vtable = IAppBroadcastTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1960113302,
        36151,
        17644,
        [148, 129, 42, 11, 152, 84, 235, 72],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastTriggerFactory {
    type Vtable = IAppBroadcastTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        671850308,
        8948,
        17944,
        [160, 46, 231, 228, 17, 235, 114, 56],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        providerkey: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerProviderInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastTriggerProviderInfo {
    type Vtable = IAppBroadcastTriggerProviderInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4061738285,
        40424,
        17440,
        [156, 226, 94, 255, 143, 23, 55, 107],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerProviderInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IApplicationTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IApplicationTrigger {
    type Vtable = IApplicationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        189171248,
        38260,
        18732,
        [158, 147, 26, 58, 230, 51, 95, 233],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        arguments: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IApplicationTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IApplicationTriggerDetails {
    type Vtable = IApplicationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2547804850,
        8729,
        19102,
        [156, 94, 65, 208, 71, 247, 110, 130],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationTriggerDetails_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAppointmentStoreNotificationTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentStoreNotificationTrigger {
    type Vtable = IAppointmentStoreNotificationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1691616268,
        49665,
        17069,
        [170, 42, 226, 27, 163, 66, 91, 109],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreNotificationTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IBackgroundCondition(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundCondition {
    type Vtable = IBackgroundCondition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2923995630,
        35153,
        16394,
        [131, 2, 156, 156, 154, 42, 58, 59],
    );
}
impl IBackgroundCondition {}
unsafe impl ::windows::runtime::RuntimeType for IBackgroundCondition {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{ae48a1ee-8951-400a-8302-9c9c9a2a3a3b}");
}
impl ::std::convert::From<IBackgroundCondition> for ::windows::runtime::IUnknown {
    fn from(value: IBackgroundCondition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBackgroundCondition> for ::windows::runtime::IUnknown {
    fn from(value: &IBackgroundCondition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBackgroundCondition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBackgroundCondition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IBackgroundCondition> for ::windows::runtime::IInspectable {
    fn from(value: IBackgroundCondition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBackgroundCondition> for ::windows::runtime::IInspectable {
    fn from(value: &IBackgroundCondition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IBackgroundCondition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IBackgroundCondition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCondition_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBackgroundExecutionManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundExecutionManagerStatics {
    type Vtable = IBackgroundExecutionManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3894864472,
        26281,
        19777,
        [131, 212, 180, 193, 140, 135, 184, 70],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundExecutionManagerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut BackgroundAccessStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut BackgroundAccessStatus,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBackgroundExecutionManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundExecutionManagerStatics2 {
    type Vtable = IBackgroundExecutionManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1184572655,
        39867,
        19992,
        [153, 154, 253, 101, 18, 147, 27, 233],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundExecutionManagerStatics2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        requestedaccess: BackgroundAccessRequestKind,
        reason: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBackgroundExecutionManagerStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundExecutionManagerStatics3 {
    type Vtable = IBackgroundExecutionManagerStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2561004534,
        23077,
        23404,
        [145, 146, 215, 122, 67, 223, 237, 196],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundExecutionManagerStatics3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        requestedaccess: BackgroundAccessRequestKind,
        reason: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut BackgroundAccessStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut BackgroundAccessStatus,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IBackgroundTask(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTask {
    type Vtable = IBackgroundTask_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2098451764,
        64786,
        17358,
        [140, 34, 234, 31, 241, 60, 6, 223],
    );
}
impl IBackgroundTask {
    pub fn Run<'a, Param0: ::windows::runtime::IntoParam<'a, IBackgroundTaskInstance>>(
        &self,
        taskinstance: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                taskinstance.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBackgroundTask {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{7d13d534-fd12-43ce-8c22-ea1ff13c06df}");
}
impl ::std::convert::From<IBackgroundTask> for ::windows::runtime::IUnknown {
    fn from(value: IBackgroundTask) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBackgroundTask> for ::windows::runtime::IUnknown {
    fn from(value: &IBackgroundTask) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBackgroundTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBackgroundTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IBackgroundTask> for ::windows::runtime::IInspectable {
    fn from(value: IBackgroundTask) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBackgroundTask> for ::windows::runtime::IInspectable {
    fn from(value: &IBackgroundTask) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IBackgroundTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IBackgroundTask
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTask_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        taskinstance: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskBuilder {
    type Vtable = IBackgroundTaskBuilder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        55661838,
        15972,
        17778,
        [169, 58, 132, 7, 90, 55, 201, 23],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        trigger: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        condition: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskBuilder2 {
    type Vtable = IBackgroundTaskBuilder2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1793576881,
        4175,
        16493,
        [141, 182, 132, 74, 87, 15, 66, 187],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskBuilder3 {
    type Vtable = IBackgroundTaskBuilder3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        684150602,
        35753,
        19465,
        [162, 79, 25, 104, 62, 44, 146, 76],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskBuilder4 {
    type Vtable = IBackgroundTaskBuilder4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1196811554,
        52130,
        20021,
        [189, 22, 166, 218, 127, 28, 25, 170],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder4_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskBuilder5 {
    type Vtable = IBackgroundTaskBuilder5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        124847094,
        39413,
        19188,
        [188, 173, 71, 49, 208, 51, 13, 67],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder5_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        taskentrypoint: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBackgroundTaskCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskCompletedEventArgs {
    type Vtable = IBackgroundTaskCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1448945103,
        61961,
        18676,
        [153, 103, 43, 24, 79, 123, 251, 240],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskCompletedEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBackgroundTaskDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskDeferral {
    type Vtable = IBackgroundTaskDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2479625581,
        44839,
        19923,
        [132, 110, 36, 238, 64, 202, 221, 37],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskDeferral_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IBackgroundTaskInstance(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskInstance {
    type Vtable = IBackgroundTaskInstance_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2254166650,
        8664,
        17779,
        [143, 50, 146, 138, 27, 6, 65, 246],
    );
}
impl IBackgroundTaskInstance {
    pub fn InstanceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    pub fn Task(&self) -> ::windows::runtime::Result<BackgroundTaskRegistration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BackgroundTaskRegistration>(result__)
        }
    }
    pub fn Progress(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetProgress(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TriggerDetails(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Canceled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, BackgroundTaskCanceledEventHandler>,
    >(
        &self,
        cancelhandler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                cancelhandler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanceled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SuspendedCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<BackgroundTaskDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BackgroundTaskDeferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBackgroundTaskInstance {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{865bda7a-21d8-4573-8f32-928a1b0641f6}");
}
impl ::std::convert::From<IBackgroundTaskInstance> for ::windows::runtime::IUnknown {
    fn from(value: IBackgroundTaskInstance) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBackgroundTaskInstance> for ::windows::runtime::IUnknown {
    fn from(value: &IBackgroundTaskInstance) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IBackgroundTaskInstance
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IBackgroundTaskInstance
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IBackgroundTaskInstance> for ::windows::runtime::IInspectable {
    fn from(value: IBackgroundTaskInstance) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBackgroundTaskInstance> for ::windows::runtime::IInspectable {
    fn from(value: &IBackgroundTaskInstance) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IBackgroundTaskInstance
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IBackgroundTaskInstance
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskInstance_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cancelhandler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IBackgroundTaskInstance2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskInstance2 {
    type Vtable = IBackgroundTaskInstance2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1333592438,
        3190,
        20404,
        [137, 109, 93, 225, 134, 65, 34, 246],
    );
}
impl IBackgroundTaskInstance2 {
    pub fn GetThrottleCount(
        &self,
        counter: BackgroundTaskThrottleCounter,
    ) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                counter,
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    pub fn Task(&self) -> ::windows::runtime::Result<BackgroundTaskRegistration> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BackgroundTaskRegistration>(result__)
        }
    }
    pub fn Progress(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetProgress(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TriggerDetails(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Canceled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, BackgroundTaskCanceledEventHandler>,
    >(
        &self,
        cancelhandler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                cancelhandler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanceled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SuspendedCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<BackgroundTaskDeferral> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BackgroundTaskDeferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBackgroundTaskInstance2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{4f7d0176-0c76-4fb4-896d-5de1864122f6}");
}
impl ::std::convert::From<IBackgroundTaskInstance2> for ::windows::runtime::IUnknown {
    fn from(value: IBackgroundTaskInstance2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBackgroundTaskInstance2> for ::windows::runtime::IUnknown {
    fn from(value: &IBackgroundTaskInstance2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IBackgroundTaskInstance2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IBackgroundTaskInstance2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IBackgroundTaskInstance2> for ::windows::runtime::IInspectable {
    fn from(value: IBackgroundTaskInstance2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBackgroundTaskInstance2> for ::windows::runtime::IInspectable {
    fn from(value: &IBackgroundTaskInstance2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IBackgroundTaskInstance2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IBackgroundTaskInstance2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<IBackgroundTaskInstance2> for IBackgroundTaskInstance {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IBackgroundTaskInstance2) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IBackgroundTaskInstance2> for IBackgroundTaskInstance {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IBackgroundTaskInstance2) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTaskInstance> for IBackgroundTaskInstance2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTaskInstance> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTaskInstance> for &IBackgroundTaskInstance2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTaskInstance> {
        ::std::convert::TryInto::<IBackgroundTaskInstance>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskInstance2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        counter: BackgroundTaskThrottleCounter,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IBackgroundTaskInstance4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskInstance4 {
    type Vtable = IBackgroundTaskInstance4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2133455420,
        43524,
        19208,
        [151, 176, 6, 216, 116, 205, 171, 245],
    );
}
impl IBackgroundTaskInstance4 {
    pub fn InstanceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    pub fn Task(&self) -> ::windows::runtime::Result<BackgroundTaskRegistration> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BackgroundTaskRegistration>(result__)
        }
    }
    pub fn Progress(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetProgress(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TriggerDetails(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Canceled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, BackgroundTaskCanceledEventHandler>,
    >(
        &self,
        cancelhandler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                cancelhandler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanceled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SuspendedCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<BackgroundTaskDeferral> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BackgroundTaskDeferral>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBackgroundTaskInstance4 {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{7f29f23c-aa04-4b08-97b0-06d874cdabf5}");
}
impl ::std::convert::From<IBackgroundTaskInstance4> for ::windows::runtime::IUnknown {
    fn from(value: IBackgroundTaskInstance4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBackgroundTaskInstance4> for ::windows::runtime::IUnknown {
    fn from(value: &IBackgroundTaskInstance4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IBackgroundTaskInstance4
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IBackgroundTaskInstance4
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IBackgroundTaskInstance4> for ::windows::runtime::IInspectable {
    fn from(value: IBackgroundTaskInstance4) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBackgroundTaskInstance4> for ::windows::runtime::IInspectable {
    fn from(value: &IBackgroundTaskInstance4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IBackgroundTaskInstance4
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IBackgroundTaskInstance4
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<IBackgroundTaskInstance4> for IBackgroundTaskInstance {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IBackgroundTaskInstance4) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IBackgroundTaskInstance4> for IBackgroundTaskInstance {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IBackgroundTaskInstance4) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTaskInstance> for IBackgroundTaskInstance4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTaskInstance> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTaskInstance> for &IBackgroundTaskInstance4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTaskInstance> {
        ::std::convert::TryInto::<IBackgroundTaskInstance>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskInstance4_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBackgroundTaskProgressEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskProgressEventArgs {
    type Vtable = IBackgroundTaskProgressEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4212418732,
        33586,
        19722,
        [149, 50, 3, 234, 230, 132, 218, 49],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskProgressEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IBackgroundTaskRegistration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskRegistration {
    type Vtable = IBackgroundTaskRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        275074242,
        41582,
        17343,
        [140, 18, 31, 180, 13, 191, 191, 160],
    );
}
impl IBackgroundTaskRegistration {
    pub fn TaskId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Progress<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, BackgroundTaskProgressEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgress<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Completed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, BackgroundTaskCompletedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Unregister(&self, canceltask: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                canceltask,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBackgroundTaskRegistration {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{10654cc2-a26e-43bf-8c12-1fb40dbfbfa0}");
}
impl ::std::convert::From<IBackgroundTaskRegistration> for ::windows::runtime::IUnknown {
    fn from(value: IBackgroundTaskRegistration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBackgroundTaskRegistration> for ::windows::runtime::IUnknown {
    fn from(value: &IBackgroundTaskRegistration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IBackgroundTaskRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IBackgroundTaskRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IBackgroundTaskRegistration> for ::windows::runtime::IInspectable {
    fn from(value: IBackgroundTaskRegistration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBackgroundTaskRegistration> for ::windows::runtime::IInspectable {
    fn from(value: &IBackgroundTaskRegistration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IBackgroundTaskRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IBackgroundTaskRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistration_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        canceltask: bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IBackgroundTaskRegistration2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskRegistration2 {
    type Vtable = IBackgroundTaskRegistration2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1631110915,
        48006,
        16658,
        [175, 195, 127, 147, 155, 22, 110, 59],
    );
}
impl IBackgroundTaskRegistration2 {
    pub fn Trigger(&self) -> ::windows::runtime::Result<IBackgroundTrigger> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IBackgroundTrigger>(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Progress<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, BackgroundTaskProgressEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgress<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Completed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, BackgroundTaskCompletedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Unregister(&self, canceltask: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                canceltask,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBackgroundTaskRegistration2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{6138c703-bb86-4112-afc3-7f939b166e3b}");
}
impl ::std::convert::From<IBackgroundTaskRegistration2> for ::windows::runtime::IUnknown {
    fn from(value: IBackgroundTaskRegistration2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBackgroundTaskRegistration2> for ::windows::runtime::IUnknown {
    fn from(value: &IBackgroundTaskRegistration2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IBackgroundTaskRegistration2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IBackgroundTaskRegistration2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IBackgroundTaskRegistration2> for ::windows::runtime::IInspectable {
    fn from(value: IBackgroundTaskRegistration2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBackgroundTaskRegistration2> for ::windows::runtime::IInspectable {
    fn from(value: &IBackgroundTaskRegistration2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IBackgroundTaskRegistration2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IBackgroundTaskRegistration2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<IBackgroundTaskRegistration2> for IBackgroundTaskRegistration {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IBackgroundTaskRegistration2) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IBackgroundTaskRegistration2> for IBackgroundTaskRegistration {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IBackgroundTaskRegistration2) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTaskRegistration>
    for IBackgroundTaskRegistration2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTaskRegistration> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTaskRegistration>
    for &IBackgroundTaskRegistration2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTaskRegistration> {
        ::std::convert::TryInto::<IBackgroundTaskRegistration>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistration2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IBackgroundTaskRegistration3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskRegistration3 {
    type Vtable = IBackgroundTaskRegistration3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4264788373,
        37923,
        19851,
        [131, 13, 177, 221, 44, 123, 173, 213],
    );
}
impl IBackgroundTaskRegistration3 {
    pub fn TaskId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Progress<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, BackgroundTaskProgressEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgress<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Completed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, BackgroundTaskCompletedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Unregister(&self, canceltask: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                canceltask,
            )
            .ok()
        }
    }
    pub fn TaskGroup(&self) -> ::windows::runtime::Result<BackgroundTaskRegistrationGroup> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BackgroundTaskRegistrationGroup>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBackgroundTaskRegistration3 {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{fe338195-9423-4d8b-830d-b1dd2c7badd5}");
}
impl ::std::convert::From<IBackgroundTaskRegistration3> for ::windows::runtime::IUnknown {
    fn from(value: IBackgroundTaskRegistration3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBackgroundTaskRegistration3> for ::windows::runtime::IUnknown {
    fn from(value: &IBackgroundTaskRegistration3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IBackgroundTaskRegistration3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IBackgroundTaskRegistration3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IBackgroundTaskRegistration3> for ::windows::runtime::IInspectable {
    fn from(value: IBackgroundTaskRegistration3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBackgroundTaskRegistration3> for ::windows::runtime::IInspectable {
    fn from(value: &IBackgroundTaskRegistration3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IBackgroundTaskRegistration3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IBackgroundTaskRegistration3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<IBackgroundTaskRegistration3> for IBackgroundTaskRegistration {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IBackgroundTaskRegistration3) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IBackgroundTaskRegistration3> for IBackgroundTaskRegistration {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IBackgroundTaskRegistration3) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTaskRegistration>
    for IBackgroundTaskRegistration3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTaskRegistration> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTaskRegistration>
    for &IBackgroundTaskRegistration3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTaskRegistration> {
        ::std::convert::TryInto::<IBackgroundTaskRegistration>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistration3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationGroup(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskRegistrationGroup {
    type Vtable = IBackgroundTaskRegistrationGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        716280218,
        34587,
        16743,
        [138, 118, 5, 92, 214, 123, 91, 35],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationGroup_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationGroupFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskRegistrationGroupFactory {
    type Vtable = IBackgroundTaskRegistrationGroupFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2212047721,
        17615,
        17969,
        [151, 64, 3, 199, 216, 116, 27, 197],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationGroupFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskRegistrationStatics {
    type Vtable = IBackgroundTaskRegistrationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1280585577,
        45056,
        17082,
        [160, 147, 106, 86, 60, 101, 227, 248],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTaskRegistrationStatics2 {
    type Vtable = IBackgroundTaskRegistrationStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        390817566,
        45581,
        20393,
        [173, 154, 233, 58, 214, 199, 30, 1],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationStatics2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        groupid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IBackgroundTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl IBackgroundTrigger {}
unsafe impl ::windows::runtime::RuntimeType for IBackgroundTrigger {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{84b3a058-6027-4b87-9790-bdf3f757dbd7}");
}
impl ::std::convert::From<IBackgroundTrigger> for ::windows::runtime::IUnknown {
    fn from(value: IBackgroundTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBackgroundTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &IBackgroundTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBackgroundTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBackgroundTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IBackgroundTrigger> for ::windows::runtime::IInspectable {
    fn from(value: IBackgroundTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBackgroundTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &IBackgroundTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IBackgroundTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IBackgroundTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBackgroundWorkCostStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundWorkCostStatics {
    type Vtable = IBackgroundWorkCostStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3342902882,
        49936,
        19330,
        [179, 227, 59, 207, 185, 228, 199, 125],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundWorkCostStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut BackgroundWorkCostValue,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementPublisherTrigger {
    type Vtable = IBluetoothLEAdvertisementPublisherTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2872976914,
        9683,
        18606,
        [135, 36, 216, 24, 119, 174, 97, 41],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTrigger2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementPublisherTrigger2 {
    type Vtable = IBluetoothLEAdvertisementPublisherTrigger2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2854801508,
        14580,
        22909,
        [181, 151, 78, 85, 88, 140, 101, 3],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTrigger2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementWatcherTrigger {
    type Vtable = IBluetoothLEAdvertisementWatcherTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        447420441,
        48353,
        18667,
        [168, 39, 89, 251, 124, 238, 82, 166],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Devices_Bluetooth")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))] usize,
    #[cfg(feature = "Devices_Bluetooth")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))] usize,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))] usize,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherTrigger2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementWatcherTrigger2 {
    type Vtable = IBluetoothLEAdvertisementWatcherTrigger2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        968189849,
        60217,
        23222,
        [153, 50, 170, 158, 69, 73, 96, 77],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherTrigger2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICachedFileUpdaterTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICachedFileUpdaterTrigger {
    type Vtable = ICachedFileUpdaterTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3793530603,
        13042,
        19761,
        [181, 83, 185, 224, 27, 222, 55, 224],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICachedFileUpdaterTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICachedFileUpdaterTriggerDetails {
    type Vtable = ICachedFileUpdaterTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1904446483,
        4884,
        18356,
        [149, 151, 220, 126, 36, 140, 23, 204],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterTriggerDetails_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Provider")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Storage::Provider::CachedFileTarget,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))] usize,
    #[cfg(feature = "Storage_Provider")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IChatMessageNotificationTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageNotificationTrigger {
    type Vtable = IChatMessageNotificationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1362838463,
        7488,
        23645,
        [120, 245, 201, 35, 254, 227, 115, 158],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageNotificationTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IChatMessageReceivedNotificationTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageReceivedNotificationTrigger {
    type Vtable = IChatMessageReceivedNotificationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1050899982,
        47861,
        16503,
        [136, 233, 6, 12, 246, 240, 198, 213],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageReceivedNotificationTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppSetAsActiveTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICommunicationBlockingAppSetAsActiveTrigger {
    type Vtable = ICommunicationBlockingAppSetAsActiveTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4220646026,
        5797,
        18541,
        [151, 76, 120, 53, 168, 71, 123, 226],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppSetAsActiveTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IContactStoreNotificationTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactStoreNotificationTrigger {
    type Vtable = IContactStoreNotificationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3358802331,
        18181,
        17777,
        [154, 22, 6, 185, 151, 191, 156, 150],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStoreNotificationTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IContentPrefetchTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContentPrefetchTrigger {
    type Vtable = IContentPrefetchTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1896228846,
        1274,
        17419,
        [128, 192, 23, 50, 2, 25, 158, 93],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentPrefetchTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IContentPrefetchTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContentPrefetchTriggerFactory {
    type Vtable = IContentPrefetchTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3261349594,
        35331,
        16542,
        [184, 196, 136, 129, 76, 40, 204, 182],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentPrefetchTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        waitinterval: super::super::Foundation::TimeSpan,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICustomSystemEventTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomSystemEventTrigger {
    type Vtable = ICustomSystemEventTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4082722712,
        53099,
        20212,
        [160, 202, 41, 207, 74, 39, 140, 135],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSystemEventTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut CustomSystemEventTriggerRecurrence,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICustomSystemEventTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomSystemEventTriggerFactory {
    type Vtable = ICustomSystemEventTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1808471749,
        62172,
        16818,
        [158, 253, 185, 107, 220, 209, 60, 237],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSystemEventTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        triggerid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        recurrence: CustomSystemEventTriggerRecurrence,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDeviceConnectionChangeTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceConnectionChangeTrigger {
    type Vtable = IDeviceConnectionChangeTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2424790628,
        15581,
        20219,
        [171, 28, 91, 59, 106, 96, 206, 52],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceConnectionChangeTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDeviceConnectionChangeTriggerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceConnectionChangeTriggerStatics {
    type Vtable = IDeviceConnectionChangeTriggerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3286901866,
        20221,
        17560,
        [170, 96, 164, 228, 227, 177, 122, 185],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceConnectionChangeTriggerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDeviceManufacturerNotificationTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceManufacturerNotificationTrigger {
    type Vtable = IDeviceManufacturerNotificationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2166852277,
        16811,
        5850,
        [134, 194, 127, 123, 240, 145, 47, 91],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceManufacturerNotificationTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDeviceManufacturerNotificationTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceManufacturerNotificationTriggerFactory {
    type Vtable = IDeviceManufacturerNotificationTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2035670645,
        9659,
        16723,
        [161, 162, 48, 41, 252, 171, 182, 82],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceManufacturerNotificationTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        triggerqualifier: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        oneshot: bool,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDeviceServicingTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceServicingTrigger {
    type Vtable = IDeviceServicingTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        447879085,
        28212,
        18899,
        [158, 111, 23, 241, 182, 223, 168, 129],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceServicingTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        expectedduration: super::super::Foundation::TimeSpan,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        expectedduration: super::super::Foundation::TimeSpan,
        arguments: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDeviceUseTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceUseTrigger {
    type Vtable = IDeviceUseTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        229015569,
        13135,
        19799,
        [182, 236, 109, 202, 100, 180, 18, 228],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceUseTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        arguments: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDeviceWatcherTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceWatcherTrigger {
    type Vtable = IDeviceWatcherTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2757853149,
        34163,
        16992,
        [190, 252, 91, 236, 137, 203, 105, 61],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcherTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IEmailStoreNotificationTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailStoreNotificationTrigger {
    type Vtable = IEmailStoreNotificationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2557282010,
        18411,
        17000,
        [164, 242, 243, 247, 113, 136, 56, 138],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailStoreNotificationTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGattCharacteristicNotificationTrigger {
    type Vtable = IGattCharacteristicNotificationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3797913544,
        1686,
        18255,
        [167, 50, 242, 146, 176, 206, 188, 93],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTrigger2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGattCharacteristicNotificationTrigger2 {
    type Vtable = IGattCharacteristicNotificationTrigger2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2468520644,
        44558,
        17138,
        [178, 140, 245, 19, 114, 230, 146, 69],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTrigger2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGattCharacteristicNotificationTriggerFactory {
    type Vtable = IGattCharacteristicNotificationTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1471814037,
        45379,
        17781,
        [159, 107, 253, 89, 217, 58, 206, 26],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        characteristic: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerFactory2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGattCharacteristicNotificationTriggerFactory2 {
    type Vtable = IGattCharacteristicNotificationTriggerFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1503193375,
        35411,
        20127,
        [163, 44, 35, 205, 51, 102, 76, 238],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerFactory2_abi ( pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , iid : & :: windows :: runtime :: GUID , interface : * mut :: windows :: runtime :: RawPtr ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr ) -> u32 , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr ) -> u32 , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , count : * mut u32 , values : * mut * mut :: windows :: runtime :: GUID ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , value : * mut :: windows :: runtime :: RawPtr ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , value : * mut i32 ) -> :: windows :: runtime :: HRESULT , #[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile"))] pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , characteristic : :: windows :: runtime :: RawPtr , eventtriggeringmode : super::super::Devices::Bluetooth::Background:: BluetoothEventTriggeringMode , result__ : * mut :: windows :: runtime :: RawPtr ) -> :: windows :: runtime :: HRESULT , #[cfg(not(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile")))] usize , ) where ;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IGattServiceProviderTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGattServiceProviderTrigger {
    type Vtable = IGattServiceProviderTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3720782825,
        5463,
        19416,
        [133, 66, 70, 138, 160, 198, 150, 246],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))] usize,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))] usize,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGattServiceProviderTriggerResult {
    type Vtable = IGattServiceProviderTriggerResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1011257777,
        45464,
        20100,
        [186, 212, 207, 74, 210, 153, 237, 58],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Bluetooth")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Devices::Bluetooth::BluetoothError,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGattServiceProviderTriggerStatics {
    type Vtable = IGattServiceProviderTriggerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3021185898,
        58004,
        17809,
        [165, 166, 100, 137, 26, 130, 129, 83],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        triggerid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        serviceuuid: ::windows::runtime::GUID,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IGeovisitTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeovisitTrigger {
    type Vtable = IGeovisitTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1209593258,
        1249,
        16679,
        [154, 76, 25, 53, 27, 138, 128, 164],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Devices::Geolocation::VisitMonitoringScope,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::Devices::Geolocation::VisitMonitoringScope,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ILocationTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILocationTrigger {
    type Vtable = ILocationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1197894172,
        26743,
        18462,
        [128, 38, 255, 126, 20, 168, 17, 160],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut LocationTriggerType,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ILocationTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILocationTriggerFactory {
    type Vtable = ILocationTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        285653767,
        65385,
        19977,
        [170, 139, 19, 132, 234, 71, 94, 152],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        triggertype: LocationTriggerType,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IMaintenanceTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMaintenanceTrigger {
    type Vtable = IMaintenanceTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1746422915,
        64546,
        19685,
        [132, 26, 114, 57, 169, 129, 0, 71],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMaintenanceTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IMaintenanceTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMaintenanceTriggerFactory {
    type Vtable = IMaintenanceTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1262345006,
        38877,
        17961,
        [136, 176, 176, 108, 249, 72, 42, 229],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMaintenanceTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        freshnesstime: u32,
        oneshot: bool,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IMediaProcessingTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaProcessingTrigger {
    type Vtable = IMediaProcessingTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2593504869,
        35410,
        19248,
        [144, 17, 207, 56, 4, 14, 168, 176],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProcessingTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        arguments: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct INetworkOperatorHotspotAuthenticationTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkOperatorHotspotAuthenticationTrigger {
    type Vtable = INetworkOperatorHotspotAuthenticationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3881224081,
        12289,
        19941,
        [131, 199, 222, 97, 216, 136, 49, 208],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorHotspotAuthenticationTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct INetworkOperatorNotificationTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkOperatorNotificationTrigger {
    type Vtable = INetworkOperatorNotificationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2416483526,
        25549,
        18444,
        [149, 209, 110, 106, 239, 128, 30, 74],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorNotificationTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct INetworkOperatorNotificationTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkOperatorNotificationTriggerFactory {
    type Vtable = INetworkOperatorNotificationTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        170016256,
        10199,
        17235,
        [173, 185, 146, 101, 170, 234, 87, 157],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorNotificationTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        networkaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IPhoneTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneTrigger {
    type Vtable = IPhoneTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2379213211,
        54469,
        18929,
        [183, 211, 130, 232, 122, 14, 157, 222],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::Calls::Background::PhoneTriggerType,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls_Background"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IPhoneTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneTriggerFactory {
    type Vtable = IPhoneTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2698591450,
        24513,
        18683,
        [165, 70, 50, 38, 32, 64, 21, 123],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        r#type: super::Calls::Background::PhoneTriggerType,
        oneshot: bool,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls_Background"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IPushNotificationTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPushNotificationTriggerFactory {
    type Vtable = IPushNotificationTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1842933019,
        17806,
        20418,
        [188, 46, 213, 102, 79, 119, 237, 25],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAvailableTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRcsEndUserMessageAvailableTrigger {
    type Vtable = IRcsEndUserMessageAvailableTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2557283690,
        45814,
        18047,
        [169, 120, 164, 64, 145, 193, 26, 102],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAvailableTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRfcommConnectionTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRfcommConnectionTrigger {
    type Vtable = IRfcommConnectionTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3905211106,
        2899,
        17508,
        [147, 148, 253, 135, 86, 84, 222, 100],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommConnectionTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))] usize,
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Networking::Sockets::SocketProtectionLevel,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))] usize,
    #[cfg(feature = "Networking_Sockets")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::Networking::Sockets::SocketProtectionLevel,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))] usize,
    #[cfg(feature = "Networking")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking"))] usize,
    #[cfg(feature = "Networking")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryAuthenticationFactorAuthenticationTrigger {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4063752999,
        20865,
        20260,
        [150, 167, 112, 10, 78, 95, 172, 98],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISensorDataThresholdTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISensorDataThresholdTrigger {
    type Vtable = ISensorDataThresholdTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1539371890,
        54411,
        19327,
        [171, 236, 21, 249, 186, 204, 18, 226],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataThresholdTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISensorDataThresholdTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISensorDataThresholdTriggerFactory {
    type Vtable = ISensorDataThresholdTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2451564149,
        32240,
        19875,
        [151, 179, 229, 68, 238, 133, 127, 230],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataThresholdTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Sensors")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        threshold: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Sensors"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISmartCardTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardTrigger {
    type Vtable = ISmartCardTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4114335148,
        33994,
        18802,
        [140, 233, 229, 143, 151, 179, 122, 80],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_SmartCards")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Devices::SmartCards::SmartCardTriggerType,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_SmartCards"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISmartCardTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardTriggerFactory {
    type Vtable = ISmartCardTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1673483459,
        35265,
        19968,
        [169, 211, 151, 198, 41, 38, 157, 173],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_SmartCards")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        triggertype: super::super::Devices::SmartCards::SmartCardTriggerType,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_SmartCards"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISmsMessageReceivedTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsMessageReceivedTriggerFactory {
    type Vtable = ISmsMessageReceivedTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3929725128,
        27556,
        19122,
        [141, 33, 188, 107, 9, 199, 117, 100],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageReceivedTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Sms")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        filterrules: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISocketActivityTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISocketActivityTrigger {
    type Vtable = ISocketActivityTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2847668240,
        40414,
        20362,
        [131, 227, 176, 224, 231, 165, 13, 112],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocketActivityTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTrackerTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStorageLibraryChangeTrackerTriggerFactory {
    type Vtable = IStorageLibraryChangeTrackerTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        514916304,
        23173,
        18846,
        [168, 136, 130, 70, 7, 18, 79, 80],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTrackerTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tracker: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IStorageLibraryContentChangedTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStorageLibraryContentChangedTrigger {
    type Vtable = IStorageLibraryContentChangedTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        372760743,
        33436,
        17852,
        [146, 155, 161, 231, 234, 120, 216, 155],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryContentChangedTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IStorageLibraryContentChangedTriggerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStorageLibraryContentChangedTriggerStatics {
    type Vtable = IStorageLibraryContentChangedTriggerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2141133625,
        24464,
        19986,
        [145, 78, 167, 216, 224, 187, 251, 24],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryContentChangedTriggerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        storagelibrary: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        storagelibraries: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISystemCondition(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemCondition {
    type Vtable = ISystemCondition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3244274806,
        35269,
        16907,
        [171, 211, 251, 48, 48, 71, 33, 40],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemCondition_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut SystemConditionType,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISystemConditionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemConditionFactory {
    type Vtable = ISystemConditionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3530150385,
        1447,
        18862,
        [135, 215, 22, 178, 184, 185, 165, 83],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemConditionFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        conditiontype: SystemConditionType,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISystemTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemTrigger {
    type Vtable = ISystemTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        494978934,
        14152,
        17507,
        [141, 126, 39, 109, 193, 57, 172, 28],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut SystemTriggerType,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISystemTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemTriggerFactory {
    type Vtable = ISystemTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3892585428,
        34705,
        17785,
        [129, 38, 135, 236, 138, 170, 64, 122],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        triggertype: SystemTriggerType,
        oneshot: bool,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ITimeTrigger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITimeTrigger {
    type Vtable = ITimeTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1701729622,
        2858,
        17271,
        [186, 112, 59, 69, 169, 53, 84, 127],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeTrigger_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ITimeTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITimeTriggerFactory {
    type Vtable = ITimeTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        952533758,
        39764,
        17894,
        [178, 243, 38, 155, 135, 166, 247, 52],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        freshnesstime: u32,
        oneshot: bool,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IToastNotificationActionTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationActionTriggerFactory {
    type Vtable = IToastNotificationActionTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2963143719,
        25728,
        17225,
        [129, 37, 151, 179, 239, 170, 10, 58],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationActionTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IToastNotificationHistoryChangedTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationHistoryChangedTriggerFactory {
    type Vtable = IToastNotificationHistoryChangedTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2177301165,
        34711,
        18309,
        [129, 180, 176, 204, 203, 115, 209, 217],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistoryChangedTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IUserNotificationChangedTriggerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserNotificationChangedTriggerFactory {
    type Vtable = IUserNotificationChangedTriggerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3402908524,
        27051,
        19992,
        [164, 138, 94, 210, 172, 67, 89, 87],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotificationChangedTriggerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Notifications")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        notificationkinds: super::super::UI::Notifications::NotificationKinds,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct LocationTrigger(::windows::runtime::IInspectable);
impl LocationTrigger {
    pub fn TriggerType(&self) -> ::windows::runtime::Result<LocationTriggerType> {
        let this = self;
        unsafe {
            let mut result__: LocationTriggerType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<LocationTriggerType>(result__)
        }
    }
    pub fn Create(triggertype: LocationTriggerType) -> ::windows::runtime::Result<LocationTrigger> {
        Self::ILocationTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                triggertype,
                &mut result__,
            )
            .from_abi::<LocationTrigger>(result__)
        })
    }
    pub fn ILocationTriggerFactory<
        R,
        F: FnOnce(&ILocationTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            LocationTrigger,
            ILocationTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LocationTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.LocationTrigger;{47666a1c-6877-481e-8026-ff7e14a811a0})" ) ;
}
unsafe impl ::windows::runtime::Interface for LocationTrigger {
    type Vtable = ILocationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1197894172,
        26743,
        18462,
        [128, 38, 255, 126, 20, 168, 17, 160],
    );
}
impl ::windows::runtime::RuntimeName for LocationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.LocationTrigger";
}
impl ::std::convert::From<LocationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: LocationTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LocationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &LocationTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LocationTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LocationTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<LocationTrigger> for ::windows::runtime::IInspectable {
    fn from(value: LocationTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LocationTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &LocationTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LocationTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a LocationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<LocationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LocationTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&LocationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LocationTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for LocationTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &LocationTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for LocationTrigger {}
unsafe impl ::std::marker::Sync for LocationTrigger {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct LocationTriggerType(pub i32);
impl LocationTriggerType {
    pub const Geofence: LocationTriggerType = LocationTriggerType(0i32);
}
impl ::std::convert::From<i32> for LocationTriggerType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LocationTriggerType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LocationTriggerType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.Background.LocationTriggerType;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct MaintenanceTrigger(::windows::runtime::IInspectable);
impl MaintenanceTrigger {
    pub fn FreshnessTime(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn OneShot(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn Create(
        freshnesstime: u32,
        oneshot: bool,
    ) -> ::windows::runtime::Result<MaintenanceTrigger> {
        Self::IMaintenanceTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                freshnesstime,
                oneshot,
                &mut result__,
            )
            .from_abi::<MaintenanceTrigger>(result__)
        })
    }
    pub fn IMaintenanceTriggerFactory<
        R,
        F: FnOnce(&IMaintenanceTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            MaintenanceTrigger,
            IMaintenanceTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MaintenanceTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.MaintenanceTrigger;{68184c83-fc22-4ce5-841a-7239a9810047})" ) ;
}
unsafe impl ::windows::runtime::Interface for MaintenanceTrigger {
    type Vtable = IMaintenanceTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1746422915,
        64546,
        19685,
        [132, 26, 114, 57, 169, 129, 0, 71],
    );
}
impl ::windows::runtime::RuntimeName for MaintenanceTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MaintenanceTrigger";
}
impl ::std::convert::From<MaintenanceTrigger> for ::windows::runtime::IUnknown {
    fn from(value: MaintenanceTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MaintenanceTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &MaintenanceTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MaintenanceTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MaintenanceTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<MaintenanceTrigger> for ::windows::runtime::IInspectable {
    fn from(value: MaintenanceTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MaintenanceTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &MaintenanceTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for MaintenanceTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a MaintenanceTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<MaintenanceTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MaintenanceTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MaintenanceTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MaintenanceTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for MaintenanceTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &MaintenanceTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct MediaProcessingTrigger(::windows::runtime::IInspectable);
impl MediaProcessingTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            MediaProcessingTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAsync(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>>(
                result__,
            )
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RequestAsyncWithArguments<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::ValueSet>,
    >(
        &self,
        arguments: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                arguments.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>>(
                result__,
            )
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaProcessingTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.MediaProcessingTrigger;{9a95be65-8a52-4b30-9011-cf38040ea8b0})" ) ;
}
unsafe impl ::windows::runtime::Interface for MediaProcessingTrigger {
    type Vtable = IMediaProcessingTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2593504869,
        35410,
        19248,
        [144, 17, 207, 56, 4, 14, 168, 176],
    );
}
impl ::windows::runtime::RuntimeName for MediaProcessingTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MediaProcessingTrigger";
}
impl ::std::convert::From<MediaProcessingTrigger> for ::windows::runtime::IUnknown {
    fn from(value: MediaProcessingTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MediaProcessingTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &MediaProcessingTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for MediaProcessingTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &MediaProcessingTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<MediaProcessingTrigger> for ::windows::runtime::IInspectable {
    fn from(value: MediaProcessingTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MediaProcessingTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &MediaProcessingTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for MediaProcessingTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a MediaProcessingTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<MediaProcessingTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MediaProcessingTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MediaProcessingTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MediaProcessingTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for MediaProcessingTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &MediaProcessingTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MediaProcessingTriggerResult(pub i32);
impl MediaProcessingTriggerResult {
    pub const Allowed: MediaProcessingTriggerResult = MediaProcessingTriggerResult(0i32);
    pub const CurrentlyRunning: MediaProcessingTriggerResult = MediaProcessingTriggerResult(1i32);
    pub const DisabledByPolicy: MediaProcessingTriggerResult = MediaProcessingTriggerResult(2i32);
    pub const UnknownError: MediaProcessingTriggerResult = MediaProcessingTriggerResult(3i32);
}
impl ::std::convert::From<i32> for MediaProcessingTriggerResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaProcessingTriggerResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaProcessingTriggerResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.Background.MediaProcessingTriggerResult;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct MobileBroadbandDeviceServiceNotificationTrigger(::windows::runtime::IInspectable);
impl MobileBroadbandDeviceServiceNotificationTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            MobileBroadbandDeviceServiceNotificationTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandDeviceServiceNotificationTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.MobileBroadbandDeviceServiceNotificationTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})" ) ;
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandDeviceServiceNotificationTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl ::windows::runtime::RuntimeName for MobileBroadbandDeviceServiceNotificationTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.MobileBroadbandDeviceServiceNotificationTrigger";
}
impl ::std::convert::From<MobileBroadbandDeviceServiceNotificationTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: MobileBroadbandDeviceServiceNotificationTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceServiceNotificationTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: &MobileBroadbandDeviceServiceNotificationTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for MobileBroadbandDeviceServiceNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &MobileBroadbandDeviceServiceNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<MobileBroadbandDeviceServiceNotificationTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: MobileBroadbandDeviceServiceNotificationTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceServiceNotificationTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &MobileBroadbandDeviceServiceNotificationTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for MobileBroadbandDeviceServiceNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a MobileBroadbandDeviceServiceNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<MobileBroadbandDeviceServiceNotificationTrigger> for IBackgroundTrigger {
    fn from(value: MobileBroadbandDeviceServiceNotificationTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceServiceNotificationTrigger> for IBackgroundTrigger {
    fn from(value: &MobileBroadbandDeviceServiceNotificationTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for MobileBroadbandDeviceServiceNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &MobileBroadbandDeviceServiceNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandDeviceServiceNotificationTrigger {}
unsafe impl ::std::marker::Sync for MobileBroadbandDeviceServiceNotificationTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct MobileBroadbandPcoDataChangeTrigger(::windows::runtime::IInspectable);
impl MobileBroadbandPcoDataChangeTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            MobileBroadbandPcoDataChangeTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandPcoDataChangeTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.MobileBroadbandPcoDataChangeTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})" ) ;
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandPcoDataChangeTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl ::windows::runtime::RuntimeName for MobileBroadbandPcoDataChangeTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.MobileBroadbandPcoDataChangeTrigger";
}
impl ::std::convert::From<MobileBroadbandPcoDataChangeTrigger> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandPcoDataChangeTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandPcoDataChangeTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandPcoDataChangeTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for MobileBroadbandPcoDataChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &MobileBroadbandPcoDataChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<MobileBroadbandPcoDataChangeTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: MobileBroadbandPcoDataChangeTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandPcoDataChangeTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &MobileBroadbandPcoDataChangeTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for MobileBroadbandPcoDataChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a MobileBroadbandPcoDataChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<MobileBroadbandPcoDataChangeTrigger> for IBackgroundTrigger {
    fn from(value: MobileBroadbandPcoDataChangeTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandPcoDataChangeTrigger> for IBackgroundTrigger {
    fn from(value: &MobileBroadbandPcoDataChangeTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for MobileBroadbandPcoDataChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &MobileBroadbandPcoDataChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandPcoDataChangeTrigger {}
unsafe impl ::std::marker::Sync for MobileBroadbandPcoDataChangeTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct MobileBroadbandPinLockStateChangeTrigger(::windows::runtime::IInspectable);
impl MobileBroadbandPinLockStateChangeTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            MobileBroadbandPinLockStateChangeTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandPinLockStateChangeTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.MobileBroadbandPinLockStateChangeTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})" ) ;
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandPinLockStateChangeTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl ::windows::runtime::RuntimeName for MobileBroadbandPinLockStateChangeTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.MobileBroadbandPinLockStateChangeTrigger";
}
impl ::std::convert::From<MobileBroadbandPinLockStateChangeTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: MobileBroadbandPinLockStateChangeTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandPinLockStateChangeTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: &MobileBroadbandPinLockStateChangeTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for MobileBroadbandPinLockStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &MobileBroadbandPinLockStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<MobileBroadbandPinLockStateChangeTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: MobileBroadbandPinLockStateChangeTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandPinLockStateChangeTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &MobileBroadbandPinLockStateChangeTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for MobileBroadbandPinLockStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a MobileBroadbandPinLockStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<MobileBroadbandPinLockStateChangeTrigger> for IBackgroundTrigger {
    fn from(value: MobileBroadbandPinLockStateChangeTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandPinLockStateChangeTrigger> for IBackgroundTrigger {
    fn from(value: &MobileBroadbandPinLockStateChangeTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for MobileBroadbandPinLockStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &MobileBroadbandPinLockStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandPinLockStateChangeTrigger {}
unsafe impl ::std::marker::Sync for MobileBroadbandPinLockStateChangeTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct MobileBroadbandRadioStateChangeTrigger(::windows::runtime::IInspectable);
impl MobileBroadbandRadioStateChangeTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            MobileBroadbandRadioStateChangeTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandRadioStateChangeTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.MobileBroadbandRadioStateChangeTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})" ) ;
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandRadioStateChangeTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl ::windows::runtime::RuntimeName for MobileBroadbandRadioStateChangeTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.MobileBroadbandRadioStateChangeTrigger";
}
impl ::std::convert::From<MobileBroadbandRadioStateChangeTrigger> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandRadioStateChangeTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandRadioStateChangeTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: &MobileBroadbandRadioStateChangeTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for MobileBroadbandRadioStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &MobileBroadbandRadioStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<MobileBroadbandRadioStateChangeTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: MobileBroadbandRadioStateChangeTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandRadioStateChangeTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &MobileBroadbandRadioStateChangeTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for MobileBroadbandRadioStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a MobileBroadbandRadioStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<MobileBroadbandRadioStateChangeTrigger> for IBackgroundTrigger {
    fn from(value: MobileBroadbandRadioStateChangeTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandRadioStateChangeTrigger> for IBackgroundTrigger {
    fn from(value: &MobileBroadbandRadioStateChangeTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for MobileBroadbandRadioStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &MobileBroadbandRadioStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandRadioStateChangeTrigger {}
unsafe impl ::std::marker::Sync for MobileBroadbandRadioStateChangeTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct MobileBroadbandRegistrationStateChangeTrigger(::windows::runtime::IInspectable);
impl MobileBroadbandRegistrationStateChangeTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            MobileBroadbandRegistrationStateChangeTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandRegistrationStateChangeTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.MobileBroadbandRegistrationStateChangeTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})" ) ;
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandRegistrationStateChangeTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl ::windows::runtime::RuntimeName for MobileBroadbandRegistrationStateChangeTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.MobileBroadbandRegistrationStateChangeTrigger";
}
impl ::std::convert::From<MobileBroadbandRegistrationStateChangeTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: MobileBroadbandRegistrationStateChangeTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandRegistrationStateChangeTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: &MobileBroadbandRegistrationStateChangeTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for MobileBroadbandRegistrationStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &MobileBroadbandRegistrationStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<MobileBroadbandRegistrationStateChangeTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: MobileBroadbandRegistrationStateChangeTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandRegistrationStateChangeTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &MobileBroadbandRegistrationStateChangeTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for MobileBroadbandRegistrationStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a MobileBroadbandRegistrationStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<MobileBroadbandRegistrationStateChangeTrigger> for IBackgroundTrigger {
    fn from(value: MobileBroadbandRegistrationStateChangeTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandRegistrationStateChangeTrigger> for IBackgroundTrigger {
    fn from(value: &MobileBroadbandRegistrationStateChangeTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for MobileBroadbandRegistrationStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &MobileBroadbandRegistrationStateChangeTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandRegistrationStateChangeTrigger {}
unsafe impl ::std::marker::Sync for MobileBroadbandRegistrationStateChangeTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct NetworkOperatorDataUsageTrigger(::windows::runtime::IInspectable);
impl NetworkOperatorDataUsageTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            NetworkOperatorDataUsageTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NetworkOperatorDataUsageTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.NetworkOperatorDataUsageTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})" ) ;
}
unsafe impl ::windows::runtime::Interface for NetworkOperatorDataUsageTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl ::windows::runtime::RuntimeName for NetworkOperatorDataUsageTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.NetworkOperatorDataUsageTrigger";
}
impl ::std::convert::From<NetworkOperatorDataUsageTrigger> for ::windows::runtime::IUnknown {
    fn from(value: NetworkOperatorDataUsageTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NetworkOperatorDataUsageTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &NetworkOperatorDataUsageTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for NetworkOperatorDataUsageTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &NetworkOperatorDataUsageTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<NetworkOperatorDataUsageTrigger> for ::windows::runtime::IInspectable {
    fn from(value: NetworkOperatorDataUsageTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NetworkOperatorDataUsageTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &NetworkOperatorDataUsageTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for NetworkOperatorDataUsageTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a NetworkOperatorDataUsageTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<NetworkOperatorDataUsageTrigger> for IBackgroundTrigger {
    fn from(value: NetworkOperatorDataUsageTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NetworkOperatorDataUsageTrigger> for IBackgroundTrigger {
    fn from(value: &NetworkOperatorDataUsageTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for NetworkOperatorDataUsageTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &NetworkOperatorDataUsageTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for NetworkOperatorDataUsageTrigger {}
unsafe impl ::std::marker::Sync for NetworkOperatorDataUsageTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct NetworkOperatorHotspotAuthenticationTrigger(::windows::runtime::IInspectable);
impl NetworkOperatorHotspotAuthenticationTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            NetworkOperatorHotspotAuthenticationTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NetworkOperatorHotspotAuthenticationTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.NetworkOperatorHotspotAuthenticationTrigger;{e756c791-3001-4de5-83c7-de61d88831d0})" ) ;
}
unsafe impl ::windows::runtime::Interface for NetworkOperatorHotspotAuthenticationTrigger {
    type Vtable = INetworkOperatorHotspotAuthenticationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3881224081,
        12289,
        19941,
        [131, 199, 222, 97, 216, 136, 49, 208],
    );
}
impl ::windows::runtime::RuntimeName for NetworkOperatorHotspotAuthenticationTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.NetworkOperatorHotspotAuthenticationTrigger";
}
impl ::std::convert::From<NetworkOperatorHotspotAuthenticationTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: NetworkOperatorHotspotAuthenticationTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NetworkOperatorHotspotAuthenticationTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: &NetworkOperatorHotspotAuthenticationTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for NetworkOperatorHotspotAuthenticationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &NetworkOperatorHotspotAuthenticationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<NetworkOperatorHotspotAuthenticationTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: NetworkOperatorHotspotAuthenticationTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NetworkOperatorHotspotAuthenticationTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &NetworkOperatorHotspotAuthenticationTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for NetworkOperatorHotspotAuthenticationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a NetworkOperatorHotspotAuthenticationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<NetworkOperatorHotspotAuthenticationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: NetworkOperatorHotspotAuthenticationTrigger,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&NetworkOperatorHotspotAuthenticationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &NetworkOperatorHotspotAuthenticationTrigger,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for NetworkOperatorHotspotAuthenticationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &NetworkOperatorHotspotAuthenticationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct NetworkOperatorNotificationTrigger(::windows::runtime::IInspectable);
impl NetworkOperatorNotificationTrigger {
    pub fn NetworkAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        networkaccountid: Param0,
    ) -> ::windows::runtime::Result<NetworkOperatorNotificationTrigger> {
        Self::INetworkOperatorNotificationTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                networkaccountid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<NetworkOperatorNotificationTrigger>(result__)
        })
    }
    pub fn INetworkOperatorNotificationTriggerFactory<
        R,
        F: FnOnce(&INetworkOperatorNotificationTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            NetworkOperatorNotificationTrigger,
            INetworkOperatorNotificationTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NetworkOperatorNotificationTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.NetworkOperatorNotificationTrigger;{90089cc6-63cd-480c-95d1-6e6aef801e4a})" ) ;
}
unsafe impl ::windows::runtime::Interface for NetworkOperatorNotificationTrigger {
    type Vtable = INetworkOperatorNotificationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2416483526,
        25549,
        18444,
        [149, 209, 110, 106, 239, 128, 30, 74],
    );
}
impl ::windows::runtime::RuntimeName for NetworkOperatorNotificationTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.NetworkOperatorNotificationTrigger";
}
impl ::std::convert::From<NetworkOperatorNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: NetworkOperatorNotificationTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NetworkOperatorNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &NetworkOperatorNotificationTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for NetworkOperatorNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &NetworkOperatorNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<NetworkOperatorNotificationTrigger> for ::windows::runtime::IInspectable {
    fn from(value: NetworkOperatorNotificationTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NetworkOperatorNotificationTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &NetworkOperatorNotificationTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for NetworkOperatorNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a NetworkOperatorNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<NetworkOperatorNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: NetworkOperatorNotificationTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&NetworkOperatorNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &NetworkOperatorNotificationTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for NetworkOperatorNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &NetworkOperatorNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct PaymentAppCanMakePaymentTrigger(::windows::runtime::IInspectable);
impl PaymentAppCanMakePaymentTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            PaymentAppCanMakePaymentTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentAppCanMakePaymentTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.PaymentAppCanMakePaymentTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})" ) ;
}
unsafe impl ::windows::runtime::Interface for PaymentAppCanMakePaymentTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl ::windows::runtime::RuntimeName for PaymentAppCanMakePaymentTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.PaymentAppCanMakePaymentTrigger";
}
impl ::std::convert::From<PaymentAppCanMakePaymentTrigger> for ::windows::runtime::IUnknown {
    fn from(value: PaymentAppCanMakePaymentTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PaymentAppCanMakePaymentTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentAppCanMakePaymentTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for PaymentAppCanMakePaymentTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &PaymentAppCanMakePaymentTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<PaymentAppCanMakePaymentTrigger> for ::windows::runtime::IInspectable {
    fn from(value: PaymentAppCanMakePaymentTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentAppCanMakePaymentTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentAppCanMakePaymentTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for PaymentAppCanMakePaymentTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a PaymentAppCanMakePaymentTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<PaymentAppCanMakePaymentTrigger> for IBackgroundTrigger {
    fn from(value: PaymentAppCanMakePaymentTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PaymentAppCanMakePaymentTrigger> for IBackgroundTrigger {
    fn from(value: &PaymentAppCanMakePaymentTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for PaymentAppCanMakePaymentTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &PaymentAppCanMakePaymentTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for PaymentAppCanMakePaymentTrigger {}
unsafe impl ::std::marker::Sync for PaymentAppCanMakePaymentTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct PhoneTrigger(::windows::runtime::IInspectable);
impl PhoneTrigger {
    pub fn OneShot(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub fn TriggerType(
        &self,
    ) -> ::windows::runtime::Result<super::Calls::Background::PhoneTriggerType> {
        let this = self;
        unsafe {
            let mut result__: super::Calls::Background::PhoneTriggerType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Calls::Background::PhoneTriggerType>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub fn Create(
        r#type: super::Calls::Background::PhoneTriggerType,
        oneshot: bool,
    ) -> ::windows::runtime::Result<PhoneTrigger> {
        Self::IPhoneTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                r#type,
                oneshot,
                &mut result__,
            )
            .from_abi::<PhoneTrigger>(result__)
        })
    }
    pub fn IPhoneTriggerFactory<
        R,
        F: FnOnce(&IPhoneTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneTrigger, IPhoneTriggerFactory> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.PhoneTrigger;{8dcfe99b-d4c5-49f1-b7d3-82e87a0e9dde})" ) ;
}
unsafe impl ::windows::runtime::Interface for PhoneTrigger {
    type Vtable = IPhoneTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2379213211,
        54469,
        18929,
        [183, 211, 130, 232, 122, 14, 157, 222],
    );
}
impl ::windows::runtime::RuntimeName for PhoneTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.PhoneTrigger";
}
impl ::std::convert::From<PhoneTrigger> for ::windows::runtime::IUnknown {
    fn from(value: PhoneTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PhoneTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PhoneTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<PhoneTrigger> for ::windows::runtime::IInspectable {
    fn from(value: PhoneTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<PhoneTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PhoneTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PhoneTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PhoneTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for PhoneTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &PhoneTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for PhoneTrigger {}
unsafe impl ::std::marker::Sync for PhoneTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct PushNotificationTrigger(::windows::runtime::IInspectable);
impl PushNotificationTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            PushNotificationTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        applicationid: Param0,
    ) -> ::windows::runtime::Result<PushNotificationTrigger> {
        Self::IPushNotificationTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                applicationid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PushNotificationTrigger>(result__)
        })
    }
    pub fn IPushNotificationTriggerFactory<
        R,
        F: FnOnce(&IPushNotificationTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            PushNotificationTrigger,
            IPushNotificationTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PushNotificationTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.PushNotificationTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})" ) ;
}
unsafe impl ::windows::runtime::Interface for PushNotificationTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl ::windows::runtime::RuntimeName for PushNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.PushNotificationTrigger";
}
impl ::std::convert::From<PushNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: PushNotificationTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PushNotificationTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &PushNotificationTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for PushNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &PushNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<PushNotificationTrigger> for ::windows::runtime::IInspectable {
    fn from(value: PushNotificationTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PushNotificationTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &PushNotificationTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for PushNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a PushNotificationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<PushNotificationTrigger> for IBackgroundTrigger {
    fn from(value: PushNotificationTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PushNotificationTrigger> for IBackgroundTrigger {
    fn from(value: &PushNotificationTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for PushNotificationTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &PushNotificationTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for PushNotificationTrigger {}
unsafe impl ::std::marker::Sync for PushNotificationTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RcsEndUserMessageAvailableTrigger(::windows::runtime::IInspectable);
impl RcsEndUserMessageAvailableTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RcsEndUserMessageAvailableTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RcsEndUserMessageAvailableTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.RcsEndUserMessageAvailableTrigger;{986d0d6a-b2f6-467f-a978-a44091c11a66})" ) ;
}
unsafe impl ::windows::runtime::Interface for RcsEndUserMessageAvailableTrigger {
    type Vtable = IRcsEndUserMessageAvailableTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2557283690,
        45814,
        18047,
        [169, 120, 164, 64, 145, 193, 26, 102],
    );
}
impl ::windows::runtime::RuntimeName for RcsEndUserMessageAvailableTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.RcsEndUserMessageAvailableTrigger";
}
impl ::std::convert::From<RcsEndUserMessageAvailableTrigger> for ::windows::runtime::IUnknown {
    fn from(value: RcsEndUserMessageAvailableTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RcsEndUserMessageAvailableTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &RcsEndUserMessageAvailableTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RcsEndUserMessageAvailableTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RcsEndUserMessageAvailableTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RcsEndUserMessageAvailableTrigger> for ::windows::runtime::IInspectable {
    fn from(value: RcsEndUserMessageAvailableTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RcsEndUserMessageAvailableTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &RcsEndUserMessageAvailableTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RcsEndUserMessageAvailableTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RcsEndUserMessageAvailableTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<RcsEndUserMessageAvailableTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RcsEndUserMessageAvailableTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&RcsEndUserMessageAvailableTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RcsEndUserMessageAvailableTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for RcsEndUserMessageAvailableTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &RcsEndUserMessageAvailableTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for RcsEndUserMessageAvailableTrigger {}
unsafe impl ::std::marker::Sync for RcsEndUserMessageAvailableTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RfcommConnectionTrigger(::windows::runtime::IInspectable);
impl RfcommConnectionTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RfcommConnectionTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub fn InboundConnection(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Devices::Bluetooth::Background::RfcommInboundConnectionInformation,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::Devices::Bluetooth::Background:: RfcommInboundConnectionInformation > ( result__ )
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub fn OutboundConnection(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Devices::Bluetooth::Background::RfcommOutboundConnectionInformation,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .7 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::Devices::Bluetooth::Background:: RfcommOutboundConnectionInformation > ( result__ )
        }
    }
    pub fn AllowMultipleConnections(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowMultipleConnections(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn ProtectionLevel(
        &self,
    ) -> ::windows::runtime::Result<super::super::Networking::Sockets::SocketProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: super::super::Networking::Sockets::SocketProtectionLevel =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Networking::Sockets::SocketProtectionLevel>(result__)
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn SetProtectionLevel(
        &self,
        value: super::super::Networking::Sockets::SocketProtectionLevel,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "Networking")]
    pub fn RemoteHostName(&self) -> ::windows::runtime::Result<super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Networking::HostName>(result__)
        }
    }
    #[cfg(feature = "Networking")]
    pub fn SetRemoteHostName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Networking::HostName>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RfcommConnectionTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.RfcommConnectionTrigger;{e8c4cae2-0b53-4464-9394-fd875654de64})" ) ;
}
unsafe impl ::windows::runtime::Interface for RfcommConnectionTrigger {
    type Vtable = IRfcommConnectionTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3905211106,
        2899,
        17508,
        [147, 148, 253, 135, 86, 84, 222, 100],
    );
}
impl ::windows::runtime::RuntimeName for RfcommConnectionTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.RfcommConnectionTrigger";
}
impl ::std::convert::From<RfcommConnectionTrigger> for ::windows::runtime::IUnknown {
    fn from(value: RfcommConnectionTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RfcommConnectionTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &RfcommConnectionTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RfcommConnectionTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RfcommConnectionTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RfcommConnectionTrigger> for ::windows::runtime::IInspectable {
    fn from(value: RfcommConnectionTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RfcommConnectionTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &RfcommConnectionTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RfcommConnectionTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RfcommConnectionTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<RfcommConnectionTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RfcommConnectionTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&RfcommConnectionTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RfcommConnectionTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for RfcommConnectionTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &RfcommConnectionTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for RfcommConnectionTrigger {}
unsafe impl ::std::marker::Sync for RfcommConnectionTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SecondaryAuthenticationFactorAuthenticationTrigger(::windows::runtime::IInspectable);
impl SecondaryAuthenticationFactorAuthenticationTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SecondaryAuthenticationFactorAuthenticationTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorAuthenticationTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.SecondaryAuthenticationFactorAuthenticationTrigger;{f237f327-5181-4f24-96a7-700a4e5fac62})" ) ;
}
unsafe impl ::windows::runtime::Interface for SecondaryAuthenticationFactorAuthenticationTrigger {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4063752999,
        20865,
        20260,
        [150, 167, 112, 10, 78, 95, 172, 98],
    );
}
impl ::windows::runtime::RuntimeName for SecondaryAuthenticationFactorAuthenticationTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.SecondaryAuthenticationFactorAuthenticationTrigger";
}
impl ::std::convert::From<SecondaryAuthenticationFactorAuthenticationTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: SecondaryAuthenticationFactorAuthenticationTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SecondaryAuthenticationFactorAuthenticationTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: &SecondaryAuthenticationFactorAuthenticationTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SecondaryAuthenticationFactorAuthenticationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SecondaryAuthenticationFactorAuthenticationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SecondaryAuthenticationFactorAuthenticationTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: SecondaryAuthenticationFactorAuthenticationTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SecondaryAuthenticationFactorAuthenticationTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &SecondaryAuthenticationFactorAuthenticationTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SecondaryAuthenticationFactorAuthenticationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SecondaryAuthenticationFactorAuthenticationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SecondaryAuthenticationFactorAuthenticationTrigger>
    for IBackgroundTrigger
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: SecondaryAuthenticationFactorAuthenticationTrigger,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SecondaryAuthenticationFactorAuthenticationTrigger>
    for IBackgroundTrigger
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &SecondaryAuthenticationFactorAuthenticationTrigger,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for SecondaryAuthenticationFactorAuthenticationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &SecondaryAuthenticationFactorAuthenticationTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SensorDataThresholdTrigger(::windows::runtime::IInspectable);
impl SensorDataThresholdTrigger {
    #[cfg(feature = "Devices_Sensors")]
    pub fn Create<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Devices::Sensors::ISensorDataThreshold>,
    >(
        threshold: Param0,
    ) -> ::windows::runtime::Result<SensorDataThresholdTrigger> {
        Self::ISensorDataThresholdTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                threshold.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SensorDataThresholdTrigger>(result__)
        })
    }
    pub fn ISensorDataThresholdTriggerFactory<
        R,
        F: FnOnce(&ISensorDataThresholdTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SensorDataThresholdTrigger,
            ISensorDataThresholdTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SensorDataThresholdTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.SensorDataThresholdTrigger;{5bc0f372-d48b-4b7f-abec-15f9bacc12e2})" ) ;
}
unsafe impl ::windows::runtime::Interface for SensorDataThresholdTrigger {
    type Vtable = ISensorDataThresholdTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1539371890,
        54411,
        19327,
        [171, 236, 21, 249, 186, 204, 18, 226],
    );
}
impl ::windows::runtime::RuntimeName for SensorDataThresholdTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SensorDataThresholdTrigger";
}
impl ::std::convert::From<SensorDataThresholdTrigger> for ::windows::runtime::IUnknown {
    fn from(value: SensorDataThresholdTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SensorDataThresholdTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &SensorDataThresholdTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SensorDataThresholdTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SensorDataThresholdTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SensorDataThresholdTrigger> for ::windows::runtime::IInspectable {
    fn from(value: SensorDataThresholdTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SensorDataThresholdTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &SensorDataThresholdTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SensorDataThresholdTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SensorDataThresholdTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SensorDataThresholdTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SensorDataThresholdTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SensorDataThresholdTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SensorDataThresholdTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for SensorDataThresholdTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &SensorDataThresholdTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SensorDataThresholdTrigger {}
unsafe impl ::std::marker::Sync for SensorDataThresholdTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SmartCardTrigger(::windows::runtime::IInspectable);
impl SmartCardTrigger {
    #[cfg(feature = "Devices_SmartCards")]
    pub fn TriggerType(
        &self,
    ) -> ::windows::runtime::Result<super::super::Devices::SmartCards::SmartCardTriggerType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::SmartCards::SmartCardTriggerType =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Devices::SmartCards::SmartCardTriggerType>(result__)
        }
    }
    #[cfg(feature = "Devices_SmartCards")]
    pub fn Create(
        triggertype: super::super::Devices::SmartCards::SmartCardTriggerType,
    ) -> ::windows::runtime::Result<SmartCardTrigger> {
        Self::ISmartCardTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                triggertype,
                &mut result__,
            )
            .from_abi::<SmartCardTrigger>(result__)
        })
    }
    pub fn ISmartCardTriggerFactory<
        R,
        F: FnOnce(&ISmartCardTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SmartCardTrigger,
            ISmartCardTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.SmartCardTrigger;{f53bc5ac-84ca-4972-8ce9-e58f97b37a50})" ) ;
}
unsafe impl ::windows::runtime::Interface for SmartCardTrigger {
    type Vtable = ISmartCardTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4114335148,
        33994,
        18802,
        [140, 233, 229, 143, 151, 179, 122, 80],
    );
}
impl ::windows::runtime::RuntimeName for SmartCardTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SmartCardTrigger";
}
impl ::std::convert::From<SmartCardTrigger> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmartCardTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmartCardTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SmartCardTrigger> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmartCardTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SmartCardTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SmartCardTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SmartCardTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SmartCardTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SmartCardTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for SmartCardTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &SmartCardTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SmsMessageReceivedTrigger(::windows::runtime::IInspectable);
impl SmsMessageReceivedTrigger {
    #[cfg(feature = "Devices_Sms")]
    pub fn Create<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Devices::Sms::SmsFilterRules>,
    >(
        filterrules: Param0,
    ) -> ::windows::runtime::Result<SmsMessageReceivedTrigger> {
        Self::ISmsMessageReceivedTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                filterrules.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SmsMessageReceivedTrigger>(result__)
        })
    }
    pub fn ISmsMessageReceivedTriggerFactory<
        R,
        F: FnOnce(&ISmsMessageReceivedTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SmsMessageReceivedTrigger,
            ISmsMessageReceivedTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsMessageReceivedTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.SmsMessageReceivedTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})" ) ;
}
unsafe impl ::windows::runtime::Interface for SmsMessageReceivedTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl ::windows::runtime::RuntimeName for SmsMessageReceivedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SmsMessageReceivedTrigger";
}
impl ::std::convert::From<SmsMessageReceivedTrigger> for ::windows::runtime::IUnknown {
    fn from(value: SmsMessageReceivedTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsMessageReceivedTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &SmsMessageReceivedTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SmsMessageReceivedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SmsMessageReceivedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SmsMessageReceivedTrigger> for ::windows::runtime::IInspectable {
    fn from(value: SmsMessageReceivedTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsMessageReceivedTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &SmsMessageReceivedTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SmsMessageReceivedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SmsMessageReceivedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<SmsMessageReceivedTrigger> for IBackgroundTrigger {
    fn from(value: SmsMessageReceivedTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsMessageReceivedTrigger> for IBackgroundTrigger {
    fn from(value: &SmsMessageReceivedTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for SmsMessageReceivedTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &SmsMessageReceivedTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SmsMessageReceivedTrigger {}
unsafe impl ::std::marker::Sync for SmsMessageReceivedTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SocketActivityTrigger(::windows::runtime::IInspectable);
impl SocketActivityTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SocketActivityTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IsWakeFromLowPowerSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISocketActivityTrigger>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SocketActivityTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.SocketActivityTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})" ) ;
}
unsafe impl ::windows::runtime::Interface for SocketActivityTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl ::windows::runtime::RuntimeName for SocketActivityTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SocketActivityTrigger";
}
impl ::std::convert::From<SocketActivityTrigger> for ::windows::runtime::IUnknown {
    fn from(value: SocketActivityTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SocketActivityTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &SocketActivityTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SocketActivityTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SocketActivityTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SocketActivityTrigger> for ::windows::runtime::IInspectable {
    fn from(value: SocketActivityTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SocketActivityTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &SocketActivityTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SocketActivityTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SocketActivityTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<SocketActivityTrigger> for IBackgroundTrigger {
    fn from(value: SocketActivityTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SocketActivityTrigger> for IBackgroundTrigger {
    fn from(value: &SocketActivityTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for SocketActivityTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &SocketActivityTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SocketActivityTrigger {}
unsafe impl ::std::marker::Sync for SocketActivityTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct StorageLibraryChangeTrackerTrigger(::windows::runtime::IInspectable);
impl StorageLibraryChangeTrackerTrigger {
    #[cfg(feature = "Storage")]
    pub fn Create<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageLibraryChangeTracker>,
    >(
        tracker: Param0,
    ) -> ::windows::runtime::Result<StorageLibraryChangeTrackerTrigger> {
        Self::IStorageLibraryChangeTrackerTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                tracker.into_param().abi(),
                &mut result__,
            )
            .from_abi::<StorageLibraryChangeTrackerTrigger>(result__)
        })
    }
    pub fn IStorageLibraryChangeTrackerTriggerFactory<
        R,
        F: FnOnce(&IStorageLibraryChangeTrackerTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            StorageLibraryChangeTrackerTrigger,
            IStorageLibraryChangeTrackerTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for StorageLibraryChangeTrackerTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.StorageLibraryChangeTrackerTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})" ) ;
}
unsafe impl ::windows::runtime::Interface for StorageLibraryChangeTrackerTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl ::windows::runtime::RuntimeName for StorageLibraryChangeTrackerTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.StorageLibraryChangeTrackerTrigger";
}
impl ::std::convert::From<StorageLibraryChangeTrackerTrigger> for ::windows::runtime::IUnknown {
    fn from(value: StorageLibraryChangeTrackerTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&StorageLibraryChangeTrackerTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &StorageLibraryChangeTrackerTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for StorageLibraryChangeTrackerTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &StorageLibraryChangeTrackerTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<StorageLibraryChangeTrackerTrigger> for ::windows::runtime::IInspectable {
    fn from(value: StorageLibraryChangeTrackerTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StorageLibraryChangeTrackerTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &StorageLibraryChangeTrackerTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for StorageLibraryChangeTrackerTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a StorageLibraryChangeTrackerTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<StorageLibraryChangeTrackerTrigger> for IBackgroundTrigger {
    fn from(value: StorageLibraryChangeTrackerTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&StorageLibraryChangeTrackerTrigger> for IBackgroundTrigger {
    fn from(value: &StorageLibraryChangeTrackerTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for StorageLibraryChangeTrackerTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &StorageLibraryChangeTrackerTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for StorageLibraryChangeTrackerTrigger {}
unsafe impl ::std::marker::Sync for StorageLibraryChangeTrackerTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct StorageLibraryContentChangedTrigger(::windows::runtime::IInspectable);
impl StorageLibraryContentChangedTrigger {
    #[cfg(feature = "Storage")]
    pub fn Create<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageLibrary>,
    >(
        storagelibrary: Param0,
    ) -> ::windows::runtime::Result<StorageLibraryContentChangedTrigger> {
        Self::IStorageLibraryContentChangedTriggerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                storagelibrary.into_param().abi(),
                &mut result__,
            )
            .from_abi::<StorageLibraryContentChangedTrigger>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn CreateFromLibraries<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<super::super::Storage::StorageLibrary>,
        >,
    >(
        storagelibraries: Param0,
    ) -> ::windows::runtime::Result<StorageLibraryContentChangedTrigger> {
        Self::IStorageLibraryContentChangedTriggerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                storagelibraries.into_param().abi(),
                &mut result__,
            )
            .from_abi::<StorageLibraryContentChangedTrigger>(result__)
        })
    }
    pub fn IStorageLibraryContentChangedTriggerStatics<
        R,
        F: FnOnce(&IStorageLibraryContentChangedTriggerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            StorageLibraryContentChangedTrigger,
            IStorageLibraryContentChangedTriggerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for StorageLibraryContentChangedTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.StorageLibraryContentChangedTrigger;{1637e0a7-829c-45bc-929b-a1e7ea78d89b})" ) ;
}
unsafe impl ::windows::runtime::Interface for StorageLibraryContentChangedTrigger {
    type Vtable = IStorageLibraryContentChangedTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        372760743,
        33436,
        17852,
        [146, 155, 161, 231, 234, 120, 216, 155],
    );
}
impl ::windows::runtime::RuntimeName for StorageLibraryContentChangedTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.StorageLibraryContentChangedTrigger";
}
impl ::std::convert::From<StorageLibraryContentChangedTrigger> for ::windows::runtime::IUnknown {
    fn from(value: StorageLibraryContentChangedTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&StorageLibraryContentChangedTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &StorageLibraryContentChangedTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for StorageLibraryContentChangedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &StorageLibraryContentChangedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<StorageLibraryContentChangedTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: StorageLibraryContentChangedTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StorageLibraryContentChangedTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &StorageLibraryContentChangedTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for StorageLibraryContentChangedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a StorageLibraryContentChangedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<StorageLibraryContentChangedTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: StorageLibraryContentChangedTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&StorageLibraryContentChangedTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &StorageLibraryContentChangedTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for StorageLibraryContentChangedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &StorageLibraryContentChangedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SystemCondition(::windows::runtime::IInspectable);
impl SystemCondition {
    pub fn ConditionType(&self) -> ::windows::runtime::Result<SystemConditionType> {
        let this = self;
        unsafe {
            let mut result__: SystemConditionType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SystemConditionType>(result__)
        }
    }
    pub fn Create(
        conditiontype: SystemConditionType,
    ) -> ::windows::runtime::Result<SystemCondition> {
        Self::ISystemConditionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                conditiontype,
                &mut result__,
            )
            .from_abi::<SystemCondition>(result__)
        })
    }
    pub fn ISystemConditionFactory<
        R,
        F: FnOnce(&ISystemConditionFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SystemCondition,
            ISystemConditionFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemCondition {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.SystemCondition;{c15fb476-89c5-420b-abd3-fb3030472128})" ) ;
}
unsafe impl ::windows::runtime::Interface for SystemCondition {
    type Vtable = ISystemCondition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3244274806,
        35269,
        16907,
        [171, 211, 251, 48, 48, 71, 33, 40],
    );
}
impl ::windows::runtime::RuntimeName for SystemCondition {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SystemCondition";
}
impl ::std::convert::From<SystemCondition> for ::windows::runtime::IUnknown {
    fn from(value: SystemCondition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SystemCondition> for ::windows::runtime::IUnknown {
    fn from(value: &SystemCondition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemCondition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SystemCondition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SystemCondition> for ::windows::runtime::IInspectable {
    fn from(value: SystemCondition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SystemCondition> for ::windows::runtime::IInspectable {
    fn from(value: &SystemCondition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemCondition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SystemCondition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SystemCondition> for IBackgroundCondition {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SystemCondition) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SystemCondition> for IBackgroundCondition {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SystemCondition) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundCondition> for SystemCondition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundCondition> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundCondition> for &SystemCondition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundCondition> {
        ::std::convert::TryInto::<IBackgroundCondition>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SystemConditionType(pub i32);
impl SystemConditionType {
    pub const Invalid: SystemConditionType = SystemConditionType(0i32);
    pub const UserPresent: SystemConditionType = SystemConditionType(1i32);
    pub const UserNotPresent: SystemConditionType = SystemConditionType(2i32);
    pub const InternetAvailable: SystemConditionType = SystemConditionType(3i32);
    pub const InternetNotAvailable: SystemConditionType = SystemConditionType(4i32);
    pub const SessionConnected: SystemConditionType = SystemConditionType(5i32);
    pub const SessionDisconnected: SystemConditionType = SystemConditionType(6i32);
    pub const FreeNetworkAvailable: SystemConditionType = SystemConditionType(7i32);
    pub const BackgroundWorkCostNotHigh: SystemConditionType = SystemConditionType(8i32);
}
impl ::std::convert::From<i32> for SystemConditionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SystemConditionType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SystemConditionType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.Background.SystemConditionType;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SystemTrigger(::windows::runtime::IInspectable);
impl SystemTrigger {
    pub fn OneShot(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn TriggerType(&self) -> ::windows::runtime::Result<SystemTriggerType> {
        let this = self;
        unsafe {
            let mut result__: SystemTriggerType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SystemTriggerType>(result__)
        }
    }
    pub fn Create(
        triggertype: SystemTriggerType,
        oneshot: bool,
    ) -> ::windows::runtime::Result<SystemTrigger> {
        Self::ISystemTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                triggertype,
                oneshot,
                &mut result__,
            )
            .from_abi::<SystemTrigger>(result__)
        })
    }
    pub fn ISystemTriggerFactory<
        R,
        F: FnOnce(&ISystemTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SystemTrigger, ISystemTriggerFactory> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.SystemTrigger;{1d80c776-3748-4463-8d7e-276dc139ac1c})" ) ;
}
unsafe impl ::windows::runtime::Interface for SystemTrigger {
    type Vtable = ISystemTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        494978934,
        14152,
        17507,
        [141, 126, 39, 109, 193, 57, 172, 28],
    );
}
impl ::windows::runtime::RuntimeName for SystemTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SystemTrigger";
}
impl ::std::convert::From<SystemTrigger> for ::windows::runtime::IUnknown {
    fn from(value: SystemTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SystemTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &SystemTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SystemTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SystemTrigger> for ::windows::runtime::IInspectable {
    fn from(value: SystemTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SystemTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &SystemTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SystemTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SystemTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SystemTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SystemTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SystemTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for SystemTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &SystemTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SystemTriggerType(pub i32);
impl SystemTriggerType {
    pub const Invalid: SystemTriggerType = SystemTriggerType(0i32);
    pub const SmsReceived: SystemTriggerType = SystemTriggerType(1i32);
    pub const UserPresent: SystemTriggerType = SystemTriggerType(2i32);
    pub const UserAway: SystemTriggerType = SystemTriggerType(3i32);
    pub const NetworkStateChange: SystemTriggerType = SystemTriggerType(4i32);
    pub const ControlChannelReset: SystemTriggerType = SystemTriggerType(5i32);
    pub const InternetAvailable: SystemTriggerType = SystemTriggerType(6i32);
    pub const SessionConnected: SystemTriggerType = SystemTriggerType(7i32);
    pub const ServicingComplete: SystemTriggerType = SystemTriggerType(8i32);
    pub const LockScreenApplicationAdded: SystemTriggerType = SystemTriggerType(9i32);
    pub const LockScreenApplicationRemoved: SystemTriggerType = SystemTriggerType(10i32);
    pub const TimeZoneChange: SystemTriggerType = SystemTriggerType(11i32);
    pub const OnlineIdConnectedStateChange: SystemTriggerType = SystemTriggerType(12i32);
    pub const BackgroundWorkCostChange: SystemTriggerType = SystemTriggerType(13i32);
    pub const PowerStateChange: SystemTriggerType = SystemTriggerType(14i32);
    pub const DefaultSignInAccountChange: SystemTriggerType = SystemTriggerType(15i32);
}
impl ::std::convert::From<i32> for SystemTriggerType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SystemTriggerType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SystemTriggerType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.Background.SystemTriggerType;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TetheringEntitlementCheckTrigger(::windows::runtime::IInspectable);
impl TetheringEntitlementCheckTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            TetheringEntitlementCheckTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TetheringEntitlementCheckTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.TetheringEntitlementCheckTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})" ) ;
}
unsafe impl ::windows::runtime::Interface for TetheringEntitlementCheckTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl ::windows::runtime::RuntimeName for TetheringEntitlementCheckTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.TetheringEntitlementCheckTrigger";
}
impl ::std::convert::From<TetheringEntitlementCheckTrigger> for ::windows::runtime::IUnknown {
    fn from(value: TetheringEntitlementCheckTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TetheringEntitlementCheckTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &TetheringEntitlementCheckTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for TetheringEntitlementCheckTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &TetheringEntitlementCheckTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<TetheringEntitlementCheckTrigger> for ::windows::runtime::IInspectable {
    fn from(value: TetheringEntitlementCheckTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TetheringEntitlementCheckTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &TetheringEntitlementCheckTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for TetheringEntitlementCheckTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a TetheringEntitlementCheckTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<TetheringEntitlementCheckTrigger> for IBackgroundTrigger {
    fn from(value: TetheringEntitlementCheckTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TetheringEntitlementCheckTrigger> for IBackgroundTrigger {
    fn from(value: &TetheringEntitlementCheckTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for TetheringEntitlementCheckTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &TetheringEntitlementCheckTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for TetheringEntitlementCheckTrigger {}
unsafe impl ::std::marker::Sync for TetheringEntitlementCheckTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TimeTrigger(::windows::runtime::IInspectable);
impl TimeTrigger {
    pub fn FreshnessTime(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn OneShot(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn Create(freshnesstime: u32, oneshot: bool) -> ::windows::runtime::Result<TimeTrigger> {
        Self::ITimeTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                freshnesstime,
                oneshot,
                &mut result__,
            )
            .from_abi::<TimeTrigger>(result__)
        })
    }
    pub fn ITimeTriggerFactory<
        R,
        F: FnOnce(&ITimeTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TimeTrigger, ITimeTriggerFactory> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TimeTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.TimeTrigger;{656e5556-0b2a-4377-ba70-3b45a935547f})" ) ;
}
unsafe impl ::windows::runtime::Interface for TimeTrigger {
    type Vtable = ITimeTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1701729622,
        2858,
        17271,
        [186, 112, 59, 69, 169, 53, 84, 127],
    );
}
impl ::windows::runtime::RuntimeName for TimeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.TimeTrigger";
}
impl ::std::convert::From<TimeTrigger> for ::windows::runtime::IUnknown {
    fn from(value: TimeTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TimeTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &TimeTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TimeTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TimeTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<TimeTrigger> for ::windows::runtime::IInspectable {
    fn from(value: TimeTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TimeTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &TimeTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TimeTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TimeTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<TimeTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TimeTrigger) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TimeTrigger> for IBackgroundTrigger {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TimeTrigger) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for TimeTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &TimeTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::std::convert::TryInto::<IBackgroundTrigger>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ToastNotificationActionTrigger(::windows::runtime::IInspectable);
impl ToastNotificationActionTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ToastNotificationActionTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        applicationid: Param0,
    ) -> ::windows::runtime::Result<ToastNotificationActionTrigger> {
        Self::IToastNotificationActionTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                applicationid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ToastNotificationActionTrigger>(result__)
        })
    }
    pub fn IToastNotificationActionTriggerFactory<
        R,
        F: FnOnce(&IToastNotificationActionTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ToastNotificationActionTrigger,
            IToastNotificationActionTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ToastNotificationActionTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.ToastNotificationActionTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})" ) ;
}
unsafe impl ::windows::runtime::Interface for ToastNotificationActionTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl ::windows::runtime::RuntimeName for ToastNotificationActionTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ToastNotificationActionTrigger";
}
impl ::std::convert::From<ToastNotificationActionTrigger> for ::windows::runtime::IUnknown {
    fn from(value: ToastNotificationActionTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ToastNotificationActionTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &ToastNotificationActionTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ToastNotificationActionTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ToastNotificationActionTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ToastNotificationActionTrigger> for ::windows::runtime::IInspectable {
    fn from(value: ToastNotificationActionTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ToastNotificationActionTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &ToastNotificationActionTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ToastNotificationActionTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ToastNotificationActionTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ToastNotificationActionTrigger> for IBackgroundTrigger {
    fn from(value: ToastNotificationActionTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ToastNotificationActionTrigger> for IBackgroundTrigger {
    fn from(value: &ToastNotificationActionTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for ToastNotificationActionTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &ToastNotificationActionTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for ToastNotificationActionTrigger {}
unsafe impl ::std::marker::Sync for ToastNotificationActionTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ToastNotificationHistoryChangedTrigger(::windows::runtime::IInspectable);
impl ToastNotificationHistoryChangedTrigger {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ToastNotificationHistoryChangedTrigger,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        applicationid: Param0,
    ) -> ::windows::runtime::Result<ToastNotificationHistoryChangedTrigger> {
        Self::IToastNotificationHistoryChangedTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                applicationid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ToastNotificationHistoryChangedTrigger>(result__)
        })
    }
    pub fn IToastNotificationHistoryChangedTriggerFactory<
        R,
        F: FnOnce(&IToastNotificationHistoryChangedTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ToastNotificationHistoryChangedTrigger,
            IToastNotificationHistoryChangedTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ToastNotificationHistoryChangedTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.ToastNotificationHistoryChangedTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})" ) ;
}
unsafe impl ::windows::runtime::Interface for ToastNotificationHistoryChangedTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl ::windows::runtime::RuntimeName for ToastNotificationHistoryChangedTrigger {
    const NAME: &'static str =
        "Windows.ApplicationModel.Background.ToastNotificationHistoryChangedTrigger";
}
impl ::std::convert::From<ToastNotificationHistoryChangedTrigger> for ::windows::runtime::IUnknown {
    fn from(value: ToastNotificationHistoryChangedTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ToastNotificationHistoryChangedTrigger>
    for ::windows::runtime::IUnknown
{
    fn from(value: &ToastNotificationHistoryChangedTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ToastNotificationHistoryChangedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ToastNotificationHistoryChangedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ToastNotificationHistoryChangedTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: ToastNotificationHistoryChangedTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ToastNotificationHistoryChangedTrigger>
    for ::windows::runtime::IInspectable
{
    fn from(value: &ToastNotificationHistoryChangedTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ToastNotificationHistoryChangedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ToastNotificationHistoryChangedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ToastNotificationHistoryChangedTrigger> for IBackgroundTrigger {
    fn from(value: ToastNotificationHistoryChangedTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ToastNotificationHistoryChangedTrigger> for IBackgroundTrigger {
    fn from(value: &ToastNotificationHistoryChangedTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for ToastNotificationHistoryChangedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger>
    for &ToastNotificationHistoryChangedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for ToastNotificationHistoryChangedTrigger {}
unsafe impl ::std::marker::Sync for ToastNotificationHistoryChangedTrigger {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct UserNotificationChangedTrigger(::windows::runtime::IInspectable);
impl UserNotificationChangedTrigger {
    #[cfg(feature = "UI_Notifications")]
    pub fn Create(
        notificationkinds: super::super::UI::Notifications::NotificationKinds,
    ) -> ::windows::runtime::Result<UserNotificationChangedTrigger> {
        Self::IUserNotificationChangedTriggerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                notificationkinds,
                &mut result__,
            )
            .from_abi::<UserNotificationChangedTrigger>(result__)
        })
    }
    pub fn IUserNotificationChangedTriggerFactory<
        R,
        F: FnOnce(&IUserNotificationChangedTriggerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            UserNotificationChangedTrigger,
            IUserNotificationChangedTriggerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserNotificationChangedTrigger {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.Background.UserNotificationChangedTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})" ) ;
}
unsafe impl ::windows::runtime::Interface for UserNotificationChangedTrigger {
    type Vtable = IBackgroundTrigger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2226364504,
        24615,
        19335,
        [151, 144, 189, 243, 247, 87, 219, 215],
    );
}
impl ::windows::runtime::RuntimeName for UserNotificationChangedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.UserNotificationChangedTrigger";
}
impl ::std::convert::From<UserNotificationChangedTrigger> for ::windows::runtime::IUnknown {
    fn from(value: UserNotificationChangedTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserNotificationChangedTrigger> for ::windows::runtime::IUnknown {
    fn from(value: &UserNotificationChangedTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for UserNotificationChangedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &UserNotificationChangedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<UserNotificationChangedTrigger> for ::windows::runtime::IInspectable {
    fn from(value: UserNotificationChangedTrigger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserNotificationChangedTrigger> for ::windows::runtime::IInspectable {
    fn from(value: &UserNotificationChangedTrigger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for UserNotificationChangedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a UserNotificationChangedTrigger
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<UserNotificationChangedTrigger> for IBackgroundTrigger {
    fn from(value: UserNotificationChangedTrigger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserNotificationChangedTrigger> for IBackgroundTrigger {
    fn from(value: &UserNotificationChangedTrigger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for UserNotificationChangedTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTrigger> for &UserNotificationChangedTrigger {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTrigger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBackgroundTrigger>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for UserNotificationChangedTrigger {}
unsafe impl ::std::marker::Sync for UserNotificationChangedTrigger {}
