#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Phone_ApplicationModel\"`*"]
pub struct ApplicationProfile {}
impl ApplicationProfile {
    #[doc = "*Required features: `\"Phone_ApplicationModel\"`*"]
    pub fn Modes() -> ::windows::core::Result<ApplicationProfileModes> {
        Self::IApplicationProfileStatics(|this| unsafe {
            let mut result__: ApplicationProfileModes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Modes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationProfileModes>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IApplicationProfileStatics<R, F: FnOnce(&IApplicationProfileStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ApplicationProfile, IApplicationProfileStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ApplicationProfile {
    const NAME: &'static str = "Windows.Phone.ApplicationModel.ApplicationProfile";
}
#[doc = "*Required features: `\"Phone_ApplicationModel\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ApplicationProfileModes(pub u32);
impl ApplicationProfileModes {
    pub const Default: Self = Self(0u32);
    pub const Alternate: Self = Self(1u32);
}
impl ::core::marker::Copy for ApplicationProfileModes {}
impl ::core::clone::Clone for ApplicationProfileModes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationProfileModes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ApplicationProfileModes {
    type Abi = Self;
}
impl ::core::fmt::Debug for ApplicationProfileModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationProfileModes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ApplicationProfileModes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ApplicationProfileModes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ApplicationProfileModes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ApplicationProfileModes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ApplicationProfileModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationProfileModes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.ApplicationModel.ApplicationProfileModes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationProfileStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationProfileStatics {
    type Vtable = IApplicationProfileStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5008ab4_7e7a_11e1_a7f2_b0a14824019b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationProfileStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Modes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ApplicationProfileModes) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
