#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComponentConnector(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IComponentConnector {
    type Vtable = IComponentConnector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6790987_e6e5_47f2_92c6_eccce4ba159a);
}
impl IComponentConnector {
    pub fn Connect<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, connectionid: i32, target: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), connectionid, target.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IComponentConnector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f6790987-e6e5-47f2-92c6-eccce4ba159a}");
}
impl ::core::convert::From<IComponentConnector> for ::windows::core::IUnknown {
    fn from(value: IComponentConnector) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IComponentConnector> for ::windows::core::IUnknown {
    fn from(value: &IComponentConnector) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IComponentConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IComponentConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IComponentConnector> for ::windows::core::IInspectable {
    fn from(value: IComponentConnector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComponentConnector> for ::windows::core::IInspectable {
    fn from(value: &IComponentConnector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IComponentConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IComponentConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentConnector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, connectionid: i32, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComponentConnector2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IComponentConnector2 {
    type Vtable = IComponentConnector2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc8f368b_eccc_498e_b139_91142254d7ae);
}
impl IComponentConnector2 {
    pub fn GetBindingConnector<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, connectionid: i32, target: Param1) -> ::windows::core::Result<IComponentConnector> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), connectionid, target.into_param().abi(), &mut result__).from_abi::<IComponentConnector>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IComponentConnector2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{dc8f368b-eccc-498e-b139-91142254d7ae}");
}
impl ::core::convert::From<IComponentConnector2> for ::windows::core::IUnknown {
    fn from(value: IComponentConnector2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IComponentConnector2> for ::windows::core::IUnknown {
    fn from(value: &IComponentConnector2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IComponentConnector2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IComponentConnector2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IComponentConnector2> for ::windows::core::IInspectable {
    fn from(value: IComponentConnector2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComponentConnector2> for ::windows::core::IInspectable {
    fn from(value: &IComponentConnector2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IComponentConnector2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IComponentConnector2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentConnector2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, connectionid: i32, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDataTemplateComponent(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDataTemplateComponent {
    type Vtable = IDataTemplateComponent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08429dc8_8ab0_4747_aa9a_feadfc8da8e1);
}
impl IDataTemplateComponent {
    pub fn Recycle(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn ProcessBindings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, item: Param0, itemindex: i32, phase: i32, nextphase: &mut i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), item.into_param().abi(), itemindex, phase, nextphase).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IDataTemplateComponent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{08429dc8-8ab0-4747-aa9a-feadfc8da8e1}");
}
impl ::core::convert::From<IDataTemplateComponent> for ::windows::core::IUnknown {
    fn from(value: IDataTemplateComponent) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IDataTemplateComponent> for ::windows::core::IUnknown {
    fn from(value: &IDataTemplateComponent) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataTemplateComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataTemplateComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IDataTemplateComponent> for ::windows::core::IInspectable {
    fn from(value: IDataTemplateComponent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDataTemplateComponent> for ::windows::core::IInspectable {
    fn from(value: &IDataTemplateComponent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDataTemplateComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IDataTemplateComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTemplateComponent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, item: ::windows::core::RawPtr, itemindex: i32, phase: i32, nextphase: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMarkupExtension(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMarkupExtension {
    type Vtable = IMarkupExtension_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ee3416d_562b_486e_9ee5_0f0cbcc8048c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtension_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMarkupExtensionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMarkupExtensionFactory {
    type Vtable = IMarkupExtensionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65329c05_fb5a_4567_9d55_5cdfbada2739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtensionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMarkupExtensionOverrides(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMarkupExtensionOverrides {
    type Vtable = IMarkupExtensionOverrides_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x393779bf_b9c0_4ffb_a57f_58e7356e425f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtensionOverrides_abi(
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
pub struct IXamlBinaryWriter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x829d2ad3_620a_46f6_845d_436a05927100);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBinaryWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlBinaryWriterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlBinaryWriterStatics {
    type Vtable = IXamlBinaryWriterStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d8ed07a_9b82_4aa8_b68b_026f2de1cc86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBinaryWriterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstreams: ::windows::core::RawPtr, outputstreams: ::windows::core::RawPtr, xamlmetadataprovider: ::windows::core::RawPtr, result__: *mut XamlBinaryWriterErrorInformation) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXamlBindScopeDiagnostics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlBindScopeDiagnostics {
    type Vtable = IXamlBindScopeDiagnostics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf264a29d_bded_43aa_a5b0_26ac21a81eb8);
}
impl IXamlBindScopeDiagnostics {
    pub fn Disable(&self, linenumber: i32, columnnumber: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), linenumber, columnnumber).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlBindScopeDiagnostics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f264a29d-bded-43aa-a5b0-26ac21a81eb8}");
}
impl ::core::convert::From<IXamlBindScopeDiagnostics> for ::windows::core::IUnknown {
    fn from(value: IXamlBindScopeDiagnostics) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlBindScopeDiagnostics> for ::windows::core::IUnknown {
    fn from(value: &IXamlBindScopeDiagnostics) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlBindScopeDiagnostics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlBindScopeDiagnostics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlBindScopeDiagnostics> for ::windows::core::IInspectable {
    fn from(value: IXamlBindScopeDiagnostics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlBindScopeDiagnostics> for ::windows::core::IInspectable {
    fn from(value: &IXamlBindScopeDiagnostics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlBindScopeDiagnostics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlBindScopeDiagnostics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindScopeDiagnostics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, linenumber: i32, columnnumber: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlBindingHelper(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlBindingHelper {
    type Vtable = IXamlBindingHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaa6fb06_8ab9_4ef7_8ae7_fbd30bbfd06d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindingHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlBindingHelperStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlBindingHelperStatics {
    type Vtable = IXamlBindingHelperStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf65cfb71_c80c_4ffa_86ee_558754ee336d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindingHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencyobject: ::windows::core::RawPtr, propertytoset: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlMarkupHelper(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0e6673c_5342_44ef_85a7_ed327a739d9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMarkupHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlMarkupHelperStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlMarkupHelperStatics {
    type Vtable = IXamlMarkupHelperStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9bc3725_f34f_445c_81a2_6b72a5e8f072);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMarkupHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXamlMember(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlMember {
    type Vtable = IXamlMember_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc541f58c_43a9_4216_b718_e0b11b14e93e);
}
impl IXamlMember {
    pub fn IsAttachable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsDependencyProperty(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TargetType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, instance: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), instance.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, instance: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), instance.into_param().abi(), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlMember {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c541f58c-43a9-4216-b718-e0b11b14e93e}");
}
impl ::core::convert::From<IXamlMember> for ::windows::core::IUnknown {
    fn from(value: IXamlMember) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlMember> for ::windows::core::IUnknown {
    fn from(value: &IXamlMember) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlMember> for ::windows::core::IInspectable {
    fn from(value: IXamlMember) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlMember> for ::windows::core::IInspectable {
    fn from(value: &IXamlMember) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMember_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, instance: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, instance: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXamlMetadataProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlMetadataProvider {
    type Vtable = IXamlMetadataProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3765d69_68a5_4b32_8861_fdb90c1f5836);
}
impl IXamlMetadataProvider {
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn GetXamlType<'a, Param0: ::windows::core::IntoParam<'a, super::Interop::TypeName>>(&self, r#type: Param0) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), r#type.into_param().abi(), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    pub fn GetXamlTypeByFullName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, fullname: Param0) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), fullname.into_param().abi(), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    pub fn GetXmlnsDefinitions(&self) -> ::windows::core::Result<::windows::core::Array<XmlnsDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<XmlnsDefinition> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), ::windows::core::Array::<XmlnsDefinition>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlMetadataProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b3765d69-68a5-4b32-8861-fdb90c1f5836}");
}
impl ::core::convert::From<IXamlMetadataProvider> for ::windows::core::IUnknown {
    fn from(value: IXamlMetadataProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlMetadataProvider> for ::windows::core::IUnknown {
    fn from(value: &IXamlMetadataProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlMetadataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlMetadataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlMetadataProvider> for ::windows::core::IInspectable {
    fn from(value: IXamlMetadataProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlMetadataProvider> for ::windows::core::IInspectable {
    fn from(value: &IXamlMetadataProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlMetadataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlMetadataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMetadataProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<XmlnsDefinition>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlReader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlReader {
    type Vtable = IXamlReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24374cf1_cceb_48bf_a514_41b0186f84c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlReaderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlReaderStatics {
    type Vtable = IXamlReaderStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9891c6bd_534f_4955_b85a_8a8dc0dca602);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReaderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xaml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xaml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXamlType(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlType {
    type Vtable = IXamlType_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7920eab1_a2e5_479a_bd50_6cef3c0b4970);
}
impl IXamlType {
    pub fn BaseType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    pub fn ContentProperty(&self) -> ::windows::core::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlMember>(result__)
        }
    }
    pub fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsArray(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsCollection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsConstructible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsDictionary(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsMarkupExtension(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsBindable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ItemType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    pub fn KeyType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn UnderlyingType(&self) -> ::windows::core::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<super::Interop::TypeName> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    pub fn ActivateInstance(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn CreateFromString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetMember<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<IXamlMember>(result__)
        }
    }
    pub fn AddToVector<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, instance: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), instance.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn AddToMap<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, instance: Param0, key: Param1, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), instance.into_param().abi(), key.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn RunInitializer(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{7920eab1-a2e5-479a-bd50-6cef3c0b4970}");
}
impl ::core::convert::From<IXamlType> for ::windows::core::IUnknown {
    fn from(value: IXamlType) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlType> for ::windows::core::IUnknown {
    fn from(value: &IXamlType) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlType> for ::windows::core::IInspectable {
    fn from(value: IXamlType) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlType> for ::windows::core::IInspectable {
    fn from(value: &IXamlType) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlType_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, instance: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, instance: ::windows::core::RawPtr, key: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXamlType2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlType2 {
    type Vtable = IXamlType2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f0c6e3b_433b_56ad_8f69_78a4dd3e64f9);
}
impl IXamlType2 {
    pub fn BaseType(&self) -> ::windows::core::Result<IXamlType> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    pub fn ContentProperty(&self) -> ::windows::core::Result<IXamlMember> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlMember>(result__)
        }
    }
    pub fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsArray(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsCollection(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsConstructible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsDictionary(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsMarkupExtension(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsBindable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ItemType(&self) -> ::windows::core::Result<IXamlType> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    pub fn KeyType(&self) -> ::windows::core::Result<IXamlType> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn UnderlyingType(&self) -> ::windows::core::Result<super::Interop::TypeName> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<super::Interop::TypeName> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    pub fn ActivateInstance(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn CreateFromString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetMember<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<IXamlMember> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<IXamlMember>(result__)
        }
    }
    pub fn AddToVector<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, instance: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), instance.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn AddToMap<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, instance: Param0, key: Param1, value: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), instance.into_param().abi(), key.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn RunInitializer(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn BoxedType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlType2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9f0c6e3b-433b-56ad-8f69-78a4dd3e64f9}");
}
impl ::core::convert::From<IXamlType2> for ::windows::core::IUnknown {
    fn from(value: IXamlType2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXamlType2> for ::windows::core::IUnknown {
    fn from(value: &IXamlType2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlType2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlType2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXamlType2> for ::windows::core::IInspectable {
    fn from(value: IXamlType2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlType2> for ::windows::core::IInspectable {
    fn from(value: &IXamlType2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlType2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlType2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IXamlType2> for IXamlType {
    type Error = ::windows::core::Error;
    fn try_from(value: IXamlType2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXamlType2> for IXamlType {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXamlType2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXamlType> for IXamlType2 {
    fn into_param(self) -> ::windows::core::Param<'a, IXamlType> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXamlType> for &IXamlType2 {
    fn into_param(self) -> ::windows::core::Param<'a, IXamlType> {
        ::core::convert::TryInto::<IXamlType>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlType2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MarkupExtension(pub ::windows::core::IInspectable);
impl MarkupExtension {
    pub fn new() -> ::windows::core::Result<MarkupExtension> {
        Self::IMarkupExtensionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<MarkupExtension>(result__)
        })
    }
    pub fn IMarkupExtensionFactory<R, F: FnOnce(&IMarkupExtensionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MarkupExtension, IMarkupExtensionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MarkupExtension {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Markup.MarkupExtension;{1ee3416d-562b-486e-9ee5-0f0cbcc8048c})");
}
unsafe impl ::windows::core::Interface for MarkupExtension {
    type Vtable = IMarkupExtension_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ee3416d_562b_486e_9ee5_0f0cbcc8048c);
}
impl ::windows::core::RuntimeName for MarkupExtension {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.MarkupExtension";
}
impl ::core::convert::From<MarkupExtension> for ::windows::core::IUnknown {
    fn from(value: MarkupExtension) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MarkupExtension> for ::windows::core::IUnknown {
    fn from(value: &MarkupExtension) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MarkupExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MarkupExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MarkupExtension> for ::windows::core::IInspectable {
    fn from(value: MarkupExtension) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MarkupExtension> for ::windows::core::IInspectable {
    fn from(value: &MarkupExtension) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MarkupExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MarkupExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MarkupExtension {}
unsafe impl ::core::marker::Sync for MarkupExtension {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XamlBinaryWriter(pub ::windows::core::IInspectable);
impl XamlBinaryWriter {
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn Write<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream>>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream>>, Param2: ::windows::core::IntoParam<'a, IXamlMetadataProvider>>(
        inputstreams: Param0,
        outputstreams: Param1,
        xamlmetadataprovider: Param2,
    ) -> ::windows::core::Result<XamlBinaryWriterErrorInformation> {
        Self::IXamlBinaryWriterStatics(|this| unsafe {
            let mut result__: XamlBinaryWriterErrorInformation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inputstreams.into_param().abi(), outputstreams.into_param().abi(), xamlmetadataprovider.into_param().abi(), &mut result__).from_abi::<XamlBinaryWriterErrorInformation>(result__)
        })
    }
    pub fn IXamlBinaryWriterStatics<R, F: FnOnce(&IXamlBinaryWriterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlBinaryWriter, IXamlBinaryWriterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XamlBinaryWriter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Markup.XamlBinaryWriter;{829d2ad3-620a-46f6-845d-436a05927100})");
}
unsafe impl ::windows::core::Interface for XamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x829d2ad3_620a_46f6_845d_436a05927100);
}
impl ::windows::core::RuntimeName for XamlBinaryWriter {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlBinaryWriter";
}
impl ::core::convert::From<XamlBinaryWriter> for ::windows::core::IUnknown {
    fn from(value: XamlBinaryWriter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlBinaryWriter> for ::windows::core::IUnknown {
    fn from(value: &XamlBinaryWriter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlBinaryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlBinaryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlBinaryWriter> for ::windows::core::IInspectable {
    fn from(value: XamlBinaryWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlBinaryWriter> for ::windows::core::IInspectable {
    fn from(value: &XamlBinaryWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlBinaryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlBinaryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlBinaryWriter {}
unsafe impl ::core::marker::Sync for XamlBinaryWriter {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for XamlBinaryWriterErrorInformation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for XamlBinaryWriterErrorInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Markup.XamlBinaryWriterErrorInformation;u4;u4;u4)");
}
impl ::windows::core::DefaultType for XamlBinaryWriterErrorInformation {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XamlBindingHelper(pub ::windows::core::IInspectable);
impl XamlBindingHelper {
    pub fn DataTemplateComponentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetDataTemplateComponent<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<IDataTemplateComponent> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<IDataTemplateComponent>(result__)
        })
    }
    pub fn SetDataTemplateComponent<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, IDataTemplateComponent>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn SuspendRendering<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(target: Param0) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), target.into_param().abi()).ok() })
    }
    pub fn ResumeRendering<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(target: Param0) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), target.into_param().abi()).ok() })
    }
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn ConvertValue<'a, Param0: ::windows::core::IntoParam<'a, super::Interop::TypeName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(r#type: Param0, value: Param1) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), r#type.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn SetPropertyFromString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn SetPropertyFromBoolean<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: bool) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    pub fn SetPropertyFromChar16<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: u16) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPropertyFromDateTime<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::DateTime>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn SetPropertyFromDouble<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: f64) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    pub fn SetPropertyFromInt32<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: i32) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    pub fn SetPropertyFromUInt32<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: u32) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    pub fn SetPropertyFromInt64<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: i64) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    pub fn SetPropertyFromUInt64<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: u64) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    pub fn SetPropertyFromSingle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: f32) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPropertyFromPoint<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPropertyFromRect<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPropertyFromSize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPropertyFromTimeSpan<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn SetPropertyFromByte<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: u8) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPropertyFromUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn SetPropertyFromObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn IXamlBindingHelperStatics<R, F: FnOnce(&IXamlBindingHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlBindingHelper, IXamlBindingHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XamlBindingHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Markup.XamlBindingHelper;{faa6fb06-8ab9-4ef7-8ae7-fbd30bbfd06d})");
}
unsafe impl ::windows::core::Interface for XamlBindingHelper {
    type Vtable = IXamlBindingHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaa6fb06_8ab9_4ef7_8ae7_fbd30bbfd06d);
}
impl ::windows::core::RuntimeName for XamlBindingHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlBindingHelper";
}
impl ::core::convert::From<XamlBindingHelper> for ::windows::core::IUnknown {
    fn from(value: XamlBindingHelper) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlBindingHelper> for ::windows::core::IUnknown {
    fn from(value: &XamlBindingHelper) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlBindingHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlBindingHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlBindingHelper> for ::windows::core::IInspectable {
    fn from(value: XamlBindingHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlBindingHelper> for ::windows::core::IInspectable {
    fn from(value: &XamlBindingHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlBindingHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlBindingHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlBindingHelper {}
unsafe impl ::core::marker::Sync for XamlBindingHelper {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XamlMarkupHelper(pub ::windows::core::IInspectable);
impl XamlMarkupHelper {
    pub fn UnloadObject<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<()> {
        Self::IXamlMarkupHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), element.into_param().abi()).ok() })
    }
    pub fn IXamlMarkupHelperStatics<R, F: FnOnce(&IXamlMarkupHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlMarkupHelper, IXamlMarkupHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XamlMarkupHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Markup.XamlMarkupHelper;{d0e6673c-5342-44ef-85a7-ed327a739d9a})");
}
unsafe impl ::windows::core::Interface for XamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0e6673c_5342_44ef_85a7_ed327a739d9a);
}
impl ::windows::core::RuntimeName for XamlMarkupHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlMarkupHelper";
}
impl ::core::convert::From<XamlMarkupHelper> for ::windows::core::IUnknown {
    fn from(value: XamlMarkupHelper) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlMarkupHelper> for ::windows::core::IUnknown {
    fn from(value: &XamlMarkupHelper) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlMarkupHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlMarkupHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlMarkupHelper> for ::windows::core::IInspectable {
    fn from(value: XamlMarkupHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlMarkupHelper> for ::windows::core::IInspectable {
    fn from(value: &XamlMarkupHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlMarkupHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlMarkupHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlMarkupHelper {}
unsafe impl ::core::marker::Sync for XamlMarkupHelper {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XamlReader(pub ::windows::core::IInspectable);
impl XamlReader {
    pub fn Load<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(xaml: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xaml.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn LoadWithInitialTemplateValidation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(xaml: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xaml.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn IXamlReaderStatics<R, F: FnOnce(&IXamlReaderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlReader, IXamlReaderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XamlReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Markup.XamlReader;{24374cf1-cceb-48bf-a514-41b0186f84c2})");
}
unsafe impl ::windows::core::Interface for XamlReader {
    type Vtable = IXamlReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24374cf1_cceb_48bf_a514_41b0186f84c2);
}
impl ::windows::core::RuntimeName for XamlReader {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlReader";
}
impl ::core::convert::From<XamlReader> for ::windows::core::IUnknown {
    fn from(value: XamlReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlReader> for ::windows::core::IUnknown {
    fn from(value: &XamlReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlReader> for ::windows::core::IInspectable {
    fn from(value: XamlReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlReader> for ::windows::core::IInspectable {
    fn from(value: &XamlReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XamlReader {}
unsafe impl ::core::marker::Sync for XamlReader {}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
pub struct XmlnsDefinition {
    pub XmlNamespace: ::windows::core::HSTRING,
    pub Namespace: ::windows::core::HSTRING,
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
unsafe impl ::windows::core::Abi for XmlnsDefinition {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows::core::RuntimeType for XmlnsDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Markup.XmlnsDefinition;string;string)");
}
impl ::windows::core::DefaultType for XmlnsDefinition {
    type DefaultType = Self;
}
