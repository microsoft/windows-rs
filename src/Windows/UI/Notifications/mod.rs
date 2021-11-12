#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "UI_Notifications_Management")]
pub mod Management;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AdaptiveNotificationContentKind(pub i32);
impl AdaptiveNotificationContentKind {
    pub const Text: AdaptiveNotificationContentKind = AdaptiveNotificationContentKind(0i32);
}
impl ::core::convert::From<i32> for AdaptiveNotificationContentKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AdaptiveNotificationContentKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AdaptiveNotificationContentKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.AdaptiveNotificationContentKind;i4)");
}
impl ::windows::core::DefaultType for AdaptiveNotificationContentKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AdaptiveNotificationText(pub ::windows::core::IInspectable);
impl AdaptiveNotificationText {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AdaptiveNotificationText, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<AdaptiveNotificationContentKind> {
        let this = &::windows::core::Interface::cast::<IAdaptiveNotificationContent>(self)?;
        unsafe {
            let mut result__: AdaptiveNotificationContentKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveNotificationContentKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Hints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IAdaptiveNotificationContent>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveNotificationText {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.AdaptiveNotificationText;{46d4a3be-609a-4326-a40b-bfde872034a3})");
}
unsafe impl ::windows::core::Interface for AdaptiveNotificationText {
    type Vtable = IAdaptiveNotificationText_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46d4a3be_609a_4326_a40b_bfde872034a3);
}
impl ::windows::core::RuntimeName for AdaptiveNotificationText {
    const NAME: &'static str = "Windows.UI.Notifications.AdaptiveNotificationText";
}
impl ::core::convert::From<AdaptiveNotificationText> for ::windows::core::IUnknown {
    fn from(value: AdaptiveNotificationText) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AdaptiveNotificationText> for ::windows::core::IUnknown {
    fn from(value: &AdaptiveNotificationText) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdaptiveNotificationText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdaptiveNotificationText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AdaptiveNotificationText> for ::windows::core::IInspectable {
    fn from(value: AdaptiveNotificationText) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AdaptiveNotificationText> for ::windows::core::IInspectable {
    fn from(value: &AdaptiveNotificationText) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdaptiveNotificationText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdaptiveNotificationText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<AdaptiveNotificationText> for IAdaptiveNotificationContent {
    type Error = ::windows::core::Error;
    fn try_from(value: AdaptiveNotificationText) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AdaptiveNotificationText> for IAdaptiveNotificationContent {
    type Error = ::windows::core::Error;
    fn try_from(value: &AdaptiveNotificationText) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAdaptiveNotificationContent> for AdaptiveNotificationText {
    fn into_param(self) -> ::windows::core::Param<'a, IAdaptiveNotificationContent> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAdaptiveNotificationContent> for &AdaptiveNotificationText {
    fn into_param(self) -> ::windows::core::Param<'a, IAdaptiveNotificationContent> {
        ::core::convert::TryInto::<IAdaptiveNotificationContent>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AdaptiveNotificationText {}
unsafe impl ::core::marker::Sync for AdaptiveNotificationText {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BadgeNotification(pub ::windows::core::IInspectable);
impl BadgeNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateBadgeNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>>(content: Param0) -> ::windows::core::Result<BadgeNotification> {
        Self::IBadgeNotificationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), content.into_param().abi(), &mut result__).from_abi::<BadgeNotification>(result__)
        })
    }
    pub fn IBadgeNotificationFactory<R, F: FnOnce(&IBadgeNotificationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BadgeNotification, IBadgeNotificationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BadgeNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.BadgeNotification;{075cb4ca-d08a-4e2f-9233-7e289c1f7722})");
}
unsafe impl ::windows::core::Interface for BadgeNotification {
    type Vtable = IBadgeNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x075cb4ca_d08a_4e2f_9233_7e289c1f7722);
}
impl ::windows::core::RuntimeName for BadgeNotification {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeNotification";
}
impl ::core::convert::From<BadgeNotification> for ::windows::core::IUnknown {
    fn from(value: BadgeNotification) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BadgeNotification> for ::windows::core::IUnknown {
    fn from(value: &BadgeNotification) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BadgeNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BadgeNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BadgeNotification> for ::windows::core::IInspectable {
    fn from(value: BadgeNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BadgeNotification> for ::windows::core::IInspectable {
    fn from(value: &BadgeNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BadgeNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BadgeNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BadgeNotification {}
unsafe impl ::core::marker::Sync for BadgeNotification {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BadgeTemplateType(pub i32);
impl BadgeTemplateType {
    pub const BadgeGlyph: BadgeTemplateType = BadgeTemplateType(0i32);
    pub const BadgeNumber: BadgeTemplateType = BadgeTemplateType(1i32);
}
impl ::core::convert::From<i32> for BadgeTemplateType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BadgeTemplateType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BadgeTemplateType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.BadgeTemplateType;i4)");
}
impl ::windows::core::DefaultType for BadgeTemplateType {
    type DefaultType = Self;
}
pub struct BadgeUpdateManager {}
impl BadgeUpdateManager {
    pub fn CreateBadgeUpdaterForApplication() -> ::windows::core::Result<BadgeUpdater> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BadgeUpdater>(result__)
        })
    }
    pub fn CreateBadgeUpdaterForApplicationWithId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(applicationid: Param0) -> ::windows::core::Result<BadgeUpdater> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<BadgeUpdater>(result__)
        })
    }
    pub fn CreateBadgeUpdaterForSecondaryTile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(tileid: Param0) -> ::windows::core::Result<BadgeUpdater> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<BadgeUpdater>(result__)
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(r#type: BadgeTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<BadgeUpdateManagerForUser> {
        Self::IBadgeUpdateManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<BadgeUpdateManagerForUser>(result__)
        })
    }
    pub fn IBadgeUpdateManagerStatics<R, F: FnOnce(&IBadgeUpdateManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BadgeUpdateManager, IBadgeUpdateManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBadgeUpdateManagerStatics2<R, F: FnOnce(&IBadgeUpdateManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BadgeUpdateManager, IBadgeUpdateManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for BadgeUpdateManager {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeUpdateManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BadgeUpdateManagerForUser(pub ::windows::core::IInspectable);
impl BadgeUpdateManagerForUser {
    pub fn CreateBadgeUpdaterForApplication(&self) -> ::windows::core::Result<BadgeUpdater> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BadgeUpdater>(result__)
        }
    }
    pub fn CreateBadgeUpdaterForApplicationWithId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, applicationid: Param0) -> ::windows::core::Result<BadgeUpdater> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<BadgeUpdater>(result__)
        }
    }
    pub fn CreateBadgeUpdaterForSecondaryTile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tileid: Param0) -> ::windows::core::Result<BadgeUpdater> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<BadgeUpdater>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BadgeUpdateManagerForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.BadgeUpdateManagerForUser;{996b21bc-0386-44e5-ba8d-0c1077a62e92})");
}
unsafe impl ::windows::core::Interface for BadgeUpdateManagerForUser {
    type Vtable = IBadgeUpdateManagerForUser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x996b21bc_0386_44e5_ba8d_0c1077a62e92);
}
impl ::windows::core::RuntimeName for BadgeUpdateManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeUpdateManagerForUser";
}
impl ::core::convert::From<BadgeUpdateManagerForUser> for ::windows::core::IUnknown {
    fn from(value: BadgeUpdateManagerForUser) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BadgeUpdateManagerForUser> for ::windows::core::IUnknown {
    fn from(value: &BadgeUpdateManagerForUser) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BadgeUpdateManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BadgeUpdateManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BadgeUpdateManagerForUser> for ::windows::core::IInspectable {
    fn from(value: BadgeUpdateManagerForUser) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BadgeUpdateManagerForUser> for ::windows::core::IInspectable {
    fn from(value: &BadgeUpdateManagerForUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BadgeUpdateManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BadgeUpdateManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BadgeUpdateManagerForUser {}
unsafe impl ::core::marker::Sync for BadgeUpdateManagerForUser {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BadgeUpdater(pub ::windows::core::IInspectable);
impl BadgeUpdater {
    pub fn Update<'a, Param0: ::windows::core::IntoParam<'a, BadgeNotification>>(&self, notification: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), notification.into_param().abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, badgecontent: Param0, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), badgecontent.into_param().abi(), requestedinterval).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdateAtTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, badgecontent: Param0, starttime: Param1, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), badgecontent.into_param().abi(), starttime.into_param().abi(), requestedinterval).ok() }
    }
    pub fn StopPeriodicUpdate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for BadgeUpdater {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.BadgeUpdater;{b5fa1fd4-7562-4f6c-bfa3-1b6ed2e57f2f})");
}
unsafe impl ::windows::core::Interface for BadgeUpdater {
    type Vtable = IBadgeUpdater_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5fa1fd4_7562_4f6c_bfa3_1b6ed2e57f2f);
}
impl ::windows::core::RuntimeName for BadgeUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeUpdater";
}
impl ::core::convert::From<BadgeUpdater> for ::windows::core::IUnknown {
    fn from(value: BadgeUpdater) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BadgeUpdater> for ::windows::core::IUnknown {
    fn from(value: &BadgeUpdater) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BadgeUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BadgeUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BadgeUpdater> for ::windows::core::IInspectable {
    fn from(value: BadgeUpdater) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BadgeUpdater> for ::windows::core::IInspectable {
    fn from(value: &BadgeUpdater) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BadgeUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BadgeUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BadgeUpdater {}
unsafe impl ::core::marker::Sync for BadgeUpdater {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAdaptiveNotificationContent(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdaptiveNotificationContent {
    type Vtable = IAdaptiveNotificationContent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb0dbe66_7448_448d_9db8_d78acd2abba9);
}
impl IAdaptiveNotificationContent {
    pub fn Kind(&self) -> ::windows::core::Result<AdaptiveNotificationContentKind> {
        let this = self;
        unsafe {
            let mut result__: AdaptiveNotificationContentKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveNotificationContentKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Hints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IAdaptiveNotificationContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{eb0dbe66-7448-448d-9db8-d78acd2abba9}");
}
impl ::core::convert::From<IAdaptiveNotificationContent> for ::windows::core::IUnknown {
    fn from(value: IAdaptiveNotificationContent) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAdaptiveNotificationContent> for ::windows::core::IUnknown {
    fn from(value: &IAdaptiveNotificationContent) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAdaptiveNotificationContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAdaptiveNotificationContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAdaptiveNotificationContent> for ::windows::core::IInspectable {
    fn from(value: IAdaptiveNotificationContent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAdaptiveNotificationContent> for ::windows::core::IInspectable {
    fn from(value: &IAdaptiveNotificationContent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAdaptiveNotificationContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAdaptiveNotificationContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveNotificationContent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AdaptiveNotificationContentKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveNotificationText(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdaptiveNotificationText {
    type Vtable = IAdaptiveNotificationText_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46d4a3be_609a_4326_a40b_bfde872034a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveNotificationText_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBadgeNotification(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBadgeNotification {
    type Vtable = IBadgeNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x075cb4ca_d08a_4e2f_9233_7e289c1f7722);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBadgeNotificationFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBadgeNotificationFactory {
    type Vtable = IBadgeNotificationFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedf255ce_0618_4d59_948a_5a61040c52f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeNotificationFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerForUser(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBadgeUpdateManagerForUser {
    type Vtable = IBadgeUpdateManagerForUser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x996b21bc_0386_44e5_ba8d_0c1077a62e92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerForUser_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBadgeUpdateManagerStatics {
    type Vtable = IBadgeUpdateManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33400faa_6dd5_4105_aebc_9b50fca492da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: BadgeTemplateType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBadgeUpdateManagerStatics2 {
    type Vtable = IBadgeUpdateManagerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x979a35ce_f940_48bf_94e8_ca244d400b41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBadgeUpdater(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBadgeUpdater {
    type Vtable = IBadgeUpdater_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5fa1fd4_7562_4f6c_bfa3_1b6ed2e57f2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeUpdater_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, notification: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, badgecontent: ::windows::core::RawPtr, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, badgecontent: ::windows::core::RawPtr, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownAdaptiveNotificationHintsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKnownAdaptiveNotificationHintsStatics {
    type Vtable = IKnownAdaptiveNotificationHintsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06206598_d496_497d_8692_4f7d7c2770df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownAdaptiveNotificationHintsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownAdaptiveNotificationTextStylesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKnownAdaptiveNotificationTextStylesStatics {
    type Vtable = IKnownAdaptiveNotificationTextStylesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x202192d7_8996_45aa_8ba1_d461d72c2a1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownAdaptiveNotificationTextStylesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownNotificationBindingsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKnownNotificationBindingsStatics {
    type Vtable = IKnownNotificationBindingsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79427bae_a8b7_4d58_89ea_76a7b7bccded);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownNotificationBindingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INotification(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INotification {
    type Vtable = INotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x108037fe_eb76_4f82_97bc_da07530a2e20);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotification_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INotificationBinding(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INotificationBinding {
    type Vtable = INotificationBinding_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf29e4b85_0370_4ad3_b4ea_da9e35e7eabf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationBinding_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INotificationData(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INotificationData {
    type Vtable = INotificationData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ffd2312_9d6a_4aaf_b6ac_ff17f0c1f280);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INotificationDataFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INotificationDataFactory {
    type Vtable = INotificationDataFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23c1e33a_1c10_46fb_8040_dec384621cf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationDataFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, initialvalues: ::windows::core::RawPtr, sequencenumber: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, initialvalues: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INotificationVisual(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INotificationVisual {
    type Vtable = INotificationVisual_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68835b8e_aa56_4e11_86d3_5f9a6957bc5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationVisual_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, templatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScheduledTileNotification(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScheduledTileNotification {
    type Vtable = IScheduledTileNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0abca6d5_99dc_4c78_a11c_c9e7f86d7ef7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledTileNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScheduledTileNotificationFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScheduledTileNotificationFactory {
    type Vtable = IScheduledTileNotificationFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3383138a_98c0_4c3b_bbd6_4a633c7cfc29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledTileNotificationFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr, deliverytime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Data_Xml_Dom", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScheduledToastNotification(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScheduledToastNotification {
    type Vtable = IScheduledToastNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79f577f8_0de7_48cd_9740_9b370490c838);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScheduledToastNotification2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScheduledToastNotification2 {
    type Vtable = IScheduledToastNotification2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa66ea09c_31b4_43b0_b5dd_7a40e85363b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotification2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScheduledToastNotification3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScheduledToastNotification3 {
    type Vtable = IScheduledToastNotification3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98429e8b_bd32_4a3b_9d15_22aea49462a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotification3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut NotificationMirroring) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: NotificationMirroring) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScheduledToastNotification4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScheduledToastNotification4 {
    type Vtable = IScheduledToastNotification4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d4761fd_bdef_4e4a_96be_0101369b58d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotification4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScheduledToastNotificationFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScheduledToastNotificationFactory {
    type Vtable = IScheduledToastNotificationFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7bed191_0bb9_4189_8394_31761b476fd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotificationFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr, deliverytime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Data_Xml_Dom", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr, deliverytime: super::super::Foundation::DateTime, snoozeinterval: super::super::Foundation::TimeSpan, maximumsnoozecount: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Data_Xml_Dom", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScheduledToastNotificationShowingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScheduledToastNotificationShowingEventArgs {
    type Vtable = IScheduledToastNotificationShowingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6173f6b4_412a_5e2c_a6ed_a0209aef9a09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotificationShowingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShownTileNotification(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IShownTileNotification {
    type Vtable = IShownTileNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x342d8988_5af2_481a_a6a3_f2fdc78de88e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShownTileNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITileFlyoutNotification(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileFlyoutNotification {
    type Vtable = ITileFlyoutNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a53b261_c70c_42be_b2f3_f42aa97d34e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileFlyoutNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITileFlyoutNotificationFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileFlyoutNotificationFactory {
    type Vtable = ITileFlyoutNotificationFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef556ff5_5226_4f2b_b278_88a35dfe569f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileFlyoutNotificationFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITileFlyoutUpdateManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileFlyoutUpdateManagerStatics {
    type Vtable = ITileFlyoutUpdateManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04363b0b_1ac0_4b99_88e7_ada83e953d48);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileFlyoutUpdateManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: TileFlyoutTemplateType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITileFlyoutUpdater(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileFlyoutUpdater {
    type Vtable = ITileFlyoutUpdater_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d40c76a_c465_4052_a740_5c2654c1a089);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileFlyoutUpdater_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, notification: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tileflyoutcontent: ::windows::core::RawPtr, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tileflyoutcontent: ::windows::core::RawPtr, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut NotificationSetting) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITileNotification(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileNotification {
    type Vtable = ITileNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebaec8fa_50ec_4c18_b4d0_3af02e5540ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITileNotificationFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileNotificationFactory {
    type Vtable = ITileNotificationFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6abdd6e_4928_46c8_bdbf_81a047dea0d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileNotificationFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITileUpdateManagerForUser(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileUpdateManagerForUser {
    type Vtable = ITileUpdateManagerForUser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55141348_2ee2_4e2d_9cc1_216a20decc9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdateManagerForUser_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITileUpdateManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileUpdateManagerStatics {
    type Vtable = ITileUpdateManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda159e5d_3ea9_4986_8d84_b09d5e12276d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdateManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: TileTemplateType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITileUpdateManagerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileUpdateManagerStatics2 {
    type Vtable = ITileUpdateManagerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x731c1ddc_8e14_4b7c_a34b_9d22de76c84d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdateManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITileUpdater(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileUpdater {
    type Vtable = ITileUpdater_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0942a48b_1d91_44ec_9243_c1e821c29a20);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdater_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, notification: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enable: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut NotificationSetting) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scheduledtile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scheduledtile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tilecontent: ::windows::core::RawPtr, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tilecontent: ::windows::core::RawPtr, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tilecontents: ::windows::core::RawPtr, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tilecontents: ::windows::core::RawPtr, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITileUpdater2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileUpdater2 {
    type Vtable = ITileUpdater2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2266e12_15ee_43ed_83f5_65b352bb1a84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdater2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enable: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enable: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enable: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastActivatedEventArgs {
    type Vtable = IToastActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3bf92f3_c197_436f_8265_0625824f8dac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastActivatedEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastActivatedEventArgs2 {
    type Vtable = IToastActivatedEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab7da512_cc61_568e_81be_304ac31038fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastActivatedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastCollection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastCollection {
    type Vtable = IToastCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a8bc3b0_e0be_4858_bc2a_89dfe0b32863);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastCollectionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastCollectionFactory {
    type Vtable = IToastCollectionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x164dd3d7_73c4_44f7_b4ff_fb6d4bf1f4c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastCollectionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, collectionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, launchargs: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, iconuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastCollectionManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastCollectionManager {
    type Vtable = IToastCollectionManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a1821fe_179d_49bc_b79d_a527920d3665);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastCollectionManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, collection: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, collectionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, collectionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastDismissedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastDismissedEventArgs {
    type Vtable = IToastDismissedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f89d935_d9cb_4538_a0f0_ffe7659938f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastDismissedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ToastDismissalReason) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastFailedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastFailedEventArgs {
    type Vtable = IToastFailedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35176862_cfd4_44f8_ad64_f500fd896c3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastFailedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotification(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotification {
    type Vtable = IToastNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x997e2675_059e_4e60_8b06_1760917c8b80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotification2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotification2 {
    type Vtable = IToastNotification2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9dfb9fd1_143a_490e_90bf_b9fba7132de7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotification3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotification3 {
    type Vtable = IToastNotification3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31e8aed8_8141_4f99_bc0a_c4ed21297d77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut NotificationMirroring) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: NotificationMirroring) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotification4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotification4 {
    type Vtable = IToastNotification4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15154935_28ea_4727_88e9_c58680e2d118);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ToastNotificationPriority) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ToastNotificationPriority) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotification6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotification6 {
    type Vtable = IToastNotification6_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43ebfe53_89ae_5c1e_a279_3aecfe9b6f54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification6_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotificationActionTriggerDetail(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotificationActionTriggerDetail {
    type Vtable = IToastNotificationActionTriggerDetail_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9445135a_38f3_42f6_96aa_7955b0f03da2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationActionTriggerDetail_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotificationFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotificationFactory {
    type Vtable = IToastNotificationFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04124b20_82c6_4229_b109_fd9ed4662b53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotificationHistory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotificationHistory {
    type Vtable = IToastNotificationHistory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5caddc63_01d3_4c97_986f_0533483fee14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotificationHistory2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotificationHistory2 {
    type Vtable = IToastNotificationHistory2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bc3d253_2f31_4092_9129_8ad5abf067da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistory2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotificationHistoryChangedTriggerDetail(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotificationHistoryChangedTriggerDetail {
    type Vtable = IToastNotificationHistoryChangedTriggerDetail_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb037ffa_0068_412c_9c83_267c37f65670);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistoryChangedTriggerDetail_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ToastHistoryChangedType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotificationHistoryChangedTriggerDetail2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotificationHistoryChangedTriggerDetail2 {
    type Vtable = IToastNotificationHistoryChangedTriggerDetail2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b36e982_c871_49fb_babb_25bdbc4cc45b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistoryChangedTriggerDetail2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotificationManagerForUser(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotificationManagerForUser {
    type Vtable = IToastNotificationManagerForUser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79ab57f6_43fe_487b_8a7f_99567200ae94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerForUser_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotificationManagerForUser2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotificationManagerForUser2 {
    type Vtable = IToastNotificationManagerForUser2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x679c64b7_81ab_42c2_8819_c958767753f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerForUser2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, collectionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, collectionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotificationManagerStatics {
    type Vtable = IToastNotificationManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ac103f_d235_4598_bbef_98fe4d1a3ad4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: ToastTemplateType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotificationManagerStatics2 {
    type Vtable = IToastNotificationManagerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ab93c52_0e48_4750_ba9d_1a4113981847);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotificationManagerStatics4 {
    type Vtable = IToastNotificationManagerStatics4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f993fd3_e516_45fb_8130_398e93fa52c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: NotificationMirroring) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotificationManagerStatics5 {
    type Vtable = IToastNotificationManagerStatics5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6f5f569_d40d_407c_8989_88cab42cfd14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotifier(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotifier {
    type Vtable = IToastNotifier_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75927b93_03f3_41ec_91d3_6e5bac1b38e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotifier_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, notification: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, notification: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut NotificationSetting) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scheduledtoast: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scheduledtoast: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotifier2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotifier2 {
    type Vtable = IToastNotifier2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x354389c6_7c01_4bd5_9c20_604340cd2b74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotifier2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::windows::core::RawPtr, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut NotificationUpdateResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::windows::core::RawPtr, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut NotificationUpdateResult) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotifier3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotifier3 {
    type Vtable = IToastNotifier3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae75a04a_3b0c_51ad_b7e8_b08ab6052549);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotifier3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserNotification(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserNotification {
    type Vtable = IUserNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadf7e52f_4e53_42d5_9c33_eb5ea515b23e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserNotificationChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserNotificationChangedEventArgs {
    type Vtable = IUserNotificationChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6bd6839_79cf_4b25_82c0_0ce1eef81f8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotificationChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserNotificationChangedKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
pub struct KnownAdaptiveNotificationHints {}
impl KnownAdaptiveNotificationHints {
    pub fn Style() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Wrap() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MaxLines() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MinLines() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TextStacking() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Align() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IKnownAdaptiveNotificationHintsStatics<R, F: FnOnce(&IKnownAdaptiveNotificationHintsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownAdaptiveNotificationHints, IKnownAdaptiveNotificationHintsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KnownAdaptiveNotificationHints {
    const NAME: &'static str = "Windows.UI.Notifications.KnownAdaptiveNotificationHints";
}
pub struct KnownAdaptiveNotificationTextStyles {}
impl KnownAdaptiveNotificationTextStyles {
    pub fn Caption() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Body() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Base() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Subtitle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Title() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Subheader() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Header() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TitleNumeral() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SubheaderNumeral() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HeaderNumeral() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CaptionSubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BodySubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BaseSubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SubtitleSubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TitleSubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SubheaderSubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SubheaderNumeralSubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HeaderSubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HeaderNumeralSubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IKnownAdaptiveNotificationTextStylesStatics<R, F: FnOnce(&IKnownAdaptiveNotificationTextStylesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownAdaptiveNotificationTextStyles, IKnownAdaptiveNotificationTextStylesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KnownAdaptiveNotificationTextStyles {
    const NAME: &'static str = "Windows.UI.Notifications.KnownAdaptiveNotificationTextStyles";
}
pub struct KnownNotificationBindings {}
impl KnownNotificationBindings {
    pub fn ToastGeneric() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownNotificationBindingsStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IKnownNotificationBindingsStatics<R, F: FnOnce(&IKnownNotificationBindingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownNotificationBindings, IKnownNotificationBindingsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KnownNotificationBindings {
    const NAME: &'static str = "Windows.UI.Notifications.KnownNotificationBindings";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Notification(pub ::windows::core::IInspectable);
impl Notification {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Notification, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Visual(&self) -> ::windows::core::Result<NotificationVisual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NotificationVisual>(result__)
        }
    }
    pub fn SetVisual<'a, Param0: ::windows::core::IntoParam<'a, NotificationVisual>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for Notification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.Notification;{108037fe-eb76-4f82-97bc-da07530a2e20})");
}
unsafe impl ::windows::core::Interface for Notification {
    type Vtable = INotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x108037fe_eb76_4f82_97bc_da07530a2e20);
}
impl ::windows::core::RuntimeName for Notification {
    const NAME: &'static str = "Windows.UI.Notifications.Notification";
}
impl ::core::convert::From<Notification> for ::windows::core::IUnknown {
    fn from(value: Notification) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Notification> for ::windows::core::IUnknown {
    fn from(value: &Notification) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Notification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Notification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Notification> for ::windows::core::IInspectable {
    fn from(value: Notification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Notification> for ::windows::core::IInspectable {
    fn from(value: &Notification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Notification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Notification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Notification {}
unsafe impl ::core::marker::Sync for Notification {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NotificationBinding(pub ::windows::core::IInspectable);
impl NotificationBinding {
    pub fn Template(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTemplate<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Hints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextElements(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AdaptiveNotificationText>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AdaptiveNotificationText>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for NotificationBinding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.NotificationBinding;{f29e4b85-0370-4ad3-b4ea-da9e35e7eabf})");
}
unsafe impl ::windows::core::Interface for NotificationBinding {
    type Vtable = INotificationBinding_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf29e4b85_0370_4ad3_b4ea_da9e35e7eabf);
}
impl ::windows::core::RuntimeName for NotificationBinding {
    const NAME: &'static str = "Windows.UI.Notifications.NotificationBinding";
}
impl ::core::convert::From<NotificationBinding> for ::windows::core::IUnknown {
    fn from(value: NotificationBinding) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NotificationBinding> for ::windows::core::IUnknown {
    fn from(value: &NotificationBinding) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NotificationBinding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NotificationBinding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NotificationBinding> for ::windows::core::IInspectable {
    fn from(value: NotificationBinding) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NotificationBinding> for ::windows::core::IInspectable {
    fn from(value: &NotificationBinding) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NotificationBinding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NotificationBinding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for NotificationBinding {}
unsafe impl ::core::marker::Sync for NotificationBinding {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NotificationData(pub ::windows::core::IInspectable);
impl NotificationData {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NotificationData, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Values(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    pub fn SequenceNumber(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetSequenceNumber(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNotificationDataWithValuesAndSequenceNumber<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>>(initialvalues: Param0, sequencenumber: u32) -> ::windows::core::Result<NotificationData> {
        Self::INotificationDataFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), initialvalues.into_param().abi(), sequencenumber, &mut result__).from_abi::<NotificationData>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNotificationDataWithValues<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>>(initialvalues: Param0) -> ::windows::core::Result<NotificationData> {
        Self::INotificationDataFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), initialvalues.into_param().abi(), &mut result__).from_abi::<NotificationData>(result__)
        })
    }
    pub fn INotificationDataFactory<R, F: FnOnce(&INotificationDataFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NotificationData, INotificationDataFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for NotificationData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.NotificationData;{9ffd2312-9d6a-4aaf-b6ac-ff17f0c1f280})");
}
unsafe impl ::windows::core::Interface for NotificationData {
    type Vtable = INotificationData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ffd2312_9d6a_4aaf_b6ac_ff17f0c1f280);
}
impl ::windows::core::RuntimeName for NotificationData {
    const NAME: &'static str = "Windows.UI.Notifications.NotificationData";
}
impl ::core::convert::From<NotificationData> for ::windows::core::IUnknown {
    fn from(value: NotificationData) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NotificationData> for ::windows::core::IUnknown {
    fn from(value: &NotificationData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NotificationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NotificationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NotificationData> for ::windows::core::IInspectable {
    fn from(value: NotificationData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NotificationData> for ::windows::core::IInspectable {
    fn from(value: &NotificationData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NotificationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NotificationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for NotificationData {}
unsafe impl ::core::marker::Sync for NotificationData {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NotificationKinds(pub u32);
impl NotificationKinds {
    pub const Unknown: NotificationKinds = NotificationKinds(0u32);
    pub const Toast: NotificationKinds = NotificationKinds(1u32);
}
impl ::core::convert::From<u32> for NotificationKinds {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NotificationKinds {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NotificationKinds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.NotificationKinds;u4)");
}
impl ::windows::core::DefaultType for NotificationKinds {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for NotificationKinds {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for NotificationKinds {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for NotificationKinds {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for NotificationKinds {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for NotificationKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NotificationMirroring(pub i32);
impl NotificationMirroring {
    pub const Allowed: NotificationMirroring = NotificationMirroring(0i32);
    pub const Disabled: NotificationMirroring = NotificationMirroring(1i32);
}
impl ::core::convert::From<i32> for NotificationMirroring {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NotificationMirroring {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NotificationMirroring {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.NotificationMirroring;i4)");
}
impl ::windows::core::DefaultType for NotificationMirroring {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NotificationSetting(pub i32);
impl NotificationSetting {
    pub const Enabled: NotificationSetting = NotificationSetting(0i32);
    pub const DisabledForApplication: NotificationSetting = NotificationSetting(1i32);
    pub const DisabledForUser: NotificationSetting = NotificationSetting(2i32);
    pub const DisabledByGroupPolicy: NotificationSetting = NotificationSetting(3i32);
    pub const DisabledByManifest: NotificationSetting = NotificationSetting(4i32);
}
impl ::core::convert::From<i32> for NotificationSetting {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NotificationSetting {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NotificationSetting {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.NotificationSetting;i4)");
}
impl ::windows::core::DefaultType for NotificationSetting {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NotificationUpdateResult(pub i32);
impl NotificationUpdateResult {
    pub const Succeeded: NotificationUpdateResult = NotificationUpdateResult(0i32);
    pub const Failed: NotificationUpdateResult = NotificationUpdateResult(1i32);
    pub const NotificationNotFound: NotificationUpdateResult = NotificationUpdateResult(2i32);
}
impl ::core::convert::From<i32> for NotificationUpdateResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NotificationUpdateResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NotificationUpdateResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.NotificationUpdateResult;i4)");
}
impl ::windows::core::DefaultType for NotificationUpdateResult {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NotificationVisual(pub ::windows::core::IInspectable);
impl NotificationVisual {
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Bindings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<NotificationBinding>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<NotificationBinding>>(result__)
        }
    }
    pub fn GetBinding<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, templatename: Param0) -> ::windows::core::Result<NotificationBinding> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), templatename.into_param().abi(), &mut result__).from_abi::<NotificationBinding>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for NotificationVisual {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.NotificationVisual;{68835b8e-aa56-4e11-86d3-5f9a6957bc5b})");
}
unsafe impl ::windows::core::Interface for NotificationVisual {
    type Vtable = INotificationVisual_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68835b8e_aa56_4e11_86d3_5f9a6957bc5b);
}
impl ::windows::core::RuntimeName for NotificationVisual {
    const NAME: &'static str = "Windows.UI.Notifications.NotificationVisual";
}
impl ::core::convert::From<NotificationVisual> for ::windows::core::IUnknown {
    fn from(value: NotificationVisual) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NotificationVisual> for ::windows::core::IUnknown {
    fn from(value: &NotificationVisual) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NotificationVisual {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NotificationVisual {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NotificationVisual> for ::windows::core::IInspectable {
    fn from(value: NotificationVisual) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NotificationVisual> for ::windows::core::IInspectable {
    fn from(value: &NotificationVisual) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NotificationVisual {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NotificationVisual {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for NotificationVisual {}
unsafe impl ::core::marker::Sync for NotificationVisual {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PeriodicUpdateRecurrence(pub i32);
impl PeriodicUpdateRecurrence {
    pub const HalfHour: PeriodicUpdateRecurrence = PeriodicUpdateRecurrence(0i32);
    pub const Hour: PeriodicUpdateRecurrence = PeriodicUpdateRecurrence(1i32);
    pub const SixHours: PeriodicUpdateRecurrence = PeriodicUpdateRecurrence(2i32);
    pub const TwelveHours: PeriodicUpdateRecurrence = PeriodicUpdateRecurrence(3i32);
    pub const Daily: PeriodicUpdateRecurrence = PeriodicUpdateRecurrence(4i32);
}
impl ::core::convert::From<i32> for PeriodicUpdateRecurrence {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PeriodicUpdateRecurrence {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PeriodicUpdateRecurrence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.PeriodicUpdateRecurrence;i4)");
}
impl ::windows::core::DefaultType for PeriodicUpdateRecurrence {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ScheduledTileNotification(pub ::windows::core::IInspectable);
impl ScheduledTileNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeliveryTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    pub fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub fn CreateScheduledTileNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(content: Param0, deliverytime: Param1) -> ::windows::core::Result<ScheduledTileNotification> {
        Self::IScheduledTileNotificationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), content.into_param().abi(), deliverytime.into_param().abi(), &mut result__).from_abi::<ScheduledTileNotification>(result__)
        })
    }
    pub fn IScheduledTileNotificationFactory<R, F: FnOnce(&IScheduledTileNotificationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ScheduledTileNotification, IScheduledTileNotificationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ScheduledTileNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ScheduledTileNotification;{0abca6d5-99dc-4c78-a11c-c9e7f86d7ef7})");
}
unsafe impl ::windows::core::Interface for ScheduledTileNotification {
    type Vtable = IScheduledTileNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0abca6d5_99dc_4c78_a11c_c9e7f86d7ef7);
}
impl ::windows::core::RuntimeName for ScheduledTileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ScheduledTileNotification";
}
impl ::core::convert::From<ScheduledTileNotification> for ::windows::core::IUnknown {
    fn from(value: ScheduledTileNotification) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ScheduledTileNotification> for ::windows::core::IUnknown {
    fn from(value: &ScheduledTileNotification) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ScheduledTileNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ScheduledTileNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ScheduledTileNotification> for ::windows::core::IInspectable {
    fn from(value: ScheduledTileNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ScheduledTileNotification> for ::windows::core::IInspectable {
    fn from(value: &ScheduledTileNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ScheduledTileNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ScheduledTileNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ScheduledTileNotification {}
unsafe impl ::core::marker::Sync for ScheduledTileNotification {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ScheduledToastNotification(pub ::windows::core::IInspectable);
impl ScheduledToastNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeliveryTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SnoozeInterval(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    pub fn MaximumSnoozeCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetGroup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSuppressPopup(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SuppressPopup(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub fn CreateScheduledToastNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(content: Param0, deliverytime: Param1) -> ::windows::core::Result<ScheduledToastNotification> {
        Self::IScheduledToastNotificationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), content.into_param().abi(), deliverytime.into_param().abi(), &mut result__).from_abi::<ScheduledToastNotification>(result__)
        })
    }
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub fn CreateScheduledToastNotificationRecurring<'a, Param0: ::windows::core::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(content: Param0, deliverytime: Param1, snoozeinterval: Param2, maximumsnoozecount: u32) -> ::windows::core::Result<ScheduledToastNotification> {
        Self::IScheduledToastNotificationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), content.into_param().abi(), deliverytime.into_param().abi(), snoozeinterval.into_param().abi(), maximumsnoozecount, &mut result__).from_abi::<ScheduledToastNotification>(result__)
        })
    }
    pub fn NotificationMirroring(&self) -> ::windows::core::Result<NotificationMirroring> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe {
            let mut result__: NotificationMirroring = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NotificationMirroring>(result__)
        }
    }
    pub fn SetNotificationMirroring(&self, value: NotificationMirroring) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetRemoteId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IScheduledToastNotificationFactory<R, F: FnOnce(&IScheduledToastNotificationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ScheduledToastNotification, IScheduledToastNotificationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ScheduledToastNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ScheduledToastNotification;{79f577f8-0de7-48cd-9740-9b370490c838})");
}
unsafe impl ::windows::core::Interface for ScheduledToastNotification {
    type Vtable = IScheduledToastNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79f577f8_0de7_48cd_9740_9b370490c838);
}
impl ::windows::core::RuntimeName for ScheduledToastNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ScheduledToastNotification";
}
impl ::core::convert::From<ScheduledToastNotification> for ::windows::core::IUnknown {
    fn from(value: ScheduledToastNotification) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ScheduledToastNotification> for ::windows::core::IUnknown {
    fn from(value: &ScheduledToastNotification) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ScheduledToastNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ScheduledToastNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ScheduledToastNotification> for ::windows::core::IInspectable {
    fn from(value: ScheduledToastNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ScheduledToastNotification> for ::windows::core::IInspectable {
    fn from(value: &ScheduledToastNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ScheduledToastNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ScheduledToastNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ScheduledToastNotification {}
unsafe impl ::core::marker::Sync for ScheduledToastNotification {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ScheduledToastNotificationShowingEventArgs(pub ::windows::core::IInspectable);
impl ScheduledToastNotificationShowingEventArgs {
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ScheduledToastNotification(&self) -> ::windows::core::Result<ScheduledToastNotification> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ScheduledToastNotification>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ScheduledToastNotificationShowingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ScheduledToastNotificationShowingEventArgs;{6173f6b4-412a-5e2c-a6ed-a0209aef9a09})");
}
unsafe impl ::windows::core::Interface for ScheduledToastNotificationShowingEventArgs {
    type Vtable = IScheduledToastNotificationShowingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6173f6b4_412a_5e2c_a6ed_a0209aef9a09);
}
impl ::windows::core::RuntimeName for ScheduledToastNotificationShowingEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ScheduledToastNotificationShowingEventArgs";
}
impl ::core::convert::From<ScheduledToastNotificationShowingEventArgs> for ::windows::core::IUnknown {
    fn from(value: ScheduledToastNotificationShowingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ScheduledToastNotificationShowingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ScheduledToastNotificationShowingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ScheduledToastNotificationShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ScheduledToastNotificationShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ScheduledToastNotificationShowingEventArgs> for ::windows::core::IInspectable {
    fn from(value: ScheduledToastNotificationShowingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ScheduledToastNotificationShowingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ScheduledToastNotificationShowingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ScheduledToastNotificationShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ScheduledToastNotificationShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ScheduledToastNotificationShowingEventArgs {}
unsafe impl ::core::marker::Sync for ScheduledToastNotificationShowingEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ShownTileNotification(pub ::windows::core::IInspectable);
impl ShownTileNotification {
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ShownTileNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ShownTileNotification;{342d8988-5af2-481a-a6a3-f2fdc78de88e})");
}
unsafe impl ::windows::core::Interface for ShownTileNotification {
    type Vtable = IShownTileNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x342d8988_5af2_481a_a6a3_f2fdc78de88e);
}
impl ::windows::core::RuntimeName for ShownTileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ShownTileNotification";
}
impl ::core::convert::From<ShownTileNotification> for ::windows::core::IUnknown {
    fn from(value: ShownTileNotification) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ShownTileNotification> for ::windows::core::IUnknown {
    fn from(value: &ShownTileNotification) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ShownTileNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ShownTileNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ShownTileNotification> for ::windows::core::IInspectable {
    fn from(value: ShownTileNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ShownTileNotification> for ::windows::core::IInspectable {
    fn from(value: &ShownTileNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ShownTileNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ShownTileNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ShownTileNotification {}
unsafe impl ::core::marker::Sync for ShownTileNotification {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TileFlyoutNotification(pub ::windows::core::IInspectable);
impl TileFlyoutNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateTileFlyoutNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>>(content: Param0) -> ::windows::core::Result<TileFlyoutNotification> {
        Self::ITileFlyoutNotificationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), content.into_param().abi(), &mut result__).from_abi::<TileFlyoutNotification>(result__)
        })
    }
    pub fn ITileFlyoutNotificationFactory<R, F: FnOnce(&ITileFlyoutNotificationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TileFlyoutNotification, ITileFlyoutNotificationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TileFlyoutNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.TileFlyoutNotification;{9a53b261-c70c-42be-b2f3-f42aa97d34e5})");
}
unsafe impl ::windows::core::Interface for TileFlyoutNotification {
    type Vtable = ITileFlyoutNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a53b261_c70c_42be_b2f3_f42aa97d34e5);
}
impl ::windows::core::RuntimeName for TileFlyoutNotification {
    const NAME: &'static str = "Windows.UI.Notifications.TileFlyoutNotification";
}
impl ::core::convert::From<TileFlyoutNotification> for ::windows::core::IUnknown {
    fn from(value: TileFlyoutNotification) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TileFlyoutNotification> for ::windows::core::IUnknown {
    fn from(value: &TileFlyoutNotification) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TileFlyoutNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TileFlyoutNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TileFlyoutNotification> for ::windows::core::IInspectable {
    fn from(value: TileFlyoutNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TileFlyoutNotification> for ::windows::core::IInspectable {
    fn from(value: &TileFlyoutNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TileFlyoutNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TileFlyoutNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TileFlyoutNotification {}
unsafe impl ::core::marker::Sync for TileFlyoutNotification {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TileFlyoutTemplateType(pub i32);
impl TileFlyoutTemplateType {
    pub const TileFlyoutTemplate01: TileFlyoutTemplateType = TileFlyoutTemplateType(0i32);
}
impl ::core::convert::From<i32> for TileFlyoutTemplateType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TileFlyoutTemplateType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TileFlyoutTemplateType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.TileFlyoutTemplateType;i4)");
}
impl ::windows::core::DefaultType for TileFlyoutTemplateType {
    type DefaultType = Self;
}
pub struct TileFlyoutUpdateManager {}
impl TileFlyoutUpdateManager {
    pub fn CreateTileFlyoutUpdaterForApplication() -> ::windows::core::Result<TileFlyoutUpdater> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TileFlyoutUpdater>(result__)
        })
    }
    pub fn CreateTileFlyoutUpdaterForApplicationWithId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(applicationid: Param0) -> ::windows::core::Result<TileFlyoutUpdater> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<TileFlyoutUpdater>(result__)
        })
    }
    pub fn CreateTileFlyoutUpdaterForSecondaryTile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(tileid: Param0) -> ::windows::core::Result<TileFlyoutUpdater> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<TileFlyoutUpdater>(result__)
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(r#type: TileFlyoutTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        })
    }
    pub fn ITileFlyoutUpdateManagerStatics<R, F: FnOnce(&ITileFlyoutUpdateManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TileFlyoutUpdateManager, ITileFlyoutUpdateManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for TileFlyoutUpdateManager {
    const NAME: &'static str = "Windows.UI.Notifications.TileFlyoutUpdateManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TileFlyoutUpdater(pub ::windows::core::IInspectable);
impl TileFlyoutUpdater {
    pub fn Update<'a, Param0: ::windows::core::IntoParam<'a, TileFlyoutNotification>>(&self, notification: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), notification.into_param().abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, tileflyoutcontent: Param0, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), tileflyoutcontent.into_param().abi(), requestedinterval).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdateAtTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, tileflyoutcontent: Param0, starttime: Param1, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), tileflyoutcontent.into_param().abi(), starttime.into_param().abi(), requestedinterval).ok() }
    }
    pub fn StopPeriodicUpdate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Setting(&self) -> ::windows::core::Result<NotificationSetting> {
        let this = self;
        unsafe {
            let mut result__: NotificationSetting = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NotificationSetting>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TileFlyoutUpdater {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.TileFlyoutUpdater;{8d40c76a-c465-4052-a740-5c2654c1a089})");
}
unsafe impl ::windows::core::Interface for TileFlyoutUpdater {
    type Vtable = ITileFlyoutUpdater_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d40c76a_c465_4052_a740_5c2654c1a089);
}
impl ::windows::core::RuntimeName for TileFlyoutUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.TileFlyoutUpdater";
}
impl ::core::convert::From<TileFlyoutUpdater> for ::windows::core::IUnknown {
    fn from(value: TileFlyoutUpdater) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TileFlyoutUpdater> for ::windows::core::IUnknown {
    fn from(value: &TileFlyoutUpdater) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TileFlyoutUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TileFlyoutUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TileFlyoutUpdater> for ::windows::core::IInspectable {
    fn from(value: TileFlyoutUpdater) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TileFlyoutUpdater> for ::windows::core::IInspectable {
    fn from(value: &TileFlyoutUpdater) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TileFlyoutUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TileFlyoutUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TileNotification(pub ::windows::core::IInspectable);
impl TileNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    pub fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateTileNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>>(content: Param0) -> ::windows::core::Result<TileNotification> {
        Self::ITileNotificationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), content.into_param().abi(), &mut result__).from_abi::<TileNotification>(result__)
        })
    }
    pub fn ITileNotificationFactory<R, F: FnOnce(&ITileNotificationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TileNotification, ITileNotificationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TileNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.TileNotification;{ebaec8fa-50ec-4c18-b4d0-3af02e5540ab})");
}
unsafe impl ::windows::core::Interface for TileNotification {
    type Vtable = ITileNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebaec8fa_50ec_4c18_b4d0_3af02e5540ab);
}
impl ::windows::core::RuntimeName for TileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.TileNotification";
}
impl ::core::convert::From<TileNotification> for ::windows::core::IUnknown {
    fn from(value: TileNotification) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TileNotification> for ::windows::core::IUnknown {
    fn from(value: &TileNotification) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TileNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TileNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TileNotification> for ::windows::core::IInspectable {
    fn from(value: TileNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TileNotification> for ::windows::core::IInspectable {
    fn from(value: &TileNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TileNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TileNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TileNotification {}
unsafe impl ::core::marker::Sync for TileNotification {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for TileTemplateType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TileTemplateType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TileTemplateType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.TileTemplateType;i4)");
}
impl ::windows::core::DefaultType for TileTemplateType {
    type DefaultType = Self;
}
pub struct TileUpdateManager {}
impl TileUpdateManager {
    pub fn CreateTileUpdaterForApplication() -> ::windows::core::Result<TileUpdater> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TileUpdater>(result__)
        })
    }
    pub fn CreateTileUpdaterForApplicationWithId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(applicationid: Param0) -> ::windows::core::Result<TileUpdater> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<TileUpdater>(result__)
        })
    }
    pub fn CreateTileUpdaterForSecondaryTile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(tileid: Param0) -> ::windows::core::Result<TileUpdater> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<TileUpdater>(result__)
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(r#type: TileTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<TileUpdateManagerForUser> {
        Self::ITileUpdateManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<TileUpdateManagerForUser>(result__)
        })
    }
    pub fn ITileUpdateManagerStatics<R, F: FnOnce(&ITileUpdateManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TileUpdateManager, ITileUpdateManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITileUpdateManagerStatics2<R, F: FnOnce(&ITileUpdateManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TileUpdateManager, ITileUpdateManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for TileUpdateManager {
    const NAME: &'static str = "Windows.UI.Notifications.TileUpdateManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TileUpdateManagerForUser(pub ::windows::core::IInspectable);
impl TileUpdateManagerForUser {
    pub fn CreateTileUpdaterForApplication(&self) -> ::windows::core::Result<TileUpdater> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TileUpdater>(result__)
        }
    }
    pub fn CreateTileUpdaterForApplicationWithId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, applicationid: Param0) -> ::windows::core::Result<TileUpdater> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<TileUpdater>(result__)
        }
    }
    pub fn CreateTileUpdaterForSecondaryTile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tileid: Param0) -> ::windows::core::Result<TileUpdater> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<TileUpdater>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TileUpdateManagerForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.TileUpdateManagerForUser;{55141348-2ee2-4e2d-9cc1-216a20decc9f})");
}
unsafe impl ::windows::core::Interface for TileUpdateManagerForUser {
    type Vtable = ITileUpdateManagerForUser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55141348_2ee2_4e2d_9cc1_216a20decc9f);
}
impl ::windows::core::RuntimeName for TileUpdateManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.TileUpdateManagerForUser";
}
impl ::core::convert::From<TileUpdateManagerForUser> for ::windows::core::IUnknown {
    fn from(value: TileUpdateManagerForUser) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TileUpdateManagerForUser> for ::windows::core::IUnknown {
    fn from(value: &TileUpdateManagerForUser) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TileUpdateManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TileUpdateManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TileUpdateManagerForUser> for ::windows::core::IInspectable {
    fn from(value: TileUpdateManagerForUser) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TileUpdateManagerForUser> for ::windows::core::IInspectable {
    fn from(value: &TileUpdateManagerForUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TileUpdateManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TileUpdateManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TileUpdateManagerForUser {}
unsafe impl ::core::marker::Sync for TileUpdateManagerForUser {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TileUpdater(pub ::windows::core::IInspectable);
impl TileUpdater {
    pub fn Update<'a, Param0: ::windows::core::IntoParam<'a, TileNotification>>(&self, notification: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), notification.into_param().abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn EnableNotificationQueue(&self, enable: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), enable).ok() }
    }
    pub fn Setting(&self) -> ::windows::core::Result<NotificationSetting> {
        let this = self;
        unsafe {
            let mut result__: NotificationSetting = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NotificationSetting>(result__)
        }
    }
    pub fn AddToSchedule<'a, Param0: ::windows::core::IntoParam<'a, ScheduledTileNotification>>(&self, scheduledtile: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), scheduledtile.into_param().abi()).ok() }
    }
    pub fn RemoveFromSchedule<'a, Param0: ::windows::core::IntoParam<'a, ScheduledTileNotification>>(&self, scheduledtile: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), scheduledtile.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetScheduledTileNotifications(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ScheduledTileNotification>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ScheduledTileNotification>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, tilecontent: Param0, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), tilecontent.into_param().abi(), requestedinterval).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdateAtTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, tilecontent: Param0, starttime: Param1, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), tilecontent.into_param().abi(), starttime.into_param().abi(), requestedinterval).ok() }
    }
    pub fn StopPeriodicUpdate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn StartPeriodicUpdateBatch<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>>(&self, tilecontents: Param0, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), tilecontents.into_param().abi(), requestedinterval).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn StartPeriodicUpdateBatchAtTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, tilecontents: Param0, starttime: Param1, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), tilecontents.into_param().abi(), starttime.into_param().abi(), requestedinterval).ok() }
    }
    pub fn EnableNotificationQueueForSquare150x150(&self, enable: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITileUpdater2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), enable).ok() }
    }
    pub fn EnableNotificationQueueForWide310x150(&self, enable: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITileUpdater2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), enable).ok() }
    }
    pub fn EnableNotificationQueueForSquare310x310(&self, enable: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITileUpdater2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), enable).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for TileUpdater {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.TileUpdater;{0942a48b-1d91-44ec-9243-c1e821c29a20})");
}
unsafe impl ::windows::core::Interface for TileUpdater {
    type Vtable = ITileUpdater_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0942a48b_1d91_44ec_9243_c1e821c29a20);
}
impl ::windows::core::RuntimeName for TileUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.TileUpdater";
}
impl ::core::convert::From<TileUpdater> for ::windows::core::IUnknown {
    fn from(value: TileUpdater) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TileUpdater> for ::windows::core::IUnknown {
    fn from(value: &TileUpdater) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TileUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TileUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TileUpdater> for ::windows::core::IInspectable {
    fn from(value: TileUpdater) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TileUpdater> for ::windows::core::IInspectable {
    fn from(value: &TileUpdater) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TileUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TileUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TileUpdater {}
unsafe impl ::core::marker::Sync for TileUpdater {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToastActivatedEventArgs(pub ::windows::core::IInspectable);
impl ToastActivatedEventArgs {
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IToastActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ToastActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastActivatedEventArgs;{e3bf92f3-c197-436f-8265-0625824f8dac})");
}
unsafe impl ::windows::core::Interface for ToastActivatedEventArgs {
    type Vtable = IToastActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3bf92f3_c197_436f_8265_0625824f8dac);
}
impl ::windows::core::RuntimeName for ToastActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ToastActivatedEventArgs";
}
impl ::core::convert::From<ToastActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ToastActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToastActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ToastActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToastActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ToastActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToastActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ToastActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToastActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ToastActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToastActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ToastActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToastCollection(pub ::windows::core::IInspectable);
impl ToastCollection {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn LaunchArgs(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLaunchArgs<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Icon(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetIcon<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(collectionid: Param0, displayname: Param1, launchargs: Param2, iconuri: Param3) -> ::windows::core::Result<ToastCollection> {
        Self::IToastCollectionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), collectionid.into_param().abi(), displayname.into_param().abi(), launchargs.into_param().abi(), iconuri.into_param().abi(), &mut result__).from_abi::<ToastCollection>(result__)
        })
    }
    pub fn IToastCollectionFactory<R, F: FnOnce(&IToastCollectionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ToastCollection, IToastCollectionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ToastCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastCollection;{0a8bc3b0-e0be-4858-bc2a-89dfe0b32863})");
}
unsafe impl ::windows::core::Interface for ToastCollection {
    type Vtable = IToastCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a8bc3b0_e0be_4858_bc2a_89dfe0b32863);
}
impl ::windows::core::RuntimeName for ToastCollection {
    const NAME: &'static str = "Windows.UI.Notifications.ToastCollection";
}
impl ::core::convert::From<ToastCollection> for ::windows::core::IUnknown {
    fn from(value: ToastCollection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToastCollection> for ::windows::core::IUnknown {
    fn from(value: &ToastCollection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToastCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ToastCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToastCollection> for ::windows::core::IInspectable {
    fn from(value: ToastCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToastCollection> for ::windows::core::IInspectable {
    fn from(value: &ToastCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToastCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ToastCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ToastCollection {}
unsafe impl ::core::marker::Sync for ToastCollection {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToastCollectionManager(pub ::windows::core::IInspectable);
impl ToastCollectionManager {
    #[cfg(feature = "Foundation")]
    pub fn SaveToastCollectionAsync<'a, Param0: ::windows::core::IntoParam<'a, ToastCollection>>(&self, collection: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), collection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllToastCollectionsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ToastCollection>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ToastCollection>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetToastCollectionAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, collectionid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ToastCollection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), collectionid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ToastCollection>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveToastCollectionAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, collectionid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), collectionid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAllToastCollectionsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ToastCollectionManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastCollectionManager;{2a1821fe-179d-49bc-b79d-a527920d3665})");
}
unsafe impl ::windows::core::Interface for ToastCollectionManager {
    type Vtable = IToastCollectionManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a1821fe_179d_49bc_b79d_a527920d3665);
}
impl ::windows::core::RuntimeName for ToastCollectionManager {
    const NAME: &'static str = "Windows.UI.Notifications.ToastCollectionManager";
}
impl ::core::convert::From<ToastCollectionManager> for ::windows::core::IUnknown {
    fn from(value: ToastCollectionManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToastCollectionManager> for ::windows::core::IUnknown {
    fn from(value: &ToastCollectionManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToastCollectionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ToastCollectionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToastCollectionManager> for ::windows::core::IInspectable {
    fn from(value: ToastCollectionManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToastCollectionManager> for ::windows::core::IInspectable {
    fn from(value: &ToastCollectionManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToastCollectionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ToastCollectionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ToastCollectionManager {}
unsafe impl ::core::marker::Sync for ToastCollectionManager {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ToastDismissalReason(pub i32);
impl ToastDismissalReason {
    pub const UserCanceled: ToastDismissalReason = ToastDismissalReason(0i32);
    pub const ApplicationHidden: ToastDismissalReason = ToastDismissalReason(1i32);
    pub const TimedOut: ToastDismissalReason = ToastDismissalReason(2i32);
}
impl ::core::convert::From<i32> for ToastDismissalReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ToastDismissalReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ToastDismissalReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastDismissalReason;i4)");
}
impl ::windows::core::DefaultType for ToastDismissalReason {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToastDismissedEventArgs(pub ::windows::core::IInspectable);
impl ToastDismissedEventArgs {
    pub fn Reason(&self) -> ::windows::core::Result<ToastDismissalReason> {
        let this = self;
        unsafe {
            let mut result__: ToastDismissalReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ToastDismissalReason>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ToastDismissedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastDismissedEventArgs;{3f89d935-d9cb-4538-a0f0-ffe7659938f8})");
}
unsafe impl ::windows::core::Interface for ToastDismissedEventArgs {
    type Vtable = IToastDismissedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f89d935_d9cb_4538_a0f0_ffe7659938f8);
}
impl ::windows::core::RuntimeName for ToastDismissedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ToastDismissedEventArgs";
}
impl ::core::convert::From<ToastDismissedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ToastDismissedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToastDismissedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ToastDismissedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToastDismissedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ToastDismissedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToastDismissedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ToastDismissedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToastDismissedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ToastDismissedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToastDismissedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ToastDismissedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ToastDismissedEventArgs {}
unsafe impl ::core::marker::Sync for ToastDismissedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToastFailedEventArgs(pub ::windows::core::IInspectable);
impl ToastFailedEventArgs {
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ToastFailedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastFailedEventArgs;{35176862-cfd4-44f8-ad64-f500fd896c3b})");
}
unsafe impl ::windows::core::Interface for ToastFailedEventArgs {
    type Vtable = IToastFailedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35176862_cfd4_44f8_ad64_f500fd896c3b);
}
impl ::windows::core::RuntimeName for ToastFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ToastFailedEventArgs";
}
impl ::core::convert::From<ToastFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ToastFailedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToastFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ToastFailedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToastFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ToastFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToastFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ToastFailedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToastFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ToastFailedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToastFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ToastFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ToastFailedEventArgs {}
unsafe impl ::core::marker::Sync for ToastFailedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ToastHistoryChangedType(pub i32);
impl ToastHistoryChangedType {
    pub const Cleared: ToastHistoryChangedType = ToastHistoryChangedType(0i32);
    pub const Removed: ToastHistoryChangedType = ToastHistoryChangedType(1i32);
    pub const Expired: ToastHistoryChangedType = ToastHistoryChangedType(2i32);
    pub const Added: ToastHistoryChangedType = ToastHistoryChangedType(3i32);
}
impl ::core::convert::From<i32> for ToastHistoryChangedType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ToastHistoryChangedType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ToastHistoryChangedType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastHistoryChangedType;i4)");
}
impl ::windows::core::DefaultType for ToastHistoryChangedType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToastNotification(pub ::windows::core::IInspectable);
impl ToastNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Dismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ToastNotification, ToastDismissedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Activated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ToastNotification, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Failed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ToastNotification, ToastFailedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotification2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetGroup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotification2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSuppressPopup(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotification2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SuppressPopup(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateToastNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>>(content: Param0) -> ::windows::core::Result<ToastNotification> {
        Self::IToastNotificationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), content.into_param().abi(), &mut result__).from_abi::<ToastNotification>(result__)
        })
    }
    pub fn NotificationMirroring(&self) -> ::windows::core::Result<NotificationMirroring> {
        let this = &::windows::core::Interface::cast::<IToastNotification3>(self)?;
        unsafe {
            let mut result__: NotificationMirroring = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NotificationMirroring>(result__)
        }
    }
    pub fn SetNotificationMirroring(&self, value: NotificationMirroring) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotification3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IToastNotification3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetRemoteId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotification3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Data(&self) -> ::windows::core::Result<NotificationData> {
        let this = &::windows::core::Interface::cast::<IToastNotification4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NotificationData>(result__)
        }
    }
    pub fn SetData<'a, Param0: ::windows::core::IntoParam<'a, NotificationData>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotification4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Priority(&self) -> ::windows::core::Result<ToastNotificationPriority> {
        let this = &::windows::core::Interface::cast::<IToastNotification4>(self)?;
        unsafe {
            let mut result__: ToastNotificationPriority = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ToastNotificationPriority>(result__)
        }
    }
    pub fn SetPriority(&self, value: ToastNotificationPriority) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotification4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ExpiresOnReboot(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IToastNotification6>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetExpiresOnReboot(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotification6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IToastNotificationFactory<R, F: FnOnce(&IToastNotificationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ToastNotification, IToastNotificationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotification;{997e2675-059e-4e60-8b06-1760917c8b80})");
}
unsafe impl ::windows::core::Interface for ToastNotification {
    type Vtable = IToastNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x997e2675_059e_4e60_8b06_1760917c8b80);
}
impl ::windows::core::RuntimeName for ToastNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotification";
}
impl ::core::convert::From<ToastNotification> for ::windows::core::IUnknown {
    fn from(value: ToastNotification) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToastNotification> for ::windows::core::IUnknown {
    fn from(value: &ToastNotification) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToastNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ToastNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToastNotification> for ::windows::core::IInspectable {
    fn from(value: ToastNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToastNotification> for ::windows::core::IInspectable {
    fn from(value: &ToastNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToastNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ToastNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ToastNotification {}
unsafe impl ::core::marker::Sync for ToastNotification {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToastNotificationActionTriggerDetail(pub ::windows::core::IInspectable);
impl ToastNotificationActionTriggerDetail {
    pub fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotificationActionTriggerDetail {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotificationActionTriggerDetail;{9445135a-38f3-42f6-96aa-7955b0f03da2})");
}
unsafe impl ::windows::core::Interface for ToastNotificationActionTriggerDetail {
    type Vtable = IToastNotificationActionTriggerDetail_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9445135a_38f3_42f6_96aa_7955b0f03da2);
}
impl ::windows::core::RuntimeName for ToastNotificationActionTriggerDetail {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationActionTriggerDetail";
}
impl ::core::convert::From<ToastNotificationActionTriggerDetail> for ::windows::core::IUnknown {
    fn from(value: ToastNotificationActionTriggerDetail) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToastNotificationActionTriggerDetail> for ::windows::core::IUnknown {
    fn from(value: &ToastNotificationActionTriggerDetail) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToastNotificationActionTriggerDetail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ToastNotificationActionTriggerDetail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToastNotificationActionTriggerDetail> for ::windows::core::IInspectable {
    fn from(value: ToastNotificationActionTriggerDetail) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToastNotificationActionTriggerDetail> for ::windows::core::IInspectable {
    fn from(value: &ToastNotificationActionTriggerDetail) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToastNotificationActionTriggerDetail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ToastNotificationActionTriggerDetail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToastNotificationHistory(pub ::windows::core::IInspectable);
impl ToastNotificationHistory {
    pub fn RemoveGroup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, group: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), group.into_param().abi()).ok() }
    }
    pub fn RemoveGroupWithId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, group: Param0, applicationid: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), group.into_param().abi(), applicationid.into_param().abi()).ok() }
    }
    pub fn RemoveGroupedTagWithId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tag: Param0, group: Param1, applicationid: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), tag.into_param().abi(), group.into_param().abi(), applicationid.into_param().abi()).ok() }
    }
    pub fn RemoveGroupedTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tag: Param0, group: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), tag.into_param().abi(), group.into_param().abi()).ok() }
    }
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tag: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), tag.into_param().abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn ClearWithId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, applicationid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), applicationid.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetHistory(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ToastNotification>> {
        let this = &::windows::core::Interface::cast::<IToastNotificationHistory2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ToastNotification>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetHistoryWithId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, applicationid: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ToastNotification>> {
        let this = &::windows::core::Interface::cast::<IToastNotificationHistory2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ToastNotification>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotificationHistory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotificationHistory;{5caddc63-01d3-4c97-986f-0533483fee14})");
}
unsafe impl ::windows::core::Interface for ToastNotificationHistory {
    type Vtable = IToastNotificationHistory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5caddc63_01d3_4c97_986f_0533483fee14);
}
impl ::windows::core::RuntimeName for ToastNotificationHistory {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationHistory";
}
impl ::core::convert::From<ToastNotificationHistory> for ::windows::core::IUnknown {
    fn from(value: ToastNotificationHistory) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToastNotificationHistory> for ::windows::core::IUnknown {
    fn from(value: &ToastNotificationHistory) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToastNotificationHistory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ToastNotificationHistory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToastNotificationHistory> for ::windows::core::IInspectable {
    fn from(value: ToastNotificationHistory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToastNotificationHistory> for ::windows::core::IInspectable {
    fn from(value: &ToastNotificationHistory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToastNotificationHistory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ToastNotificationHistory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToastNotificationHistoryChangedTriggerDetail(pub ::windows::core::IInspectable);
impl ToastNotificationHistoryChangedTriggerDetail {
    pub fn ChangeType(&self) -> ::windows::core::Result<ToastHistoryChangedType> {
        let this = self;
        unsafe {
            let mut result__: ToastHistoryChangedType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ToastHistoryChangedType>(result__)
        }
    }
    pub fn CollectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IToastNotificationHistoryChangedTriggerDetail2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotificationHistoryChangedTriggerDetail {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail;{db037ffa-0068-412c-9c83-267c37f65670})");
}
unsafe impl ::windows::core::Interface for ToastNotificationHistoryChangedTriggerDetail {
    type Vtable = IToastNotificationHistoryChangedTriggerDetail_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb037ffa_0068_412c_9c83_267c37f65670);
}
impl ::windows::core::RuntimeName for ToastNotificationHistoryChangedTriggerDetail {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail";
}
impl ::core::convert::From<ToastNotificationHistoryChangedTriggerDetail> for ::windows::core::IUnknown {
    fn from(value: ToastNotificationHistoryChangedTriggerDetail) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToastNotificationHistoryChangedTriggerDetail> for ::windows::core::IUnknown {
    fn from(value: &ToastNotificationHistoryChangedTriggerDetail) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToastNotificationHistoryChangedTriggerDetail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ToastNotificationHistoryChangedTriggerDetail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToastNotificationHistoryChangedTriggerDetail> for ::windows::core::IInspectable {
    fn from(value: ToastNotificationHistoryChangedTriggerDetail) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToastNotificationHistoryChangedTriggerDetail> for ::windows::core::IInspectable {
    fn from(value: &ToastNotificationHistoryChangedTriggerDetail) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToastNotificationHistoryChangedTriggerDetail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ToastNotificationHistoryChangedTriggerDetail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
pub struct ToastNotificationManager {}
impl ToastNotificationManager {
    pub fn CreateToastNotifier() -> ::windows::core::Result<ToastNotifier> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ToastNotifier>(result__)
        })
    }
    pub fn CreateToastNotifierWithId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(applicationid: Param0) -> ::windows::core::Result<ToastNotifier> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<ToastNotifier>(result__)
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(r#type: ToastTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        })
    }
    pub fn History() -> ::windows::core::Result<ToastNotificationHistory> {
        Self::IToastNotificationManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ToastNotificationHistory>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<ToastNotificationManagerForUser> {
        Self::IToastNotificationManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<ToastNotificationManagerForUser>(result__)
        })
    }
    pub fn ConfigureNotificationMirroring(value: NotificationMirroring) -> ::windows::core::Result<()> {
        Self::IToastNotificationManagerStatics4(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() })
    }
    pub fn GetDefault() -> ::windows::core::Result<ToastNotificationManagerForUser> {
        Self::IToastNotificationManagerStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ToastNotificationManagerForUser>(result__)
        })
    }
    pub fn IToastNotificationManagerStatics<R, F: FnOnce(&IToastNotificationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ToastNotificationManager, IToastNotificationManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IToastNotificationManagerStatics2<R, F: FnOnce(&IToastNotificationManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ToastNotificationManager, IToastNotificationManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IToastNotificationManagerStatics4<R, F: FnOnce(&IToastNotificationManagerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ToastNotificationManager, IToastNotificationManagerStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IToastNotificationManagerStatics5<R, F: FnOnce(&IToastNotificationManagerStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ToastNotificationManager, IToastNotificationManagerStatics5> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ToastNotificationManager {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToastNotificationManagerForUser(pub ::windows::core::IInspectable);
impl ToastNotificationManagerForUser {
    pub fn CreateToastNotifier(&self) -> ::windows::core::Result<ToastNotifier> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ToastNotifier>(result__)
        }
    }
    pub fn CreateToastNotifierWithId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, applicationid: Param0) -> ::windows::core::Result<ToastNotifier> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<ToastNotifier>(result__)
        }
    }
    pub fn History(&self) -> ::windows::core::Result<ToastNotificationHistory> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ToastNotificationHistory>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetToastNotifierForToastCollectionIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, collectionid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ToastNotifier>> {
        let this = &::windows::core::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), collectionid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ToastNotifier>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetHistoryForToastCollectionIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, collectionid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ToastNotificationHistory>> {
        let this = &::windows::core::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), collectionid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ToastNotificationHistory>>(result__)
        }
    }
    pub fn GetToastCollectionManager(&self) -> ::windows::core::Result<ToastCollectionManager> {
        let this = &::windows::core::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ToastCollectionManager>(result__)
        }
    }
    pub fn GetToastCollectionManagerWithAppId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, appid: Param0) -> ::windows::core::Result<ToastCollectionManager> {
        let this = &::windows::core::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), appid.into_param().abi(), &mut result__).from_abi::<ToastCollectionManager>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotificationManagerForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotificationManagerForUser;{79ab57f6-43fe-487b-8a7f-99567200ae94})");
}
unsafe impl ::windows::core::Interface for ToastNotificationManagerForUser {
    type Vtable = IToastNotificationManagerForUser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79ab57f6_43fe_487b_8a7f_99567200ae94);
}
impl ::windows::core::RuntimeName for ToastNotificationManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationManagerForUser";
}
impl ::core::convert::From<ToastNotificationManagerForUser> for ::windows::core::IUnknown {
    fn from(value: ToastNotificationManagerForUser) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToastNotificationManagerForUser> for ::windows::core::IUnknown {
    fn from(value: &ToastNotificationManagerForUser) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToastNotificationManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ToastNotificationManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToastNotificationManagerForUser> for ::windows::core::IInspectable {
    fn from(value: ToastNotificationManagerForUser) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToastNotificationManagerForUser> for ::windows::core::IInspectable {
    fn from(value: &ToastNotificationManagerForUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToastNotificationManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ToastNotificationManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ToastNotificationManagerForUser {}
unsafe impl ::core::marker::Sync for ToastNotificationManagerForUser {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ToastNotificationPriority(pub i32);
impl ToastNotificationPriority {
    pub const Default: ToastNotificationPriority = ToastNotificationPriority(0i32);
    pub const High: ToastNotificationPriority = ToastNotificationPriority(1i32);
}
impl ::core::convert::From<i32> for ToastNotificationPriority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ToastNotificationPriority {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ToastNotificationPriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastNotificationPriority;i4)");
}
impl ::windows::core::DefaultType for ToastNotificationPriority {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToastNotifier(pub ::windows::core::IInspectable);
impl ToastNotifier {
    pub fn Show<'a, Param0: ::windows::core::IntoParam<'a, ToastNotification>>(&self, notification: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), notification.into_param().abi()).ok() }
    }
    pub fn Hide<'a, Param0: ::windows::core::IntoParam<'a, ToastNotification>>(&self, notification: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), notification.into_param().abi()).ok() }
    }
    pub fn Setting(&self) -> ::windows::core::Result<NotificationSetting> {
        let this = self;
        unsafe {
            let mut result__: NotificationSetting = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NotificationSetting>(result__)
        }
    }
    pub fn AddToSchedule<'a, Param0: ::windows::core::IntoParam<'a, ScheduledToastNotification>>(&self, scheduledtoast: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), scheduledtoast.into_param().abi()).ok() }
    }
    pub fn RemoveFromSchedule<'a, Param0: ::windows::core::IntoParam<'a, ScheduledToastNotification>>(&self, scheduledtoast: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), scheduledtoast.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetScheduledToastNotifications(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ScheduledToastNotification>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ScheduledToastNotification>>(result__)
        }
    }
    pub fn UpdateWithTagAndGroup<'a, Param0: ::windows::core::IntoParam<'a, NotificationData>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0, tag: Param1, group: Param2) -> ::windows::core::Result<NotificationUpdateResult> {
        let this = &::windows::core::Interface::cast::<IToastNotifier2>(self)?;
        unsafe {
            let mut result__: NotificationUpdateResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), data.into_param().abi(), tag.into_param().abi(), group.into_param().abi(), &mut result__).from_abi::<NotificationUpdateResult>(result__)
        }
    }
    pub fn UpdateWithTag<'a, Param0: ::windows::core::IntoParam<'a, NotificationData>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0, tag: Param1) -> ::windows::core::Result<NotificationUpdateResult> {
        let this = &::windows::core::Interface::cast::<IToastNotifier2>(self)?;
        unsafe {
            let mut result__: NotificationUpdateResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), data.into_param().abi(), tag.into_param().abi(), &mut result__).from_abi::<NotificationUpdateResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ScheduledToastNotificationShowing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ToastNotifier, ScheduledToastNotificationShowingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IToastNotifier3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveScheduledToastNotificationShowing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotifier3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotifier;{75927b93-03f3-41ec-91d3-6e5bac1b38e7})");
}
unsafe impl ::windows::core::Interface for ToastNotifier {
    type Vtable = IToastNotifier_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75927b93_03f3_41ec_91d3_6e5bac1b38e7);
}
impl ::windows::core::RuntimeName for ToastNotifier {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotifier";
}
impl ::core::convert::From<ToastNotifier> for ::windows::core::IUnknown {
    fn from(value: ToastNotifier) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToastNotifier> for ::windows::core::IUnknown {
    fn from(value: &ToastNotifier) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToastNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ToastNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToastNotifier> for ::windows::core::IInspectable {
    fn from(value: ToastNotifier) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToastNotifier> for ::windows::core::IInspectable {
    fn from(value: &ToastNotifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToastNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ToastNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ToastNotifier {}
unsafe impl ::core::marker::Sync for ToastNotifier {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for ToastTemplateType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ToastTemplateType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ToastTemplateType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastTemplateType;i4)");
}
impl ::windows::core::DefaultType for ToastTemplateType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserNotification(pub ::windows::core::IInspectable);
impl UserNotification {
    pub fn Notification(&self) -> ::windows::core::Result<Notification> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Notification>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn AppInfo(&self) -> ::windows::core::Result<super::super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::AppInfo>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreationTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for UserNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.UserNotification;{adf7e52f-4e53-42d5-9c33-eb5ea515b23e})");
}
unsafe impl ::windows::core::Interface for UserNotification {
    type Vtable = IUserNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadf7e52f_4e53_42d5_9c33_eb5ea515b23e);
}
impl ::windows::core::RuntimeName for UserNotification {
    const NAME: &'static str = "Windows.UI.Notifications.UserNotification";
}
impl ::core::convert::From<UserNotification> for ::windows::core::IUnknown {
    fn from(value: UserNotification) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserNotification> for ::windows::core::IUnknown {
    fn from(value: &UserNotification) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserNotification> for ::windows::core::IInspectable {
    fn from(value: UserNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserNotification> for ::windows::core::IInspectable {
    fn from(value: &UserNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserNotification {}
unsafe impl ::core::marker::Sync for UserNotification {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserNotificationChangedEventArgs(pub ::windows::core::IInspectable);
impl UserNotificationChangedEventArgs {
    pub fn ChangeKind(&self) -> ::windows::core::Result<UserNotificationChangedKind> {
        let this = self;
        unsafe {
            let mut result__: UserNotificationChangedKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserNotificationChangedKind>(result__)
        }
    }
    pub fn UserNotificationId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for UserNotificationChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.UserNotificationChangedEventArgs;{b6bd6839-79cf-4b25-82c0-0ce1eef81f8c})");
}
unsafe impl ::windows::core::Interface for UserNotificationChangedEventArgs {
    type Vtable = IUserNotificationChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6bd6839_79cf_4b25_82c0_0ce1eef81f8c);
}
impl ::windows::core::RuntimeName for UserNotificationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.UserNotificationChangedEventArgs";
}
impl ::core::convert::From<UserNotificationChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: UserNotificationChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserNotificationChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UserNotificationChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserNotificationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserNotificationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserNotificationChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: UserNotificationChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserNotificationChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UserNotificationChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserNotificationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserNotificationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserNotificationChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserNotificationChangedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserNotificationChangedKind(pub i32);
impl UserNotificationChangedKind {
    pub const Added: UserNotificationChangedKind = UserNotificationChangedKind(0i32);
    pub const Removed: UserNotificationChangedKind = UserNotificationChangedKind(1i32);
}
impl ::core::convert::From<i32> for UserNotificationChangedKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserNotificationChangedKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserNotificationChangedKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.UserNotificationChangedKind;i4)");
}
impl ::windows::core::DefaultType for UserNotificationChangedKind {
    type DefaultType = Self;
}
