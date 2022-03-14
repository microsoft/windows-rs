#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"System_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HingeState(pub i32);
#[cfg(feature = "deprecated")]
impl HingeState {
    pub const Unknown: Self = Self(0i32);
    pub const Closed: Self = Self(1i32);
    pub const Concave: Self = Self(2i32);
    pub const Flat: Self = Self(3i32);
    pub const Convex: Self = Self(4i32);
    pub const Full: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for HingeState {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for HingeState {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for HingeState {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for HingeState {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for HingeState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HingeState").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for HingeState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Preview.HingeState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ITwoPanelHingedDevicePosturePreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ITwoPanelHingedDevicePosturePreview {
    type Vtable = ITwoPanelHingedDevicePosturePreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72245c31_4b39_42a6_8e73_7235ade16853);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreview_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetCurrentPostureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetCurrentPostureAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PostureChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PostureChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePostureChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePostureChanged: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ITwoPanelHingedDevicePosturePreviewReading(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ITwoPanelHingedDevicePosturePreviewReading {
    type Vtable = ITwoPanelHingedDevicePosturePreviewReading_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0251452_4ad6_4b38_8426_c59a15493a7d);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewReading_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Timestamp: usize,
    #[cfg(feature = "deprecated")]
    pub HingeState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HingeState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    HingeState: usize,
    #[cfg(all(feature = "Devices_Sensors", feature = "deprecated"))]
    pub Panel1Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Sensors::SimpleOrientation) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Sensors", feature = "deprecated")))]
    Panel1Orientation: usize,
    #[cfg(feature = "deprecated")]
    pub Panel1Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Panel1Id: usize,
    #[cfg(all(feature = "Devices_Sensors", feature = "deprecated"))]
    pub Panel2Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Sensors::SimpleOrientation) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Sensors", feature = "deprecated")))]
    Panel2Orientation: usize,
    #[cfg(feature = "deprecated")]
    pub Panel2Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Panel2Id: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    type Vtable = ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d2d1bc6_02ce_474a_a556_a75b1cf93a03);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Reading: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ITwoPanelHingedDevicePosturePreviewStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ITwoPanelHingedDevicePosturePreviewStatics {
    type Vtable = ITwoPanelHingedDevicePosturePreviewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c4733d2_57e0_4180_bd5e_f31a2138423e);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDefaultAsync: usize,
}
#[doc = "*Required features: `\"System_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct TwoPanelHingedDevicePosturePreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl TwoPanelHingedDevicePosturePreview {
    #[doc = "*Required features: `\"System_Preview\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetCurrentPostureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreviewReading>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentPostureAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreviewReading>>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Preview\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PostureChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<TwoPanelHingedDevicePosturePreview, TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PostureChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Preview\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemovePostureChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePostureChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System_Preview\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreview>> {
        Self::ITwoPanelHingedDevicePosturePreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreview>>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ITwoPanelHingedDevicePosturePreviewStatics<R, F: FnOnce(&ITwoPanelHingedDevicePosturePreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TwoPanelHingedDevicePosturePreview, ITwoPanelHingedDevicePosturePreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for TwoPanelHingedDevicePosturePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for TwoPanelHingedDevicePosturePreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for TwoPanelHingedDevicePosturePreview {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for TwoPanelHingedDevicePosturePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TwoPanelHingedDevicePosturePreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for TwoPanelHingedDevicePosturePreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Preview.TwoPanelHingedDevicePosturePreview;{72245c31-4b39-42a6-8e73-7235ade16853})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for TwoPanelHingedDevicePosturePreview {
    type Vtable = ITwoPanelHingedDevicePosturePreview_Vtbl;
    const IID: ::windows::core::GUID = <ITwoPanelHingedDevicePosturePreview as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for TwoPanelHingedDevicePosturePreview {
    const NAME: &'static str = "Windows.System.Preview.TwoPanelHingedDevicePosturePreview";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<TwoPanelHingedDevicePosturePreview> for ::windows::core::IUnknown {
    fn from(value: TwoPanelHingedDevicePosturePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreview> for ::windows::core::IUnknown {
    fn from(value: &TwoPanelHingedDevicePosturePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TwoPanelHingedDevicePosturePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TwoPanelHingedDevicePosturePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<TwoPanelHingedDevicePosturePreview> for ::windows::core::IInspectable {
    fn from(value: TwoPanelHingedDevicePosturePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreview> for ::windows::core::IInspectable {
    fn from(value: &TwoPanelHingedDevicePosturePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TwoPanelHingedDevicePosturePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TwoPanelHingedDevicePosturePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for TwoPanelHingedDevicePosturePreview {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for TwoPanelHingedDevicePosturePreview {}
#[doc = "*Required features: `\"System_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct TwoPanelHingedDevicePosturePreviewReading(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl TwoPanelHingedDevicePosturePreviewReading {
    #[doc = "*Required features: `\"System_Preview\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Preview\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn HingeState(&self) -> ::windows::core::Result<HingeState> {
        let this = self;
        unsafe {
            let mut result__: HingeState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HingeState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HingeState>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Preview\"`, `\"Devices_Sensors\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Devices_Sensors", feature = "deprecated"))]
    pub fn Panel1Orientation(&self) -> ::windows::core::Result<super::super::Devices::Sensors::SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Sensors::SimpleOrientation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Panel1Orientation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Sensors::SimpleOrientation>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Preview\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Panel1Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Panel1Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Preview\"`, `\"Devices_Sensors\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Devices_Sensors", feature = "deprecated"))]
    pub fn Panel2Orientation(&self) -> ::windows::core::Result<super::super::Devices::Sensors::SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Sensors::SimpleOrientation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Panel2Orientation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Sensors::SimpleOrientation>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Preview\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Panel2Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Panel2Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for TwoPanelHingedDevicePosturePreviewReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for TwoPanelHingedDevicePosturePreviewReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for TwoPanelHingedDevicePosturePreviewReading {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for TwoPanelHingedDevicePosturePreviewReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TwoPanelHingedDevicePosturePreviewReading").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for TwoPanelHingedDevicePosturePreviewReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReading;{a0251452-4ad6-4b38-8426-c59a15493a7d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for TwoPanelHingedDevicePosturePreviewReading {
    type Vtable = ITwoPanelHingedDevicePosturePreviewReading_Vtbl;
    const IID: ::windows::core::GUID = <ITwoPanelHingedDevicePosturePreviewReading as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for TwoPanelHingedDevicePosturePreviewReading {
    const NAME: &'static str = "Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReading";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<TwoPanelHingedDevicePosturePreviewReading> for ::windows::core::IUnknown {
    fn from(value: TwoPanelHingedDevicePosturePreviewReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreviewReading> for ::windows::core::IUnknown {
    fn from(value: &TwoPanelHingedDevicePosturePreviewReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TwoPanelHingedDevicePosturePreviewReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TwoPanelHingedDevicePosturePreviewReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<TwoPanelHingedDevicePosturePreviewReading> for ::windows::core::IInspectable {
    fn from(value: TwoPanelHingedDevicePosturePreviewReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreviewReading> for ::windows::core::IInspectable {
    fn from(value: &TwoPanelHingedDevicePosturePreviewReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TwoPanelHingedDevicePosturePreviewReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TwoPanelHingedDevicePosturePreviewReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for TwoPanelHingedDevicePosturePreviewReading {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for TwoPanelHingedDevicePosturePreviewReading {}
#[doc = "*Required features: `\"System_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    #[doc = "*Required features: `\"System_Preview\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Reading(&self) -> ::windows::core::Result<TwoPanelHingedDevicePosturePreviewReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reading)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TwoPanelHingedDevicePosturePreviewReading>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs;{2d2d1bc6-02ce-474a-a556-a75b1cf93a03})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    type Vtable = ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    const NAME: &'static str = "Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
