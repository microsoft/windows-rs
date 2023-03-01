#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceLockdownProfileInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceLockdownProfileInformation {
    type Vtable = IDeviceLockdownProfileInformation_Vtbl;
}
impl ::core::clone::Clone for IDeviceLockdownProfileInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDeviceLockdownProfileInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7980e14e_45b1_4a96_92fc_62756b739678);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceLockdownProfileInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceLockdownProfileStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceLockdownProfileStatics {
    type Vtable = IDeviceLockdownProfileStatics_Vtbl;
}
impl ::core::clone::Clone for IDeviceLockdownProfileStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDeviceLockdownProfileStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x622f6965_f9a8_41a1_a691_88cd80c7a069);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceLockdownProfileStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedLockdownProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedLockdownProfiles: usize,
    pub GetCurrentLockdownProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplyLockdownProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profileid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplyLockdownProfileAsync: usize,
    pub GetLockdownProfileInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profileid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Embedded_DeviceLockdown\"`*"]
pub struct DeviceLockdownProfile;
impl DeviceLockdownProfile {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedLockdownProfiles() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::GUID>> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<::windows::core::GUID>>();
            (::windows::core::Interface::vtable(this).GetSupportedLockdownProfiles)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetCurrentLockdownProfile() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).GetCurrentLockdownProfile)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApplyLockdownProfileAsync(profileid: ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ApplyLockdownProfileAsync)(::windows::core::Interface::as_raw(this), profileid, &mut result__).from_abi(result__)
        })
    }
    pub fn GetLockdownProfileInformation(profileid: ::windows::core::GUID) -> ::windows::core::Result<DeviceLockdownProfileInformation> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<DeviceLockdownProfileInformation>();
            (::windows::core::Interface::vtable(this).GetLockdownProfileInformation)(::windows::core::Interface::as_raw(this), profileid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeviceLockdownProfileStatics<R, F: FnOnce(&IDeviceLockdownProfileStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<DeviceLockdownProfile, IDeviceLockdownProfileStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for DeviceLockdownProfile {
    const NAME: &'static str = "Windows.Embedded.DeviceLockdown.DeviceLockdownProfile";
}
#[doc = "*Required features: `\"Embedded_DeviceLockdown\"`*"]
#[repr(transparent)]
pub struct DeviceLockdownProfileInformation(::windows::core::IUnknown);
impl DeviceLockdownProfileInformation {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeviceLockdownProfileInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceLockdownProfileInformation {}
impl ::core::fmt::Debug for DeviceLockdownProfileInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceLockdownProfileInformation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DeviceLockdownProfileInformation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Embedded.DeviceLockdown.DeviceLockdownProfileInformation;{7980e14e-45b1-4a96-92fc-62756b739678})");
}
impl ::core::clone::Clone for DeviceLockdownProfileInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DeviceLockdownProfileInformation {
    type Vtable = IDeviceLockdownProfileInformation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DeviceLockdownProfileInformation {
    const IID: ::windows::core::GUID = <IDeviceLockdownProfileInformation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DeviceLockdownProfileInformation {
    const NAME: &'static str = "Windows.Embedded.DeviceLockdown.DeviceLockdownProfileInformation";
}
::windows::imp::interface_hierarchy!(DeviceLockdownProfileInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DeviceLockdownProfileInformation {}
unsafe impl ::core::marker::Sync for DeviceLockdownProfileInformation {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
