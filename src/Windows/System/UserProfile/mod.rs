#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `System_UserProfile`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AccountPictureKind(pub i32);
impl AccountPictureKind {
    pub const SmallImage: AccountPictureKind = AccountPictureKind(0i32);
    pub const LargeImage: AccountPictureKind = AccountPictureKind(1i32);
    pub const Video: AccountPictureKind = AccountPictureKind(2i32);
}
impl ::std::convert::From<i32> for AccountPictureKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AccountPictureKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AccountPictureKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.UserProfile.AccountPictureKind;i4)");
}
#[doc = "*Required features: `System_UserProfile`*"]
pub struct AdvertisingManager {}
impl AdvertisingManager {
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn AdvertisingId() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IAdvertisingManagerStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::User>>(user: Param0) -> ::windows::runtime::Result<AdvertisingManagerForUser> {
        Self::IAdvertisingManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<AdvertisingManagerForUser>(result__)
        })
    }
    pub fn IAdvertisingManagerStatics<R, F: FnOnce(&IAdvertisingManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AdvertisingManager, IAdvertisingManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAdvertisingManagerStatics2<R, F: FnOnce(&IAdvertisingManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AdvertisingManager, IAdvertisingManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for AdvertisingManager {
    const NAME: &'static str = "Windows.System.UserProfile.AdvertisingManager";
}
#[doc = "*Required features: `System_UserProfile`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AdvertisingManagerForUser(::windows::runtime::IInspectable);
impl AdvertisingManagerForUser {
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn AdvertisingId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdvertisingManagerForUser {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.AdvertisingManagerForUser;{928bf3d0-cf7c-4ab0-a7dc-6dc5bcd44252})");
}
unsafe impl ::windows::runtime::Interface for AdvertisingManagerForUser {
    type Vtable = IAdvertisingManagerForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2458645456, 53116, 19120, [167, 220, 109, 197, 188, 212, 66, 82]);
}
impl ::windows::runtime::RuntimeName for AdvertisingManagerForUser {
    const NAME: &'static str = "Windows.System.UserProfile.AdvertisingManagerForUser";
}
impl ::std::convert::From<AdvertisingManagerForUser> for ::windows::runtime::IUnknown {
    fn from(value: AdvertisingManagerForUser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AdvertisingManagerForUser> for ::windows::runtime::IUnknown {
    fn from(value: &AdvertisingManagerForUser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AdvertisingManagerForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AdvertisingManagerForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AdvertisingManagerForUser> for ::windows::runtime::IInspectable {
    fn from(value: AdvertisingManagerForUser) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AdvertisingManagerForUser> for ::windows::runtime::IInspectable {
    fn from(value: &AdvertisingManagerForUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AdvertisingManagerForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AdvertisingManagerForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AdvertisingManagerForUser {}
unsafe impl ::std::marker::Sync for AdvertisingManagerForUser {}
#[doc = "*Required features: `System_UserProfile`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AssignedAccessSettings(::windows::runtime::IInspectable);
impl AssignedAccessSettings {
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn IsSingleAppKioskMode(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::User>(result__)
        }
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<AssignedAccessSettings> {
        Self::IAssignedAccessSettingsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AssignedAccessSettings>(result__)
        })
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::User>>(user: Param0) -> ::windows::runtime::Result<AssignedAccessSettings> {
        Self::IAssignedAccessSettingsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<AssignedAccessSettings>(result__)
        })
    }
    pub fn IAssignedAccessSettingsStatics<R, F: FnOnce(&IAssignedAccessSettingsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AssignedAccessSettings, IAssignedAccessSettingsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AssignedAccessSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.AssignedAccessSettings;{1bc57f1c-e971-5757-b8e0-512f8b8c46d2})");
}
unsafe impl ::windows::runtime::Interface for AssignedAccessSettings {
    type Vtable = IAssignedAccessSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(465927964, 59761, 22359, [184, 224, 81, 47, 139, 140, 70, 210]);
}
impl ::windows::runtime::RuntimeName for AssignedAccessSettings {
    const NAME: &'static str = "Windows.System.UserProfile.AssignedAccessSettings";
}
impl ::std::convert::From<AssignedAccessSettings> for ::windows::runtime::IUnknown {
    fn from(value: AssignedAccessSettings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AssignedAccessSettings> for ::windows::runtime::IUnknown {
    fn from(value: &AssignedAccessSettings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AssignedAccessSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AssignedAccessSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AssignedAccessSettings> for ::windows::runtime::IInspectable {
    fn from(value: AssignedAccessSettings) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AssignedAccessSettings> for ::windows::runtime::IInspectable {
    fn from(value: &AssignedAccessSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AssignedAccessSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AssignedAccessSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AssignedAccessSettings {}
unsafe impl ::std::marker::Sync for AssignedAccessSettings {}
#[doc = "*Required features: `System_UserProfile`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DiagnosticsSettings(::windows::runtime::IInspectable);
impl DiagnosticsSettings {
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn CanUseDiagnosticsToTailorExperiences(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::User>(result__)
        }
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<DiagnosticsSettings> {
        Self::IDiagnosticsSettingsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DiagnosticsSettings>(result__)
        })
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::User>>(user: Param0) -> ::windows::runtime::Result<DiagnosticsSettings> {
        Self::IDiagnosticsSettingsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<DiagnosticsSettings>(result__)
        })
    }
    pub fn IDiagnosticsSettingsStatics<R, F: FnOnce(&IDiagnosticsSettingsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DiagnosticsSettings, IDiagnosticsSettingsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DiagnosticsSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.DiagnosticsSettings;{e5e9eccd-2711-44e0-973c-491d78048d24})");
}
unsafe impl ::windows::runtime::Interface for DiagnosticsSettings {
    type Vtable = IDiagnosticsSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3857312973, 10001, 17632, [151, 60, 73, 29, 120, 4, 141, 36]);
}
impl ::windows::runtime::RuntimeName for DiagnosticsSettings {
    const NAME: &'static str = "Windows.System.UserProfile.DiagnosticsSettings";
}
impl ::std::convert::From<DiagnosticsSettings> for ::windows::runtime::IUnknown {
    fn from(value: DiagnosticsSettings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DiagnosticsSettings> for ::windows::runtime::IUnknown {
    fn from(value: &DiagnosticsSettings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DiagnosticsSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DiagnosticsSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DiagnosticsSettings> for ::windows::runtime::IInspectable {
    fn from(value: DiagnosticsSettings) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DiagnosticsSettings> for ::windows::runtime::IInspectable {
    fn from(value: &DiagnosticsSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DiagnosticsSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DiagnosticsSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DiagnosticsSettings {}
unsafe impl ::std::marker::Sync for DiagnosticsSettings {}
#[doc = "*Required features: `System_UserProfile`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct FirstSignInSettings(::windows::runtime::IInspectable);
impl FirstSignInSettings {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation_Collections`*"]
    pub fn Split(&self, first: &mut ::std::option::Option<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>, second: &mut ::std::option::Option<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<FirstSignInSettings> {
        Self::IFirstSignInSettingsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FirstSignInSettings>(result__)
        })
    }
    pub fn IFirstSignInSettingsStatics<R, F: FnOnce(&IFirstSignInSettingsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FirstSignInSettings, IFirstSignInSettingsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FirstSignInSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.FirstSignInSettings;{3e945153-3a5e-452e-a601-f5baad2a4870})");
}
unsafe impl ::windows::runtime::Interface for FirstSignInSettings {
    type Vtable = IFirstSignInSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1049907539, 14942, 17710, [166, 1, 245, 186, 173, 42, 72, 112]);
}
impl ::windows::runtime::RuntimeName for FirstSignInSettings {
    const NAME: &'static str = "Windows.System.UserProfile.FirstSignInSettings";
}
impl ::std::convert::From<FirstSignInSettings> for ::windows::runtime::IUnknown {
    fn from(value: FirstSignInSettings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FirstSignInSettings> for ::windows::runtime::IUnknown {
    fn from(value: &FirstSignInSettings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FirstSignInSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &FirstSignInSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<FirstSignInSettings> for ::windows::runtime::IInspectable {
    fn from(value: FirstSignInSettings) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FirstSignInSettings> for ::windows::runtime::IInspectable {
    fn from(value: &FirstSignInSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FirstSignInSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FirstSignInSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<FirstSignInSettings> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FirstSignInSettings) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&FirstSignInSettings> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FirstSignInSettings) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> for FirstSignInSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> for &FirstSignInSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        ::std::convert::TryInto::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<FirstSignInSettings> for super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FirstSignInSettings) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&FirstSignInSettings> for super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FirstSignInSettings) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for FirstSignInSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for &FirstSignInSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::std::convert::TryInto::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for FirstSignInSettings {}
unsafe impl ::std::marker::Sync for FirstSignInSettings {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for FirstSignInSettings {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &FirstSignInSettings {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `System_UserProfile`*"]
pub struct GlobalizationPreferences {}
impl GlobalizationPreferences {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation_Collections`*"]
    pub fn Calendars() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation_Collections`*"]
    pub fn Clocks() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation_Collections`*"]
    pub fn Currencies() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation_Collections`*"]
    pub fn Languages() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn HomeGeographicRegion() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Globalization")]
    #[doc = "*Required features: `System_UserProfile`, `Globalization`*"]
    pub fn WeekStartsOn() -> ::windows::runtime::Result<super::super::Globalization::DayOfWeek> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__: super::super::Globalization::DayOfWeek = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Globalization::DayOfWeek>(result__)
        })
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn TrySetHomeGeographicRegion<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(region: Param0) -> ::windows::runtime::Result<bool> {
        Self::IGlobalizationPreferencesStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), region.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation_Collections`*"]
    pub fn TrySetLanguages<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(languagetags: Param0) -> ::windows::runtime::Result<bool> {
        Self::IGlobalizationPreferencesStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), languagetags.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::User>>(user: Param0) -> ::windows::runtime::Result<GlobalizationPreferencesForUser> {
        Self::IGlobalizationPreferencesStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<GlobalizationPreferencesForUser>(result__)
        })
    }
    pub fn IGlobalizationPreferencesStatics<R, F: FnOnce(&IGlobalizationPreferencesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GlobalizationPreferences, IGlobalizationPreferencesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGlobalizationPreferencesStatics2<R, F: FnOnce(&IGlobalizationPreferencesStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GlobalizationPreferences, IGlobalizationPreferencesStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGlobalizationPreferencesStatics3<R, F: FnOnce(&IGlobalizationPreferencesStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GlobalizationPreferences, IGlobalizationPreferencesStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for GlobalizationPreferences {
    const NAME: &'static str = "Windows.System.UserProfile.GlobalizationPreferences";
}
#[doc = "*Required features: `System_UserProfile`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GlobalizationPreferencesForUser(::windows::runtime::IInspectable);
impl GlobalizationPreferencesForUser {
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::User>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation_Collections`*"]
    pub fn Calendars(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation_Collections`*"]
    pub fn Clocks(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation_Collections`*"]
    pub fn Currencies(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation_Collections`*"]
    pub fn Languages(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn HomeGeographicRegion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Globalization")]
    #[doc = "*Required features: `System_UserProfile`, `Globalization`*"]
    pub fn WeekStartsOn(&self) -> ::windows::runtime::Result<super::super::Globalization::DayOfWeek> {
        let this = self;
        unsafe {
            let mut result__: super::super::Globalization::DayOfWeek = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Globalization::DayOfWeek>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GlobalizationPreferencesForUser {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.GlobalizationPreferencesForUser;{150f0795-4f6e-40ba-a010-e27d81bda7f5})");
}
unsafe impl ::windows::runtime::Interface for GlobalizationPreferencesForUser {
    type Vtable = IGlobalizationPreferencesForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(353306517, 20334, 16570, [160, 16, 226, 125, 129, 189, 167, 245]);
}
impl ::windows::runtime::RuntimeName for GlobalizationPreferencesForUser {
    const NAME: &'static str = "Windows.System.UserProfile.GlobalizationPreferencesForUser";
}
impl ::std::convert::From<GlobalizationPreferencesForUser> for ::windows::runtime::IUnknown {
    fn from(value: GlobalizationPreferencesForUser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GlobalizationPreferencesForUser> for ::windows::runtime::IUnknown {
    fn from(value: &GlobalizationPreferencesForUser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GlobalizationPreferencesForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GlobalizationPreferencesForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GlobalizationPreferencesForUser> for ::windows::runtime::IInspectable {
    fn from(value: GlobalizationPreferencesForUser) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GlobalizationPreferencesForUser> for ::windows::runtime::IInspectable {
    fn from(value: &GlobalizationPreferencesForUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GlobalizationPreferencesForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GlobalizationPreferencesForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GlobalizationPreferencesForUser {}
unsafe impl ::std::marker::Sync for GlobalizationPreferencesForUser {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAdvertisingManagerForUser(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdvertisingManagerForUser {
    type Vtable = IAdvertisingManagerForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2458645456, 53116, 19120, [167, 220, 109, 197, 188, 212, 66, 82]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvertisingManagerForUser_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAdvertisingManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdvertisingManagerStatics {
    type Vtable = IAdvertisingManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2916304524, 41587, 18635, [179, 70, 53, 68, 82, 45, 85, 129]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvertisingManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAdvertisingManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdvertisingManagerStatics2 {
    type Vtable = IAdvertisingManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3708372911, 6765, 18096, [149, 188, 243, 249, 214, 190, 185, 251]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvertisingManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAssignedAccessSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAssignedAccessSettings {
    type Vtable = IAssignedAccessSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(465927964, 59761, 22359, [184, 224, 81, 47, 139, 140, 70, 210]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssignedAccessSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAssignedAccessSettingsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAssignedAccessSettingsStatics {
    type Vtable = IAssignedAccessSettingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(883432717, 35369, 24307, [167, 190, 97, 142, 106, 195, 189, 1]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssignedAccessSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IDiagnosticsSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDiagnosticsSettings {
    type Vtable = IDiagnosticsSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3857312973, 10001, 17632, [151, 60, 73, 29, 120, 4, 141, 36]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticsSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IDiagnosticsSettingsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDiagnosticsSettingsStatics {
    type Vtable = IDiagnosticsSettingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1926424591, 21392, 18323, [153, 11, 60, 204, 125, 106, 201, 200]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticsSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IFirstSignInSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFirstSignInSettings {
    type Vtable = IFirstSignInSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1049907539, 14942, 17710, [166, 1, 245, 186, 173, 42, 72, 112]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFirstSignInSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IFirstSignInSettingsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFirstSignInSettingsStatics {
    type Vtable = IFirstSignInSettingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(484544271, 7233, 20128, [183, 162, 111, 12, 28, 126, 132, 56]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFirstSignInSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesForUser(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGlobalizationPreferencesForUser {
    type Vtable = IGlobalizationPreferencesForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(353306517, 20334, 16570, [160, 16, 226, 125, 129, 189, 167, 245]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesForUser_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Globalization::DayOfWeek) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGlobalizationPreferencesStatics {
    type Vtable = IGlobalizationPreferencesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(29311782, 60727, 20118, [176, 233, 193, 52, 13, 30, 161, 88]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Globalization::DayOfWeek) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGlobalizationPreferencesStatics2 {
    type Vtable = IGlobalizationPreferencesStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4241393137, 17152, 19664, [156, 172, 26, 142, 123, 126, 24, 244]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, region: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languagetags: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGlobalizationPreferencesStatics3 {
    type Vtable = IGlobalizationPreferencesStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(503682867, 13813, 16600, [185, 232, 174, 243, 239, 133, 111, 206]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILockScreenImageFeedStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILockScreenImageFeedStatics {
    type Vtable = ILockScreenImageFeedStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(739079158, 937, 16806, [155, 1, 73, 82, 81, 255, 81, 213]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenImageFeedStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, syndicationfeeduri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILockScreenStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILockScreenStatics {
    type Vtable = ILockScreenStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1055511469, 46599, 16558, [180, 38, 118, 49, 217, 130, 18, 105]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IUserInformationStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserInformationStatics {
    type Vtable = IUserInformationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2012457232, 18682, 18588, [147, 78, 42, 232, 91, 168, 247, 114]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserInformationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: AccountPictureKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, image: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, smallimage: ::windows::runtime::RawPtr, largeimage: ::windows::runtime::RawPtr, video: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, image: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, smallimage: ::windows::runtime::RawPtr, largeimage: ::windows::runtime::RawPtr, video: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, changehandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IUserProfilePersonalizationSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserProfilePersonalizationSettings {
    type Vtable = IUserProfilePersonalizationSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2364398260, 31128, 18133, [141, 211, 24, 79, 28, 95, 154, 185]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserProfilePersonalizationSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagefile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagefile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IUserProfilePersonalizationSettingsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserProfilePersonalizationSettingsStatics {
    type Vtable = IUserProfilePersonalizationSettingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2444015681, 20535, 17739, [152, 131, 187, 119, 45, 8, 221, 22]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserProfilePersonalizationSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `System_UserProfile`*"]
pub struct LockScreen {}
impl LockScreen {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`*"]
    pub fn RequestSetImageFeedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(syndicationfeeduri: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SetImageFeedResult>> {
        Self::ILockScreenImageFeedStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), syndicationfeeduri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SetImageFeedResult>>(result__)
        })
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn TryRemoveImageFeed() -> ::windows::runtime::Result<bool> {
        Self::ILockScreenImageFeedStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`*"]
    pub fn OriginalImageFile() -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `System_UserProfile`, `Storage_Streams`*"]
    pub fn GetImageStream() -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStream> {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`, `Storage`*"]
    pub fn SetImageFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`, `Storage_Streams`*"]
    pub fn SetImageStreamAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn ILockScreenImageFeedStatics<R, F: FnOnce(&ILockScreenImageFeedStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LockScreen, ILockScreenImageFeedStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILockScreenStatics<R, F: FnOnce(&ILockScreenStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LockScreen, ILockScreenStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for LockScreen {
    const NAME: &'static str = "Windows.System.UserProfile.LockScreen";
}
#[doc = "*Required features: `System_UserProfile`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SetAccountPictureResult(pub i32);
impl SetAccountPictureResult {
    pub const Success: SetAccountPictureResult = SetAccountPictureResult(0i32);
    pub const ChangeDisabled: SetAccountPictureResult = SetAccountPictureResult(1i32);
    pub const LargeOrDynamicError: SetAccountPictureResult = SetAccountPictureResult(2i32);
    pub const VideoFrameSizeError: SetAccountPictureResult = SetAccountPictureResult(3i32);
    pub const FileSizeError: SetAccountPictureResult = SetAccountPictureResult(4i32);
    pub const Failure: SetAccountPictureResult = SetAccountPictureResult(5i32);
}
impl ::std::convert::From<i32> for SetAccountPictureResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SetAccountPictureResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SetAccountPictureResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.UserProfile.SetAccountPictureResult;i4)");
}
#[doc = "*Required features: `System_UserProfile`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SetImageFeedResult(pub i32);
impl SetImageFeedResult {
    pub const Success: SetImageFeedResult = SetImageFeedResult(0i32);
    pub const ChangeDisabled: SetImageFeedResult = SetImageFeedResult(1i32);
    pub const UserCanceled: SetImageFeedResult = SetImageFeedResult(2i32);
}
impl ::std::convert::From<i32> for SetImageFeedResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SetImageFeedResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SetImageFeedResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.UserProfile.SetImageFeedResult;i4)");
}
#[doc = "*Required features: `System_UserProfile`*"]
pub struct UserInformation {}
impl UserInformation {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn AccountPictureChangeEnabled() -> ::windows::runtime::Result<bool> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn NameAccessAllowed() -> ::windows::runtime::Result<bool> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `System_UserProfile`, `Storage`*"]
    pub fn GetAccountPicture(kind: AccountPictureKind) -> ::windows::runtime::Result<super::super::Storage::IStorageFile> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), kind, &mut result__).from_abi::<super::super::Storage::IStorageFile>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`, `Storage`*"]
    pub fn SetAccountPictureAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(image: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), image.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`, `Storage`*"]
    pub fn SetAccountPicturesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>, Param2: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(smallimage: Param0, largeimage: Param1, video: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), smallimage.into_param().abi(), largeimage.into_param().abi(), video.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`, `Storage_Streams`*"]
    pub fn SetAccountPictureFromStreamAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(image: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), image.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`, `Storage_Streams`*"]
    pub fn SetAccountPicturesFromStreamsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(smallimage: Param0, largeimage: Param1, video: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), smallimage.into_param().abi(), largeimage.into_param().abi(), video.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`*"]
    pub fn AccountPictureChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(changehandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), changehandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`*"]
    pub fn RemoveAccountPictureChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IUserInformationStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`*"]
    pub fn GetDisplayNameAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`*"]
    pub fn GetFirstNameAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`*"]
    pub fn GetLastNameAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`*"]
    pub fn GetPrincipalNameAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`*"]
    pub fn GetSessionInitiationProtocolUriAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`*"]
    pub fn GetDomainNameAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        })
    }
    pub fn IUserInformationStatics<R, F: FnOnce(&IUserInformationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UserInformation, IUserInformationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for UserInformation {
    const NAME: &'static str = "Windows.System.UserProfile.UserInformation";
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct UserProfileContract(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct UserProfileLockScreenContract(pub u8);
#[doc = "*Required features: `System_UserProfile`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UserProfilePersonalizationSettings(::windows::runtime::IInspectable);
impl UserProfilePersonalizationSettings {
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`, `Storage`*"]
    pub fn TrySetLockScreenImageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>>(&self, imagefile: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), imagefile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `System_UserProfile`, `Foundation`, `Storage`*"]
    pub fn TrySetWallpaperImageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>>(&self, imagefile: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), imagefile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn Current() -> ::windows::runtime::Result<UserProfilePersonalizationSettings> {
        Self::IUserProfilePersonalizationSettingsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserProfilePersonalizationSettings>(result__)
        })
    }
    #[doc = "*Required features: `System_UserProfile`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IUserProfilePersonalizationSettingsStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IUserProfilePersonalizationSettingsStatics<R, F: FnOnce(&IUserProfilePersonalizationSettingsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UserProfilePersonalizationSettings, IUserProfilePersonalizationSettingsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserProfilePersonalizationSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.UserProfilePersonalizationSettings;{8ceddab4-7998-46d5-8dd3-184f1c5f9ab9})");
}
unsafe impl ::windows::runtime::Interface for UserProfilePersonalizationSettings {
    type Vtable = IUserProfilePersonalizationSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2364398260, 31128, 18133, [141, 211, 24, 79, 28, 95, 154, 185]);
}
impl ::windows::runtime::RuntimeName for UserProfilePersonalizationSettings {
    const NAME: &'static str = "Windows.System.UserProfile.UserProfilePersonalizationSettings";
}
impl ::std::convert::From<UserProfilePersonalizationSettings> for ::windows::runtime::IUnknown {
    fn from(value: UserProfilePersonalizationSettings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserProfilePersonalizationSettings> for ::windows::runtime::IUnknown {
    fn from(value: &UserProfilePersonalizationSettings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserProfilePersonalizationSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UserProfilePersonalizationSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UserProfilePersonalizationSettings> for ::windows::runtime::IInspectable {
    fn from(value: UserProfilePersonalizationSettings) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserProfilePersonalizationSettings> for ::windows::runtime::IInspectable {
    fn from(value: &UserProfilePersonalizationSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserProfilePersonalizationSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserProfilePersonalizationSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UserProfilePersonalizationSettings {}
unsafe impl ::std::marker::Sync for UserProfilePersonalizationSettings {}
