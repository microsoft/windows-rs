#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockApplicationHost(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILockApplicationHost {
    type Vtable = ILockApplicationHost_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38ee31ad_d94f_4e7c_81fa_4f4436506281);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockApplicationHost_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RequestUnlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Unlocking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Unlocking: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnlocking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnlocking: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockApplicationHostStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILockApplicationHostStatics {
    type Vtable = ILockApplicationHostStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf48fab8e_23d7_4e63_96a1_666ff52d3b2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockApplicationHostStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockScreenBadge(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILockScreenBadge {
    type Vtable = ILockScreenBadge_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe95105d9_2bff_4db0_9b4f_3824778b9c9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenBadge_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Glyph: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Glyph: usize,
    #[cfg(feature = "Foundation")]
    pub Number: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Number: usize,
    pub AutomationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LaunchApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockScreenInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILockScreenInfo {
    type Vtable = ILockScreenInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf59aa65c_9711_4dc9_a630_95b6cb8cdad0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub LockScreenImageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LockScreenImageChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLockScreenImageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLockScreenImageChanged: usize,
    #[cfg(feature = "Storage_Streams")]
    pub LockScreenImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LockScreenImage: usize,
    #[cfg(feature = "Foundation")]
    pub BadgesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BadgesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBadgesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBadgesChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Badges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Badges: usize,
    #[cfg(feature = "Foundation")]
    pub DetailTextChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DetailTextChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDetailTextChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDetailTextChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DetailText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DetailText: usize,
    #[cfg(feature = "Foundation")]
    pub AlarmIconChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AlarmIconChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAlarmIconChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAlarmIconChanged: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AlarmIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AlarmIcon: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockScreenUnlockingDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILockScreenUnlockingDeferral {
    type Vtable = ILockScreenUnlockingDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e7d1ad6_5203_43e7_9bd6_7c3947d1e3fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenUnlockingDeferral_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockScreenUnlockingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILockScreenUnlockingEventArgs {
    type Vtable = ILockScreenUnlockingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44e6c007_75fb_4abb_9f8b_824748900c71);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenUnlockingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
}
#[doc = "*Required features: `\"ApplicationModel_LockScreen\"`*"]
#[repr(transparent)]
pub struct LockApplicationHost(::windows::core::IUnknown);
impl LockApplicationHost {
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`*"]
    pub fn RequestUnlock(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RequestUnlock)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Unlocking<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<LockApplicationHost, LockScreenUnlockingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Unlocking)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnlocking<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUnlocking)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<LockApplicationHost> {
        Self::ILockApplicationHostStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LockApplicationHost>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILockApplicationHostStatics<R, F: FnOnce(&ILockApplicationHostStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LockApplicationHost, ILockApplicationHostStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LockApplicationHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockApplicationHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockApplicationHost {}
impl ::core::fmt::Debug for LockApplicationHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockApplicationHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LockApplicationHost {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.LockScreen.LockApplicationHost;{38ee31ad-d94f-4e7c-81fa-4f4436506281})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LockApplicationHost {
    type Vtable = ILockApplicationHost_Vtbl;
    const IID: ::windows::core::GUID = <ILockApplicationHost as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LockApplicationHost {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.LockApplicationHost";
}
impl ::core::convert::From<LockApplicationHost> for ::windows::core::IUnknown {
    fn from(value: LockApplicationHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockApplicationHost> for ::windows::core::IUnknown {
    fn from(value: &LockApplicationHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockApplicationHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LockApplicationHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LockApplicationHost> for ::windows::core::IInspectable {
    fn from(value: LockApplicationHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockApplicationHost> for ::windows::core::IInspectable {
    fn from(value: &LockApplicationHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockApplicationHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LockApplicationHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LockApplicationHost {}
unsafe impl ::core::marker::Sync for LockApplicationHost {}
#[doc = "*Required features: `\"ApplicationModel_LockScreen\"`*"]
#[repr(transparent)]
pub struct LockScreenBadge(::windows::core::IUnknown);
impl LockScreenBadge {
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Logo(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Logo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Glyph(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Glyph)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Number(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Number)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`*"]
    pub fn AutomationName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutomationName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`*"]
    pub fn LaunchApp(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LaunchApp)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for LockScreenBadge {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockScreenBadge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenBadge {}
impl ::core::fmt::Debug for LockScreenBadge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenBadge").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LockScreenBadge {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.LockScreen.LockScreenBadge;{e95105d9-2bff-4db0-9b4f-3824778b9c9a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LockScreenBadge {
    type Vtable = ILockScreenBadge_Vtbl;
    const IID: ::windows::core::GUID = <ILockScreenBadge as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LockScreenBadge {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.LockScreenBadge";
}
impl ::core::convert::From<LockScreenBadge> for ::windows::core::IUnknown {
    fn from(value: LockScreenBadge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenBadge> for ::windows::core::IUnknown {
    fn from(value: &LockScreenBadge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockScreenBadge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LockScreenBadge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LockScreenBadge> for ::windows::core::IInspectable {
    fn from(value: LockScreenBadge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenBadge> for ::windows::core::IInspectable {
    fn from(value: &LockScreenBadge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockScreenBadge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LockScreenBadge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LockScreenBadge {}
unsafe impl ::core::marker::Sync for LockScreenBadge {}
#[doc = "*Required features: `\"ApplicationModel_LockScreen\"`*"]
#[repr(transparent)]
pub struct LockScreenInfo(::windows::core::IUnknown);
impl LockScreenInfo {
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LockScreenImageChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LockScreenImageChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLockScreenImageChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLockScreenImageChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LockScreenImage(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LockScreenImage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BadgesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BadgesChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBadgesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveBadgesChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Badges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<LockScreenBadge>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Badges)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<LockScreenBadge>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DetailTextChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DetailTextChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDetailTextChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDetailTextChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DetailText(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DetailText)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AlarmIconChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlarmIconChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAlarmIconChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAlarmIconChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AlarmIcon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlarmIcon)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
}
impl ::core::clone::Clone for LockScreenInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockScreenInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenInfo {}
impl ::core::fmt::Debug for LockScreenInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LockScreenInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.LockScreen.LockScreenInfo;{f59aa65c-9711-4dc9-a630-95b6cb8cdad0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LockScreenInfo {
    type Vtable = ILockScreenInfo_Vtbl;
    const IID: ::windows::core::GUID = <ILockScreenInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LockScreenInfo {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.LockScreenInfo";
}
impl ::core::convert::From<LockScreenInfo> for ::windows::core::IUnknown {
    fn from(value: LockScreenInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenInfo> for ::windows::core::IUnknown {
    fn from(value: &LockScreenInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockScreenInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LockScreenInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LockScreenInfo> for ::windows::core::IInspectable {
    fn from(value: LockScreenInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenInfo> for ::windows::core::IInspectable {
    fn from(value: &LockScreenInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockScreenInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LockScreenInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LockScreenInfo {}
unsafe impl ::core::marker::Sync for LockScreenInfo {}
#[doc = "*Required features: `\"ApplicationModel_LockScreen\"`*"]
#[repr(transparent)]
pub struct LockScreenUnlockingDeferral(::windows::core::IUnknown);
impl LockScreenUnlockingDeferral {
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for LockScreenUnlockingDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockScreenUnlockingDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenUnlockingDeferral {}
impl ::core::fmt::Debug for LockScreenUnlockingDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenUnlockingDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LockScreenUnlockingDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.LockScreen.LockScreenUnlockingDeferral;{7e7d1ad6-5203-43e7-9bd6-7c3947d1e3fe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LockScreenUnlockingDeferral {
    type Vtable = ILockScreenUnlockingDeferral_Vtbl;
    const IID: ::windows::core::GUID = <ILockScreenUnlockingDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LockScreenUnlockingDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.LockScreenUnlockingDeferral";
}
impl ::core::convert::From<LockScreenUnlockingDeferral> for ::windows::core::IUnknown {
    fn from(value: LockScreenUnlockingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenUnlockingDeferral> for ::windows::core::IUnknown {
    fn from(value: &LockScreenUnlockingDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockScreenUnlockingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LockScreenUnlockingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LockScreenUnlockingDeferral> for ::windows::core::IInspectable {
    fn from(value: LockScreenUnlockingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenUnlockingDeferral> for ::windows::core::IInspectable {
    fn from(value: &LockScreenUnlockingDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockScreenUnlockingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LockScreenUnlockingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LockScreenUnlockingDeferral {}
unsafe impl ::core::marker::Sync for LockScreenUnlockingDeferral {}
#[doc = "*Required features: `\"ApplicationModel_LockScreen\"`*"]
#[repr(transparent)]
pub struct LockScreenUnlockingEventArgs(::windows::core::IUnknown);
impl LockScreenUnlockingEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<LockScreenUnlockingDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LockScreenUnlockingDeferral>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_LockScreen\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Deadline)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for LockScreenUnlockingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockScreenUnlockingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenUnlockingEventArgs {}
impl ::core::fmt::Debug for LockScreenUnlockingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenUnlockingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LockScreenUnlockingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.LockScreen.LockScreenUnlockingEventArgs;{44e6c007-75fb-4abb-9f8b-824748900c71})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LockScreenUnlockingEventArgs {
    type Vtable = ILockScreenUnlockingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ILockScreenUnlockingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LockScreenUnlockingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.LockScreenUnlockingEventArgs";
}
impl ::core::convert::From<LockScreenUnlockingEventArgs> for ::windows::core::IUnknown {
    fn from(value: LockScreenUnlockingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenUnlockingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LockScreenUnlockingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockScreenUnlockingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LockScreenUnlockingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LockScreenUnlockingEventArgs> for ::windows::core::IInspectable {
    fn from(value: LockScreenUnlockingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenUnlockingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LockScreenUnlockingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockScreenUnlockingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LockScreenUnlockingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LockScreenUnlockingEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenUnlockingEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
