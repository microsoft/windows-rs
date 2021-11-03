#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Perception_Automation")]
pub mod Automation;
#[cfg(feature = "Perception_People")]
pub mod People;
#[cfg(feature = "Perception_Spatial")]
pub mod Spatial;
#[repr(transparent)]
#[doc(hidden)]
pub struct IPerceptionTimestamp(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionTimestamp {
    type Vtable = IPerceptionTimestamp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2277656580, 41518, 19163, [186, 38, 215, 142, 246, 57, 188, 244]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionTimestamp_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPerceptionTimestamp2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionTimestamp2 {
    type Vtable = IPerceptionTimestamp2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3813980141, 11217, 16823, [158, 208, 116, 161, 92, 53, 69, 55]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionTimestamp2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPerceptionTimestampHelperStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionTimestampHelperStatics {
    type Vtable = IPerceptionTimestampHelperStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1202065876, 43487, 20188, [133, 93, 244, 211, 57, 217, 103, 172]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionTimestampHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targettime: super::Foundation::DateTime, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPerceptionTimestampHelperStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionTimestampHelperStatics2 {
    type Vtable = IPerceptionTimestampHelperStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1943119870, 16313, 17777, [135, 212, 60, 146, 10, 94, 134, 235]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionTimestampHelperStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targettime: super::Foundation::TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `Perception`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PerceptionTimestamp(::windows::runtime::IInspectable);
impl PerceptionTimestamp {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Perception`, `Foundation`*"]
    pub fn TargetTime(&self) -> ::windows::runtime::Result<super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Perception`, `Foundation`*"]
    pub fn PredictionAmount(&self) -> ::windows::runtime::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Perception`, `Foundation`*"]
    pub fn SystemRelativeTargetTime(&self) -> ::windows::runtime::Result<super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IPerceptionTimestamp2>(self)?;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PerceptionTimestamp {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Perception.PerceptionTimestamp;{87c24804-a22e-4adb-ba26-d78ef639bcf4})");
}
unsafe impl ::windows::runtime::Interface for PerceptionTimestamp {
    type Vtable = IPerceptionTimestamp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2277656580, 41518, 19163, [186, 38, 215, 142, 246, 57, 188, 244]);
}
impl ::windows::runtime::RuntimeName for PerceptionTimestamp {
    const NAME: &'static str = "Windows.Perception.PerceptionTimestamp";
}
impl ::std::convert::From<PerceptionTimestamp> for ::windows::runtime::IUnknown {
    fn from(value: PerceptionTimestamp) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PerceptionTimestamp> for ::windows::runtime::IUnknown {
    fn from(value: &PerceptionTimestamp) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PerceptionTimestamp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PerceptionTimestamp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PerceptionTimestamp> for ::windows::runtime::IInspectable {
    fn from(value: PerceptionTimestamp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PerceptionTimestamp> for ::windows::runtime::IInspectable {
    fn from(value: &PerceptionTimestamp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PerceptionTimestamp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PerceptionTimestamp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PerceptionTimestamp {}
unsafe impl ::std::marker::Sync for PerceptionTimestamp {}
#[doc = "*Required features: `Perception`*"]
pub struct PerceptionTimestampHelper {}
impl PerceptionTimestampHelper {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Perception`, `Foundation`*"]
    pub fn FromHistoricalTargetTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::DateTime>>(targettime: Param0) -> ::windows::runtime::Result<PerceptionTimestamp> {
        Self::IPerceptionTimestampHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), targettime.into_param().abi(), &mut result__).from_abi::<PerceptionTimestamp>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Perception`, `Foundation`*"]
    pub fn FromSystemRelativeTargetTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TimeSpan>>(targettime: Param0) -> ::windows::runtime::Result<PerceptionTimestamp> {
        Self::IPerceptionTimestampHelperStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), targettime.into_param().abi(), &mut result__).from_abi::<PerceptionTimestamp>(result__)
        })
    }
    pub fn IPerceptionTimestampHelperStatics<R, F: FnOnce(&IPerceptionTimestampHelperStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PerceptionTimestampHelper, IPerceptionTimestampHelperStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPerceptionTimestampHelperStatics2<R, F: FnOnce(&IPerceptionTimestampHelperStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PerceptionTimestampHelper, IPerceptionTimestampHelperStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PerceptionTimestampHelper {
    const NAME: &'static str = "Windows.Perception.PerceptionTimestampHelper";
}
