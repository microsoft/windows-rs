#[doc(hidden)]
#[repr(transparent)]
pub struct IGeolocationProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGeolocationProvider {
    type Vtable = IGeolocationProvider_Vtbl;
}
impl ::core::clone::Clone for IGeolocationProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGeolocationProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4cf071d_3f64_509f_8dc2_0b74a059829d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocationProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsOverridden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetOverridePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newposition: super::BasicGeoposition, positionsource: super::PositionSource, accuracyinmeters: f64, result__: *mut LocationOverrideStatus) -> ::windows::core::HRESULT,
    pub ClearOverridePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsOverriddenChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsOverriddenChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsOverriddenChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsOverriddenChanged: usize,
}
#[doc = "*Required features: `\"Devices_Geolocation_Provider\"`*"]
#[repr(transparent)]
pub struct GeolocationProvider(::windows::core::IUnknown);
impl GeolocationProvider {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GeolocationProvider, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsOverridden(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsOverridden)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOverridePosition(&self, newposition: super::BasicGeoposition, positionsource: super::PositionSource, accuracyinmeters: f64) -> ::windows::core::Result<LocationOverrideStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LocationOverrideStatus>();
            (::windows::core::Interface::vtable(this).SetOverridePosition)(::windows::core::Interface::as_raw(this), newposition, positionsource, accuracyinmeters, &mut result__).from_abi(result__)
        }
    }
    pub fn ClearOverridePosition(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ClearOverridePosition)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsOverriddenChanged(&self, handler: &super::super::super::Foundation::EventHandler<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).IsOverriddenChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsOverriddenChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveIsOverriddenChanged)(::windows::core::Interface::as_raw(this), token).ok() }
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
impl ::windows::core::RuntimeType for GeolocationProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Provider.GeolocationProvider;{e4cf071d-3f64-509f-8dc2-0b74a059829d})");
}
impl ::core::clone::Clone for GeolocationProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GeolocationProvider {
    type Vtable = IGeolocationProvider_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GeolocationProvider {
    const IID: ::windows::core::GUID = <IGeolocationProvider as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GeolocationProvider {
    const NAME: &'static str = "Windows.Devices.Geolocation.Provider.GeolocationProvider";
}
::windows::imp::interface_hierarchy!(GeolocationProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::TypeKind for LocationOverrideStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LocationOverrideStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocationOverrideStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LocationOverrideStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Provider.LocationOverrideStatus;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
