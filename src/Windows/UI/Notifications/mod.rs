#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "UI_Notifications_Management")]
pub mod Management;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AdaptiveNotificationContentKind(pub i32);
impl AdaptiveNotificationContentKind {
    pub const Text: AdaptiveNotificationContentKind = AdaptiveNotificationContentKind(0i32);
}
impl ::std::convert::From<i32> for AdaptiveNotificationContentKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AdaptiveNotificationContentKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveNotificationContentKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Notifications.AdaptiveNotificationContentKind;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AdaptiveNotificationText(::windows::runtime::IInspectable);
impl AdaptiveNotificationText {
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
            AdaptiveNotificationText,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<AdaptiveNotificationContentKind> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveNotificationContent>(self)?;
        unsafe {
            let mut result__: AdaptiveNotificationContentKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AdaptiveNotificationContentKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Hints(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IMap<
            ::windows::runtime::HSTRING,
            ::windows::runtime::HSTRING,
        >,
    > {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveNotificationContent>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IMap<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveNotificationText {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Notifications.AdaptiveNotificationText;{46d4a3be-609a-4326-a40b-bfde872034a3})" ) ;
}
unsafe impl ::windows::runtime::Interface for AdaptiveNotificationText {
    type Vtable = IAdaptiveNotificationText_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1188340670,
        24730,
        17190,
        [164, 11, 191, 222, 135, 32, 52, 163],
    );
}
impl ::windows::runtime::RuntimeName for AdaptiveNotificationText {
    const NAME: &'static str = "Windows.UI.Notifications.AdaptiveNotificationText";
}
impl ::std::convert::From<AdaptiveNotificationText> for ::windows::runtime::IUnknown {
    fn from(value: AdaptiveNotificationText) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AdaptiveNotificationText> for ::windows::runtime::IUnknown {
    fn from(value: &AdaptiveNotificationText) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AdaptiveNotificationText
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AdaptiveNotificationText
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AdaptiveNotificationText> for ::windows::runtime::IInspectable {
    fn from(value: AdaptiveNotificationText) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AdaptiveNotificationText> for ::windows::runtime::IInspectable {
    fn from(value: &AdaptiveNotificationText) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AdaptiveNotificationText
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AdaptiveNotificationText
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<AdaptiveNotificationText> for IAdaptiveNotificationContent {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AdaptiveNotificationText) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AdaptiveNotificationText> for IAdaptiveNotificationContent {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AdaptiveNotificationText) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAdaptiveNotificationContent>
    for AdaptiveNotificationText
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IAdaptiveNotificationContent> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAdaptiveNotificationContent>
    for &AdaptiveNotificationText
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IAdaptiveNotificationContent> {
        ::std::convert::TryInto::<IAdaptiveNotificationContent>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AdaptiveNotificationText {}
unsafe impl ::std::marker::Sync for AdaptiveNotificationText {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BadgeNotification(::windows::runtime::IInspectable);
impl BadgeNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::IReference<super::super::Foundation::DateTime>,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IReference<super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(
                result__,
            )
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateBadgeNotification<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>,
    >(
        content: Param0,
    ) -> ::windows::runtime::Result<BadgeNotification> {
        Self::IBadgeNotificationFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                content.into_param().abi(),
                &mut result__,
            )
            .from_abi::<BadgeNotification>(result__)
        })
    }
    pub fn IBadgeNotificationFactory<
        R,
        F: FnOnce(&IBadgeNotificationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            BadgeNotification,
            IBadgeNotificationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BadgeNotification {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.Notifications.BadgeNotification;{075cb4ca-d08a-4e2f-9233-7e289c1f7722})",
    );
}
unsafe impl ::windows::runtime::Interface for BadgeNotification {
    type Vtable = IBadgeNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        123516106,
        53386,
        20015,
        [146, 51, 126, 40, 156, 31, 119, 34],
    );
}
impl ::windows::runtime::RuntimeName for BadgeNotification {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeNotification";
}
impl ::std::convert::From<BadgeNotification> for ::windows::runtime::IUnknown {
    fn from(value: BadgeNotification) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BadgeNotification> for ::windows::runtime::IUnknown {
    fn from(value: &BadgeNotification) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BadgeNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BadgeNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<BadgeNotification> for ::windows::runtime::IInspectable {
    fn from(value: BadgeNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BadgeNotification> for ::windows::runtime::IInspectable {
    fn from(value: &BadgeNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BadgeNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a BadgeNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BadgeNotification {}
unsafe impl ::std::marker::Sync for BadgeNotification {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct BadgeTemplateType(pub i32);
impl BadgeTemplateType {
    pub const BadgeGlyph: BadgeTemplateType = BadgeTemplateType(0i32);
    pub const BadgeNumber: BadgeTemplateType = BadgeTemplateType(1i32);
}
impl ::std::convert::From<i32> for BadgeTemplateType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BadgeTemplateType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BadgeTemplateType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Notifications.BadgeTemplateType;i4)",
    );
}
pub struct BadgeUpdateManager {}
impl BadgeUpdateManager {
    pub fn CreateBadgeUpdaterForApplication() -> ::windows::runtime::Result<BadgeUpdater> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BadgeUpdater>(result__)
        })
    }
    pub fn CreateBadgeUpdaterForApplicationWithId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        applicationid: Param0,
    ) -> ::windows::runtime::Result<BadgeUpdater> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                applicationid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<BadgeUpdater>(result__)
        })
    }
    pub fn CreateBadgeUpdaterForSecondaryTile<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        tileid: Param0,
    ) -> ::windows::runtime::Result<BadgeUpdater> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                tileid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<BadgeUpdater>(result__)
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(
        r#type: BadgeTemplateType,
    ) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                r#type,
                &mut result__,
            )
            .from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(
        user: Param0,
    ) -> ::windows::runtime::Result<BadgeUpdateManagerForUser> {
        Self::IBadgeUpdateManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                user.into_param().abi(),
                &mut result__,
            )
            .from_abi::<BadgeUpdateManagerForUser>(result__)
        })
    }
    pub fn IBadgeUpdateManagerStatics<
        R,
        F: FnOnce(&IBadgeUpdateManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            BadgeUpdateManager,
            IBadgeUpdateManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBadgeUpdateManagerStatics2<
        R,
        F: FnOnce(&IBadgeUpdateManagerStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            BadgeUpdateManager,
            IBadgeUpdateManagerStatics2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for BadgeUpdateManager {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeUpdateManager";
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BadgeUpdateManagerForUser(::windows::runtime::IInspectable);
impl BadgeUpdateManagerForUser {
    pub fn CreateBadgeUpdaterForApplication(&self) -> ::windows::runtime::Result<BadgeUpdater> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BadgeUpdater>(result__)
        }
    }
    pub fn CreateBadgeUpdaterForApplicationWithId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        applicationid: Param0,
    ) -> ::windows::runtime::Result<BadgeUpdater> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                applicationid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<BadgeUpdater>(result__)
        }
    }
    pub fn CreateBadgeUpdaterForSecondaryTile<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        tileid: Param0,
    ) -> ::windows::runtime::Result<BadgeUpdater> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                tileid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<BadgeUpdater>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BadgeUpdateManagerForUser {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Notifications.BadgeUpdateManagerForUser;{996b21bc-0386-44e5-ba8d-0c1077a62e92})" ) ;
}
unsafe impl ::windows::runtime::Interface for BadgeUpdateManagerForUser {
    type Vtable = IBadgeUpdateManagerForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2573935036,
        902,
        17637,
        [186, 141, 12, 16, 119, 166, 46, 146],
    );
}
impl ::windows::runtime::RuntimeName for BadgeUpdateManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeUpdateManagerForUser";
}
impl ::std::convert::From<BadgeUpdateManagerForUser> for ::windows::runtime::IUnknown {
    fn from(value: BadgeUpdateManagerForUser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BadgeUpdateManagerForUser> for ::windows::runtime::IUnknown {
    fn from(value: &BadgeUpdateManagerForUser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for BadgeUpdateManagerForUser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &BadgeUpdateManagerForUser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<BadgeUpdateManagerForUser> for ::windows::runtime::IInspectable {
    fn from(value: BadgeUpdateManagerForUser) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BadgeUpdateManagerForUser> for ::windows::runtime::IInspectable {
    fn from(value: &BadgeUpdateManagerForUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for BadgeUpdateManagerForUser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a BadgeUpdateManagerForUser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BadgeUpdateManagerForUser {}
unsafe impl ::std::marker::Sync for BadgeUpdateManagerForUser {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BadgeUpdater(::windows::runtime::IInspectable);
impl BadgeUpdater {
    pub fn Update<'a, Param0: ::windows::runtime::IntoParam<'a, BadgeNotification>>(
        &self,
        notification: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                notification.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
    >(
        &self,
        badgecontent: Param0,
        requestedinterval: PeriodicUpdateRecurrence,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                badgecontent.into_param().abi(),
                requestedinterval,
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdateAtTime<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>,
    >(
        &self,
        badgecontent: Param0,
        starttime: Param1,
        requestedinterval: PeriodicUpdateRecurrence,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                badgecontent.into_param().abi(),
                starttime.into_param().abi(),
                requestedinterval,
            )
            .ok()
        }
    }
    pub fn StopPeriodicUpdate(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BadgeUpdater {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.Notifications.BadgeUpdater;{b5fa1fd4-7562-4f6c-bfa3-1b6ed2e57f2f})",
    );
}
unsafe impl ::windows::runtime::Interface for BadgeUpdater {
    type Vtable = IBadgeUpdater_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3053068244,
        30050,
        20332,
        [191, 163, 27, 110, 210, 229, 127, 47],
    );
}
impl ::windows::runtime::RuntimeName for BadgeUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeUpdater";
}
impl ::std::convert::From<BadgeUpdater> for ::windows::runtime::IUnknown {
    fn from(value: BadgeUpdater) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BadgeUpdater> for ::windows::runtime::IUnknown {
    fn from(value: &BadgeUpdater) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BadgeUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BadgeUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<BadgeUpdater> for ::windows::runtime::IInspectable {
    fn from(value: BadgeUpdater) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BadgeUpdater> for ::windows::runtime::IInspectable {
    fn from(value: &BadgeUpdater) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BadgeUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BadgeUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BadgeUpdater {}
unsafe impl ::std::marker::Sync for BadgeUpdater {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IAdaptiveNotificationContent(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveNotificationContent {
    type Vtable = IAdaptiveNotificationContent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3943546470,
        29768,
        17549,
        [157, 184, 215, 138, 205, 42, 187, 169],
    );
}
impl IAdaptiveNotificationContent {
    pub fn Kind(&self) -> ::windows::runtime::Result<AdaptiveNotificationContentKind> {
        let this = self;
        unsafe {
            let mut result__: AdaptiveNotificationContentKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AdaptiveNotificationContentKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Hints(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IMap<
            ::windows::runtime::HSTRING,
            ::windows::runtime::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IMap<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAdaptiveNotificationContent {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{eb0dbe66-7448-448d-9db8-d78acd2abba9}");
}
impl ::std::convert::From<IAdaptiveNotificationContent> for ::windows::runtime::IUnknown {
    fn from(value: IAdaptiveNotificationContent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAdaptiveNotificationContent> for ::windows::runtime::IUnknown {
    fn from(value: &IAdaptiveNotificationContent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IAdaptiveNotificationContent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IAdaptiveNotificationContent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IAdaptiveNotificationContent> for ::windows::runtime::IInspectable {
    fn from(value: IAdaptiveNotificationContent) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAdaptiveNotificationContent> for ::windows::runtime::IInspectable {
    fn from(value: &IAdaptiveNotificationContent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IAdaptiveNotificationContent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IAdaptiveNotificationContent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveNotificationContent_abi(
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
        result__: *mut AdaptiveNotificationContentKind,
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
pub struct IAdaptiveNotificationText(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveNotificationText {
    type Vtable = IAdaptiveNotificationText_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1188340670,
        24730,
        17190,
        [164, 11, 191, 222, 135, 32, 52, 163],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveNotificationText_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBadgeNotification(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBadgeNotification {
    type Vtable = IBadgeNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        123516106,
        53386,
        20015,
        [146, 51, 126, 40, 156, 31, 119, 34],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeNotification_abi(
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
    #[cfg(feature = "Data_Xml_Dom")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
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
pub struct IBadgeNotificationFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBadgeNotificationFactory {
    type Vtable = IBadgeNotificationFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3992081870,
        1560,
        19801,
        [148, 138, 90, 97, 4, 12, 82, 249],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeNotificationFactory_abi(
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
    #[cfg(feature = "Data_Xml_Dom")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        content: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerForUser(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBadgeUpdateManagerForUser {
    type Vtable = IBadgeUpdateManagerForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2573935036,
        902,
        17637,
        [186, 141, 12, 16, 119, 166, 46, 146],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerForUser_abi(
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
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tileid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
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
pub struct IBadgeUpdateManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBadgeUpdateManagerStatics {
    type Vtable = IBadgeUpdateManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        859836330,
        28117,
        16645,
        [174, 188, 155, 80, 252, 164, 146, 218],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerStatics_abi(
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
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tileid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        r#type: BadgeTemplateType,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBadgeUpdateManagerStatics2 {
    type Vtable = IBadgeUpdateManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2543465934,
        63808,
        18623,
        [148, 232, 202, 36, 77, 64, 11, 65],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerStatics2_abi(
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
        user: ::windows::runtime::RawPtr,
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
pub struct IBadgeUpdater(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBadgeUpdater {
    type Vtable = IBadgeUpdater_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3053068244,
        30050,
        20332,
        [191, 163, 27, 110, 210, 229, 127, 47],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeUpdater_abi(
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
        notification: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        badgecontent: ::windows::runtime::RawPtr,
        requestedinterval: PeriodicUpdateRecurrence,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        badgecontent: ::windows::runtime::RawPtr,
        starttime: super::super::Foundation::DateTime,
        requestedinterval: PeriodicUpdateRecurrence,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
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
pub struct IKnownAdaptiveNotificationHintsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownAdaptiveNotificationHintsStatics {
    type Vtable = IKnownAdaptiveNotificationHintsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        102786456,
        54422,
        18813,
        [134, 146, 79, 125, 124, 39, 112, 223],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownAdaptiveNotificationHintsStatics_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IKnownAdaptiveNotificationTextStylesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownAdaptiveNotificationTextStylesStatics {
    type Vtable = IKnownAdaptiveNotificationTextStylesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        539071191,
        35222,
        17834,
        [139, 161, 212, 97, 215, 44, 42, 27],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownAdaptiveNotificationTextStylesStatics_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IKnownNotificationBindingsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownNotificationBindingsStatics {
    type Vtable = IKnownNotificationBindingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2034400174,
        43191,
        19800,
        [137, 234, 118, 167, 183, 188, 205, 237],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownNotificationBindingsStatics_abi(
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
pub struct INotification(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INotification {
    type Vtable = INotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        276838398,
        60278,
        20354,
        [151, 188, 218, 7, 83, 10, 46, 32],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INotification_abi(
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
pub struct INotificationBinding(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INotificationBinding {
    type Vtable = INotificationBinding_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4070460293,
        880,
        19155,
        [180, 234, 218, 158, 53, 231, 234, 191],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationBinding_abi(
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
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
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
pub struct INotificationData(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INotificationData {
    type Vtable = INotificationData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2684166930,
        40298,
        19119,
        [182, 172, 255, 23, 240, 193, 242, 128],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationData_abi(
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
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
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
pub struct INotificationDataFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INotificationDataFactory {
    type Vtable = INotificationDataFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        599909178,
        7184,
        18171,
        [128, 64, 222, 195, 132, 98, 28, 248],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationDataFactory_abi(
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
        initialvalues: ::windows::runtime::RawPtr,
        sequencenumber: u32,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        initialvalues: ::windows::runtime::RawPtr,
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
pub struct INotificationVisual(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INotificationVisual {
    type Vtable = INotificationVisual_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1753439118,
        43606,
        19985,
        [134, 211, 95, 154, 105, 87, 188, 91],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationVisual_abi(
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
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        templatename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IScheduledTileNotification(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScheduledTileNotification {
    type Vtable = IScheduledTileNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        180135637,
        39388,
        19576,
        [161, 28, 201, 231, 248, 109, 126, 247],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledTileNotification_abi(
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
    #[cfg(feature = "Data_Xml_Dom")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::DateTime,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IScheduledTileNotificationFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScheduledTileNotificationFactory {
    type Vtable = IScheduledTileNotificationFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        864228234,
        39104,
        19515,
        [187, 214, 74, 99, 60, 124, 252, 41],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledTileNotificationFactory_abi(
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
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        content: ::windows::runtime::RawPtr,
        deliverytime: super::super::Foundation::DateTime,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Data_Xml_Dom", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IScheduledToastNotification(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScheduledToastNotification {
    type Vtable = IScheduledToastNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2046130168,
        3559,
        18637,
        [151, 64, 155, 55, 4, 144, 200, 56],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotification_abi(
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
    #[cfg(feature = "Data_Xml_Dom")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::DateTime,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IScheduledToastNotification2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScheduledToastNotification2 {
    type Vtable = IScheduledToastNotification2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2792267932,
        12724,
        17328,
        [181, 221, 122, 64, 232, 83, 99, 177],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotification2_abi(
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
pub struct IScheduledToastNotification3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScheduledToastNotification3 {
    type Vtable = IScheduledToastNotification3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2554502795,
        48434,
        19003,
        [157, 21, 34, 174, 164, 148, 98, 161],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotification3_abi(
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
        result__: *mut NotificationMirroring,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: NotificationMirroring,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IScheduledToastNotification4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScheduledToastNotification4 {
    type Vtable = IScheduledToastNotification4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        491217405,
        48623,
        20042,
        [150, 190, 1, 1, 54, 155, 88, 210],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotification4_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IScheduledToastNotificationFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScheduledToastNotificationFactory {
    type Vtable = IScheduledToastNotificationFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3888042385,
        3001,
        16777,
        [131, 148, 49, 118, 27, 71, 111, 215],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotificationFactory_abi(
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
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        content: ::windows::runtime::RawPtr,
        deliverytime: super::super::Foundation::DateTime,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Data_Xml_Dom", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        content: ::windows::runtime::RawPtr,
        deliverytime: super::super::Foundation::DateTime,
        snoozeinterval: super::super::Foundation::TimeSpan,
        maximumsnoozecount: u32,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Data_Xml_Dom", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IScheduledToastNotificationShowingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScheduledToastNotificationShowingEventArgs {
    type Vtable = IScheduledToastNotificationShowingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1634989748,
        16682,
        24108,
        [166, 237, 160, 32, 154, 239, 154, 9],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotificationShowingEventArgs_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
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
pub struct IShownTileNotification(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IShownTileNotification {
    type Vtable = IShownTileNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        875399560,
        23282,
        18458,
        [166, 163, 242, 253, 199, 141, 232, 142],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IShownTileNotification_abi(
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
pub struct ITileFlyoutNotification(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITileFlyoutNotification {
    type Vtable = ITileFlyoutNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2589176417,
        50956,
        17086,
        [178, 243, 244, 42, 169, 125, 52, 229],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileFlyoutNotification_abi(
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
    #[cfg(feature = "Data_Xml_Dom")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
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
pub struct ITileFlyoutNotificationFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITileFlyoutNotificationFactory {
    type Vtable = ITileFlyoutNotificationFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4015353845,
        21030,
        20267,
        [178, 120, 136, 163, 93, 254, 86, 159],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileFlyoutNotificationFactory_abi(
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
    #[cfg(feature = "Data_Xml_Dom")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        content: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ITileFlyoutUpdateManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITileFlyoutUpdateManagerStatics {
    type Vtable = ITileFlyoutUpdateManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        70662923,
        6848,
        19353,
        [136, 231, 173, 168, 62, 149, 61, 72],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileFlyoutUpdateManagerStatics_abi(
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
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tileid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        r#type: TileFlyoutTemplateType,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ITileFlyoutUpdater(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITileFlyoutUpdater {
    type Vtable = ITileFlyoutUpdater_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2369832810,
        50277,
        16466,
        [167, 64, 92, 38, 84, 193, 160, 137],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileFlyoutUpdater_abi(
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
        notification: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tileflyoutcontent: ::windows::runtime::RawPtr,
        requestedinterval: PeriodicUpdateRecurrence,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tileflyoutcontent: ::windows::runtime::RawPtr,
        starttime: super::super::Foundation::DateTime,
        requestedinterval: PeriodicUpdateRecurrence,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut NotificationSetting,
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
pub struct ITileNotification(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITileNotification {
    type Vtable = ITileNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3954100474,
        20716,
        19480,
        [180, 208, 58, 240, 46, 85, 64, 171],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileNotification_abi(
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
    #[cfg(feature = "Data_Xml_Dom")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct ITileNotificationFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITileNotificationFactory {
    type Vtable = ITileNotificationFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3333152110,
        18728,
        18120,
        [189, 191, 129, 160, 71, 222, 160, 212],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileNotificationFactory_abi(
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
    #[cfg(feature = "Data_Xml_Dom")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        content: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ITileUpdateManagerForUser(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITileUpdateManagerForUser {
    type Vtable = ITileUpdateManagerForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1427379016,
        12002,
        20013,
        [156, 193, 33, 106, 32, 222, 204, 159],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdateManagerForUser_abi(
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
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tileid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
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
pub struct ITileUpdateManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITileUpdateManagerStatics {
    type Vtable = ITileUpdateManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3658849885,
        16041,
        18822,
        [141, 132, 176, 157, 94, 18, 39, 109],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdateManagerStatics_abi(
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
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tileid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        r#type: TileTemplateType,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ITileUpdateManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITileUpdateManagerStatics2 {
    type Vtable = ITileUpdateManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1931222492,
        36372,
        19324,
        [163, 75, 157, 34, 222, 118, 200, 77],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdateManagerStatics2_abi(
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
        user: ::windows::runtime::RawPtr,
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
pub struct ITileUpdater(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITileUpdater {
    type Vtable = ITileUpdater_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        155362443,
        7569,
        17644,
        [146, 67, 193, 232, 33, 194, 154, 32],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdater_abi(
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
        notification: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enable: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut NotificationSetting,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scheduledtile: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scheduledtile: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tilecontent: ::windows::runtime::RawPtr,
        requestedinterval: PeriodicUpdateRecurrence,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tilecontent: ::windows::runtime::RawPtr,
        starttime: super::super::Foundation::DateTime,
        requestedinterval: PeriodicUpdateRecurrence,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tilecontents: ::windows::runtime::RawPtr,
        requestedinterval: PeriodicUpdateRecurrence,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tilecontents: ::windows::runtime::RawPtr,
        starttime: super::super::Foundation::DateTime,
        requestedinterval: PeriodicUpdateRecurrence,
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
pub struct ITileUpdater2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITileUpdater2 {
    type Vtable = ITileUpdater2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2720427538,
        5614,
        17389,
        [131, 245, 101, 179, 82, 187, 26, 132],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdater2_abi(
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
        enable: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enable: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enable: bool,
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
pub struct IToastActivatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastActivatedEventArgs {
    type Vtable = IToastActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3820983027,
        49559,
        17263,
        [130, 101, 6, 37, 130, 79, 141, 172],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastActivatedEventArgs_abi(
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
pub struct IToastActivatedEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastActivatedEventArgs2 {
    type Vtable = IToastActivatedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2877138194,
        52321,
        22158,
        [129, 190, 48, 74, 195, 16, 56, 250],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastActivatedEventArgs2_abi(
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
pub struct IToastCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastCollection {
    type Vtable = IToastCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        176931760,
        57534,
        18520,
        [188, 42, 137, 223, 224, 179, 40, 99],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastCollection_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IToastCollectionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastCollectionFactory {
    type Vtable = IToastCollectionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        374199255,
        29636,
        17655,
        [180, 255, 251, 109, 75, 241, 244, 198],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastCollectionFactory_abi(
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
        collectionid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        launchargs: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        iconuri: ::windows::runtime::RawPtr,
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
pub struct IToastCollectionManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastCollectionManager {
    type Vtable = IToastCollectionManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        706224638,
        6045,
        18876,
        [183, 157, 165, 39, 146, 13, 54, 101],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastCollectionManager_abi(
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
        collection: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        collectionid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        collectionid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "System")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
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
pub struct IToastDismissedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastDismissedEventArgs {
    type Vtable = IToastDismissedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1065998645,
        55755,
        17720,
        [160, 240, 255, 231, 101, 153, 56, 248],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastDismissedEventArgs_abi(
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
        result__: *mut ToastDismissalReason,
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
pub struct IToastFailedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastFailedEventArgs {
    type Vtable = IToastFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        890726498,
        53204,
        17656,
        [173, 100, 245, 0, 253, 137, 108, 59],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastFailedEventArgs_abi(
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
        result__: *mut ::windows::runtime::HRESULT,
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
pub struct IToastNotification(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotification {
    type Vtable = IToastNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2575181429,
        1438,
        20064,
        [139, 6, 23, 96, 145, 124, 139, 128],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification_abi(
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
    #[cfg(feature = "Data_Xml_Dom")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
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
        token: super::super::Foundation::EventRegistrationToken,
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
        token: super::super::Foundation::EventRegistrationToken,
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
        token: super::super::Foundation::EventRegistrationToken,
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
pub struct IToastNotification2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotification2 {
    type Vtable = IToastNotification2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2650513361,
        5178,
        18702,
        [144, 191, 185, 251, 167, 19, 45, 231],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification2_abi(
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
pub struct IToastNotification3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotification3 {
    type Vtable = IToastNotification3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        837332696,
        33089,
        20377,
        [188, 10, 196, 237, 33, 41, 125, 119],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification3_abi(
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
        result__: *mut NotificationMirroring,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: NotificationMirroring,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IToastNotification4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotification4 {
    type Vtable = IToastNotification4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        353716533,
        10474,
        18215,
        [136, 233, 197, 134, 128, 226, 209, 24],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification4_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ToastNotificationPriority,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ToastNotificationPriority,
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
pub struct IToastNotification6(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotification6 {
    type Vtable = IToastNotification6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1139539539,
        35246,
        23582,
        [162, 121, 58, 236, 254, 155, 111, 84],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification6_abi(
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
pub struct IToastNotificationActionTriggerDetail(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationActionTriggerDetail {
    type Vtable = IToastNotificationActionTriggerDetail_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2487554906,
        14579,
        17142,
        [150, 170, 121, 85, 176, 240, 61, 162],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationActionTriggerDetail_abi(
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
pub struct IToastNotificationFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationFactory {
    type Vtable = IToastNotificationFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        68307744,
        33478,
        16937,
        [177, 9, 253, 158, 212, 102, 43, 83],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationFactory_abi(
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
    #[cfg(feature = "Data_Xml_Dom")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        content: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IToastNotificationHistory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationHistory {
    type Vtable = IToastNotificationHistory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1554898019,
        467,
        19607,
        [152, 111, 5, 51, 72, 63, 238, 20],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistory_abi(
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
        group: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        group: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tag: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        group: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tag: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        group: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tag: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IToastNotificationHistory2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationHistory2 {
    type Vtable = IToastNotificationHistory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1002689107,
        12081,
        16530,
        [145, 41, 138, 213, 171, 240, 103, 218],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistory2_abi(
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
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IToastNotificationHistoryChangedTriggerDetail(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationHistoryChangedTriggerDetail {
    type Vtable = IToastNotificationHistoryChangedTriggerDetail_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3674439674,
        104,
        16684,
        [156, 131, 38, 124, 55, 246, 86, 112],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistoryChangedTriggerDetail_abi(
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
        result__: *mut ToastHistoryChangedType,
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
pub struct IToastNotificationHistoryChangedTriggerDetail2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationHistoryChangedTriggerDetail2 {
    type Vtable = IToastNotificationHistoryChangedTriggerDetail2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        188148098,
        51313,
        18939,
        [186, 187, 37, 189, 188, 76, 196, 91],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistoryChangedTriggerDetail2_abi(
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
pub struct IToastNotificationManagerForUser(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationManagerForUser {
    type Vtable = IToastNotificationManagerForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2041272310,
        17406,
        18555,
        [138, 127, 153, 86, 114, 0, 174, 148],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerForUser_abi(
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
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
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
pub struct IToastNotificationManagerForUser2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationManagerForUser2 {
    type Vtable = IToastNotificationManagerForUser2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1738302647,
        33195,
        17090,
        [136, 25, 201, 88, 118, 119, 83, 244],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerForUser2_abi(
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
        collectionid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        collectionid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IToastNotificationManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationManagerStatics {
    type Vtable = IToastNotificationManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1353453631,
        53813,
        17816,
        [187, 239, 152, 254, 77, 26, 58, 212],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics_abi(
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
        applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        r#type: ToastTemplateType,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationManagerStatics2 {
    type Vtable = IToastNotificationManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2058959954,
        3656,
        18256,
        [186, 157, 26, 65, 19, 152, 24, 71],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics2_abi(
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
pub struct IToastNotificationManagerStatics4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationManagerStatics4 {
    type Vtable = IToastNotificationManagerStatics4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2409185235,
        58646,
        17915,
        [129, 48, 57, 142, 147, 250, 82, 195],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics4_abi(
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
        user: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: NotificationMirroring,
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
pub struct IToastNotificationManagerStatics5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationManagerStatics5 {
    type Vtable = IToastNotificationManagerStatics5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3606443369,
        54285,
        16508,
        [137, 137, 136, 202, 180, 44, 253, 20],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics5_abi(
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
pub struct IToastNotifier(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotifier {
    type Vtable = IToastNotifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1972534163,
        1011,
        16876,
        [145, 211, 110, 91, 172, 27, 56, 231],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotifier_abi(
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
        notification: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        notification: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut NotificationSetting,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scheduledtoast: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scheduledtoast: ::windows::runtime::RawPtr,
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
pub struct IToastNotifier2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotifier2 {
    type Vtable = IToastNotifier2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        893618630,
        31745,
        19413,
        [156, 32, 96, 67, 64, 205, 43, 116],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotifier2_abi(
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
        data: ::windows::runtime::RawPtr,
        tag: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        group: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut NotificationUpdateResult,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        data: ::windows::runtime::RawPtr,
        tag: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut NotificationUpdateResult,
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
pub struct IToastNotifier3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotifier3 {
    type Vtable = IToastNotifier3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2926944330,
        15116,
        20909,
        [183, 232, 176, 138, 182, 5, 37, 73],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotifier3_abi(
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
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
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
pub struct IUserNotification(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserNotification {
    type Vtable = IUserNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2918704431,
        20051,
        17109,
        [156, 51, 235, 94, 165, 21, 178, 62],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotification_abi(
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
    #[cfg(feature = "ApplicationModel")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::DateTime,
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
pub struct IUserNotificationChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserNotificationChangedEventArgs {
    type Vtable = IUserNotificationChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3065866297,
        31183,
        19237,
        [130, 192, 12, 225, 238, 248, 31, 140],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotificationChangedEventArgs_abi(
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
        result__: *mut UserNotificationChangedKind,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
pub struct KnownAdaptiveNotificationHints {}
impl KnownAdaptiveNotificationHints {
    pub fn Style() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn Wrap() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn MaxLines() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn MinLines() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn TextStacking() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn Align() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IKnownAdaptiveNotificationHintsStatics<
        R,
        F: FnOnce(&IKnownAdaptiveNotificationHintsStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            KnownAdaptiveNotificationHints,
            IKnownAdaptiveNotificationHintsStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KnownAdaptiveNotificationHints {
    const NAME: &'static str = "Windows.UI.Notifications.KnownAdaptiveNotificationHints";
}
pub struct KnownAdaptiveNotificationTextStyles {}
impl KnownAdaptiveNotificationTextStyles {
    pub fn Caption() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn Body() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn Base() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn Subtitle() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn Title() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn Subheader() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn Header() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn TitleNumeral() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn SubheaderNumeral() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn HeaderNumeral() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn CaptionSubtle() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn BodySubtle() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn BaseSubtle() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn SubtitleSubtle() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn TitleSubtle() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn SubheaderSubtle() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn SubheaderNumeralSubtle() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn HeaderSubtle() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn HeaderNumeralSubtle() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IKnownAdaptiveNotificationTextStylesStatics<
        R,
        F: FnOnce(&IKnownAdaptiveNotificationTextStylesStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            KnownAdaptiveNotificationTextStyles,
            IKnownAdaptiveNotificationTextStylesStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KnownAdaptiveNotificationTextStyles {
    const NAME: &'static str = "Windows.UI.Notifications.KnownAdaptiveNotificationTextStyles";
}
pub struct KnownNotificationBindings {}
impl KnownNotificationBindings {
    pub fn ToastGeneric() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownNotificationBindingsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IKnownNotificationBindingsStatics<
        R,
        F: FnOnce(&IKnownNotificationBindingsStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            KnownNotificationBindings,
            IKnownNotificationBindingsStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KnownNotificationBindings {
    const NAME: &'static str = "Windows.UI.Notifications.KnownNotificationBindings";
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Notification(::windows::runtime::IInspectable);
impl Notification {
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
            Notification,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IReference<super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(
                result__,
            )
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::IReference<super::super::Foundation::DateTime>,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Visual(&self) -> ::windows::runtime::Result<NotificationVisual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<NotificationVisual>(result__)
        }
    }
    pub fn SetVisual<'a, Param0: ::windows::runtime::IntoParam<'a, NotificationVisual>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Notification {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.Notifications.Notification;{108037fe-eb76-4f82-97bc-da07530a2e20})",
    );
}
unsafe impl ::windows::runtime::Interface for Notification {
    type Vtable = INotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        276838398,
        60278,
        20354,
        [151, 188, 218, 7, 83, 10, 46, 32],
    );
}
impl ::windows::runtime::RuntimeName for Notification {
    const NAME: &'static str = "Windows.UI.Notifications.Notification";
}
impl ::std::convert::From<Notification> for ::windows::runtime::IUnknown {
    fn from(value: Notification) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Notification> for ::windows::runtime::IUnknown {
    fn from(value: &Notification) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Notification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Notification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<Notification> for ::windows::runtime::IInspectable {
    fn from(value: Notification) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Notification> for ::windows::runtime::IInspectable {
    fn from(value: &Notification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Notification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Notification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Notification {}
unsafe impl ::std::marker::Sync for Notification {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct NotificationBinding(::windows::runtime::IInspectable);
impl NotificationBinding {
    pub fn Template(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetTemplate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Hints(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IMap<
            ::windows::runtime::HSTRING,
            ::windows::runtime::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IMap<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextElements(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<AdaptiveNotificationText>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<
                AdaptiveNotificationText,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NotificationBinding {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.Notifications.NotificationBinding;{f29e4b85-0370-4ad3-b4ea-da9e35e7eabf})",
    );
}
unsafe impl ::windows::runtime::Interface for NotificationBinding {
    type Vtable = INotificationBinding_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4070460293,
        880,
        19155,
        [180, 234, 218, 158, 53, 231, 234, 191],
    );
}
impl ::windows::runtime::RuntimeName for NotificationBinding {
    const NAME: &'static str = "Windows.UI.Notifications.NotificationBinding";
}
impl ::std::convert::From<NotificationBinding> for ::windows::runtime::IUnknown {
    fn from(value: NotificationBinding) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NotificationBinding> for ::windows::runtime::IUnknown {
    fn from(value: &NotificationBinding) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NotificationBinding {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NotificationBinding {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<NotificationBinding> for ::windows::runtime::IInspectable {
    fn from(value: NotificationBinding) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NotificationBinding> for ::windows::runtime::IInspectable {
    fn from(value: &NotificationBinding) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for NotificationBinding
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a NotificationBinding
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for NotificationBinding {}
unsafe impl ::std::marker::Sync for NotificationBinding {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct NotificationData(::windows::runtime::IInspectable);
impl NotificationData {
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
            NotificationData,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Values(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IMap<
            ::windows::runtime::HSTRING,
            ::windows::runtime::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IMap<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
    pub fn SequenceNumber(&self) -> ::windows::runtime::Result<u32> {
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
    pub fn SetSequenceNumber(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNotificationDataWithValuesAndSequenceNumber<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<
                super::super::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    ::windows::runtime::HSTRING,
                >,
            >,
        >,
    >(
        initialvalues: Param0,
        sequencenumber: u32,
    ) -> ::windows::runtime::Result<NotificationData> {
        Self::INotificationDataFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                initialvalues.into_param().abi(),
                sequencenumber,
                &mut result__,
            )
            .from_abi::<NotificationData>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNotificationDataWithValues<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<
                super::super::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    ::windows::runtime::HSTRING,
                >,
            >,
        >,
    >(
        initialvalues: Param0,
    ) -> ::windows::runtime::Result<NotificationData> {
        Self::INotificationDataFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                initialvalues.into_param().abi(),
                &mut result__,
            )
            .from_abi::<NotificationData>(result__)
        })
    }
    pub fn INotificationDataFactory<
        R,
        F: FnOnce(&INotificationDataFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            NotificationData,
            INotificationDataFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NotificationData {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.Notifications.NotificationData;{9ffd2312-9d6a-4aaf-b6ac-ff17f0c1f280})",
    );
}
unsafe impl ::windows::runtime::Interface for NotificationData {
    type Vtable = INotificationData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2684166930,
        40298,
        19119,
        [182, 172, 255, 23, 240, 193, 242, 128],
    );
}
impl ::windows::runtime::RuntimeName for NotificationData {
    const NAME: &'static str = "Windows.UI.Notifications.NotificationData";
}
impl ::std::convert::From<NotificationData> for ::windows::runtime::IUnknown {
    fn from(value: NotificationData) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NotificationData> for ::windows::runtime::IUnknown {
    fn from(value: &NotificationData) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NotificationData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NotificationData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<NotificationData> for ::windows::runtime::IInspectable {
    fn from(value: NotificationData) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NotificationData> for ::windows::runtime::IInspectable {
    fn from(value: &NotificationData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NotificationData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a NotificationData
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for NotificationData {}
unsafe impl ::std::marker::Sync for NotificationData {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct NotificationKinds(pub u32);
impl NotificationKinds {
    pub const Unknown: NotificationKinds = NotificationKinds(0u32);
    pub const Toast: NotificationKinds = NotificationKinds(1u32);
}
impl ::std::convert::From<u32> for NotificationKinds {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NotificationKinds {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NotificationKinds {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Notifications.NotificationKinds;u4)",
    );
}
impl ::std::ops::BitOr for NotificationKinds {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NotificationKinds {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NotificationKinds {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NotificationKinds {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NotificationKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
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
pub struct NotificationMirroring(pub i32);
impl NotificationMirroring {
    pub const Allowed: NotificationMirroring = NotificationMirroring(0i32);
    pub const Disabled: NotificationMirroring = NotificationMirroring(1i32);
}
impl ::std::convert::From<i32> for NotificationMirroring {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NotificationMirroring {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NotificationMirroring {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Notifications.NotificationMirroring;i4)",
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
pub struct NotificationSetting(pub i32);
impl NotificationSetting {
    pub const Enabled: NotificationSetting = NotificationSetting(0i32);
    pub const DisabledForApplication: NotificationSetting = NotificationSetting(1i32);
    pub const DisabledForUser: NotificationSetting = NotificationSetting(2i32);
    pub const DisabledByGroupPolicy: NotificationSetting = NotificationSetting(3i32);
    pub const DisabledByManifest: NotificationSetting = NotificationSetting(4i32);
}
impl ::std::convert::From<i32> for NotificationSetting {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NotificationSetting {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NotificationSetting {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Notifications.NotificationSetting;i4)",
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
pub struct NotificationUpdateResult(pub i32);
impl NotificationUpdateResult {
    pub const Succeeded: NotificationUpdateResult = NotificationUpdateResult(0i32);
    pub const Failed: NotificationUpdateResult = NotificationUpdateResult(1i32);
    pub const NotificationNotFound: NotificationUpdateResult = NotificationUpdateResult(2i32);
}
impl ::std::convert::From<i32> for NotificationUpdateResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NotificationUpdateResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NotificationUpdateResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Notifications.NotificationUpdateResult;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct NotificationVisual(::windows::runtime::IInspectable);
impl NotificationVisual {
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Bindings(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<NotificationBinding>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<NotificationBinding>>(
                result__,
            )
        }
    }
    pub fn GetBinding<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        templatename: Param0,
    ) -> ::windows::runtime::Result<NotificationBinding> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                templatename.into_param().abi(),
                &mut result__,
            )
            .from_abi::<NotificationBinding>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NotificationVisual {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.Notifications.NotificationVisual;{68835b8e-aa56-4e11-86d3-5f9a6957bc5b})",
    );
}
unsafe impl ::windows::runtime::Interface for NotificationVisual {
    type Vtable = INotificationVisual_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1753439118,
        43606,
        19985,
        [134, 211, 95, 154, 105, 87, 188, 91],
    );
}
impl ::windows::runtime::RuntimeName for NotificationVisual {
    const NAME: &'static str = "Windows.UI.Notifications.NotificationVisual";
}
impl ::std::convert::From<NotificationVisual> for ::windows::runtime::IUnknown {
    fn from(value: NotificationVisual) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NotificationVisual> for ::windows::runtime::IUnknown {
    fn from(value: &NotificationVisual) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NotificationVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NotificationVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<NotificationVisual> for ::windows::runtime::IInspectable {
    fn from(value: NotificationVisual) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NotificationVisual> for ::windows::runtime::IInspectable {
    fn from(value: &NotificationVisual) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for NotificationVisual
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a NotificationVisual
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for NotificationVisual {}
unsafe impl ::std::marker::Sync for NotificationVisual {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PeriodicUpdateRecurrence(pub i32);
impl PeriodicUpdateRecurrence {
    pub const HalfHour: PeriodicUpdateRecurrence = PeriodicUpdateRecurrence(0i32);
    pub const Hour: PeriodicUpdateRecurrence = PeriodicUpdateRecurrence(1i32);
    pub const SixHours: PeriodicUpdateRecurrence = PeriodicUpdateRecurrence(2i32);
    pub const TwelveHours: PeriodicUpdateRecurrence = PeriodicUpdateRecurrence(3i32);
    pub const Daily: PeriodicUpdateRecurrence = PeriodicUpdateRecurrence(4i32);
}
impl ::std::convert::From<i32> for PeriodicUpdateRecurrence {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PeriodicUpdateRecurrence {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PeriodicUpdateRecurrence {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Notifications.PeriodicUpdateRecurrence;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ScheduledTileNotification(::windows::runtime::IInspectable);
impl ScheduledTileNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeliveryTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::IReference<super::super::Foundation::DateTime>,
        >,
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
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IReference<super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(
                result__,
            )
        }
    }
    pub fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
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
    pub fn Tag(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub fn CreateScheduledTileNotification<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>,
    >(
        content: Param0,
        deliverytime: Param1,
    ) -> ::windows::runtime::Result<ScheduledTileNotification> {
        Self::IScheduledTileNotificationFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                content.into_param().abi(),
                deliverytime.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ScheduledTileNotification>(result__)
        })
    }
    pub fn IScheduledTileNotificationFactory<
        R,
        F: FnOnce(&IScheduledTileNotificationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ScheduledTileNotification,
            IScheduledTileNotificationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ScheduledTileNotification {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Notifications.ScheduledTileNotification;{0abca6d5-99dc-4c78-a11c-c9e7f86d7ef7})" ) ;
}
unsafe impl ::windows::runtime::Interface for ScheduledTileNotification {
    type Vtable = IScheduledTileNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        180135637,
        39388,
        19576,
        [161, 28, 201, 231, 248, 109, 126, 247],
    );
}
impl ::windows::runtime::RuntimeName for ScheduledTileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ScheduledTileNotification";
}
impl ::std::convert::From<ScheduledTileNotification> for ::windows::runtime::IUnknown {
    fn from(value: ScheduledTileNotification) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ScheduledTileNotification> for ::windows::runtime::IUnknown {
    fn from(value: &ScheduledTileNotification) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ScheduledTileNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ScheduledTileNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ScheduledTileNotification> for ::windows::runtime::IInspectable {
    fn from(value: ScheduledTileNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ScheduledTileNotification> for ::windows::runtime::IInspectable {
    fn from(value: &ScheduledTileNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ScheduledTileNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ScheduledTileNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ScheduledTileNotification {}
unsafe impl ::std::marker::Sync for ScheduledTileNotification {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ScheduledToastNotification(::windows::runtime::IInspectable);
impl ScheduledToastNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeliveryTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SnoozeInterval(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IReference<super::super::Foundation::TimeSpan>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(
                result__,
            )
        }
    }
    pub fn MaximumSnoozeCount(&self) -> ::windows::runtime::Result<u32> {
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
    pub fn SetId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
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
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Tag(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IScheduledToastNotification2>(self)?;
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
    pub fn SetGroup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Group(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IScheduledToastNotification2>(self)?;
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
    pub fn SetSuppressPopup(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn SuppressPopup(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub fn CreateScheduledToastNotification<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>,
    >(
        content: Param0,
        deliverytime: Param1,
    ) -> ::windows::runtime::Result<ScheduledToastNotification> {
        Self::IScheduledToastNotificationFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                content.into_param().abi(),
                deliverytime.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ScheduledToastNotification>(result__)
        })
    }
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub fn CreateScheduledToastNotificationRecurring<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>,
    >(
        content: Param0,
        deliverytime: Param1,
        snoozeinterval: Param2,
        maximumsnoozecount: u32,
    ) -> ::windows::runtime::Result<ScheduledToastNotification> {
        Self::IScheduledToastNotificationFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                content.into_param().abi(),
                deliverytime.into_param().abi(),
                snoozeinterval.into_param().abi(),
                maximumsnoozecount,
                &mut result__,
            )
            .from_abi::<ScheduledToastNotification>(result__)
        })
    }
    pub fn NotificationMirroring(&self) -> ::windows::runtime::Result<NotificationMirroring> {
        let this = &::windows::runtime::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe {
            let mut result__: NotificationMirroring = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<NotificationMirroring>(result__)
        }
    }
    pub fn SetNotificationMirroring(
        &self,
        value: NotificationMirroring,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn RemoteId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetRemoteId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IReference<super::super::Foundation::DateTime>,
    > {
        let this = &::windows::runtime::Interface::cast::<IScheduledToastNotification4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(
                result__,
            )
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::IReference<super::super::Foundation::DateTime>,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScheduledToastNotification4>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IScheduledToastNotificationFactory<
        R,
        F: FnOnce(&IScheduledToastNotificationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ScheduledToastNotification,
            IScheduledToastNotificationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ScheduledToastNotification {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Notifications.ScheduledToastNotification;{79f577f8-0de7-48cd-9740-9b370490c838})" ) ;
}
unsafe impl ::windows::runtime::Interface for ScheduledToastNotification {
    type Vtable = IScheduledToastNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2046130168,
        3559,
        18637,
        [151, 64, 155, 55, 4, 144, 200, 56],
    );
}
impl ::windows::runtime::RuntimeName for ScheduledToastNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ScheduledToastNotification";
}
impl ::std::convert::From<ScheduledToastNotification> for ::windows::runtime::IUnknown {
    fn from(value: ScheduledToastNotification) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ScheduledToastNotification> for ::windows::runtime::IUnknown {
    fn from(value: &ScheduledToastNotification) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ScheduledToastNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ScheduledToastNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ScheduledToastNotification> for ::windows::runtime::IInspectable {
    fn from(value: ScheduledToastNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ScheduledToastNotification> for ::windows::runtime::IInspectable {
    fn from(value: &ScheduledToastNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ScheduledToastNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ScheduledToastNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ScheduledToastNotification {}
unsafe impl ::std::marker::Sync for ScheduledToastNotification {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ScheduledToastNotificationShowingEventArgs(::windows::runtime::IInspectable);
impl ScheduledToastNotificationShowingEventArgs {
    pub fn Cancel(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetCancel(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ScheduledToastNotification(
        &self,
    ) -> ::windows::runtime::Result<ScheduledToastNotification> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ScheduledToastNotification>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ScheduledToastNotificationShowingEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Notifications.ScheduledToastNotificationShowingEventArgs;{6173f6b4-412a-5e2c-a6ed-a0209aef9a09})" ) ;
}
unsafe impl ::windows::runtime::Interface for ScheduledToastNotificationShowingEventArgs {
    type Vtable = IScheduledToastNotificationShowingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1634989748,
        16682,
        24108,
        [166, 237, 160, 32, 154, 239, 154, 9],
    );
}
impl ::windows::runtime::RuntimeName for ScheduledToastNotificationShowingEventArgs {
    const NAME: &'static str =
        "Windows.UI.Notifications.ScheduledToastNotificationShowingEventArgs";
}
impl ::std::convert::From<ScheduledToastNotificationShowingEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: ScheduledToastNotificationShowingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ScheduledToastNotificationShowingEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &ScheduledToastNotificationShowingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ScheduledToastNotificationShowingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ScheduledToastNotificationShowingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ScheduledToastNotificationShowingEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: ScheduledToastNotificationShowingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ScheduledToastNotificationShowingEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &ScheduledToastNotificationShowingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ScheduledToastNotificationShowingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ScheduledToastNotificationShowingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ScheduledToastNotificationShowingEventArgs {}
unsafe impl ::std::marker::Sync for ScheduledToastNotificationShowingEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ShownTileNotification(::windows::runtime::IInspectable);
impl ShownTileNotification {
    pub fn Arguments(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
}
unsafe impl ::windows::runtime::RuntimeType for ShownTileNotification {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Notifications.ShownTileNotification;{342d8988-5af2-481a-a6a3-f2fdc78de88e})" ) ;
}
unsafe impl ::windows::runtime::Interface for ShownTileNotification {
    type Vtable = IShownTileNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        875399560,
        23282,
        18458,
        [166, 163, 242, 253, 199, 141, 232, 142],
    );
}
impl ::windows::runtime::RuntimeName for ShownTileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ShownTileNotification";
}
impl ::std::convert::From<ShownTileNotification> for ::windows::runtime::IUnknown {
    fn from(value: ShownTileNotification) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ShownTileNotification> for ::windows::runtime::IUnknown {
    fn from(value: &ShownTileNotification) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ShownTileNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ShownTileNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ShownTileNotification> for ::windows::runtime::IInspectable {
    fn from(value: ShownTileNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ShownTileNotification> for ::windows::runtime::IInspectable {
    fn from(value: &ShownTileNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ShownTileNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ShownTileNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ShownTileNotification {}
unsafe impl ::std::marker::Sync for ShownTileNotification {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TileFlyoutNotification(::windows::runtime::IInspectable);
impl TileFlyoutNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::IReference<super::super::Foundation::DateTime>,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IReference<super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(
                result__,
            )
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateTileFlyoutNotification<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>,
    >(
        content: Param0,
    ) -> ::windows::runtime::Result<TileFlyoutNotification> {
        Self::ITileFlyoutNotificationFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                content.into_param().abi(),
                &mut result__,
            )
            .from_abi::<TileFlyoutNotification>(result__)
        })
    }
    pub fn ITileFlyoutNotificationFactory<
        R,
        F: FnOnce(&ITileFlyoutNotificationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            TileFlyoutNotification,
            ITileFlyoutNotificationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TileFlyoutNotification {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Notifications.TileFlyoutNotification;{9a53b261-c70c-42be-b2f3-f42aa97d34e5})" ) ;
}
unsafe impl ::windows::runtime::Interface for TileFlyoutNotification {
    type Vtable = ITileFlyoutNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2589176417,
        50956,
        17086,
        [178, 243, 244, 42, 169, 125, 52, 229],
    );
}
impl ::windows::runtime::RuntimeName for TileFlyoutNotification {
    const NAME: &'static str = "Windows.UI.Notifications.TileFlyoutNotification";
}
impl ::std::convert::From<TileFlyoutNotification> for ::windows::runtime::IUnknown {
    fn from(value: TileFlyoutNotification) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TileFlyoutNotification> for ::windows::runtime::IUnknown {
    fn from(value: &TileFlyoutNotification) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for TileFlyoutNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &TileFlyoutNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<TileFlyoutNotification> for ::windows::runtime::IInspectable {
    fn from(value: TileFlyoutNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TileFlyoutNotification> for ::windows::runtime::IInspectable {
    fn from(value: &TileFlyoutNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for TileFlyoutNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a TileFlyoutNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TileFlyoutNotification {}
unsafe impl ::std::marker::Sync for TileFlyoutNotification {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TileFlyoutTemplateType(pub i32);
impl TileFlyoutTemplateType {
    pub const TileFlyoutTemplate01: TileFlyoutTemplateType = TileFlyoutTemplateType(0i32);
}
impl ::std::convert::From<i32> for TileFlyoutTemplateType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TileFlyoutTemplateType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TileFlyoutTemplateType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Notifications.TileFlyoutTemplateType;i4)",
    );
}
pub struct TileFlyoutUpdateManager {}
impl TileFlyoutUpdateManager {
    pub fn CreateTileFlyoutUpdaterForApplication() -> ::windows::runtime::Result<TileFlyoutUpdater>
    {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TileFlyoutUpdater>(result__)
        })
    }
    pub fn CreateTileFlyoutUpdaterForApplicationWithId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        applicationid: Param0,
    ) -> ::windows::runtime::Result<TileFlyoutUpdater> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                applicationid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<TileFlyoutUpdater>(result__)
        })
    }
    pub fn CreateTileFlyoutUpdaterForSecondaryTile<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        tileid: Param0,
    ) -> ::windows::runtime::Result<TileFlyoutUpdater> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                tileid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<TileFlyoutUpdater>(result__)
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(
        r#type: TileFlyoutTemplateType,
    ) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                r#type,
                &mut result__,
            )
            .from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        })
    }
    pub fn ITileFlyoutUpdateManagerStatics<
        R,
        F: FnOnce(&ITileFlyoutUpdateManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            TileFlyoutUpdateManager,
            ITileFlyoutUpdateManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for TileFlyoutUpdateManager {
    const NAME: &'static str = "Windows.UI.Notifications.TileFlyoutUpdateManager";
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TileFlyoutUpdater(::windows::runtime::IInspectable);
impl TileFlyoutUpdater {
    pub fn Update<'a, Param0: ::windows::runtime::IntoParam<'a, TileFlyoutNotification>>(
        &self,
        notification: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                notification.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
    >(
        &self,
        tileflyoutcontent: Param0,
        requestedinterval: PeriodicUpdateRecurrence,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                tileflyoutcontent.into_param().abi(),
                requestedinterval,
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdateAtTime<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>,
    >(
        &self,
        tileflyoutcontent: Param0,
        starttime: Param1,
        requestedinterval: PeriodicUpdateRecurrence,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                tileflyoutcontent.into_param().abi(),
                starttime.into_param().abi(),
                requestedinterval,
            )
            .ok()
        }
    }
    pub fn StopPeriodicUpdate(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Setting(&self) -> ::windows::runtime::Result<NotificationSetting> {
        let this = self;
        unsafe {
            let mut result__: NotificationSetting = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<NotificationSetting>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TileFlyoutUpdater {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.Notifications.TileFlyoutUpdater;{8d40c76a-c465-4052-a740-5c2654c1a089})",
    );
}
unsafe impl ::windows::runtime::Interface for TileFlyoutUpdater {
    type Vtable = ITileFlyoutUpdater_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2369832810,
        50277,
        16466,
        [167, 64, 92, 38, 84, 193, 160, 137],
    );
}
impl ::windows::runtime::RuntimeName for TileFlyoutUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.TileFlyoutUpdater";
}
impl ::std::convert::From<TileFlyoutUpdater> for ::windows::runtime::IUnknown {
    fn from(value: TileFlyoutUpdater) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TileFlyoutUpdater> for ::windows::runtime::IUnknown {
    fn from(value: &TileFlyoutUpdater) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TileFlyoutUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TileFlyoutUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<TileFlyoutUpdater> for ::windows::runtime::IInspectable {
    fn from(value: TileFlyoutUpdater) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TileFlyoutUpdater> for ::windows::runtime::IInspectable {
    fn from(value: &TileFlyoutUpdater) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TileFlyoutUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a TileFlyoutUpdater
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
pub struct TileNotification(::windows::runtime::IInspectable);
impl TileNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::IReference<super::super::Foundation::DateTime>,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IReference<super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(
                result__,
            )
        }
    }
    pub fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Tag(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateTileNotification<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>,
    >(
        content: Param0,
    ) -> ::windows::runtime::Result<TileNotification> {
        Self::ITileNotificationFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                content.into_param().abi(),
                &mut result__,
            )
            .from_abi::<TileNotification>(result__)
        })
    }
    pub fn ITileNotificationFactory<
        R,
        F: FnOnce(&ITileNotificationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            TileNotification,
            ITileNotificationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TileNotification {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.Notifications.TileNotification;{ebaec8fa-50ec-4c18-b4d0-3af02e5540ab})",
    );
}
unsafe impl ::windows::runtime::Interface for TileNotification {
    type Vtable = ITileNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3954100474,
        20716,
        19480,
        [180, 208, 58, 240, 46, 85, 64, 171],
    );
}
impl ::windows::runtime::RuntimeName for TileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.TileNotification";
}
impl ::std::convert::From<TileNotification> for ::windows::runtime::IUnknown {
    fn from(value: TileNotification) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TileNotification> for ::windows::runtime::IUnknown {
    fn from(value: &TileNotification) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TileNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TileNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<TileNotification> for ::windows::runtime::IInspectable {
    fn from(value: TileNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TileNotification> for ::windows::runtime::IInspectable {
    fn from(value: &TileNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TileNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a TileNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TileNotification {}
unsafe impl ::std::marker::Sync for TileNotification {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TileTemplateType(pub i32);
impl TileTemplateType {
    pub const TileSquareImage: TileTemplateType = TileTemplateType(0i32);
    pub const TileSquareBlock: TileTemplateType = TileTemplateType(1i32);
    pub const TileSquareText01: TileTemplateType = TileTemplateType(2i32);
    pub const TileSquareText02: TileTemplateType = TileTemplateType(3i32);
    pub const TileSquareText03: TileTemplateType = TileTemplateType(4i32);
    pub const TileSquareText04: TileTemplateType = TileTemplateType(5i32);
    pub const TileSquarePeekImageAndText01: TileTemplateType = TileTemplateType(6i32);
    pub const TileSquarePeekImageAndText02: TileTemplateType = TileTemplateType(7i32);
    pub const TileSquarePeekImageAndText03: TileTemplateType = TileTemplateType(8i32);
    pub const TileSquarePeekImageAndText04: TileTemplateType = TileTemplateType(9i32);
    pub const TileWideImage: TileTemplateType = TileTemplateType(10i32);
    pub const TileWideImageCollection: TileTemplateType = TileTemplateType(11i32);
    pub const TileWideImageAndText01: TileTemplateType = TileTemplateType(12i32);
    pub const TileWideImageAndText02: TileTemplateType = TileTemplateType(13i32);
    pub const TileWideBlockAndText01: TileTemplateType = TileTemplateType(14i32);
    pub const TileWideBlockAndText02: TileTemplateType = TileTemplateType(15i32);
    pub const TileWidePeekImageCollection01: TileTemplateType = TileTemplateType(16i32);
    pub const TileWidePeekImageCollection02: TileTemplateType = TileTemplateType(17i32);
    pub const TileWidePeekImageCollection03: TileTemplateType = TileTemplateType(18i32);
    pub const TileWidePeekImageCollection04: TileTemplateType = TileTemplateType(19i32);
    pub const TileWidePeekImageCollection05: TileTemplateType = TileTemplateType(20i32);
    pub const TileWidePeekImageCollection06: TileTemplateType = TileTemplateType(21i32);
    pub const TileWidePeekImageAndText01: TileTemplateType = TileTemplateType(22i32);
    pub const TileWidePeekImageAndText02: TileTemplateType = TileTemplateType(23i32);
    pub const TileWidePeekImage01: TileTemplateType = TileTemplateType(24i32);
    pub const TileWidePeekImage02: TileTemplateType = TileTemplateType(25i32);
    pub const TileWidePeekImage03: TileTemplateType = TileTemplateType(26i32);
    pub const TileWidePeekImage04: TileTemplateType = TileTemplateType(27i32);
    pub const TileWidePeekImage05: TileTemplateType = TileTemplateType(28i32);
    pub const TileWidePeekImage06: TileTemplateType = TileTemplateType(29i32);
    pub const TileWideSmallImageAndText01: TileTemplateType = TileTemplateType(30i32);
    pub const TileWideSmallImageAndText02: TileTemplateType = TileTemplateType(31i32);
    pub const TileWideSmallImageAndText03: TileTemplateType = TileTemplateType(32i32);
    pub const TileWideSmallImageAndText04: TileTemplateType = TileTemplateType(33i32);
    pub const TileWideSmallImageAndText05: TileTemplateType = TileTemplateType(34i32);
    pub const TileWideText01: TileTemplateType = TileTemplateType(35i32);
    pub const TileWideText02: TileTemplateType = TileTemplateType(36i32);
    pub const TileWideText03: TileTemplateType = TileTemplateType(37i32);
    pub const TileWideText04: TileTemplateType = TileTemplateType(38i32);
    pub const TileWideText05: TileTemplateType = TileTemplateType(39i32);
    pub const TileWideText06: TileTemplateType = TileTemplateType(40i32);
    pub const TileWideText07: TileTemplateType = TileTemplateType(41i32);
    pub const TileWideText08: TileTemplateType = TileTemplateType(42i32);
    pub const TileWideText09: TileTemplateType = TileTemplateType(43i32);
    pub const TileWideText10: TileTemplateType = TileTemplateType(44i32);
    pub const TileWideText11: TileTemplateType = TileTemplateType(45i32);
    pub const TileSquare150x150Image: TileTemplateType = TileTemplateType(0i32);
    pub const TileSquare150x150Block: TileTemplateType = TileTemplateType(1i32);
    pub const TileSquare150x150Text01: TileTemplateType = TileTemplateType(2i32);
    pub const TileSquare150x150Text02: TileTemplateType = TileTemplateType(3i32);
    pub const TileSquare150x150Text03: TileTemplateType = TileTemplateType(4i32);
    pub const TileSquare150x150Text04: TileTemplateType = TileTemplateType(5i32);
    pub const TileSquare150x150PeekImageAndText01: TileTemplateType = TileTemplateType(6i32);
    pub const TileSquare150x150PeekImageAndText02: TileTemplateType = TileTemplateType(7i32);
    pub const TileSquare150x150PeekImageAndText03: TileTemplateType = TileTemplateType(8i32);
    pub const TileSquare150x150PeekImageAndText04: TileTemplateType = TileTemplateType(9i32);
    pub const TileWide310x150Image: TileTemplateType = TileTemplateType(10i32);
    pub const TileWide310x150ImageCollection: TileTemplateType = TileTemplateType(11i32);
    pub const TileWide310x150ImageAndText01: TileTemplateType = TileTemplateType(12i32);
    pub const TileWide310x150ImageAndText02: TileTemplateType = TileTemplateType(13i32);
    pub const TileWide310x150BlockAndText01: TileTemplateType = TileTemplateType(14i32);
    pub const TileWide310x150BlockAndText02: TileTemplateType = TileTemplateType(15i32);
    pub const TileWide310x150PeekImageCollection01: TileTemplateType = TileTemplateType(16i32);
    pub const TileWide310x150PeekImageCollection02: TileTemplateType = TileTemplateType(17i32);
    pub const TileWide310x150PeekImageCollection03: TileTemplateType = TileTemplateType(18i32);
    pub const TileWide310x150PeekImageCollection04: TileTemplateType = TileTemplateType(19i32);
    pub const TileWide310x150PeekImageCollection05: TileTemplateType = TileTemplateType(20i32);
    pub const TileWide310x150PeekImageCollection06: TileTemplateType = TileTemplateType(21i32);
    pub const TileWide310x150PeekImageAndText01: TileTemplateType = TileTemplateType(22i32);
    pub const TileWide310x150PeekImageAndText02: TileTemplateType = TileTemplateType(23i32);
    pub const TileWide310x150PeekImage01: TileTemplateType = TileTemplateType(24i32);
    pub const TileWide310x150PeekImage02: TileTemplateType = TileTemplateType(25i32);
    pub const TileWide310x150PeekImage03: TileTemplateType = TileTemplateType(26i32);
    pub const TileWide310x150PeekImage04: TileTemplateType = TileTemplateType(27i32);
    pub const TileWide310x150PeekImage05: TileTemplateType = TileTemplateType(28i32);
    pub const TileWide310x150PeekImage06: TileTemplateType = TileTemplateType(29i32);
    pub const TileWide310x150SmallImageAndText01: TileTemplateType = TileTemplateType(30i32);
    pub const TileWide310x150SmallImageAndText02: TileTemplateType = TileTemplateType(31i32);
    pub const TileWide310x150SmallImageAndText03: TileTemplateType = TileTemplateType(32i32);
    pub const TileWide310x150SmallImageAndText04: TileTemplateType = TileTemplateType(33i32);
    pub const TileWide310x150SmallImageAndText05: TileTemplateType = TileTemplateType(34i32);
    pub const TileWide310x150Text01: TileTemplateType = TileTemplateType(35i32);
    pub const TileWide310x150Text02: TileTemplateType = TileTemplateType(36i32);
    pub const TileWide310x150Text03: TileTemplateType = TileTemplateType(37i32);
    pub const TileWide310x150Text04: TileTemplateType = TileTemplateType(38i32);
    pub const TileWide310x150Text05: TileTemplateType = TileTemplateType(39i32);
    pub const TileWide310x150Text06: TileTemplateType = TileTemplateType(40i32);
    pub const TileWide310x150Text07: TileTemplateType = TileTemplateType(41i32);
    pub const TileWide310x150Text08: TileTemplateType = TileTemplateType(42i32);
    pub const TileWide310x150Text09: TileTemplateType = TileTemplateType(43i32);
    pub const TileWide310x150Text10: TileTemplateType = TileTemplateType(44i32);
    pub const TileWide310x150Text11: TileTemplateType = TileTemplateType(45i32);
    pub const TileSquare310x310BlockAndText01: TileTemplateType = TileTemplateType(46i32);
    pub const TileSquare310x310BlockAndText02: TileTemplateType = TileTemplateType(47i32);
    pub const TileSquare310x310Image: TileTemplateType = TileTemplateType(48i32);
    pub const TileSquare310x310ImageAndText01: TileTemplateType = TileTemplateType(49i32);
    pub const TileSquare310x310ImageAndText02: TileTemplateType = TileTemplateType(50i32);
    pub const TileSquare310x310ImageAndTextOverlay01: TileTemplateType = TileTemplateType(51i32);
    pub const TileSquare310x310ImageAndTextOverlay02: TileTemplateType = TileTemplateType(52i32);
    pub const TileSquare310x310ImageAndTextOverlay03: TileTemplateType = TileTemplateType(53i32);
    pub const TileSquare310x310ImageCollectionAndText01: TileTemplateType = TileTemplateType(54i32);
    pub const TileSquare310x310ImageCollectionAndText02: TileTemplateType = TileTemplateType(55i32);
    pub const TileSquare310x310ImageCollection: TileTemplateType = TileTemplateType(56i32);
    pub const TileSquare310x310SmallImagesAndTextList01: TileTemplateType = TileTemplateType(57i32);
    pub const TileSquare310x310SmallImagesAndTextList02: TileTemplateType = TileTemplateType(58i32);
    pub const TileSquare310x310SmallImagesAndTextList03: TileTemplateType = TileTemplateType(59i32);
    pub const TileSquare310x310SmallImagesAndTextList04: TileTemplateType = TileTemplateType(60i32);
    pub const TileSquare310x310Text01: TileTemplateType = TileTemplateType(61i32);
    pub const TileSquare310x310Text02: TileTemplateType = TileTemplateType(62i32);
    pub const TileSquare310x310Text03: TileTemplateType = TileTemplateType(63i32);
    pub const TileSquare310x310Text04: TileTemplateType = TileTemplateType(64i32);
    pub const TileSquare310x310Text05: TileTemplateType = TileTemplateType(65i32);
    pub const TileSquare310x310Text06: TileTemplateType = TileTemplateType(66i32);
    pub const TileSquare310x310Text07: TileTemplateType = TileTemplateType(67i32);
    pub const TileSquare310x310Text08: TileTemplateType = TileTemplateType(68i32);
    pub const TileSquare310x310TextList01: TileTemplateType = TileTemplateType(69i32);
    pub const TileSquare310x310TextList02: TileTemplateType = TileTemplateType(70i32);
    pub const TileSquare310x310TextList03: TileTemplateType = TileTemplateType(71i32);
    pub const TileSquare310x310SmallImageAndText01: TileTemplateType = TileTemplateType(72i32);
    pub const TileSquare310x310SmallImagesAndTextList05: TileTemplateType = TileTemplateType(73i32);
    pub const TileSquare310x310Text09: TileTemplateType = TileTemplateType(74i32);
    pub const TileSquare71x71IconWithBadge: TileTemplateType = TileTemplateType(75i32);
    pub const TileSquare150x150IconWithBadge: TileTemplateType = TileTemplateType(76i32);
    pub const TileWide310x150IconWithBadgeAndText: TileTemplateType = TileTemplateType(77i32);
    pub const TileSquare71x71Image: TileTemplateType = TileTemplateType(78i32);
    pub const TileTall150x310Image: TileTemplateType = TileTemplateType(79i32);
}
impl ::std::convert::From<i32> for TileTemplateType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TileTemplateType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TileTemplateType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Notifications.TileTemplateType;i4)",
    );
}
pub struct TileUpdateManager {}
impl TileUpdateManager {
    pub fn CreateTileUpdaterForApplication() -> ::windows::runtime::Result<TileUpdater> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TileUpdater>(result__)
        })
    }
    pub fn CreateTileUpdaterForApplicationWithId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        applicationid: Param0,
    ) -> ::windows::runtime::Result<TileUpdater> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                applicationid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<TileUpdater>(result__)
        })
    }
    pub fn CreateTileUpdaterForSecondaryTile<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        tileid: Param0,
    ) -> ::windows::runtime::Result<TileUpdater> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                tileid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<TileUpdater>(result__)
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(
        r#type: TileTemplateType,
    ) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                r#type,
                &mut result__,
            )
            .from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(
        user: Param0,
    ) -> ::windows::runtime::Result<TileUpdateManagerForUser> {
        Self::ITileUpdateManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                user.into_param().abi(),
                &mut result__,
            )
            .from_abi::<TileUpdateManagerForUser>(result__)
        })
    }
    pub fn ITileUpdateManagerStatics<
        R,
        F: FnOnce(&ITileUpdateManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            TileUpdateManager,
            ITileUpdateManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITileUpdateManagerStatics2<
        R,
        F: FnOnce(&ITileUpdateManagerStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            TileUpdateManager,
            ITileUpdateManagerStatics2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for TileUpdateManager {
    const NAME: &'static str = "Windows.UI.Notifications.TileUpdateManager";
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TileUpdateManagerForUser(::windows::runtime::IInspectable);
impl TileUpdateManagerForUser {
    pub fn CreateTileUpdaterForApplication(&self) -> ::windows::runtime::Result<TileUpdater> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TileUpdater>(result__)
        }
    }
    pub fn CreateTileUpdaterForApplicationWithId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        applicationid: Param0,
    ) -> ::windows::runtime::Result<TileUpdater> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                applicationid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<TileUpdater>(result__)
        }
    }
    pub fn CreateTileUpdaterForSecondaryTile<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        tileid: Param0,
    ) -> ::windows::runtime::Result<TileUpdater> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                tileid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<TileUpdater>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TileUpdateManagerForUser {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Notifications.TileUpdateManagerForUser;{55141348-2ee2-4e2d-9cc1-216a20decc9f})" ) ;
}
unsafe impl ::windows::runtime::Interface for TileUpdateManagerForUser {
    type Vtable = ITileUpdateManagerForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1427379016,
        12002,
        20013,
        [156, 193, 33, 106, 32, 222, 204, 159],
    );
}
impl ::windows::runtime::RuntimeName for TileUpdateManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.TileUpdateManagerForUser";
}
impl ::std::convert::From<TileUpdateManagerForUser> for ::windows::runtime::IUnknown {
    fn from(value: TileUpdateManagerForUser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TileUpdateManagerForUser> for ::windows::runtime::IUnknown {
    fn from(value: &TileUpdateManagerForUser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for TileUpdateManagerForUser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &TileUpdateManagerForUser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<TileUpdateManagerForUser> for ::windows::runtime::IInspectable {
    fn from(value: TileUpdateManagerForUser) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TileUpdateManagerForUser> for ::windows::runtime::IInspectable {
    fn from(value: &TileUpdateManagerForUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for TileUpdateManagerForUser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a TileUpdateManagerForUser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TileUpdateManagerForUser {}
unsafe impl ::std::marker::Sync for TileUpdateManagerForUser {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TileUpdater(::windows::runtime::IInspectable);
impl TileUpdater {
    pub fn Update<'a, Param0: ::windows::runtime::IntoParam<'a, TileNotification>>(
        &self,
        notification: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                notification.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn EnableNotificationQueue(&self, enable: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                enable,
            )
            .ok()
        }
    }
    pub fn Setting(&self) -> ::windows::runtime::Result<NotificationSetting> {
        let this = self;
        unsafe {
            let mut result__: NotificationSetting = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<NotificationSetting>(result__)
        }
    }
    pub fn AddToSchedule<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ScheduledTileNotification>,
    >(
        &self,
        scheduledtile: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                scheduledtile.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveFromSchedule<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ScheduledTileNotification>,
    >(
        &self,
        scheduledtile: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                scheduledtile.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetScheduledTileNotifications(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<ScheduledTileNotification>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<
                ScheduledTileNotification,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
    >(
        &self,
        tilecontent: Param0,
        requestedinterval: PeriodicUpdateRecurrence,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                tilecontent.into_param().abi(),
                requestedinterval,
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdateAtTime<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>,
    >(
        &self,
        tilecontent: Param0,
        starttime: Param1,
        requestedinterval: PeriodicUpdateRecurrence,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                tilecontent.into_param().abi(),
                starttime.into_param().abi(),
                requestedinterval,
            )
            .ok()
        }
    }
    pub fn StopPeriodicUpdate(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn StartPeriodicUpdateBatch<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>,
        >,
    >(
        &self,
        tilecontents: Param0,
        requestedinterval: PeriodicUpdateRecurrence,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                tilecontents.into_param().abi(),
                requestedinterval,
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn StartPeriodicUpdateBatchAtTime<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>,
        >,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>,
    >(
        &self,
        tilecontents: Param0,
        starttime: Param1,
        requestedinterval: PeriodicUpdateRecurrence,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                tilecontents.into_param().abi(),
                starttime.into_param().abi(),
                requestedinterval,
            )
            .ok()
        }
    }
    pub fn EnableNotificationQueueForSquare150x150(
        &self,
        enable: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITileUpdater2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                enable,
            )
            .ok()
        }
    }
    pub fn EnableNotificationQueueForWide310x150(
        &self,
        enable: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITileUpdater2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                enable,
            )
            .ok()
        }
    }
    pub fn EnableNotificationQueueForSquare310x310(
        &self,
        enable: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITileUpdater2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                enable,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TileUpdater {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.Notifications.TileUpdater;{0942a48b-1d91-44ec-9243-c1e821c29a20})",
    );
}
unsafe impl ::windows::runtime::Interface for TileUpdater {
    type Vtable = ITileUpdater_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        155362443,
        7569,
        17644,
        [146, 67, 193, 232, 33, 194, 154, 32],
    );
}
impl ::windows::runtime::RuntimeName for TileUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.TileUpdater";
}
impl ::std::convert::From<TileUpdater> for ::windows::runtime::IUnknown {
    fn from(value: TileUpdater) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TileUpdater> for ::windows::runtime::IUnknown {
    fn from(value: &TileUpdater) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TileUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TileUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<TileUpdater> for ::windows::runtime::IInspectable {
    fn from(value: TileUpdater) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TileUpdater> for ::windows::runtime::IInspectable {
    fn from(value: &TileUpdater) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TileUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TileUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TileUpdater {}
unsafe impl ::std::marker::Sync for TileUpdater {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ToastActivatedEventArgs(::windows::runtime::IInspectable);
impl ToastActivatedEventArgs {
    pub fn Arguments(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::runtime::Interface::cast::<IToastActivatedEventArgs2>(self)?;
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
unsafe impl ::windows::runtime::RuntimeType for ToastActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Notifications.ToastActivatedEventArgs;{e3bf92f3-c197-436f-8265-0625824f8dac})" ) ;
}
unsafe impl ::windows::runtime::Interface for ToastActivatedEventArgs {
    type Vtable = IToastActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3820983027,
        49559,
        17263,
        [130, 101, 6, 37, 130, 79, 141, 172],
    );
}
impl ::windows::runtime::RuntimeName for ToastActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ToastActivatedEventArgs";
}
impl ::std::convert::From<ToastActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ToastActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ToastActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ToastActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ToastActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ToastActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ToastActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ToastActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ToastActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ToastActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ToastActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ToastActivatedEventArgs
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
pub struct ToastCollection(::windows::runtime::IInspectable);
impl ToastCollection {
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
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetDisplayName<
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
    pub fn LaunchArgs(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetLaunchArgs<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
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
    pub fn Icon(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetIcon<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateInstance<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
    >(
        collectionid: Param0,
        displayname: Param1,
        launchargs: Param2,
        iconuri: Param3,
    ) -> ::windows::runtime::Result<ToastCollection> {
        Self::IToastCollectionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                collectionid.into_param().abi(),
                displayname.into_param().abi(),
                launchargs.into_param().abi(),
                iconuri.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ToastCollection>(result__)
        })
    }
    pub fn IToastCollectionFactory<
        R,
        F: FnOnce(&IToastCollectionFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ToastCollection,
            IToastCollectionFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ToastCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.Notifications.ToastCollection;{0a8bc3b0-e0be-4858-bc2a-89dfe0b32863})",
    );
}
unsafe impl ::windows::runtime::Interface for ToastCollection {
    type Vtable = IToastCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        176931760,
        57534,
        18520,
        [188, 42, 137, 223, 224, 179, 40, 99],
    );
}
impl ::windows::runtime::RuntimeName for ToastCollection {
    const NAME: &'static str = "Windows.UI.Notifications.ToastCollection";
}
impl ::std::convert::From<ToastCollection> for ::windows::runtime::IUnknown {
    fn from(value: ToastCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ToastCollection> for ::windows::runtime::IUnknown {
    fn from(value: &ToastCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ToastCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ToastCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ToastCollection> for ::windows::runtime::IInspectable {
    fn from(value: ToastCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ToastCollection> for ::windows::runtime::IInspectable {
    fn from(value: &ToastCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ToastCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ToastCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ToastCollection {}
unsafe impl ::std::marker::Sync for ToastCollection {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ToastCollectionManager(::windows::runtime::IInspectable);
impl ToastCollectionManager {
    #[cfg(feature = "Foundation")]
    pub fn SaveToastCollectionAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ToastCollection>,
    >(
        &self,
        collection: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                collection.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllToastCollectionsAsync(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<
            super::super::Foundation::Collections::IVectorView<ToastCollection>,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<
                super::super::Foundation::Collections::IVectorView<ToastCollection>,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetToastCollectionAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        collectionid: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ToastCollection>>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                collectionid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<ToastCollection>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveToastCollectionAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        collectionid: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                collectionid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAllToastCollectionsAsync(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ToastCollectionManager {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Notifications.ToastCollectionManager;{2a1821fe-179d-49bc-b79d-a527920d3665})" ) ;
}
unsafe impl ::windows::runtime::Interface for ToastCollectionManager {
    type Vtable = IToastCollectionManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        706224638,
        6045,
        18876,
        [183, 157, 165, 39, 146, 13, 54, 101],
    );
}
impl ::windows::runtime::RuntimeName for ToastCollectionManager {
    const NAME: &'static str = "Windows.UI.Notifications.ToastCollectionManager";
}
impl ::std::convert::From<ToastCollectionManager> for ::windows::runtime::IUnknown {
    fn from(value: ToastCollectionManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ToastCollectionManager> for ::windows::runtime::IUnknown {
    fn from(value: &ToastCollectionManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ToastCollectionManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ToastCollectionManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ToastCollectionManager> for ::windows::runtime::IInspectable {
    fn from(value: ToastCollectionManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ToastCollectionManager> for ::windows::runtime::IInspectable {
    fn from(value: &ToastCollectionManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ToastCollectionManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ToastCollectionManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ToastCollectionManager {}
unsafe impl ::std::marker::Sync for ToastCollectionManager {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ToastDismissalReason(pub i32);
impl ToastDismissalReason {
    pub const UserCanceled: ToastDismissalReason = ToastDismissalReason(0i32);
    pub const ApplicationHidden: ToastDismissalReason = ToastDismissalReason(1i32);
    pub const TimedOut: ToastDismissalReason = ToastDismissalReason(2i32);
}
impl ::std::convert::From<i32> for ToastDismissalReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ToastDismissalReason {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ToastDismissalReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Notifications.ToastDismissalReason;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ToastDismissedEventArgs(::windows::runtime::IInspectable);
impl ToastDismissedEventArgs {
    pub fn Reason(&self) -> ::windows::runtime::Result<ToastDismissalReason> {
        let this = self;
        unsafe {
            let mut result__: ToastDismissalReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ToastDismissalReason>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ToastDismissedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Notifications.ToastDismissedEventArgs;{3f89d935-d9cb-4538-a0f0-ffe7659938f8})" ) ;
}
unsafe impl ::windows::runtime::Interface for ToastDismissedEventArgs {
    type Vtable = IToastDismissedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1065998645,
        55755,
        17720,
        [160, 240, 255, 231, 101, 153, 56, 248],
    );
}
impl ::windows::runtime::RuntimeName for ToastDismissedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ToastDismissedEventArgs";
}
impl ::std::convert::From<ToastDismissedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ToastDismissedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ToastDismissedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ToastDismissedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ToastDismissedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ToastDismissedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ToastDismissedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ToastDismissedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ToastDismissedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ToastDismissedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ToastDismissedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ToastDismissedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ToastDismissedEventArgs {}
unsafe impl ::std::marker::Sync for ToastDismissedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ToastFailedEventArgs(::windows::runtime::IInspectable);
impl ToastFailedEventArgs {
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ToastFailedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.Notifications.ToastFailedEventArgs;{35176862-cfd4-44f8-ad64-f500fd896c3b})",
    );
}
unsafe impl ::windows::runtime::Interface for ToastFailedEventArgs {
    type Vtable = IToastFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        890726498,
        53204,
        17656,
        [173, 100, 245, 0, 253, 137, 108, 59],
    );
}
impl ::windows::runtime::RuntimeName for ToastFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ToastFailedEventArgs";
}
impl ::std::convert::From<ToastFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ToastFailedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ToastFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ToastFailedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ToastFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ToastFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ToastFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ToastFailedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ToastFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ToastFailedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ToastFailedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ToastFailedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ToastFailedEventArgs {}
unsafe impl ::std::marker::Sync for ToastFailedEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ToastHistoryChangedType(pub i32);
impl ToastHistoryChangedType {
    pub const Cleared: ToastHistoryChangedType = ToastHistoryChangedType(0i32);
    pub const Removed: ToastHistoryChangedType = ToastHistoryChangedType(1i32);
    pub const Expired: ToastHistoryChangedType = ToastHistoryChangedType(2i32);
    pub const Added: ToastHistoryChangedType = ToastHistoryChangedType(3i32);
}
impl ::std::convert::From<i32> for ToastHistoryChangedType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ToastHistoryChangedType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ToastHistoryChangedType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Notifications.ToastHistoryChangedType;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ToastNotification(::windows::runtime::IInspectable);
impl ToastNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::IReference<super::super::Foundation::DateTime>,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IReference<super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(
                result__,
            )
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Dismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<ToastNotification, ToastDismissedEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Activated<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                ToastNotification,
                ::windows::runtime::IInspectable,
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
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivated<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Failed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<ToastNotification, ToastFailedEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveFailed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Tag(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IToastNotification2>(self)?;
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
    pub fn SetGroup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Group(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IToastNotification2>(self)?;
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
    pub fn SetSuppressPopup(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn SuppressPopup(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateToastNotification<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>,
    >(
        content: Param0,
    ) -> ::windows::runtime::Result<ToastNotification> {
        Self::IToastNotificationFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                content.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ToastNotification>(result__)
        })
    }
    pub fn NotificationMirroring(&self) -> ::windows::runtime::Result<NotificationMirroring> {
        let this = &::windows::runtime::Interface::cast::<IToastNotification3>(self)?;
        unsafe {
            let mut result__: NotificationMirroring = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<NotificationMirroring>(result__)
        }
    }
    pub fn SetNotificationMirroring(
        &self,
        value: NotificationMirroring,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IToastNotification3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn RemoteId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IToastNotification3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetRemoteId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IToastNotification3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Data(&self) -> ::windows::runtime::Result<NotificationData> {
        let this = &::windows::runtime::Interface::cast::<IToastNotification4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<NotificationData>(result__)
        }
    }
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, NotificationData>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IToastNotification4>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Priority(&self) -> ::windows::runtime::Result<ToastNotificationPriority> {
        let this = &::windows::runtime::Interface::cast::<IToastNotification4>(self)?;
        unsafe {
            let mut result__: ToastNotificationPriority = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ToastNotificationPriority>(result__)
        }
    }
    pub fn SetPriority(&self, value: ToastNotificationPriority) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IToastNotification4>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ExpiresOnReboot(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IToastNotification6>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExpiresOnReboot(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IToastNotification6>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IToastNotificationFactory<
        R,
        F: FnOnce(&IToastNotificationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ToastNotification,
            IToastNotificationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ToastNotification {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.Notifications.ToastNotification;{997e2675-059e-4e60-8b06-1760917c8b80})",
    );
}
unsafe impl ::windows::runtime::Interface for ToastNotification {
    type Vtable = IToastNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2575181429,
        1438,
        20064,
        [139, 6, 23, 96, 145, 124, 139, 128],
    );
}
impl ::windows::runtime::RuntimeName for ToastNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotification";
}
impl ::std::convert::From<ToastNotification> for ::windows::runtime::IUnknown {
    fn from(value: ToastNotification) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ToastNotification> for ::windows::runtime::IUnknown {
    fn from(value: &ToastNotification) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ToastNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ToastNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ToastNotification> for ::windows::runtime::IInspectable {
    fn from(value: ToastNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ToastNotification> for ::windows::runtime::IInspectable {
    fn from(value: &ToastNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ToastNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ToastNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ToastNotification {}
unsafe impl ::std::marker::Sync for ToastNotification {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ToastNotificationActionTriggerDetail(::windows::runtime::IInspectable);
impl ToastNotificationActionTriggerDetail {
    pub fn Argument(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ToastNotificationActionTriggerDetail {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Notifications.ToastNotificationActionTriggerDetail;{9445135a-38f3-42f6-96aa-7955b0f03da2})" ) ;
}
unsafe impl ::windows::runtime::Interface for ToastNotificationActionTriggerDetail {
    type Vtable = IToastNotificationActionTriggerDetail_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2487554906,
        14579,
        17142,
        [150, 170, 121, 85, 176, 240, 61, 162],
    );
}
impl ::windows::runtime::RuntimeName for ToastNotificationActionTriggerDetail {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationActionTriggerDetail";
}
impl ::std::convert::From<ToastNotificationActionTriggerDetail> for ::windows::runtime::IUnknown {
    fn from(value: ToastNotificationActionTriggerDetail) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ToastNotificationActionTriggerDetail> for ::windows::runtime::IUnknown {
    fn from(value: &ToastNotificationActionTriggerDetail) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ToastNotificationActionTriggerDetail
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ToastNotificationActionTriggerDetail
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ToastNotificationActionTriggerDetail>
    for ::windows::runtime::IInspectable
{
    fn from(value: ToastNotificationActionTriggerDetail) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ToastNotificationActionTriggerDetail>
    for ::windows::runtime::IInspectable
{
    fn from(value: &ToastNotificationActionTriggerDetail) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ToastNotificationActionTriggerDetail
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ToastNotificationActionTriggerDetail
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
pub struct ToastNotificationHistory(::windows::runtime::IInspectable);
impl ToastNotificationHistory {
    pub fn RemoveGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        group: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                group.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveGroupWithId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        group: Param0,
        applicationid: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                group.into_param().abi(),
                applicationid.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveGroupedTagWithId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        tag: Param0,
        group: Param1,
        applicationid: Param2,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                tag.into_param().abi(),
                group.into_param().abi(),
                applicationid.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveGroupedTag<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        tag: Param0,
        group: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                tag.into_param().abi(),
                group.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        tag: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                tag.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ClearWithId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        applicationid: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                applicationid.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetHistory(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<ToastNotification>,
    > {
        let this = &::windows::runtime::Interface::cast::<IToastNotificationHistory2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<ToastNotification>>(
                result__,
            )
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetHistoryWithId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        applicationid: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<ToastNotification>,
    > {
        let this = &::windows::runtime::Interface::cast::<IToastNotificationHistory2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                applicationid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<ToastNotification>>(
                result__,
            )
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ToastNotificationHistory {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Notifications.ToastNotificationHistory;{5caddc63-01d3-4c97-986f-0533483fee14})" ) ;
}
unsafe impl ::windows::runtime::Interface for ToastNotificationHistory {
    type Vtable = IToastNotificationHistory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1554898019,
        467,
        19607,
        [152, 111, 5, 51, 72, 63, 238, 20],
    );
}
impl ::windows::runtime::RuntimeName for ToastNotificationHistory {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationHistory";
}
impl ::std::convert::From<ToastNotificationHistory> for ::windows::runtime::IUnknown {
    fn from(value: ToastNotificationHistory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ToastNotificationHistory> for ::windows::runtime::IUnknown {
    fn from(value: &ToastNotificationHistory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ToastNotificationHistory
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ToastNotificationHistory
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ToastNotificationHistory> for ::windows::runtime::IInspectable {
    fn from(value: ToastNotificationHistory) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ToastNotificationHistory> for ::windows::runtime::IInspectable {
    fn from(value: &ToastNotificationHistory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ToastNotificationHistory
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ToastNotificationHistory
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
pub struct ToastNotificationHistoryChangedTriggerDetail(::windows::runtime::IInspectable);
impl ToastNotificationHistoryChangedTriggerDetail {
    pub fn ChangeType(&self) -> ::windows::runtime::Result<ToastHistoryChangedType> {
        let this = self;
        unsafe {
            let mut result__: ToastHistoryChangedType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ToastHistoryChangedType>(result__)
        }
    }
    pub fn CollectionId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            IToastNotificationHistoryChangedTriggerDetail2,
        >(self)?;
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
}
unsafe impl ::windows::runtime::RuntimeType for ToastNotificationHistoryChangedTriggerDetail {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail;{db037ffa-0068-412c-9c83-267c37f65670})" ) ;
}
unsafe impl ::windows::runtime::Interface for ToastNotificationHistoryChangedTriggerDetail {
    type Vtable = IToastNotificationHistoryChangedTriggerDetail_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3674439674,
        104,
        16684,
        [156, 131, 38, 124, 55, 246, 86, 112],
    );
}
impl ::windows::runtime::RuntimeName for ToastNotificationHistoryChangedTriggerDetail {
    const NAME: &'static str =
        "Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail";
}
impl ::std::convert::From<ToastNotificationHistoryChangedTriggerDetail>
    for ::windows::runtime::IUnknown
{
    fn from(value: ToastNotificationHistoryChangedTriggerDetail) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ToastNotificationHistoryChangedTriggerDetail>
    for ::windows::runtime::IUnknown
{
    fn from(value: &ToastNotificationHistoryChangedTriggerDetail) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ToastNotificationHistoryChangedTriggerDetail
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ToastNotificationHistoryChangedTriggerDetail
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ToastNotificationHistoryChangedTriggerDetail>
    for ::windows::runtime::IInspectable
{
    fn from(value: ToastNotificationHistoryChangedTriggerDetail) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ToastNotificationHistoryChangedTriggerDetail>
    for ::windows::runtime::IInspectable
{
    fn from(value: &ToastNotificationHistoryChangedTriggerDetail) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ToastNotificationHistoryChangedTriggerDetail
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ToastNotificationHistoryChangedTriggerDetail
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
pub struct ToastNotificationManager {}
impl ToastNotificationManager {
    pub fn CreateToastNotifier() -> ::windows::runtime::Result<ToastNotifier> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ToastNotifier>(result__)
        })
    }
    pub fn CreateToastNotifierWithId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        applicationid: Param0,
    ) -> ::windows::runtime::Result<ToastNotifier> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                applicationid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ToastNotifier>(result__)
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(
        r#type: ToastTemplateType,
    ) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                r#type,
                &mut result__,
            )
            .from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        })
    }
    pub fn History() -> ::windows::runtime::Result<ToastNotificationHistory> {
        Self::IToastNotificationManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ToastNotificationHistory>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(
        user: Param0,
    ) -> ::windows::runtime::Result<ToastNotificationManagerForUser> {
        Self::IToastNotificationManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                user.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ToastNotificationManagerForUser>(result__)
        })
    }
    pub fn ConfigureNotificationMirroring(
        value: NotificationMirroring,
    ) -> ::windows::runtime::Result<()> {
        Self::IToastNotificationManagerStatics4(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        })
    }
    pub fn GetDefault() -> ::windows::runtime::Result<ToastNotificationManagerForUser> {
        Self::IToastNotificationManagerStatics5(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ToastNotificationManagerForUser>(result__)
        })
    }
    pub fn IToastNotificationManagerStatics<
        R,
        F: FnOnce(&IToastNotificationManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ToastNotificationManager,
            IToastNotificationManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IToastNotificationManagerStatics2<
        R,
        F: FnOnce(&IToastNotificationManagerStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ToastNotificationManager,
            IToastNotificationManagerStatics2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IToastNotificationManagerStatics4<
        R,
        F: FnOnce(&IToastNotificationManagerStatics4) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ToastNotificationManager,
            IToastNotificationManagerStatics4,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IToastNotificationManagerStatics5<
        R,
        F: FnOnce(&IToastNotificationManagerStatics5) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ToastNotificationManager,
            IToastNotificationManagerStatics5,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ToastNotificationManager {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationManager";
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ToastNotificationManagerForUser(::windows::runtime::IInspectable);
impl ToastNotificationManagerForUser {
    pub fn CreateToastNotifier(&self) -> ::windows::runtime::Result<ToastNotifier> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ToastNotifier>(result__)
        }
    }
    pub fn CreateToastNotifierWithId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        applicationid: Param0,
    ) -> ::windows::runtime::Result<ToastNotifier> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                applicationid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ToastNotifier>(result__)
        }
    }
    pub fn History(&self) -> ::windows::runtime::Result<ToastNotificationHistory> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ToastNotificationHistory>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetToastNotifierForToastCollectionIdAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        collectionid: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ToastNotifier>> {
        let this = &::windows::runtime::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                collectionid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<ToastNotifier>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetHistoryForToastCollectionIdAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        collectionid: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ToastNotificationHistory>,
    > {
        let this = &::windows::runtime::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                collectionid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<ToastNotificationHistory>>(
                result__,
            )
        }
    }
    pub fn GetToastCollectionManager(&self) -> ::windows::runtime::Result<ToastCollectionManager> {
        let this = &::windows::runtime::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ToastCollectionManager>(result__)
        }
    }
    pub fn GetToastCollectionManagerWithAppId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        appid: Param0,
    ) -> ::windows::runtime::Result<ToastCollectionManager> {
        let this = &::windows::runtime::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                appid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ToastCollectionManager>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ToastNotificationManagerForUser {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Notifications.ToastNotificationManagerForUser;{79ab57f6-43fe-487b-8a7f-99567200ae94})" ) ;
}
unsafe impl ::windows::runtime::Interface for ToastNotificationManagerForUser {
    type Vtable = IToastNotificationManagerForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2041272310,
        17406,
        18555,
        [138, 127, 153, 86, 114, 0, 174, 148],
    );
}
impl ::windows::runtime::RuntimeName for ToastNotificationManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationManagerForUser";
}
impl ::std::convert::From<ToastNotificationManagerForUser> for ::windows::runtime::IUnknown {
    fn from(value: ToastNotificationManagerForUser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ToastNotificationManagerForUser> for ::windows::runtime::IUnknown {
    fn from(value: &ToastNotificationManagerForUser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ToastNotificationManagerForUser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ToastNotificationManagerForUser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ToastNotificationManagerForUser> for ::windows::runtime::IInspectable {
    fn from(value: ToastNotificationManagerForUser) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ToastNotificationManagerForUser> for ::windows::runtime::IInspectable {
    fn from(value: &ToastNotificationManagerForUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ToastNotificationManagerForUser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ToastNotificationManagerForUser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ToastNotificationManagerForUser {}
unsafe impl ::std::marker::Sync for ToastNotificationManagerForUser {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ToastNotificationPriority(pub i32);
impl ToastNotificationPriority {
    pub const Default: ToastNotificationPriority = ToastNotificationPriority(0i32);
    pub const High: ToastNotificationPriority = ToastNotificationPriority(1i32);
}
impl ::std::convert::From<i32> for ToastNotificationPriority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ToastNotificationPriority {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ToastNotificationPriority {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Notifications.ToastNotificationPriority;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ToastNotifier(::windows::runtime::IInspectable);
impl ToastNotifier {
    pub fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, ToastNotification>>(
        &self,
        notification: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                notification.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Hide<'a, Param0: ::windows::runtime::IntoParam<'a, ToastNotification>>(
        &self,
        notification: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                notification.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Setting(&self) -> ::windows::runtime::Result<NotificationSetting> {
        let this = self;
        unsafe {
            let mut result__: NotificationSetting = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<NotificationSetting>(result__)
        }
    }
    pub fn AddToSchedule<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ScheduledToastNotification>,
    >(
        &self,
        scheduledtoast: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                scheduledtoast.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveFromSchedule<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ScheduledToastNotification>,
    >(
        &self,
        scheduledtoast: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                scheduledtoast.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetScheduledToastNotifications(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<ScheduledToastNotification>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<
                ScheduledToastNotification,
            >>(result__)
        }
    }
    pub fn UpdateWithTagAndGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, NotificationData>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        data: Param0,
        tag: Param1,
        group: Param2,
    ) -> ::windows::runtime::Result<NotificationUpdateResult> {
        let this = &::windows::runtime::Interface::cast::<IToastNotifier2>(self)?;
        unsafe {
            let mut result__: NotificationUpdateResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                data.into_param().abi(),
                tag.into_param().abi(),
                group.into_param().abi(),
                &mut result__,
            )
            .from_abi::<NotificationUpdateResult>(result__)
        }
    }
    pub fn UpdateWithTag<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, NotificationData>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        data: Param0,
        tag: Param1,
    ) -> ::windows::runtime::Result<NotificationUpdateResult> {
        let this = &::windows::runtime::Interface::cast::<IToastNotifier2>(self)?;
        unsafe {
            let mut result__: NotificationUpdateResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                data.into_param().abi(),
                tag.into_param().abi(),
                &mut result__,
            )
            .from_abi::<NotificationUpdateResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ScheduledToastNotificationShowing<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                ToastNotifier,
                ScheduledToastNotificationShowingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IToastNotifier3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveScheduledToastNotificationShowing<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IToastNotifier3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ToastNotifier {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.Notifications.ToastNotifier;{75927b93-03f3-41ec-91d3-6e5bac1b38e7})",
    );
}
unsafe impl ::windows::runtime::Interface for ToastNotifier {
    type Vtable = IToastNotifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1972534163,
        1011,
        16876,
        [145, 211, 110, 91, 172, 27, 56, 231],
    );
}
impl ::windows::runtime::RuntimeName for ToastNotifier {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotifier";
}
impl ::std::convert::From<ToastNotifier> for ::windows::runtime::IUnknown {
    fn from(value: ToastNotifier) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ToastNotifier> for ::windows::runtime::IUnknown {
    fn from(value: &ToastNotifier) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ToastNotifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ToastNotifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ToastNotifier> for ::windows::runtime::IInspectable {
    fn from(value: ToastNotifier) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ToastNotifier> for ::windows::runtime::IInspectable {
    fn from(value: &ToastNotifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ToastNotifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ToastNotifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ToastNotifier {}
unsafe impl ::std::marker::Sync for ToastNotifier {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ToastTemplateType(pub i32);
impl ToastTemplateType {
    pub const ToastImageAndText01: ToastTemplateType = ToastTemplateType(0i32);
    pub const ToastImageAndText02: ToastTemplateType = ToastTemplateType(1i32);
    pub const ToastImageAndText03: ToastTemplateType = ToastTemplateType(2i32);
    pub const ToastImageAndText04: ToastTemplateType = ToastTemplateType(3i32);
    pub const ToastText01: ToastTemplateType = ToastTemplateType(4i32);
    pub const ToastText02: ToastTemplateType = ToastTemplateType(5i32);
    pub const ToastText03: ToastTemplateType = ToastTemplateType(6i32);
    pub const ToastText04: ToastTemplateType = ToastTemplateType(7i32);
}
impl ::std::convert::From<i32> for ToastTemplateType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ToastTemplateType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ToastTemplateType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Notifications.ToastTemplateType;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct UserNotification(::windows::runtime::IInspectable);
impl UserNotification {
    pub fn Notification(&self) -> ::windows::runtime::Result<Notification> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<Notification>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn AppInfo(&self) -> ::windows::runtime::Result<super::super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::AppInfo>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
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
    #[cfg(feature = "Foundation")]
    pub fn CreationTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserNotification {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.Notifications.UserNotification;{adf7e52f-4e53-42d5-9c33-eb5ea515b23e})",
    );
}
unsafe impl ::windows::runtime::Interface for UserNotification {
    type Vtable = IUserNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2918704431,
        20051,
        17109,
        [156, 51, 235, 94, 165, 21, 178, 62],
    );
}
impl ::windows::runtime::RuntimeName for UserNotification {
    const NAME: &'static str = "Windows.UI.Notifications.UserNotification";
}
impl ::std::convert::From<UserNotification> for ::windows::runtime::IUnknown {
    fn from(value: UserNotification) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserNotification> for ::windows::runtime::IUnknown {
    fn from(value: &UserNotification) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UserNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<UserNotification> for ::windows::runtime::IInspectable {
    fn from(value: UserNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserNotification> for ::windows::runtime::IInspectable {
    fn from(value: &UserNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a UserNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UserNotification {}
unsafe impl ::std::marker::Sync for UserNotification {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct UserNotificationChangedEventArgs(::windows::runtime::IInspectable);
impl UserNotificationChangedEventArgs {
    pub fn ChangeKind(&self) -> ::windows::runtime::Result<UserNotificationChangedKind> {
        let this = self;
        unsafe {
            let mut result__: UserNotificationChangedKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<UserNotificationChangedKind>(result__)
        }
    }
    pub fn UserNotificationId(&self) -> ::windows::runtime::Result<u32> {
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
unsafe impl ::windows::runtime::RuntimeType for UserNotificationChangedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Notifications.UserNotificationChangedEventArgs;{b6bd6839-79cf-4b25-82c0-0ce1eef81f8c})" ) ;
}
unsafe impl ::windows::runtime::Interface for UserNotificationChangedEventArgs {
    type Vtable = IUserNotificationChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3065866297,
        31183,
        19237,
        [130, 192, 12, 225, 238, 248, 31, 140],
    );
}
impl ::windows::runtime::RuntimeName for UserNotificationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.UserNotificationChangedEventArgs";
}
impl ::std::convert::From<UserNotificationChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: UserNotificationChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserNotificationChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &UserNotificationChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for UserNotificationChangedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &UserNotificationChangedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<UserNotificationChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: UserNotificationChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserNotificationChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &UserNotificationChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for UserNotificationChangedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a UserNotificationChangedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UserNotificationChangedEventArgs {}
unsafe impl ::std::marker::Sync for UserNotificationChangedEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct UserNotificationChangedKind(pub i32);
impl UserNotificationChangedKind {
    pub const Added: UserNotificationChangedKind = UserNotificationChangedKind(0i32);
    pub const Removed: UserNotificationChangedKind = UserNotificationChangedKind(1i32);
}
impl ::std::convert::From<i32> for UserNotificationChangedKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserNotificationChangedKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserNotificationChangedKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Notifications.UserNotificationChangedKind;i4)",
    );
}
