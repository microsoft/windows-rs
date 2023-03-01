#[cfg(feature = "Perception_Automation")]
pub mod Automation;
#[cfg(feature = "Perception_People")]
pub mod People;
#[cfg(feature = "Perception_Spatial")]
pub mod Spatial;
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionTimestamp(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionTimestamp {
    type Vtable = IPerceptionTimestamp_Vtbl;
}
impl ::core::clone::Clone for IPerceptionTimestamp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPerceptionTimestamp {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87c24804_a22e_4adb_ba26_d78ef639bcf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionTimestamp_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TargetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TargetTime: usize,
    #[cfg(feature = "Foundation")]
    pub PredictionAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PredictionAmount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionTimestamp2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionTimestamp2 {
    type Vtable = IPerceptionTimestamp2_Vtbl;
}
impl ::core::clone::Clone for IPerceptionTimestamp2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPerceptionTimestamp2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe354b7ed_2bd1_41b7_9ed0_74a15c354537);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionTimestamp2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeTargetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeTargetTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionTimestampHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionTimestampHelperStatics {
    type Vtable = IPerceptionTimestampHelperStatics_Vtbl;
}
impl ::core::clone::Clone for IPerceptionTimestampHelperStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPerceptionTimestampHelperStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47a611d4_a9df_4edc_855d_f4d339d967ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionTimestampHelperStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromHistoricalTargetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targettime: super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromHistoricalTargetTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionTimestampHelperStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionTimestampHelperStatics2 {
    type Vtable = IPerceptionTimestampHelperStatics2_Vtbl;
}
impl ::core::clone::Clone for IPerceptionTimestampHelperStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPerceptionTimestampHelperStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73d1a7fe_3fb9_4571_87d4_3c920a5e86eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionTimestampHelperStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromSystemRelativeTargetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targettime: super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromSystemRelativeTargetTime: usize,
}
#[doc = "*Required features: `\"Perception\"`*"]
#[repr(transparent)]
pub struct PerceptionTimestamp(::windows::core::IUnknown);
impl PerceptionTimestamp {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TargetTime(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).TargetTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PredictionAmount(&self) -> ::windows::core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).PredictionAmount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTargetTime(&self) -> ::windows::core::Result<super::Foundation::TimeSpan> {
        let this = &::windows::core::ComInterface::cast::<IPerceptionTimestamp2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).SystemRelativeTargetTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PerceptionTimestamp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PerceptionTimestamp {}
impl ::core::fmt::Debug for PerceptionTimestamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionTimestamp").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PerceptionTimestamp {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.PerceptionTimestamp;{87c24804-a22e-4adb-ba26-d78ef639bcf4})");
}
impl ::core::clone::Clone for PerceptionTimestamp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PerceptionTimestamp {
    type Vtable = IPerceptionTimestamp_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PerceptionTimestamp {
    const IID: ::windows::core::GUID = <IPerceptionTimestamp as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PerceptionTimestamp {
    const NAME: &'static str = "Windows.Perception.PerceptionTimestamp";
}
::windows::imp::interface_hierarchy!(PerceptionTimestamp, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PerceptionTimestamp {}
unsafe impl ::core::marker::Sync for PerceptionTimestamp {}
#[doc = "*Required features: `\"Perception\"`*"]
pub struct PerceptionTimestampHelper;
impl PerceptionTimestampHelper {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromHistoricalTargetTime(targettime: super::Foundation::DateTime) -> ::windows::core::Result<PerceptionTimestamp> {
        Self::IPerceptionTimestampHelperStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PerceptionTimestamp>();
            (::windows::core::Interface::vtable(this).FromHistoricalTargetTime)(::windows::core::Interface::as_raw(this), targettime, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromSystemRelativeTargetTime(targettime: super::Foundation::TimeSpan) -> ::windows::core::Result<PerceptionTimestamp> {
        Self::IPerceptionTimestampHelperStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PerceptionTimestamp>();
            (::windows::core::Interface::vtable(this).FromSystemRelativeTargetTime)(::windows::core::Interface::as_raw(this), targettime, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPerceptionTimestampHelperStatics<R, F: FnOnce(&IPerceptionTimestampHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PerceptionTimestampHelper, IPerceptionTimestampHelperStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPerceptionTimestampHelperStatics2<R, F: FnOnce(&IPerceptionTimestampHelperStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PerceptionTimestampHelper, IPerceptionTimestampHelperStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for PerceptionTimestampHelper {
    const NAME: &'static str = "Windows.Perception.PerceptionTimestampHelper";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
