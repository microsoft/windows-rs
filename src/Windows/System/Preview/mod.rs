#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HingeState(pub i32);
impl HingeState {
    pub const Unknown: HingeState = HingeState(0i32);
    pub const Closed: HingeState = HingeState(1i32);
    pub const Concave: HingeState = HingeState(2i32);
    pub const Flat: HingeState = HingeState(3i32);
    pub const Convex: HingeState = HingeState(4i32);
    pub const Full: HingeState = HingeState(5i32);
}
impl ::core::convert::From<i32> for HingeState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HingeState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for HingeState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Preview.HingeState;i4)");
}
impl ::windows::core::DefaultType for HingeState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITwoPanelHingedDevicePosturePreview {
    type Vtable = ITwoPanelHingedDevicePosturePreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72245c31_4b39_42a6_8e73_7235ade16853);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewReading(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITwoPanelHingedDevicePosturePreviewReading {
    type Vtable = ITwoPanelHingedDevicePosturePreviewReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0251452_4ad6_4b38_8426_c59a15493a7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewReading_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut HingeState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Sensors")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Devices::Sensors::SimpleOrientation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Sensors"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Sensors")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Devices::Sensors::SimpleOrientation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Sensors"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    type Vtable = ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d2d1bc6_02ce_474a_a556_a75b1cf93a03);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_abi(
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
pub struct ITwoPanelHingedDevicePosturePreviewStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITwoPanelHingedDevicePosturePreviewStatics {
    type Vtable = ITwoPanelHingedDevicePosturePreviewStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c4733d2_57e0_4180_bd5e_f31a2138423e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TwoPanelHingedDevicePosturePreview(pub ::windows::core::IInspectable);
impl TwoPanelHingedDevicePosturePreview {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentPostureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreviewReading>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreviewReading>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn PostureChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<TwoPanelHingedDevicePosturePreview, TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemovePostureChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreview>> {
        Self::ITwoPanelHingedDevicePosturePreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreview>>(result__)
        })
    }
    pub fn ITwoPanelHingedDevicePosturePreviewStatics<R, F: FnOnce(&ITwoPanelHingedDevicePosturePreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TwoPanelHingedDevicePosturePreview, ITwoPanelHingedDevicePosturePreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TwoPanelHingedDevicePosturePreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Preview.TwoPanelHingedDevicePosturePreview;{72245c31-4b39-42a6-8e73-7235ade16853})");
}
unsafe impl ::windows::core::Interface for TwoPanelHingedDevicePosturePreview {
    type Vtable = ITwoPanelHingedDevicePosturePreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72245c31_4b39_42a6_8e73_7235ade16853);
}
impl ::windows::core::RuntimeName for TwoPanelHingedDevicePosturePreview {
    const NAME: &'static str = "Windows.System.Preview.TwoPanelHingedDevicePosturePreview";
}
impl ::core::convert::From<TwoPanelHingedDevicePosturePreview> for ::windows::core::IUnknown {
    fn from(value: TwoPanelHingedDevicePosturePreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreview> for ::windows::core::IUnknown {
    fn from(value: &TwoPanelHingedDevicePosturePreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TwoPanelHingedDevicePosturePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TwoPanelHingedDevicePosturePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TwoPanelHingedDevicePosturePreview> for ::windows::core::IInspectable {
    fn from(value: TwoPanelHingedDevicePosturePreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreview> for ::windows::core::IInspectable {
    fn from(value: &TwoPanelHingedDevicePosturePreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TwoPanelHingedDevicePosturePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TwoPanelHingedDevicePosturePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TwoPanelHingedDevicePosturePreview {}
unsafe impl ::core::marker::Sync for TwoPanelHingedDevicePosturePreview {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TwoPanelHingedDevicePosturePreviewReading(pub ::windows::core::IInspectable);
impl TwoPanelHingedDevicePosturePreviewReading {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn HingeState(&self) -> ::windows::core::Result<HingeState> {
        let this = self;
        unsafe {
            let mut result__: HingeState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HingeState>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Devices_Sensors")]
    pub fn Panel1Orientation(&self) -> ::windows::core::Result<super::super::Devices::Sensors::SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Sensors::SimpleOrientation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Sensors::SimpleOrientation>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Panel1Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Devices_Sensors")]
    pub fn Panel2Orientation(&self) -> ::windows::core::Result<super::super::Devices::Sensors::SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Sensors::SimpleOrientation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Sensors::SimpleOrientation>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Panel2Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TwoPanelHingedDevicePosturePreviewReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReading;{a0251452-4ad6-4b38-8426-c59a15493a7d})");
}
unsafe impl ::windows::core::Interface for TwoPanelHingedDevicePosturePreviewReading {
    type Vtable = ITwoPanelHingedDevicePosturePreviewReading_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0251452_4ad6_4b38_8426_c59a15493a7d);
}
impl ::windows::core::RuntimeName for TwoPanelHingedDevicePosturePreviewReading {
    const NAME: &'static str = "Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReading";
}
impl ::core::convert::From<TwoPanelHingedDevicePosturePreviewReading> for ::windows::core::IUnknown {
    fn from(value: TwoPanelHingedDevicePosturePreviewReading) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreviewReading> for ::windows::core::IUnknown {
    fn from(value: &TwoPanelHingedDevicePosturePreviewReading) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TwoPanelHingedDevicePosturePreviewReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TwoPanelHingedDevicePosturePreviewReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TwoPanelHingedDevicePosturePreviewReading> for ::windows::core::IInspectable {
    fn from(value: TwoPanelHingedDevicePosturePreviewReading) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreviewReading> for ::windows::core::IInspectable {
    fn from(value: &TwoPanelHingedDevicePosturePreviewReading) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TwoPanelHingedDevicePosturePreviewReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TwoPanelHingedDevicePosturePreviewReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TwoPanelHingedDevicePosturePreviewReading {}
unsafe impl ::core::marker::Sync for TwoPanelHingedDevicePosturePreviewReading {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs(pub ::windows::core::IInspectable);
impl TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn Reading(&self) -> ::windows::core::Result<TwoPanelHingedDevicePosturePreviewReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TwoPanelHingedDevicePosturePreviewReading>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs;{2d2d1bc6-02ce-474a-a556-a75b1cf93a03})");
}
unsafe impl ::windows::core::Interface for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    type Vtable = ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d2d1bc6_02ce_474a_a556_a75b1cf93a03);
}
impl ::windows::core::RuntimeName for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    const NAME: &'static str = "Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs";
}
impl ::core::convert::From<TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {}
