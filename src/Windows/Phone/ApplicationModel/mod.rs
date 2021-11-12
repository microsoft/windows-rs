#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub struct ApplicationProfile {}
impl ApplicationProfile {
    pub fn Modes() -> ::windows::core::Result<ApplicationProfileModes> {
        Self::IApplicationProfileStatics(|this| unsafe {
            let mut result__: ApplicationProfileModes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationProfileModes>(result__)
        })
    }
    pub fn IApplicationProfileStatics<R, F: FnOnce(&IApplicationProfileStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ApplicationProfile, IApplicationProfileStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ApplicationProfile {
    const NAME: &'static str = "Windows.Phone.ApplicationModel.ApplicationProfile";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ApplicationProfileModes(pub u32);
impl ApplicationProfileModes {
    pub const Default: ApplicationProfileModes = ApplicationProfileModes(0u32);
    pub const Alternate: ApplicationProfileModes = ApplicationProfileModes(1u32);
}
impl ::core::convert::From<u32> for ApplicationProfileModes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ApplicationProfileModes {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ApplicationProfileModes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.ApplicationModel.ApplicationProfileModes;u4)");
}
impl ::windows::core::DefaultType for ApplicationProfileModes {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for ApplicationProfileModes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for ApplicationProfileModes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for ApplicationProfileModes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for ApplicationProfileModes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for ApplicationProfileModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IApplicationProfileStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IApplicationProfileStatics {
    type Vtable = IApplicationProfileStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5008ab4_7e7a_11e1_a7f2_b0a14824019b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationProfileStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ApplicationProfileModes) -> ::windows::core::HRESULT,
);
