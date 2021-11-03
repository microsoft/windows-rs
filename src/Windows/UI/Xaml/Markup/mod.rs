#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct IComponentConnector(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IComponentConnector {
    type Vtable = IComponentConnector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4135127431, 59109, 18418, [146, 198, 236, 204, 228, 186, 21, 154]);
}
impl IComponentConnector {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn Connect<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, connectionid: i32, target: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), connectionid, target.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IComponentConnector {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{f6790987-e6e5-47f2-92c6-eccce4ba159a}");
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct IComponentConnector2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IComponentConnector2 {
    type Vtable = IComponentConnector2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3700373131, 60620, 18830, [177, 57, 145, 20, 34, 84, 215, 174]);
}
impl IComponentConnector2 {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn GetBindingConnector<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, connectionid: i32, target: Param1) -> ::windows::runtime::Result<IComponentConnector> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), connectionid, target.into_param().abi(), &mut result__).from_abi::<IComponentConnector>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IComponentConnector2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{dc8f368b-eccc-498e-b139-91142254d7ae}");
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct IDataTemplateComponent(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataTemplateComponent {
    type Vtable = IDataTemplateComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(138583496, 35504, 18247, [170, 154, 254, 173, 252, 141, 168, 225]);
}
impl IDataTemplateComponent {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn Recycle(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn ProcessBindings<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, item: Param0, itemindex: i32, phase: i32, nextphase: &mut i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), item.into_param().abi(), itemindex, phase, nextphase).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IDataTemplateComponent {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{08429dc8-8ab0-4747-aa9a-feadfc8da8e1}");
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMarkupExtension(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMarkupExtension {
    type Vtable = IMarkupExtension_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(518209901, 22059, 18542, [158, 229, 15, 12, 188, 200, 4, 140]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMarkupExtensionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMarkupExtensionFactory {
    type Vtable = IMarkupExtensionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1697815557, 64346, 17767, [157, 85, 92, 223, 186, 218, 39, 57]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMarkupExtensionOverrides(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMarkupExtensionOverrides {
    type Vtable = IMarkupExtensionOverrides_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(959936959, 47552, 20475, [165, 127, 88, 231, 53, 110, 66, 95]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IXamlBinaryWriter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2191338195, 25098, 18166, [132, 93, 67, 106, 5, 146, 113, 0]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IXamlBinaryWriterStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlBinaryWriterStatics {
    type Vtable = IXamlBinaryWriterStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(227463290, 39810, 19112, [182, 139, 2, 111, 45, 225, 204, 134]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct IXamlBindScopeDiagnostics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlBindScopeDiagnostics {
    type Vtable = IXamlBindScopeDiagnostics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4066681501, 48621, 17322, [165, 176, 38, 172, 33, 168, 30, 184]);
}
impl IXamlBindScopeDiagnostics {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn Disable(&self, linenumber: i32, columnnumber: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), linenumber, columnnumber).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXamlBindScopeDiagnostics {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{f264a29d-bded-43aa-a5b0-26ac21a81eb8}");
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IXamlBindingHelper(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlBindingHelper {
    type Vtable = IXamlBindingHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4205247238, 35513, 20215, [138, 231, 251, 211, 11, 191, 208, 109]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IXamlBindingHelperStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlBindingHelperStatics {
    type Vtable = IXamlBindingHelperStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4133288817, 51212, 20474, [134, 238, 85, 135, 84, 238, 51, 109]);
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
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: ::std::mem::ManuallyDrop<super::Interop::TypeName>, value: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencyobject: ::windows::runtime::RawPtr, propertytoset: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IXamlMarkupHelper(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3504760636, 21314, 17647, [133, 167, 237, 50, 122, 115, 157, 154]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IXamlMarkupHelperStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlMarkupHelperStatics {
    type Vtable = IXamlMarkupHelperStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3384555301, 62287, 17500, [129, 162, 107, 114, 165, 232, 240, 114]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct IXamlMember(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlMember {
    type Vtable = IXamlMember_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3309434252, 17321, 16918, [183, 24, 224, 177, 27, 20, 233, 62]);
}
impl IXamlMember {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsAttachable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsDependencyProperty(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsReadOnly(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn TargetType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, instance: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), instance.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, instance: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), instance.into_param().abi(), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXamlMember {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{c541f58c-43a9-4216-b718-e0b11b14e93e}");
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instance: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instance: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct IXamlMetadataProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlMetadataProvider {
    type Vtable = IXamlMetadataProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3010878825, 26789, 19250, [136, 97, 253, 185, 12, 31, 88, 54]);
}
impl IXamlMetadataProvider {
    #[cfg(feature = "UI_Xaml_Interop")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `UI_Xaml_Interop`*"]
    pub fn GetXamlType<'a, Param0: ::windows::runtime::IntoParam<'a, super::Interop::TypeName>>(&self, r#type: Param0) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), r#type.into_param().abi(), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn GetXamlTypeByFullName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, fullname: Param0) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), fullname.into_param().abi(), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn GetXmlnsDefinitions(&self) -> ::windows::runtime::Result<::windows::runtime::Array<XmlnsDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<XmlnsDefinition> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<XmlnsDefinition>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXamlMetadataProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{b3765d69-68a5-4b32-8861-fdb90c1f5836}");
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
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: ::std::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut ::std::mem::ManuallyDrop<XmlnsDefinition>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IXamlReader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlReader {
    type Vtable = IXamlReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(607603953, 52459, 18623, [165, 20, 65, 176, 24, 111, 132, 194]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IXamlReaderStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlReaderStatics {
    type Vtable = IXamlReaderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2559690429, 21327, 18773, [184, 90, 138, 141, 192, 220, 166, 2]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xaml: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xaml: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct IXamlType(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlType {
    type Vtable = IXamlType_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2032200369, 41701, 18330, [189, 80, 108, 239, 60, 11, 73, 112]);
}
impl IXamlType {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn BaseType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn ContentProperty(&self) -> ::windows::runtime::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXamlMember>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn FullName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsArray(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsCollection(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsConstructible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsDictionary(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsMarkupExtension(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsBindable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn ItemType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn KeyType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Interop")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `UI_Xaml_Interop`*"]
    pub fn UnderlyingType(&self) -> ::windows::runtime::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<super::Interop::TypeName> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn ActivateInstance(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn CreateFromString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn GetMember<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<IXamlMember>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn AddToVector<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, instance: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), instance.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn AddToMap<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, instance: Param0, key: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), instance.into_param().abi(), key.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn RunInitializer(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXamlType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{7920eab1-a2e5-479a-bd50-6cef3c0b4970}");
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instance: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instance: ::windows::runtime::RawPtr, key: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct IXamlType2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlType2 {
    type Vtable = IXamlType2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2668391995, 17211, 22189, [143, 105, 120, 164, 221, 62, 100, 249]);
}
impl IXamlType2 {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn BaseType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn ContentProperty(&self) -> ::windows::runtime::Result<IXamlMember> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXamlMember>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn FullName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsArray(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsCollection(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsConstructible(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsDictionary(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsMarkupExtension(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn IsBindable(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn ItemType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn KeyType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Interop")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `UI_Xaml_Interop`*"]
    pub fn UnderlyingType(&self) -> ::windows::runtime::Result<super::Interop::TypeName> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<super::Interop::TypeName> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn ActivateInstance(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn CreateFromString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn GetMember<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<IXamlMember> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<IXamlMember>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn AddToVector<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, instance: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), instance.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn AddToMap<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, instance: Param0, key: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), instance.into_param().abi(), key.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn RunInitializer(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXamlType>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn BoxedType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXamlType2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{9f0c6e3b-433b-56ad-8f69-78a4dd3e64f9}");
}
impl ::std::convert::TryFrom<IXamlType2> for IXamlType {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IXamlType2) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IXamlType2> for IXamlType {
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
        ::std::convert::TryInto::<IXamlType>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct MarkupExtension(::windows::runtime::IInspectable);
impl MarkupExtension {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn new() -> ::windows::runtime::Result<MarkupExtension> {
        Self::IMarkupExtensionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), ::std::ptr::null_mut(), &mut ::std::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<MarkupExtension>(result__)
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(518209901, 22059, 18542, [158, 229, 15, 12, 188, 200, 4, 140]);
}
impl ::windows::runtime::RuntimeName for MarkupExtension {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.MarkupExtension";
}
unsafe impl ::std::marker::Send for MarkupExtension {}
unsafe impl ::std::marker::Sync for MarkupExtension {}
#[doc = "*Required features: `UI_Xaml_Markup`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct XamlBinaryWriter(::windows::runtime::IInspectable);
impl XamlBinaryWriter {
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Xaml_Markup`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn Write<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream>>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream>>, Param2: ::windows::runtime::IntoParam<'a, IXamlMetadataProvider>>(
        inputstreams: Param0,
        outputstreams: Param1,
        xamlmetadataprovider: Param2,
    ) -> ::windows::runtime::Result<XamlBinaryWriterErrorInformation> {
        Self::IXamlBinaryWriterStatics(|this| unsafe {
            let mut result__: XamlBinaryWriterErrorInformation = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), inputstreams.into_param().abi(), outputstreams.into_param().abi(), xamlmetadataprovider.into_param().abi(), &mut result__).from_abi::<XamlBinaryWriterErrorInformation>(result__)
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2191338195, 25098, 18166, [132, 93, 67, 106, 5, 146, 113, 0]);
}
impl ::windows::runtime::RuntimeName for XamlBinaryWriter {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlBinaryWriter";
}
unsafe impl ::std::marker::Send for XamlBinaryWriter {}
unsafe impl ::std::marker::Sync for XamlBinaryWriter {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct XamlBinaryWriterErrorInformation {
    pub InputStreamIndex: u32,
    pub LineNumber: u32,
    pub LinePosition: u32,
}
impl XamlBinaryWriterErrorInformation {}
impl ::std::default::Default for XamlBinaryWriterErrorInformation {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for XamlBinaryWriterErrorInformation {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("XamlBinaryWriterErrorInformation").field("InputStreamIndex", &self.InputStreamIndex).field("LineNumber", &self.LineNumber).field("LinePosition", &self.LinePosition).finish()
    }
}
impl ::std::cmp::PartialEq for XamlBinaryWriterErrorInformation {
    fn eq(&self, other: &Self) -> bool {
        self.InputStreamIndex == other.InputStreamIndex && self.LineNumber == other.LineNumber && self.LinePosition == other.LinePosition
    }
}
impl ::std::cmp::Eq for XamlBinaryWriterErrorInformation {}
unsafe impl ::windows::runtime::Abi for XamlBinaryWriterErrorInformation {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XamlBinaryWriterErrorInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Markup.XamlBinaryWriterErrorInformation;u4;u4;u4)");
}
#[doc = "*Required features: `UI_Xaml_Markup`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct XamlBindingHelper(::windows::runtime::IInspectable);
impl XamlBindingHelper {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn DataTemplateComponentProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn GetDataTemplateComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::runtime::Result<IDataTemplateComponent> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<IDataTemplateComponent>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetDataTemplateComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>, Param1: ::windows::runtime::IntoParam<'a, IDataTemplateComponent>>(element: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SuspendRendering<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(target: Param0) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), target.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn ResumeRendering<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(target: Param0) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), target.into_param().abi()).ok() })
    }
    #[cfg(feature = "UI_Xaml_Interop")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `UI_Xaml_Interop`*"]
    pub fn ConvertValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::Interop::TypeName>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(r#type: Param0, value: Param1) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), r#type.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromBoolean<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: bool) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromChar16<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: u16) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `Foundation`*"]
    pub fn SetPropertyFromDateTime<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::DateTime>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromDouble<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: f64) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromInt32<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: i32) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromUInt32<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: u32) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromInt64<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: i64) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromUInt64<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: u64) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromSingle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: f32) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `Foundation`*"]
    pub fn SetPropertyFromPoint<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `Foundation`*"]
    pub fn SetPropertyFromRect<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Rect>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `Foundation`*"]
    pub fn SetPropertyFromSize<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Size>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `Foundation`*"]
    pub fn SetPropertyFromTimeSpan<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromByte<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: u8) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Markup`, `Foundation`*"]
    pub fn SetPropertyFromUri<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn SetPropertyFromObject<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4205247238, 35513, 20215, [138, 231, 251, 211, 11, 191, 208, 109]);
}
impl ::windows::runtime::RuntimeName for XamlBindingHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlBindingHelper";
}
unsafe impl ::std::marker::Send for XamlBindingHelper {}
unsafe impl ::std::marker::Sync for XamlBindingHelper {}
#[doc = "*Required features: `UI_Xaml_Markup`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct XamlMarkupHelper(::windows::runtime::IInspectable);
impl XamlMarkupHelper {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn UnloadObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::runtime::Result<()> {
        Self::IXamlMarkupHelperStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), element.into_param().abi()).ok() })
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3504760636, 21314, 17647, [133, 167, 237, 50, 122, 115, 157, 154]);
}
impl ::windows::runtime::RuntimeName for XamlMarkupHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlMarkupHelper";
}
unsafe impl ::std::marker::Send for XamlMarkupHelper {}
unsafe impl ::std::marker::Sync for XamlMarkupHelper {}
#[doc = "*Required features: `UI_Xaml_Markup`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct XamlReader(::windows::runtime::IInspectable);
impl XamlReader {
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(xaml: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xaml.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Markup`*"]
    pub fn LoadWithInitialTemplateValidation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(xaml: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xaml.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(607603953, 52459, 18623, [165, 20, 65, 176, 24, 111, 132, 194]);
}
impl ::windows::runtime::RuntimeName for XamlReader {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlReader";
}
unsafe impl ::std::marker::Send for XamlReader {}
unsafe impl ::std::marker::Sync for XamlReader {}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `UI_Xaml_Markup`*"]
pub struct XmlnsDefinition {
    pub XmlNamespace: ::windows::runtime::HSTRING,
    pub Namespace: ::windows::runtime::HSTRING,
}
impl XmlnsDefinition {}
impl ::std::default::Default for XmlnsDefinition {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for XmlnsDefinition {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("XmlnsDefinition").field("XmlNamespace", &self.XmlNamespace).field("Namespace", &self.Namespace).finish()
    }
}
impl ::std::cmp::PartialEq for XmlnsDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.XmlNamespace == other.XmlNamespace && self.Namespace == other.Namespace
    }
}
impl ::std::cmp::Eq for XmlnsDefinition {}
unsafe impl ::windows::runtime::Abi for XmlnsDefinition {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XmlnsDefinition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Markup.XmlnsDefinition;string;string)");
}
