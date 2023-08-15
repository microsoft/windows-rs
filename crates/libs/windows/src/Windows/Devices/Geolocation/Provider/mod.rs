#[doc(hidden)]
#[repr(transparent)]
pub struct IGeolocationProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeolocationProvider {
    type Vtable = IGeolocationProvider_Vtbl;
}
impl ::core::clone::Clone for IGeolocationProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGeolocationProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4cf071d_3f64_509f_8dc2_0b74a059829d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocationProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsOverridden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetOverridePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newposition: super::BasicGeoposition, positionsource: super::PositionSource, accuracyinmeters: f64, result__: *mut LocationOverrideStatus) -> ::windows_core::HRESULT,
    pub ClearOverridePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsOverriddenChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsOverriddenChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsOverriddenChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsOverriddenChanged: usize,
}
#[doc = "*Required features: `\"Devices_Geolocation_Provider\"`*"]
#[repr(transparent)]
pub struct GeolocationProvider(::windows_core::IUnknown);
impl GeolocationProvider {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GeolocationProvider, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsOverridden(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsOverridden)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOverridePosition(&self, newposition: super::BasicGeoposition, positionsource: super::PositionSource, accuracyinmeters: f64) -> ::windows_core::Result<LocationOverrideStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetOverridePosition)(::windows_core::Interface::as_raw(this), newposition, positionsource, accuracyinmeters, &mut result__).from_abi(result__)
        }
    }
    pub fn ClearOverridePosition(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ClearOverridePosition)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsOverriddenChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsOverriddenChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsOverriddenChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveIsOverriddenChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for GeolocationProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeolocationProvider {}
impl ::core::fmt::Debug for GeolocationProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeolocationProvider").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GeolocationProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Provider.GeolocationProvider;{e4cf071d-3f64-509f-8dc2-0b74a059829d})");
}
impl ::core::clone::Clone for GeolocationProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GeolocationProvider {
    type Vtable = IGeolocationProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GeolocationProvider {
    const IID: ::windows_core::GUID = <IGeolocationProvider as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GeolocationProvider {
    const NAME: &'static str = "Windows.Devices.Geolocation.Provider.GeolocationProvider";
}
::windows_core::imp::interface_hierarchy!(GeolocationProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GeolocationProvider {}
unsafe impl ::core::marker::Sync for GeolocationProvider {}
#[doc = "*Required features: `\"Devices_Geolocation_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LocationOverrideStatus(pub i32);
impl LocationOverrideStatus {
    pub const Success: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const AlreadyStarted: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for LocationOverrideStatus {}
impl ::core::clone::Clone for LocationOverrideStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LocationOverrideStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for LocationOverrideStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LocationOverrideStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocationOverrideStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LocationOverrideStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Provider.LocationOverrideStatus;i4)");
}
