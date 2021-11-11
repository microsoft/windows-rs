#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct IComponentConnector(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IComponentConnector {
    type Vtable = IComponentConnector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf6790987_e6e5_47f2_92c6_eccce4ba159a);
}
impl IComponentConnector {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn Connect<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, connectionid: i32, target: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), connectionid, target.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IComponentConnector {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{f6790987-e6e5-47f2-92c6-eccce4ba159a}");
}
impl ::core::convert::From<IComponentConnector> for ::windows::runtime::IUnknown {
    fn from(value: IComponentConnector) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IComponentConnector> for ::windows::runtime::IUnknown {
    fn from(value: &IComponentConnector) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComponentConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComponentConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IComponentConnector> for ::windows::runtime::IInspectable {
    fn from(value: IComponentConnector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComponentConnector> for ::windows::runtime::IInspectable {
    fn from(value: &IComponentConnector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IComponentConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IComponentConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentConnector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectionid: i32, target: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct IComponentConnector2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IComponentConnector2 {
    type Vtable = IComponentConnector2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdc8f368b_eccc_498e_b139_91142254d7ae);
}
impl IComponentConnector2 {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn GetBindingConnector<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, connectionid: i32, target: Param1) -> ::windows::runtime::Result<IComponentConnector> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), connectionid, target.into_param().abi(), &mut result__).from_abi::<IComponentConnector>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IComponentConnector2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{dc8f368b-eccc-498e-b139-91142254d7ae}");
}
impl ::core::convert::From<IComponentConnector2> for ::windows::runtime::IUnknown {
    fn from(value: IComponentConnector2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IComponentConnector2> for ::windows::runtime::IUnknown {
    fn from(value: &IComponentConnector2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComponentConnector2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComponentConnector2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IComponentConnector2> for ::windows::runtime::IInspectable {
    fn from(value: IComponentConnector2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComponentConnector2> for ::windows::runtime::IInspectable {
    fn from(value: &IComponentConnector2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IComponentConnector2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IComponentConnector2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentConnector2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectionid: i32, target: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct IDataTemplateComponent(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataTemplateComponent {
    type Vtable = IDataTemplateComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x08429dc8_8ab0_4747_aa9a_feadfc8da8e1);
}
impl IDataTemplateComponent {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn Recycle(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn ProcessBindings<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, item: Param0, itemindex: i32, phase: i32, nextphase: &mut i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), item.into_param().abi(), itemindex, phase, nextphase).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IDataTemplateComponent {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{08429dc8-8ab0-4747-aa9a-feadfc8da8e1}");
}
impl ::core::convert::From<IDataTemplateComponent> for ::windows::runtime::IUnknown {
    fn from(value: IDataTemplateComponent) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IDataTemplateComponent> for ::windows::runtime::IUnknown {
    fn from(value: &IDataTemplateComponent) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDataTemplateComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDataTemplateComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IDataTemplateComponent> for ::windows::runtime::IInspectable {
    fn from(value: IDataTemplateComponent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDataTemplateComponent> for ::windows::runtime::IInspectable {
    fn from(value: &IDataTemplateComponent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IDataTemplateComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IDataTemplateComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTemplateComponent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, item: ::windows::runtime::RawPtr, itemindex: i32, phase: i32, nextphase: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMarkupExtension(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMarkupExtension {
    type Vtable = IMarkupExtension_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1ee3416d_562b_486e_9ee5_0f0cbcc8048c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtension_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMarkupExtensionFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMarkupExtensionFactory {
    type Vtable = IMarkupExtensionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x65329c05_fb5a_4567_9d55_5cdfbada2739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtensionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, baseinterface: ::windows::runtime::RawPtr, innerinterface: *mut ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMarkupExtensionOverrides(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMarkupExtensionOverrides {
    type Vtable = IMarkupExtensionOverrides_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x393779bf_b9c0_4ffb_a57f_58e7356e425f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtensionOverrides_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlBinaryWriter(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x829d2ad3_620a_46f6_845d_436a05927100);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBinaryWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlBinaryWriterStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlBinaryWriterStatics {
    type Vtable = IXamlBinaryWriterStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0d8ed07a_9b82_4aa8_b68b_026f2de1cc86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBinaryWriterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstreams: ::windows::runtime::RawPtr, outputstreams: ::windows::runtime::RawPtr, xamlmetadataprovider: ::windows::runtime::RawPtr, result__: *mut XamlBinaryWriterErrorInformation) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct IXamlBindScopeDiagnostics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlBindScopeDiagnostics {
    type Vtable = IXamlBindScopeDiagnostics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf264a29d_bded_43aa_a5b0_26ac21a81eb8);
}
impl IXamlBindScopeDiagnostics {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn Disable(&self, linenumber: i32, columnnumber: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), linenumber, columnnumber).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXamlBindScopeDiagnostics {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{f264a29d-bded-43aa-a5b0-26ac21a81eb8}");
}
impl ::core::convert::From<IXamlBindScopeDiagnostics> for ::windows::runtime::IUnknown {
    fn from(value: IXamlBindScopeDiagnostics) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlBindScopeDiagnostics> for ::windows::runtime::IUnknown {
    fn from(value: &IXamlBindScopeDiagnostics) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXamlBindScopeDiagnostics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXamlBindScopeDiagnostics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlBindScopeDiagnostics> for ::windows::runtime::IInspectable {
    fn from(value: IXamlBindScopeDiagnostics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlBindScopeDiagnostics> for ::windows::runtime::IInspectable {
    fn from(value: &IXamlBindScopeDiagnostics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IXamlBindScopeDiagnostics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IXamlBindScopeDiagnostics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindScopeDiagnostics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, linenumber: i32, columnnumber: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlBindingHelper(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlBindingHelper {
    type Vtable = IXamlBindingHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfaa6fb06_8ab9_4ef7_8ae7_fbd30bbfd06d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindingHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlBindingHelperStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlBindingHelperStatics {
    type Vtable = IXamlBindingHelperStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf65cfb71_c80c_4ffa_86ee_558754ee336d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindingHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, element: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, element: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, value: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlMarkupHelper(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd0e6673c_5342_44ef_85a7_ed327a739d9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMarkupHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlMarkupHelperStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlMarkupHelperStatics {
    type Vtable = IXamlMarkupHelperStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc9bc3725_f34f_445c_81a2_6b72a5e8f072);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMarkupHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, element: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct IXamlMember(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlMember {
    type Vtable = IXamlMember_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc541f58c_43a9_4216_b718_e0b11b14e93e);
}
impl IXamlMember {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsAttachable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsDependencyProperty(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsReadOnly(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn TargetType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, instance: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), instance.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, instance: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), instance.into_param().abi(), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXamlMember {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{c541f58c-43a9-4216-b718-e0b11b14e93e}");
}
impl ::core::convert::From<IXamlMember> for ::windows::runtime::IUnknown {
    fn from(value: IXamlMember) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlMember> for ::windows::runtime::IUnknown {
    fn from(value: &IXamlMember) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXamlMember {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXamlMember {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlMember> for ::windows::runtime::IInspectable {
    fn from(value: IXamlMember) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlMember> for ::windows::runtime::IInspectable {
    fn from(value: &IXamlMember) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IXamlMember {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IXamlMember {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMember_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instance: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instance: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct IXamlMetadataProvider(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlMetadataProvider {
    type Vtable = IXamlMetadataProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb3765d69_68a5_4b32_8861_fdb90c1f5836);
}
impl IXamlMetadataProvider {
    #[cfg(feature = "UI_Xaml_Interop")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `UI_Xaml_Interop`*"]
    pub fn GetXamlType<'a, Param0: ::windows::runtime::IntoParam<'a, super::Interop::TypeName>>(&self, r#type: Param0) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), r#type.into_param().abi(), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn GetXamlTypeByFullName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, fullname: Param0) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), fullname.into_param().abi(), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn GetXmlnsDefinitions(&self) -> ::windows::runtime::Result<::windows::runtime::Array<XmlnsDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<XmlnsDefinition> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), ::windows::runtime::Array::<XmlnsDefinition>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXamlMetadataProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{b3765d69-68a5-4b32-8861-fdb90c1f5836}");
}
impl ::core::convert::From<IXamlMetadataProvider> for ::windows::runtime::IUnknown {
    fn from(value: IXamlMetadataProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlMetadataProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IXamlMetadataProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXamlMetadataProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXamlMetadataProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlMetadataProvider> for ::windows::runtime::IInspectable {
    fn from(value: IXamlMetadataProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlMetadataProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IXamlMetadataProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IXamlMetadataProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IXamlMetadataProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMetadataProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fullname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<XmlnsDefinition>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlReader(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlReader {
    type Vtable = IXamlReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x24374cf1_cceb_48bf_a514_41b0186f84c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlReaderStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlReaderStatics {
    type Vtable = IXamlReaderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9891c6bd_534f_4955_b85a_8a8dc0dca602);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReaderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xaml: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xaml: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct IXamlType(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlType {
    type Vtable = IXamlType_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7920eab1_a2e5_479a_bd50_6cef3c0b4970);
}
impl IXamlType {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn BaseType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn ContentProperty(&self) -> ::windows::runtime::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlMember>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn FullName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsArray(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsCollection(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsConstructible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsDictionary(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsMarkupExtension(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsBindable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn ItemType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn KeyType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Interop")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `UI_Xaml_Interop`*"]
    pub fn UnderlyingType(&self) -> ::windows::runtime::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<super::Interop::TypeName> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn ActivateInstance(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn CreateFromString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn GetMember<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<IXamlMember>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn AddToVector<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, instance: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), instance.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn AddToMap<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, instance: Param0, key: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), instance.into_param().abi(), key.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn RunInitializer(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXamlType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{7920eab1-a2e5-479a-bd50-6cef3c0b4970}");
}
impl ::core::convert::From<IXamlType> for ::windows::runtime::IUnknown {
    fn from(value: IXamlType) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlType> for ::windows::runtime::IUnknown {
    fn from(value: &IXamlType) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXamlType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXamlType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlType> for ::windows::runtime::IInspectable {
    fn from(value: IXamlType) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlType> for ::windows::runtime::IInspectable {
    fn from(value: &IXamlType) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IXamlType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IXamlType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlType_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instance: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instance: ::windows::runtime::RawPtr, key: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct IXamlType2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlType2 {
    type Vtable = IXamlType2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9f0c6e3b_433b_56ad_8f69_78a4dd3e64f9);
}
impl IXamlType2 {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn BaseType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn ContentProperty(&self) -> ::windows::runtime::Result<IXamlMember> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlMember>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn FullName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsArray(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsCollection(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsConstructible(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsDictionary(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsMarkupExtension(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsBindable(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn ItemType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn KeyType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Interop")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `UI_Xaml_Interop`*"]
    pub fn UnderlyingType(&self) -> ::windows::runtime::Result<super::Interop::TypeName> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<super::Interop::TypeName> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn ActivateInstance(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn CreateFromString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn GetMember<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<IXamlMember> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<IXamlMember>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn AddToVector<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, instance: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), instance.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn AddToMap<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, instance: Param0, key: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), instance.into_param().abi(), key.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn RunInitializer(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn BoxedType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXamlType2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{9f0c6e3b-433b-56ad-8f69-78a4dd3e64f9}");
}
impl ::core::convert::From<IXamlType2> for ::windows::runtime::IUnknown {
    fn from(value: IXamlType2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlType2> for ::windows::runtime::IUnknown {
    fn from(value: &IXamlType2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXamlType2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXamlType2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlType2> for ::windows::runtime::IInspectable {
    fn from(value: IXamlType2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlType2> for ::windows::runtime::IInspectable {
    fn from(value: &IXamlType2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IXamlType2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IXamlType2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IXamlType2> for IXamlType {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IXamlType2) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXamlType2> for IXamlType {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IXamlType2) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXamlType> for IXamlType2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXamlType> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXamlType> for &IXamlType2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXamlType> {
        ::core::convert::TryInto::<IXamlType>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlType2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Xaml_Markup`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MarkupExtension(pub ::windows::runtime::IInspectable);
impl MarkupExtension {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn new() -> ::windows::runtime::Result<MarkupExtension> {
        Self::IMarkupExtensionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<MarkupExtension>(result__)
        })
    }
    pub fn IMarkupExtensionFactory<R, F: FnOnce(&IMarkupExtensionFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MarkupExtension, IMarkupExtensionFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MarkupExtension {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Markup.MarkupExtension;{1ee3416d-562b-486e-9ee5-0f0cbcc8048c})");
}
unsafe impl ::windows::runtime::Interface for MarkupExtension {
    type Vtable = IMarkupExtension_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1ee3416d_562b_486e_9ee5_0f0cbcc8048c);
}
impl ::windows::runtime::RuntimeName for MarkupExtension {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.MarkupExtension";
}
impl ::core::convert::From<MarkupExtension> for ::windows::runtime::IUnknown {
    fn from(value: MarkupExtension) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MarkupExtension> for ::windows::runtime::IUnknown {
    fn from(value: &MarkupExtension) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MarkupExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MarkupExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MarkupExtension> for ::windows::runtime::IInspectable {
    fn from(value: MarkupExtension) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MarkupExtension> for ::windows::runtime::IInspectable {
    fn from(value: &MarkupExtension) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MarkupExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MarkupExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MarkupExtension {}
unsafe impl ::core::marker::Sync for MarkupExtension {}
#[doc = "*Required features: `UI_Xaml_Markup`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XamlBinaryWriter(pub ::windows::runtime::IInspectable);
impl XamlBinaryWriter {
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Xaml_Markup`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn Write<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream>>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream>>, Param2: ::windows::runtime::IntoParam<'a, IXamlMetadataProvider>>(
        inputstreams: Param0,
        outputstreams: Param1,
        xamlmetadataprovider: Param2,
    ) -> ::windows::runtime::Result<XamlBinaryWriterErrorInformation> {
        Self::IXamlBinaryWriterStatics(|this| unsafe {
            let mut result__: XamlBinaryWriterErrorInformation = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inputstreams.into_param().abi(), outputstreams.into_param().abi(), xamlmetadataprovider.into_param().abi(), &mut result__).from_abi::<XamlBinaryWriterErrorInformation>(result__)
        })
    }
    pub fn IXamlBinaryWriterStatics<R, F: FnOnce(&IXamlBinaryWriterStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<XamlBinaryWriter, IXamlBinaryWriterStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XamlBinaryWriter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Markup.XamlBinaryWriter;{829d2ad3-620a-46f6-845d-436a05927100})");
}
unsafe impl ::windows::runtime::Interface for XamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x829d2ad3_620a_46f6_845d_436a05927100);
}
impl ::windows::runtime::RuntimeName for XamlBinaryWriter {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlBinaryWriter";
}
impl ::core::convert::From<XamlBinaryWriter> for ::windows::runtime::IUnknown {
    fn from(value: XamlBinaryWriter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlBinaryWriter> for ::windows::runtime::IUnknown {
    fn from(value: &XamlBinaryWriter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XamlBinaryWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a XamlBinaryWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlBinaryWriter> for ::windows::runtime::IInspectable {
    fn from(value: XamlBinaryWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlBinaryWriter> for ::windows::runtime::IInspectable {
    fn from(value: &XamlBinaryWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XamlBinaryWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XamlBinaryWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlBinaryWriter {}
unsafe impl ::core::marker::Sync for XamlBinaryWriter {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct XamlBinaryWriterErrorInformation {
    pub InputStreamIndex: u32,
    pub LineNumber: u32,
    pub LinePosition: u32,
}
impl XamlBinaryWriterErrorInformation {}
impl ::core::default::Default for XamlBinaryWriterErrorInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XamlBinaryWriterErrorInformation {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XamlBinaryWriterErrorInformation").field("InputStreamIndex", &self.InputStreamIndex).field("LineNumber", &self.LineNumber).field("LinePosition", &self.LinePosition).finish()
    }
}
impl ::core::cmp::PartialEq for XamlBinaryWriterErrorInformation {
    fn eq(&self, other: &Self) -> bool {
        self.InputStreamIndex == other.InputStreamIndex && self.LineNumber == other.LineNumber && self.LinePosition == other.LinePosition
    }
}
impl ::core::cmp::Eq for XamlBinaryWriterErrorInformation {}
unsafe impl ::windows::runtime::Abi for XamlBinaryWriterErrorInformation {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XamlBinaryWriterErrorInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Markup.XamlBinaryWriterErrorInformation;u4;u4;u4)");
}
impl ::windows::runtime::DefaultType for XamlBinaryWriterErrorInformation {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Markup`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XamlBindingHelper(pub ::windows::runtime::IInspectable);
impl XamlBindingHelper {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn DataTemplateComponentProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn GetDataTemplateComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::runtime::Result<IDataTemplateComponent> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<IDataTemplateComponent>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetDataTemplateComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>, Param1: ::windows::runtime::IntoParam<'a, IDataTemplateComponent>>(element: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SuspendRendering<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(target: Param0) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), target.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn ResumeRendering<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(target: Param0) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), target.into_param().abi()).ok() })
    }
    #[cfg(feature = "UI_Xaml_Interop")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `UI_Xaml_Interop`*"]
    pub fn ConvertValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::Interop::TypeName>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(r#type: Param0, value: Param1) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), r#type.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromBoolean<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: bool) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromChar16<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: u16) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `Foundation`*"]
    pub fn SetPropertyFromDateTime<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::DateTime>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromDouble<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: f64) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromInt32<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: i32) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromUInt32<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: u32) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromInt64<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: i64) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromUInt64<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: u64) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromSingle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: f32) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `Foundation`*"]
    pub fn SetPropertyFromPoint<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `Foundation`*"]
    pub fn SetPropertyFromRect<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Rect>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `Foundation`*"]
    pub fn SetPropertyFromSize<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Size>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `Foundation`*"]
    pub fn SetPropertyFromTimeSpan<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromByte<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: u8) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `Foundation`*"]
    pub fn SetPropertyFromUri<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromObject<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).28)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn IXamlBindingHelperStatics<R, F: FnOnce(&IXamlBindingHelperStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<XamlBindingHelper, IXamlBindingHelperStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XamlBindingHelper {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Markup.XamlBindingHelper;{faa6fb06-8ab9-4ef7-8ae7-fbd30bbfd06d})");
}
unsafe impl ::windows::runtime::Interface for XamlBindingHelper {
    type Vtable = IXamlBindingHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfaa6fb06_8ab9_4ef7_8ae7_fbd30bbfd06d);
}
impl ::windows::runtime::RuntimeName for XamlBindingHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlBindingHelper";
}
impl ::core::convert::From<XamlBindingHelper> for ::windows::runtime::IUnknown {
    fn from(value: XamlBindingHelper) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlBindingHelper> for ::windows::runtime::IUnknown {
    fn from(value: &XamlBindingHelper) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XamlBindingHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a XamlBindingHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlBindingHelper> for ::windows::runtime::IInspectable {
    fn from(value: XamlBindingHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlBindingHelper> for ::windows::runtime::IInspectable {
    fn from(value: &XamlBindingHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XamlBindingHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XamlBindingHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlBindingHelper {}
unsafe impl ::core::marker::Sync for XamlBindingHelper {}
#[doc = "*Required features: `UI_Xaml_Markup`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XamlMarkupHelper(pub ::windows::runtime::IInspectable);
impl XamlMarkupHelper {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn UnloadObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::runtime::Result<()> {
        Self::IXamlMarkupHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), element.into_param().abi()).ok() })
    }
    pub fn IXamlMarkupHelperStatics<R, F: FnOnce(&IXamlMarkupHelperStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<XamlMarkupHelper, IXamlMarkupHelperStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XamlMarkupHelper {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Markup.XamlMarkupHelper;{d0e6673c-5342-44ef-85a7-ed327a739d9a})");
}
unsafe impl ::windows::runtime::Interface for XamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd0e6673c_5342_44ef_85a7_ed327a739d9a);
}
impl ::windows::runtime::RuntimeName for XamlMarkupHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlMarkupHelper";
}
impl ::core::convert::From<XamlMarkupHelper> for ::windows::runtime::IUnknown {
    fn from(value: XamlMarkupHelper) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlMarkupHelper> for ::windows::runtime::IUnknown {
    fn from(value: &XamlMarkupHelper) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XamlMarkupHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a XamlMarkupHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlMarkupHelper> for ::windows::runtime::IInspectable {
    fn from(value: XamlMarkupHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlMarkupHelper> for ::windows::runtime::IInspectable {
    fn from(value: &XamlMarkupHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XamlMarkupHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XamlMarkupHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlMarkupHelper {}
unsafe impl ::core::marker::Sync for XamlMarkupHelper {}
#[doc = "*Required features: `UI_Xaml_Markup`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XamlReader(pub ::windows::runtime::IInspectable);
impl XamlReader {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(xaml: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xaml.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn LoadWithInitialTemplateValidation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(xaml: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xaml.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    pub fn IXamlReaderStatics<R, F: FnOnce(&IXamlReaderStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<XamlReader, IXamlReaderStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XamlReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Markup.XamlReader;{24374cf1-cceb-48bf-a514-41b0186f84c2})");
}
unsafe impl ::windows::runtime::Interface for XamlReader {
    type Vtable = IXamlReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x24374cf1_cceb_48bf_a514_41b0186f84c2);
}
impl ::windows::runtime::RuntimeName for XamlReader {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlReader";
}
impl ::core::convert::From<XamlReader> for ::windows::runtime::IUnknown {
    fn from(value: XamlReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlReader> for ::windows::runtime::IUnknown {
    fn from(value: &XamlReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XamlReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a XamlReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlReader> for ::windows::runtime::IInspectable {
    fn from(value: XamlReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlReader> for ::windows::runtime::IInspectable {
    fn from(value: &XamlReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XamlReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XamlReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlReader {}
unsafe impl ::core::marker::Sync for XamlReader {}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct XmlnsDefinition {
    pub XmlNamespace: ::windows::runtime::HSTRING,
    pub Namespace: ::windows::runtime::HSTRING,
}
impl XmlnsDefinition {}
impl ::core::default::Default for XmlnsDefinition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XmlnsDefinition {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XmlnsDefinition").field("XmlNamespace", &self.XmlNamespace).field("Namespace", &self.Namespace).finish()
    }
}
impl ::core::cmp::PartialEq for XmlnsDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.XmlNamespace == other.XmlNamespace && self.Namespace == other.Namespace
    }
}
impl ::core::cmp::Eq for XmlnsDefinition {}
unsafe impl ::windows::runtime::Abi for XmlnsDefinition {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows::runtime::RuntimeType for XmlnsDefinition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Markup.XmlnsDefinition;string;string)");
}
impl ::windows::runtime::DefaultType for XmlnsDefinition {
    type DefaultType = Self;
}
