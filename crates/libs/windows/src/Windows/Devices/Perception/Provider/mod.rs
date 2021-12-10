#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownPerceptionFrameKindStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownPerceptionFrameKindStatics {
    type Vtable = IKnownPerceptionFrameKindStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ae651d6_9669_4106_9fae_4835c1b96104);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionFrameKindStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionControlGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionControlGroup {
    type Vtable = IPerceptionControlGroupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x172c4882_2fd9_4c4e_ba34_fdf20a73dde5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionControlGroupVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionControlGroupFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionControlGroupFactory {
    type Vtable = IPerceptionControlGroupFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f1af2e0_baf1_453b_bed4_cd9d4619154c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionControlGroupFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionCorrelation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionCorrelation {
    type Vtable = IPerceptionCorrelationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4131a82_dff5_4047_8a19_3b4d805f7176);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionCorrelationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionCorrelationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionCorrelationFactory {
    type Vtable = IPerceptionCorrelationFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4a6c425_2884_4a8f_8134_2835d7286cbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionCorrelationFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, position: super::super::super::Foundation::Numerics::Vector3, orientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionCorrelationGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionCorrelationGroup {
    type Vtable = IPerceptionCorrelationGroupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x752a0906_36a7_47bb_9b79_56cc6b746770);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionCorrelationGroupVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionCorrelationGroupFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionCorrelationGroupFactory {
    type Vtable = IPerceptionCorrelationGroupFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7dfe2088_63df_48ed_83b1_4ab829132995);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionCorrelationGroupFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativelocations: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionFaceAuthenticationGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionFaceAuthenticationGroup {
    type Vtable = IPerceptionFaceAuthenticationGroupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8019814_4a91_41b0_83a6_881a1775353e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFaceAuthenticationGroupVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionFaceAuthenticationGroupFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionFaceAuthenticationGroupFactory {
    type Vtable = IPerceptionFaceAuthenticationGroupFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe68a05d4_b60c_40f4_bcb9_f24d46467320);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFaceAuthenticationGroupFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ids: ::windows::core::RawPtr, starthandler: ::windows::core::RawPtr, stophandler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionFrame {
    type Vtable = IPerceptionFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cfe7825_54bb_4d9d_bec5_8ef66151d2ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
pub struct IPerceptionFrameProvider(::windows::core::IUnknown);
impl IPerceptionFrameProvider {
    #[cfg(feature = "deprecated")]
    pub fn FrameProviderInfo(&self) -> ::windows::core::Result<PerceptionFrameProviderInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PerceptionFrameProviderInfo>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Available(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetProperty<'a, Param0: ::windows::core::IntoParam<'a, PerceptionPropertyChangeRequest>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IPerceptionFrameProvider> for ::windows::core::IInspectable {
    fn from(value: IPerceptionFrameProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPerceptionFrameProvider> for ::windows::core::IInspectable {
    fn from(value: &IPerceptionFrameProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPerceptionFrameProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IPerceptionFrameProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPerceptionFrameProvider> for ::windows::core::IUnknown {
    fn from(value: IPerceptionFrameProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPerceptionFrameProvider> for ::windows::core::IUnknown {
    fn from(value: &IPerceptionFrameProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPerceptionFrameProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPerceptionFrameProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IPerceptionFrameProvider> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IPerceptionFrameProvider) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IPerceptionFrameProvider> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPerceptionFrameProvider) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for IPerceptionFrameProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &IPerceptionFrameProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IPerceptionFrameProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPerceptionFrameProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPerceptionFrameProvider {}
unsafe impl ::windows::core::RuntimeType for IPerceptionFrameProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{794f7ab9-b37d-3b33-a10d-30626419ce65}");
}
unsafe impl ::windows::core::Interface for IPerceptionFrameProvider {
    type Vtable = IPerceptionFrameProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x794f7ab9_b37d_3b33_a10d_30626419ce65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionFrameProviderInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionFrameProviderInfo {
    type Vtable = IPerceptionFrameProviderInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcca959e8_797e_4e83_9b87_036a74142fc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameProviderInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IPerceptionFrameProviderManager(::windows::core::IUnknown);
impl IPerceptionFrameProviderManager {
    #[cfg(feature = "deprecated")]
    pub fn GetFrameProvider<'a, Param0: ::windows::core::IntoParam<'a, PerceptionFrameProviderInfo>>(&self, frameproviderinfo: Param0) -> ::windows::core::Result<IPerceptionFrameProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), frameproviderinfo.into_param().abi(), &mut result__).from_abi::<IPerceptionFrameProvider>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IPerceptionFrameProviderManager> for ::windows::core::IInspectable {
    fn from(value: IPerceptionFrameProviderManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPerceptionFrameProviderManager> for ::windows::core::IInspectable {
    fn from(value: &IPerceptionFrameProviderManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPerceptionFrameProviderManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IPerceptionFrameProviderManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPerceptionFrameProviderManager> for ::windows::core::IUnknown {
    fn from(value: IPerceptionFrameProviderManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPerceptionFrameProviderManager> for ::windows::core::IUnknown {
    fn from(value: &IPerceptionFrameProviderManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPerceptionFrameProviderManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPerceptionFrameProviderManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IPerceptionFrameProviderManager> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IPerceptionFrameProviderManager) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IPerceptionFrameProviderManager> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPerceptionFrameProviderManager) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for IPerceptionFrameProviderManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &IPerceptionFrameProviderManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IPerceptionFrameProviderManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPerceptionFrameProviderManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPerceptionFrameProviderManager {}
unsafe impl ::windows::core::RuntimeType for IPerceptionFrameProviderManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a959ce07-ead3-33df-8ec1-b924abe019c4}");
}
unsafe impl ::windows::core::Interface for IPerceptionFrameProviderManager {
    type Vtable = IPerceptionFrameProviderManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa959ce07_ead3_33df_8ec1_b924abe019c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameProviderManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameproviderinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionFrameProviderManagerServiceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionFrameProviderManagerServiceStatics {
    type Vtable = IPerceptionFrameProviderManagerServiceStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae8386e6_cad9_4359_8f96_8eae51810526);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameProviderManagerServiceStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, frameproviderinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, frameproviderinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, faceauthenticationgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, faceauthenticationgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, controlgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, controlgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, correlationgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manager: ::windows::core::RawPtr, correlationgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, available: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, frame: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionPropertyChangeRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionPropertyChangeRequest {
    type Vtable = IPerceptionPropertyChangeRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c5aeb51_350b_4df8_9414_59e09815510b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionPropertyChangeRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionVideoFrameAllocator(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionVideoFrameAllocator {
    type Vtable = IPerceptionVideoFrameAllocatorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c38a7da_fdd8_4ed4_a039_2a6f9b235038);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionVideoFrameAllocatorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frame: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionVideoFrameAllocatorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPerceptionVideoFrameAllocatorFactory {
    type Vtable = IPerceptionVideoFrameAllocatorFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a58b0e1_e91a_481e_b876_a89e2bbc6b33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionVideoFrameAllocatorFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxoutstandingframecountforwrite: u32, format: super::super::super::Graphics::Imaging::BitmapPixelFormat, resolution: super::super::super::Foundation::Size, alpha: super::super::super::Graphics::Imaging::BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))] usize,
);
pub struct KnownPerceptionFrameKind {}
impl KnownPerceptionFrameKind {
    #[cfg(feature = "deprecated")]
    pub fn Color() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionFrameKindStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn Depth() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionFrameKindStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn Infrared() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionFrameKindStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IKnownPerceptionFrameKindStatics<R, F: FnOnce(&IKnownPerceptionFrameKindStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownPerceptionFrameKind, IKnownPerceptionFrameKindStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KnownPerceptionFrameKind {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.KnownPerceptionFrameKind";
}
#[repr(transparent)]
pub struct PerceptionControlGroup(::windows::core::IUnknown);
impl PerceptionControlGroup {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FrameProviderIds(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(ids: Param0) -> ::windows::core::Result<PerceptionControlGroup> {
        Self::IPerceptionControlGroupFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ids.into_param().abi(), &mut result__).from_abi::<PerceptionControlGroup>(result__)
        })
    }
    pub fn IPerceptionControlGroupFactory<R, F: FnOnce(&IPerceptionControlGroupFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PerceptionControlGroup, IPerceptionControlGroupFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PerceptionControlGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PerceptionControlGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PerceptionControlGroup {}
unsafe impl ::windows::core::RuntimeType for PerceptionControlGroup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionControlGroup;{172c4882-2fd9-4c4e-ba34-fdf20a73dde5})");
}
unsafe impl ::windows::core::Interface for PerceptionControlGroup {
    type Vtable = IPerceptionControlGroupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x172c4882_2fd9_4c4e_ba34_fdf20a73dde5);
}
impl ::windows::core::RuntimeName for PerceptionControlGroup {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionControlGroup";
}
impl ::core::convert::From<PerceptionControlGroup> for ::windows::core::IUnknown {
    fn from(value: PerceptionControlGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionControlGroup> for ::windows::core::IUnknown {
    fn from(value: &PerceptionControlGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PerceptionControlGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PerceptionControlGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PerceptionControlGroup> for ::windows::core::IInspectable {
    fn from(value: PerceptionControlGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionControlGroup> for ::windows::core::IInspectable {
    fn from(value: &PerceptionControlGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PerceptionControlGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PerceptionControlGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PerceptionControlGroup {}
unsafe impl ::core::marker::Sync for PerceptionControlGroup {}
#[repr(transparent)]
pub struct PerceptionCorrelation(::windows::core::IUnknown);
impl PerceptionCorrelation {
    #[cfg(feature = "deprecated")]
    pub fn TargetId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Quaternion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Quaternion>>(targetid: Param0, position: Param1, orientation: Param2) -> ::windows::core::Result<PerceptionCorrelation> {
        Self::IPerceptionCorrelationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), targetid.into_param().abi(), position.into_param().abi(), orientation.into_param().abi(), &mut result__).from_abi::<PerceptionCorrelation>(result__)
        })
    }
    pub fn IPerceptionCorrelationFactory<R, F: FnOnce(&IPerceptionCorrelationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PerceptionCorrelation, IPerceptionCorrelationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PerceptionCorrelation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PerceptionCorrelation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PerceptionCorrelation {}
unsafe impl ::windows::core::RuntimeType for PerceptionCorrelation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionCorrelation;{b4131a82-dff5-4047-8a19-3b4d805f7176})");
}
unsafe impl ::windows::core::Interface for PerceptionCorrelation {
    type Vtable = IPerceptionCorrelationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4131a82_dff5_4047_8a19_3b4d805f7176);
}
impl ::windows::core::RuntimeName for PerceptionCorrelation {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionCorrelation";
}
impl ::core::convert::From<PerceptionCorrelation> for ::windows::core::IUnknown {
    fn from(value: PerceptionCorrelation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionCorrelation> for ::windows::core::IUnknown {
    fn from(value: &PerceptionCorrelation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PerceptionCorrelation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PerceptionCorrelation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PerceptionCorrelation> for ::windows::core::IInspectable {
    fn from(value: PerceptionCorrelation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionCorrelation> for ::windows::core::IInspectable {
    fn from(value: &PerceptionCorrelation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PerceptionCorrelation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PerceptionCorrelation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PerceptionCorrelation {}
unsafe impl ::core::marker::Sync for PerceptionCorrelation {}
#[repr(transparent)]
pub struct PerceptionCorrelationGroup(::windows::core::IUnknown);
impl PerceptionCorrelationGroup {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RelativeLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PerceptionCorrelation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<PerceptionCorrelation>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<PerceptionCorrelation>>>(relativelocations: Param0) -> ::windows::core::Result<PerceptionCorrelationGroup> {
        Self::IPerceptionCorrelationGroupFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), relativelocations.into_param().abi(), &mut result__).from_abi::<PerceptionCorrelationGroup>(result__)
        })
    }
    pub fn IPerceptionCorrelationGroupFactory<R, F: FnOnce(&IPerceptionCorrelationGroupFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PerceptionCorrelationGroup, IPerceptionCorrelationGroupFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PerceptionCorrelationGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PerceptionCorrelationGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PerceptionCorrelationGroup {}
unsafe impl ::windows::core::RuntimeType for PerceptionCorrelationGroup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionCorrelationGroup;{752a0906-36a7-47bb-9b79-56cc6b746770})");
}
unsafe impl ::windows::core::Interface for PerceptionCorrelationGroup {
    type Vtable = IPerceptionCorrelationGroupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x752a0906_36a7_47bb_9b79_56cc6b746770);
}
impl ::windows::core::RuntimeName for PerceptionCorrelationGroup {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionCorrelationGroup";
}
impl ::core::convert::From<PerceptionCorrelationGroup> for ::windows::core::IUnknown {
    fn from(value: PerceptionCorrelationGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionCorrelationGroup> for ::windows::core::IUnknown {
    fn from(value: &PerceptionCorrelationGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PerceptionCorrelationGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PerceptionCorrelationGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PerceptionCorrelationGroup> for ::windows::core::IInspectable {
    fn from(value: PerceptionCorrelationGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionCorrelationGroup> for ::windows::core::IInspectable {
    fn from(value: &PerceptionCorrelationGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PerceptionCorrelationGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PerceptionCorrelationGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PerceptionCorrelationGroup {}
unsafe impl ::core::marker::Sync for PerceptionCorrelationGroup {}
#[repr(transparent)]
pub struct PerceptionFaceAuthenticationGroup(::windows::core::IUnknown);
impl PerceptionFaceAuthenticationGroup {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FrameProviderIds(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param1: ::windows::core::IntoParam<'a, PerceptionStartFaceAuthenticationHandler>, Param2: ::windows::core::IntoParam<'a, PerceptionStopFaceAuthenticationHandler>>(ids: Param0, starthandler: Param1, stophandler: Param2) -> ::windows::core::Result<PerceptionFaceAuthenticationGroup> {
        Self::IPerceptionFaceAuthenticationGroupFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ids.into_param().abi(), starthandler.into_param().abi(), stophandler.into_param().abi(), &mut result__).from_abi::<PerceptionFaceAuthenticationGroup>(result__)
        })
    }
    pub fn IPerceptionFaceAuthenticationGroupFactory<R, F: FnOnce(&IPerceptionFaceAuthenticationGroupFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PerceptionFaceAuthenticationGroup, IPerceptionFaceAuthenticationGroupFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PerceptionFaceAuthenticationGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PerceptionFaceAuthenticationGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PerceptionFaceAuthenticationGroup {}
unsafe impl ::windows::core::RuntimeType for PerceptionFaceAuthenticationGroup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionFaceAuthenticationGroup;{e8019814-4a91-41b0-83a6-881a1775353e})");
}
unsafe impl ::windows::core::Interface for PerceptionFaceAuthenticationGroup {
    type Vtable = IPerceptionFaceAuthenticationGroupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8019814_4a91_41b0_83a6_881a1775353e);
}
impl ::windows::core::RuntimeName for PerceptionFaceAuthenticationGroup {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionFaceAuthenticationGroup";
}
impl ::core::convert::From<PerceptionFaceAuthenticationGroup> for ::windows::core::IUnknown {
    fn from(value: PerceptionFaceAuthenticationGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionFaceAuthenticationGroup> for ::windows::core::IUnknown {
    fn from(value: &PerceptionFaceAuthenticationGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PerceptionFaceAuthenticationGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PerceptionFaceAuthenticationGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PerceptionFaceAuthenticationGroup> for ::windows::core::IInspectable {
    fn from(value: PerceptionFaceAuthenticationGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionFaceAuthenticationGroup> for ::windows::core::IInspectable {
    fn from(value: &PerceptionFaceAuthenticationGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PerceptionFaceAuthenticationGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PerceptionFaceAuthenticationGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PerceptionFaceAuthenticationGroup {}
unsafe impl ::core::marker::Sync for PerceptionFaceAuthenticationGroup {}
#[repr(transparent)]
pub struct PerceptionFrame(::windows::core::IUnknown);
impl PerceptionFrame {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SetRelativeTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn FrameData(&self) -> ::windows::core::Result<super::super::super::Foundation::IMemoryBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IMemoryBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for PerceptionFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PerceptionFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PerceptionFrame {}
unsafe impl ::windows::core::RuntimeType for PerceptionFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionFrame;{7cfe7825-54bb-4d9d-bec5-8ef66151d2ac})");
}
unsafe impl ::windows::core::Interface for PerceptionFrame {
    type Vtable = IPerceptionFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cfe7825_54bb_4d9d_bec5_8ef66151d2ac);
}
impl ::windows::core::RuntimeName for PerceptionFrame {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionFrame";
}
impl ::core::convert::From<PerceptionFrame> for ::windows::core::IUnknown {
    fn from(value: PerceptionFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionFrame> for ::windows::core::IUnknown {
    fn from(value: &PerceptionFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PerceptionFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PerceptionFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PerceptionFrame> for ::windows::core::IInspectable {
    fn from(value: PerceptionFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionFrame> for ::windows::core::IInspectable {
    fn from(value: &PerceptionFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PerceptionFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PerceptionFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PerceptionFrame {}
unsafe impl ::core::marker::Sync for PerceptionFrame {}
#[repr(transparent)]
pub struct PerceptionFrameProviderInfo(::windows::core::IUnknown);
impl PerceptionFrameProviderInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PerceptionFrameProviderInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn DeviceKind(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetDeviceKind<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn FrameKind(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetFrameKind<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Hidden(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetHidden(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for PerceptionFrameProviderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PerceptionFrameProviderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PerceptionFrameProviderInfo {}
unsafe impl ::windows::core::RuntimeType for PerceptionFrameProviderInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionFrameProviderInfo;{cca959e8-797e-4e83-9b87-036a74142fc4})");
}
unsafe impl ::windows::core::Interface for PerceptionFrameProviderInfo {
    type Vtable = IPerceptionFrameProviderInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcca959e8_797e_4e83_9b87_036a74142fc4);
}
impl ::windows::core::RuntimeName for PerceptionFrameProviderInfo {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionFrameProviderInfo";
}
impl ::core::convert::From<PerceptionFrameProviderInfo> for ::windows::core::IUnknown {
    fn from(value: PerceptionFrameProviderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionFrameProviderInfo> for ::windows::core::IUnknown {
    fn from(value: &PerceptionFrameProviderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PerceptionFrameProviderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PerceptionFrameProviderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PerceptionFrameProviderInfo> for ::windows::core::IInspectable {
    fn from(value: PerceptionFrameProviderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionFrameProviderInfo> for ::windows::core::IInspectable {
    fn from(value: &PerceptionFrameProviderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PerceptionFrameProviderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PerceptionFrameProviderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PerceptionFrameProviderInfo {}
unsafe impl ::core::marker::Sync for PerceptionFrameProviderInfo {}
pub struct PerceptionFrameProviderManagerService {}
impl PerceptionFrameProviderManagerService {
    #[cfg(feature = "deprecated")]
    pub fn RegisterFrameProviderInfo<'a, Param0: ::windows::core::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows::core::IntoParam<'a, PerceptionFrameProviderInfo>>(manager: Param0, frameproviderinfo: Param1) -> ::windows::core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), manager.into_param().abi(), frameproviderinfo.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn UnregisterFrameProviderInfo<'a, Param0: ::windows::core::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows::core::IntoParam<'a, PerceptionFrameProviderInfo>>(manager: Param0, frameproviderinfo: Param1) -> ::windows::core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), manager.into_param().abi(), frameproviderinfo.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn RegisterFaceAuthenticationGroup<'a, Param0: ::windows::core::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows::core::IntoParam<'a, PerceptionFaceAuthenticationGroup>>(manager: Param0, faceauthenticationgroup: Param1) -> ::windows::core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), manager.into_param().abi(), faceauthenticationgroup.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn UnregisterFaceAuthenticationGroup<'a, Param0: ::windows::core::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows::core::IntoParam<'a, PerceptionFaceAuthenticationGroup>>(manager: Param0, faceauthenticationgroup: Param1) -> ::windows::core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), manager.into_param().abi(), faceauthenticationgroup.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn RegisterControlGroup<'a, Param0: ::windows::core::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows::core::IntoParam<'a, PerceptionControlGroup>>(manager: Param0, controlgroup: Param1) -> ::windows::core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), manager.into_param().abi(), controlgroup.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn UnregisterControlGroup<'a, Param0: ::windows::core::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows::core::IntoParam<'a, PerceptionControlGroup>>(manager: Param0, controlgroup: Param1) -> ::windows::core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), manager.into_param().abi(), controlgroup.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn RegisterCorrelationGroup<'a, Param0: ::windows::core::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows::core::IntoParam<'a, PerceptionCorrelationGroup>>(manager: Param0, correlationgroup: Param1) -> ::windows::core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), manager.into_param().abi(), correlationgroup.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn UnregisterCorrelationGroup<'a, Param0: ::windows::core::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows::core::IntoParam<'a, PerceptionCorrelationGroup>>(manager: Param0, correlationgroup: Param1) -> ::windows::core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), manager.into_param().abi(), correlationgroup.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn UpdateAvailabilityForProvider<'a, Param0: ::windows::core::IntoParam<'a, IPerceptionFrameProvider>>(provider: Param0, available: bool) -> ::windows::core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), provider.into_param().abi(), available).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn PublishFrameForProvider<'a, Param0: ::windows::core::IntoParam<'a, IPerceptionFrameProvider>, Param1: ::windows::core::IntoParam<'a, PerceptionFrame>>(provider: Param0, frame: Param1) -> ::windows::core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), provider.into_param().abi(), frame.into_param().abi()).ok() })
    }
    pub fn IPerceptionFrameProviderManagerServiceStatics<R, F: FnOnce(&IPerceptionFrameProviderManagerServiceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PerceptionFrameProviderManagerService, IPerceptionFrameProviderManagerServiceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PerceptionFrameProviderManagerService {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionFrameProviderManagerService";
}
#[repr(transparent)]
pub struct PerceptionPropertyChangeRequest(::windows::core::IUnknown);
impl PerceptionPropertyChangeRequest {
    #[cfg(feature = "deprecated")]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> ::windows::core::Result<super::PerceptionFrameSourcePropertyChangeStatus> {
        let this = self;
        unsafe {
            let mut result__: super::PerceptionFrameSourcePropertyChangeStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::PerceptionFrameSourcePropertyChangeStatus>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetStatus(&self, value: super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PerceptionPropertyChangeRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PerceptionPropertyChangeRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PerceptionPropertyChangeRequest {}
unsafe impl ::windows::core::RuntimeType for PerceptionPropertyChangeRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionPropertyChangeRequest;{3c5aeb51-350b-4df8-9414-59e09815510b})");
}
unsafe impl ::windows::core::Interface for PerceptionPropertyChangeRequest {
    type Vtable = IPerceptionPropertyChangeRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c5aeb51_350b_4df8_9414_59e09815510b);
}
impl ::windows::core::RuntimeName for PerceptionPropertyChangeRequest {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionPropertyChangeRequest";
}
impl ::core::convert::From<PerceptionPropertyChangeRequest> for ::windows::core::IUnknown {
    fn from(value: PerceptionPropertyChangeRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionPropertyChangeRequest> for ::windows::core::IUnknown {
    fn from(value: &PerceptionPropertyChangeRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PerceptionPropertyChangeRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PerceptionPropertyChangeRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PerceptionPropertyChangeRequest> for ::windows::core::IInspectable {
    fn from(value: PerceptionPropertyChangeRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionPropertyChangeRequest> for ::windows::core::IInspectable {
    fn from(value: &PerceptionPropertyChangeRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PerceptionPropertyChangeRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PerceptionPropertyChangeRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PerceptionPropertyChangeRequest {}
unsafe impl ::core::marker::Sync for PerceptionPropertyChangeRequest {}
#[repr(transparent)]
pub struct PerceptionStartFaceAuthenticationHandler(pub ::windows::core::IUnknown);
impl PerceptionStartFaceAuthenticationHandler {
    pub fn new<F: FnMut(&::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::core::Result<bool> + 'static>(invoke: F) -> Self {
        let com = PerceptionStartFaceAuthenticationHandlerBox::<F> { vtable: &PerceptionStartFaceAuthenticationHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "deprecated")]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, PerceptionFaceAuthenticationGroup>>(&self, sender: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
#[repr(C)]
struct PerceptionStartFaceAuthenticationHandlerBox<F: FnMut(&::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::core::Result<bool> + 'static> {
    vtable: *const PerceptionStartFaceAuthenticationHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::core::Result<bool> + 'static> PerceptionStartFaceAuthenticationHandlerBox<F> {
    const VTABLE: PerceptionStartFaceAuthenticationHandlerVtbl = PerceptionStartFaceAuthenticationHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<PerceptionStartFaceAuthenticationHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        match ((*this).invoke)(&*(&sender as *const <PerceptionFaceAuthenticationGroup as ::windows::core::Abi>::Abi as *const <PerceptionFaceAuthenticationGroup as ::windows::core::DefaultType>::DefaultType)) {
            ::core::result::Result::Ok(ok__) => {
                *result__ = ::core::mem::transmute_copy(&ok__);
                ::core::mem::forget(ok__);
                ::windows::core::HRESULT(0)
            }
            ::core::result::Result::Err(err) => err.into(),
        }
    }
}
impl ::core::clone::Clone for PerceptionStartFaceAuthenticationHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PerceptionStartFaceAuthenticationHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PerceptionStartFaceAuthenticationHandler {}
unsafe impl ::windows::core::Interface for PerceptionStartFaceAuthenticationHandler {
    type Vtable = PerceptionStartFaceAuthenticationHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74816d2a_2090_4670_8c48_ef39e7ff7c26);
}
unsafe impl ::windows::core::RuntimeType for PerceptionStartFaceAuthenticationHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{74816d2a-2090-4670-8c48-ef39e7ff7c26}");
}
#[repr(C)]
#[doc(hidden)]
pub struct PerceptionStartFaceAuthenticationHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct PerceptionStopFaceAuthenticationHandler(pub ::windows::core::IUnknown);
impl PerceptionStopFaceAuthenticationHandler {
    pub fn new<F: FnMut(&::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = PerceptionStopFaceAuthenticationHandlerBox::<F> { vtable: &PerceptionStopFaceAuthenticationHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "deprecated")]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, PerceptionFaceAuthenticationGroup>>(&self, sender: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct PerceptionStopFaceAuthenticationHandlerBox<F: FnMut(&::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const PerceptionStopFaceAuthenticationHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::core::Result<()> + 'static> PerceptionStopFaceAuthenticationHandlerBox<F> {
    const VTABLE: PerceptionStopFaceAuthenticationHandlerVtbl = PerceptionStopFaceAuthenticationHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<PerceptionStopFaceAuthenticationHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <PerceptionFaceAuthenticationGroup as ::windows::core::Abi>::Abi as *const <PerceptionFaceAuthenticationGroup as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
impl ::core::clone::Clone for PerceptionStopFaceAuthenticationHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PerceptionStopFaceAuthenticationHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PerceptionStopFaceAuthenticationHandler {}
unsafe impl ::windows::core::Interface for PerceptionStopFaceAuthenticationHandler {
    type Vtable = PerceptionStopFaceAuthenticationHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x387ee6aa_89cd_481e_aade_dd92f70b2ad7);
}
unsafe impl ::windows::core::RuntimeType for PerceptionStopFaceAuthenticationHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{387ee6aa-89cd-481e-aade-dd92f70b2ad7}");
}
#[repr(C)]
#[doc(hidden)]
pub struct PerceptionStopFaceAuthenticationHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct PerceptionVideoFrameAllocator(::windows::core::IUnknown);
impl PerceptionVideoFrameAllocator {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn AllocateFrame(&self) -> ::windows::core::Result<PerceptionFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PerceptionFrame>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Media")]
    pub fn CopyFromVideoFrame<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Media::VideoFrame>>(&self, frame: Param0) -> ::windows::core::Result<PerceptionFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), frame.into_param().abi(), &mut result__).from_abi::<PerceptionFrame>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn Create<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(maxoutstandingframecountforwrite: u32, format: super::super::super::Graphics::Imaging::BitmapPixelFormat, resolution: Param2, alpha: super::super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::Result<PerceptionVideoFrameAllocator> {
        Self::IPerceptionVideoFrameAllocatorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), maxoutstandingframecountforwrite, format, resolution.into_param().abi(), alpha, &mut result__).from_abi::<PerceptionVideoFrameAllocator>(result__)
        })
    }
    pub fn IPerceptionVideoFrameAllocatorFactory<R, F: FnOnce(&IPerceptionVideoFrameAllocatorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PerceptionVideoFrameAllocator, IPerceptionVideoFrameAllocatorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PerceptionVideoFrameAllocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PerceptionVideoFrameAllocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PerceptionVideoFrameAllocator {}
unsafe impl ::windows::core::RuntimeType for PerceptionVideoFrameAllocator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionVideoFrameAllocator;{4c38a7da-fdd8-4ed4-a039-2a6f9b235038})");
}
unsafe impl ::windows::core::Interface for PerceptionVideoFrameAllocator {
    type Vtable = IPerceptionVideoFrameAllocatorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c38a7da_fdd8_4ed4_a039_2a6f9b235038);
}
impl ::windows::core::RuntimeName for PerceptionVideoFrameAllocator {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionVideoFrameAllocator";
}
impl ::core::convert::From<PerceptionVideoFrameAllocator> for ::windows::core::IUnknown {
    fn from(value: PerceptionVideoFrameAllocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionVideoFrameAllocator> for ::windows::core::IUnknown {
    fn from(value: &PerceptionVideoFrameAllocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PerceptionVideoFrameAllocator> for ::windows::core::IInspectable {
    fn from(value: PerceptionVideoFrameAllocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionVideoFrameAllocator> for ::windows::core::IInspectable {
    fn from(value: &PerceptionVideoFrameAllocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<PerceptionVideoFrameAllocator> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PerceptionVideoFrameAllocator) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PerceptionVideoFrameAllocator> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PerceptionVideoFrameAllocator) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PerceptionVideoFrameAllocator {}
unsafe impl ::core::marker::Sync for PerceptionVideoFrameAllocator {}
