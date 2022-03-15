#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IComponentConnector(::windows::core::IUnknown);
impl IComponentConnector {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn Connect<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, connectionid: i32, target: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Connect)(::core::mem::transmute_copy(this), connectionid, target.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IComponentConnector> for ::windows::core::IUnknown {
    fn from(value: IComponentConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComponentConnector> for ::windows::core::IUnknown {
    fn from(value: &IComponentConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IComponentConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IComponentConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IComponentConnector> for ::windows::core::IInspectable {
    fn from(value: IComponentConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComponentConnector> for ::windows::core::IInspectable {
    fn from(value: &IComponentConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IComponentConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IComponentConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComponentConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComponentConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComponentConnector {}
impl ::core::fmt::Debug for IComponentConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComponentConnector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IComponentConnector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f6790987-e6e5-47f2-92c6-eccce4ba159a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IComponentConnector {
    type Vtable = IComponentConnector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6790987_e6e5_47f2_92c6_eccce4ba159a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentConnector_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: i32, target: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IComponentConnector2(::windows::core::IUnknown);
impl IComponentConnector2 {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn GetBindingConnector<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, connectionid: i32, target: Param1) -> ::windows::core::Result<IComponentConnector> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBindingConnector)(::core::mem::transmute_copy(this), connectionid, target.into_param().abi(), &mut result__).from_abi::<IComponentConnector>(result__)
        }
    }
}
impl ::core::convert::From<IComponentConnector2> for ::windows::core::IUnknown {
    fn from(value: IComponentConnector2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComponentConnector2> for ::windows::core::IUnknown {
    fn from(value: &IComponentConnector2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IComponentConnector2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IComponentConnector2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IComponentConnector2> for ::windows::core::IInspectable {
    fn from(value: IComponentConnector2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComponentConnector2> for ::windows::core::IInspectable {
    fn from(value: &IComponentConnector2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IComponentConnector2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IComponentConnector2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComponentConnector2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComponentConnector2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComponentConnector2 {}
impl ::core::fmt::Debug for IComponentConnector2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComponentConnector2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IComponentConnector2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{dc8f368b-eccc-498e-b139-91142254d7ae}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IComponentConnector2 {
    type Vtable = IComponentConnector2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc8f368b_eccc_498e_b139_91142254d7ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentConnector2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetBindingConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: i32, target: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IDataTemplateComponent(::windows::core::IUnknown);
impl IDataTemplateComponent {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn Recycle(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Recycle)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn ProcessBindings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, item: Param0, itemindex: i32, phase: i32, nextphase: &mut i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ProcessBindings)(::core::mem::transmute_copy(this), item.into_param().abi(), itemindex, phase, nextphase).ok() }
    }
}
impl ::core::convert::From<IDataTemplateComponent> for ::windows::core::IUnknown {
    fn from(value: IDataTemplateComponent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataTemplateComponent> for ::windows::core::IUnknown {
    fn from(value: &IDataTemplateComponent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataTemplateComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataTemplateComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDataTemplateComponent> for ::windows::core::IInspectable {
    fn from(value: IDataTemplateComponent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataTemplateComponent> for ::windows::core::IInspectable {
    fn from(value: &IDataTemplateComponent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDataTemplateComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IDataTemplateComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDataTemplateComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataTemplateComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataTemplateComponent {}
impl ::core::fmt::Debug for IDataTemplateComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataTemplateComponent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IDataTemplateComponent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{08429dc8-8ab0-4747-aa9a-feadfc8da8e1}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IDataTemplateComponent {
    type Vtable = IDataTemplateComponent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08429dc8_8ab0_4747_aa9a_feadfc8da8e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTemplateComponent_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Recycle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessBindings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, itemindex: i32, phase: i32, nextphase: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMarkupExtension(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMarkupExtension {
    type Vtable = IMarkupExtension_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ee3416d_562b_486e_9ee5_0f0cbcc8048c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtension_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMarkupExtensionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMarkupExtensionFactory {
    type Vtable = IMarkupExtensionFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65329c05_fb5a_4567_9d55_5cdfbada2739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtensionFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMarkupExtensionOverrides(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMarkupExtensionOverrides {
    type Vtable = IMarkupExtensionOverrides_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x393779bf_b9c0_4ffb_a57f_58e7356e425f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtensionOverrides_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ProvideValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlBinaryWriter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x829d2ad3_620a_46f6_845d_436a05927100);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBinaryWriter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlBinaryWriterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlBinaryWriterStatics {
    type Vtable = IXamlBinaryWriterStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d8ed07a_9b82_4aa8_b68b_026f2de1cc86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBinaryWriterStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstreams: ::windows::core::RawPtr, outputstreams: ::windows::core::RawPtr, xamlmetadataprovider: ::windows::core::RawPtr, result__: *mut XamlBinaryWriterErrorInformation) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    Write: usize,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IXamlBindScopeDiagnostics(::windows::core::IUnknown);
impl IXamlBindScopeDiagnostics {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn Disable(&self, linenumber: i32, columnnumber: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Disable)(::core::mem::transmute_copy(this), linenumber, columnnumber).ok() }
    }
}
impl ::core::convert::From<IXamlBindScopeDiagnostics> for ::windows::core::IUnknown {
    fn from(value: IXamlBindScopeDiagnostics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlBindScopeDiagnostics> for ::windows::core::IUnknown {
    fn from(value: &IXamlBindScopeDiagnostics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlBindScopeDiagnostics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlBindScopeDiagnostics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlBindScopeDiagnostics> for ::windows::core::IInspectable {
    fn from(value: IXamlBindScopeDiagnostics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlBindScopeDiagnostics> for ::windows::core::IInspectable {
    fn from(value: &IXamlBindScopeDiagnostics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlBindScopeDiagnostics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlBindScopeDiagnostics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlBindScopeDiagnostics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlBindScopeDiagnostics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlBindScopeDiagnostics {}
impl ::core::fmt::Debug for IXamlBindScopeDiagnostics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlBindScopeDiagnostics").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlBindScopeDiagnostics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f264a29d-bded-43aa-a5b0-26ac21a81eb8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXamlBindScopeDiagnostics {
    type Vtable = IXamlBindScopeDiagnostics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf264a29d_bded_43aa_a5b0_26ac21a81eb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindScopeDiagnostics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Disable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linenumber: i32, columnnumber: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlBindingHelper(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlBindingHelper {
    type Vtable = IXamlBindingHelper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaa6fb06_8ab9_4ef7_8ae7_fbd30bbfd06d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindingHelper_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlBindingHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlBindingHelperStatics {
    type Vtable = IXamlBindingHelperStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf65cfb71_c80c_4ffa_86ee_558754ee336d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindingHelperStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DataTemplateComponentProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDataTemplateComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetDataTemplateComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SuspendRendering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ResumeRendering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub ConvertValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    ConvertValue: usize,
    pub SetPropertyFromString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPropertyFromBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub SetPropertyFromChar16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPropertyFromDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPropertyFromDateTime: usize,
    pub SetPropertyFromDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub SetPropertyFromInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub SetPropertyFromUInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub SetPropertyFromInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: i64) -> ::windows::core::HRESULT,
    pub SetPropertyFromUInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT,
    pub SetPropertyFromSingle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPropertyFromPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPropertyFromPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetPropertyFromRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPropertyFromRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetPropertyFromSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPropertyFromSize: usize,
    #[cfg(feature = "Foundation")]
    pub SetPropertyFromTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPropertyFromTimeSpan: usize,
    pub SetPropertyFromByte: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPropertyFromUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPropertyFromUri: usize,
    pub SetPropertyFromObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlMarkupHelper(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0e6673c_5342_44ef_85a7_ed327a739d9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMarkupHelper_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlMarkupHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlMarkupHelperStatics {
    type Vtable = IXamlMarkupHelperStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9bc3725_f34f_445c_81a2_6b72a5e8f072);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMarkupHelperStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub UnloadObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IXamlMember(::windows::core::IUnknown);
impl IXamlMember {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsAttachable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAttachable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsDependencyProperty(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDependencyProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReadOnly)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn TargetType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, instance: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(::core::mem::transmute_copy(this), instance.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, instance: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::core::mem::transmute_copy(this), instance.into_param().abi(), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IXamlMember> for ::windows::core::IUnknown {
    fn from(value: IXamlMember) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlMember> for ::windows::core::IUnknown {
    fn from(value: &IXamlMember) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlMember> for ::windows::core::IInspectable {
    fn from(value: IXamlMember) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlMember> for ::windows::core::IInspectable {
    fn from(value: &IXamlMember) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlMember {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlMember {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlMember {}
impl ::core::fmt::Debug for IXamlMember {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlMember").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlMember {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c541f58c-43a9-4216-b718-e0b11b14e93e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXamlMember {
    type Vtable = IXamlMember_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc541f58c_43a9_4216_b718_e0b11b14e93e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMember_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsAttachable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDependencyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TargetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instance: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instance: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IXamlMetadataProvider(::windows::core::IUnknown);
impl IXamlMetadataProvider {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`, `\"UI_Xaml_Interop\"`*"]
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn GetXamlType<'a, Param0: ::windows::core::IntoParam<'a, super::Interop::TypeName>>(&self, r#type: Param0) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetXamlType)(::core::mem::transmute_copy(this), r#type.into_param().abi(), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn GetXamlTypeByFullName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, fullname: Param0) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetXamlTypeByFullName)(::core::mem::transmute_copy(this), fullname.into_param().abi(), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn GetXmlnsDefinitions(&self) -> ::windows::core::Result<::windows::core::Array<XmlnsDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<XmlnsDefinition> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetXmlnsDefinitions)(::core::mem::transmute_copy(this), ::windows::core::Array::<XmlnsDefinition>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
}
impl ::core::convert::From<IXamlMetadataProvider> for ::windows::core::IUnknown {
    fn from(value: IXamlMetadataProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlMetadataProvider> for ::windows::core::IUnknown {
    fn from(value: &IXamlMetadataProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlMetadataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlMetadataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlMetadataProvider> for ::windows::core::IInspectable {
    fn from(value: IXamlMetadataProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlMetadataProvider> for ::windows::core::IInspectable {
    fn from(value: &IXamlMetadataProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlMetadataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlMetadataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlMetadataProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlMetadataProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlMetadataProvider {}
impl ::core::fmt::Debug for IXamlMetadataProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlMetadataProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlMetadataProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b3765d69-68a5-4b32-8861-fdb90c1f5836}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXamlMetadataProvider {
    type Vtable = IXamlMetadataProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3765d69_68a5_4b32_8861_fdb90c1f5836);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMetadataProvider_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub GetXamlType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    GetXamlType: usize,
    pub GetXamlTypeByFullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetXmlnsDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<XmlnsDefinition>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlReader {
    type Vtable = IXamlReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24374cf1_cceb_48bf_a514_41b0186f84c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReader_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlReaderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlReaderStatics {
    type Vtable = IXamlReaderStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9891c6bd_534f_4955_b85a_8a8dc0dca602);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReaderStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xaml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LoadWithInitialTemplateValidation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xaml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IXamlType(::windows::core::IUnknown);
impl IXamlType {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn BaseType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BaseType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn ContentProperty(&self) -> ::windows::core::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlMember>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FullName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsArray(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsArray)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsCollection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCollection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsConstructible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsConstructible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsDictionary(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDictionary)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsMarkupExtension(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsMarkupExtension)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsBindable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBindable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn ItemType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn KeyType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`, `\"UI_Xaml_Interop\"`*"]
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn UnderlyingType(&self) -> ::windows::core::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<super::Interop::TypeName> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnderlyingType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn ActivateInstance(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivateInstance)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn CreateFromString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromString)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn GetMember<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMember)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<IXamlMember>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn AddToVector<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, instance: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddToVector)(::core::mem::transmute_copy(this), instance.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn AddToMap<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, instance: Param0, key: Param1, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddToMap)(::core::mem::transmute_copy(this), instance.into_param().abi(), key.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn RunInitializer(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RunInitializer)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IXamlType> for ::windows::core::IUnknown {
    fn from(value: IXamlType) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlType> for ::windows::core::IUnknown {
    fn from(value: &IXamlType) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlType> for ::windows::core::IInspectable {
    fn from(value: IXamlType) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlType> for ::windows::core::IInspectable {
    fn from(value: &IXamlType) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlType {}
impl ::core::fmt::Debug for IXamlType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{7920eab1-a2e5-479a-bd50-6cef3c0b4970}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXamlType {
    type Vtable = IXamlType_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7920eab1_a2e5_479a_bd50_6cef3c0b4970);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlType_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BaseType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ContentProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsConstructible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsMarkupExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBindable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ItemType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub KeyType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub UnderlyingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    UnderlyingType: usize,
    pub ActivateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFromString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddToVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instance: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddToMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instance: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RunInitializer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IXamlType2(::windows::core::IUnknown);
impl IXamlType2 {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn BoxedType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BoxedType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn BaseType(&self) -> ::windows::core::Result<IXamlType> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BaseType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn ContentProperty(&self) -> ::windows::core::Result<IXamlMember> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlMember>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FullName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsArray(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsArray)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsCollection(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCollection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsConstructible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsConstructible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsDictionary(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDictionary)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsMarkupExtension(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsMarkupExtension)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn IsBindable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBindable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn ItemType(&self) -> ::windows::core::Result<IXamlType> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn KeyType(&self) -> ::windows::core::Result<IXamlType> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`, `\"UI_Xaml_Interop\"`*"]
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn UnderlyingType(&self) -> ::windows::core::Result<super::Interop::TypeName> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<super::Interop::TypeName> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnderlyingType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn ActivateInstance(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivateInstance)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn CreateFromString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromString)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn GetMember<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<IXamlMember> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMember)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<IXamlMember>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn AddToVector<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, instance: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddToVector)(::core::mem::transmute_copy(this), instance.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn AddToMap<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, instance: Param0, key: Param1, value: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddToMap)(::core::mem::transmute_copy(this), instance.into_param().abi(), key.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn RunInitializer(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXamlType>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RunInitializer)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IXamlType2> for ::windows::core::IUnknown {
    fn from(value: IXamlType2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlType2> for ::windows::core::IUnknown {
    fn from(value: &IXamlType2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlType2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlType2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlType2> for ::windows::core::IInspectable {
    fn from(value: IXamlType2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlType2> for ::windows::core::IInspectable {
    fn from(value: &IXamlType2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlType2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlType2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl ::core::clone::Clone for IXamlType2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlType2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlType2 {}
impl ::core::fmt::Debug for IXamlType2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlType2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlType2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9f0c6e3b-433b-56ad-8f69-78a4dd3e64f9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXamlType2 {
    type Vtable = IXamlType2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f0c6e3b_433b_56ad_8f69_78a4dd3e64f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlType2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BoxedType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct MarkupExtension(::windows::core::IUnknown);
impl MarkupExtension {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn new() -> ::windows::core::Result<MarkupExtension> {
        Self::IMarkupExtensionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<MarkupExtension>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<MarkupExtension> {
        Self::IMarkupExtensionFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<MarkupExtension>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMarkupExtensionFactory<R, F: FnOnce(&IMarkupExtensionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MarkupExtension, IMarkupExtensionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MarkupExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MarkupExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MarkupExtension {}
impl ::core::fmt::Debug for MarkupExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MarkupExtension").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MarkupExtension {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Markup.MarkupExtension;{1ee3416d-562b-486e-9ee5-0f0cbcc8048c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MarkupExtension {
    type Vtable = IMarkupExtension_Vtbl;
    const IID: ::windows::core::GUID = <IMarkupExtension as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MarkupExtension {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.MarkupExtension";
}
impl ::core::convert::From<MarkupExtension> for ::windows::core::IUnknown {
    fn from(value: MarkupExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MarkupExtension> for ::windows::core::IUnknown {
    fn from(value: &MarkupExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MarkupExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MarkupExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MarkupExtension> for ::windows::core::IInspectable {
    fn from(value: MarkupExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MarkupExtension> for ::windows::core::IInspectable {
    fn from(value: &MarkupExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MarkupExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MarkupExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MarkupExtension {}
unsafe impl ::core::marker::Sync for MarkupExtension {}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct XamlBinaryWriter(::windows::core::IUnknown);
impl XamlBinaryWriter {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn Write<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream>>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream>>, Param2: ::windows::core::IntoParam<'a, IXamlMetadataProvider>>(inputstreams: Param0, outputstreams: Param1, xamlmetadataprovider: Param2) -> ::windows::core::Result<XamlBinaryWriterErrorInformation> {
        Self::IXamlBinaryWriterStatics(|this| unsafe {
            let mut result__: XamlBinaryWriterErrorInformation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Write)(::core::mem::transmute_copy(this), inputstreams.into_param().abi(), outputstreams.into_param().abi(), xamlmetadataprovider.into_param().abi(), &mut result__).from_abi::<XamlBinaryWriterErrorInformation>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlBinaryWriterStatics<R, F: FnOnce(&IXamlBinaryWriterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlBinaryWriter, IXamlBinaryWriterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlBinaryWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlBinaryWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlBinaryWriter {}
impl ::core::fmt::Debug for XamlBinaryWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlBinaryWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlBinaryWriter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Markup.XamlBinaryWriter;{829d2ad3-620a-46f6-845d-436a05927100})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_Vtbl;
    const IID: ::windows::core::GUID = <IXamlBinaryWriter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlBinaryWriter {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlBinaryWriter";
}
impl ::core::convert::From<XamlBinaryWriter> for ::windows::core::IUnknown {
    fn from(value: XamlBinaryWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlBinaryWriter> for ::windows::core::IUnknown {
    fn from(value: &XamlBinaryWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlBinaryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlBinaryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlBinaryWriter> for ::windows::core::IInspectable {
    fn from(value: XamlBinaryWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlBinaryWriter> for ::windows::core::IInspectable {
    fn from(value: &XamlBinaryWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlBinaryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlBinaryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlBinaryWriter {}
unsafe impl ::core::marker::Sync for XamlBinaryWriter {}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
pub struct XamlBinaryWriterErrorInformation {
    pub InputStreamIndex: u32,
    pub LineNumber: u32,
    pub LinePosition: u32,
}
impl ::core::marker::Copy for XamlBinaryWriterErrorInformation {}
impl ::core::clone::Clone for XamlBinaryWriterErrorInformation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XamlBinaryWriterErrorInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XamlBinaryWriterErrorInformation").field("InputStreamIndex", &self.InputStreamIndex).field("LineNumber", &self.LineNumber).field("LinePosition", &self.LinePosition).finish()
    }
}
unsafe impl ::windows::core::Abi for XamlBinaryWriterErrorInformation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for XamlBinaryWriterErrorInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Markup.XamlBinaryWriterErrorInformation;u4;u4;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for XamlBinaryWriterErrorInformation {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XamlBinaryWriterErrorInformation>()) == 0 }
    }
}
impl ::core::cmp::Eq for XamlBinaryWriterErrorInformation {}
impl ::core::default::Default for XamlBinaryWriterErrorInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct XamlBindingHelper(::windows::core::IUnknown);
impl XamlBindingHelper {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn DataTemplateComponentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataTemplateComponentProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn GetDataTemplateComponent<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<IDataTemplateComponent> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDataTemplateComponent)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<IDataTemplateComponent>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn SetDataTemplateComponent<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, IDataTemplateComponent>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetDataTemplateComponent)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn SuspendRendering<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(target: Param0) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SuspendRendering)(::core::mem::transmute_copy(this), target.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn ResumeRendering<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(target: Param0) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ResumeRendering)(::core::mem::transmute_copy(this), target.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`, `\"UI_Xaml_Interop\"`*"]
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn ConvertValue<'a, Param0: ::windows::core::IntoParam<'a, super::Interop::TypeName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(r#type: Param0, value: Param1) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConvertValue)(::core::mem::transmute_copy(this), r#type.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn SetPropertyFromString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromString)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn SetPropertyFromBoolean<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: bool) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromBoolean)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn SetPropertyFromChar16<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: u16) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromChar16)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPropertyFromDateTime<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::DateTime>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromDateTime)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn SetPropertyFromDouble<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: f64) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromDouble)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn SetPropertyFromInt32<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: i32) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromInt32)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn SetPropertyFromUInt32<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: u32) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromUInt32)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn SetPropertyFromInt64<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: i64) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromInt64)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn SetPropertyFromUInt64<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: u64) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromUInt64)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn SetPropertyFromSingle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: f32) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromSingle)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPropertyFromPoint<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromPoint)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPropertyFromRect<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromRect)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPropertyFromSize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromSize)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPropertyFromTimeSpan<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromTimeSpan)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn SetPropertyFromByte<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>>(dependencyobject: Param0, propertytoset: Param1, value: u8) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromByte)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPropertyFromUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromUri)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn SetPropertyFromObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(dependencyobject: Param0, propertytoset: Param1, value: Param2) -> ::windows::core::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPropertyFromObject)(::core::mem::transmute_copy(this), dependencyobject.into_param().abi(), propertytoset.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IXamlBindingHelperStatics<R, F: FnOnce(&IXamlBindingHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlBindingHelper, IXamlBindingHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlBindingHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlBindingHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlBindingHelper {}
impl ::core::fmt::Debug for XamlBindingHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlBindingHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlBindingHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Markup.XamlBindingHelper;{faa6fb06-8ab9-4ef7-8ae7-fbd30bbfd06d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XamlBindingHelper {
    type Vtable = IXamlBindingHelper_Vtbl;
    const IID: ::windows::core::GUID = <IXamlBindingHelper as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlBindingHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlBindingHelper";
}
impl ::core::convert::From<XamlBindingHelper> for ::windows::core::IUnknown {
    fn from(value: XamlBindingHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlBindingHelper> for ::windows::core::IUnknown {
    fn from(value: &XamlBindingHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlBindingHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlBindingHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlBindingHelper> for ::windows::core::IInspectable {
    fn from(value: XamlBindingHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlBindingHelper> for ::windows::core::IInspectable {
    fn from(value: &XamlBindingHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlBindingHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlBindingHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlBindingHelper {}
unsafe impl ::core::marker::Sync for XamlBindingHelper {}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct XamlMarkupHelper(::windows::core::IUnknown);
impl XamlMarkupHelper {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn UnloadObject<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<()> {
        Self::IXamlMarkupHelperStatics(|this| unsafe { (::windows::core::Interface::vtable(this).UnloadObject)(::core::mem::transmute_copy(this), element.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IXamlMarkupHelperStatics<R, F: FnOnce(&IXamlMarkupHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlMarkupHelper, IXamlMarkupHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlMarkupHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlMarkupHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlMarkupHelper {}
impl ::core::fmt::Debug for XamlMarkupHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlMarkupHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlMarkupHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Markup.XamlMarkupHelper;{d0e6673c-5342-44ef-85a7-ed327a739d9a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_Vtbl;
    const IID: ::windows::core::GUID = <IXamlMarkupHelper as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlMarkupHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlMarkupHelper";
}
impl ::core::convert::From<XamlMarkupHelper> for ::windows::core::IUnknown {
    fn from(value: XamlMarkupHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlMarkupHelper> for ::windows::core::IUnknown {
    fn from(value: &XamlMarkupHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlMarkupHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlMarkupHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlMarkupHelper> for ::windows::core::IInspectable {
    fn from(value: XamlMarkupHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlMarkupHelper> for ::windows::core::IInspectable {
    fn from(value: &XamlMarkupHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlMarkupHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlMarkupHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlMarkupHelper {}
unsafe impl ::core::marker::Sync for XamlMarkupHelper {}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct XamlReader(::windows::core::IUnknown);
impl XamlReader {
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn Load<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(xaml: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Load)(::core::mem::transmute_copy(this), xaml.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    pub fn LoadWithInitialTemplateValidation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(xaml: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadWithInitialTemplateValidation)(::core::mem::transmute_copy(this), xaml.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlReaderStatics<R, F: FnOnce(&IXamlReaderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlReader, IXamlReaderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlReader {}
impl ::core::fmt::Debug for XamlReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Markup.XamlReader;{24374cf1-cceb-48bf-a514-41b0186f84c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XamlReader {
    type Vtable = IXamlReader_Vtbl;
    const IID: ::windows::core::GUID = <IXamlReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlReader {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlReader";
}
impl ::core::convert::From<XamlReader> for ::windows::core::IUnknown {
    fn from(value: XamlReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlReader> for ::windows::core::IUnknown {
    fn from(value: &XamlReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlReader> for ::windows::core::IInspectable {
    fn from(value: XamlReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlReader> for ::windows::core::IInspectable {
    fn from(value: &XamlReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlReader {}
unsafe impl ::core::marker::Sync for XamlReader {}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
pub struct XmlnsDefinition {
    pub XmlNamespace: ::windows::core::HSTRING,
    pub Namespace: ::windows::core::HSTRING,
}
impl ::core::clone::Clone for XmlnsDefinition {
    fn clone(&self) -> Self {
        Self { XmlNamespace: self.XmlNamespace.clone(), Namespace: self.Namespace.clone() }
    }
}
impl ::core::fmt::Debug for XmlnsDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XmlnsDefinition").field("XmlNamespace", &self.XmlNamespace).field("Namespace", &self.Namespace).finish()
    }
}
unsafe impl ::windows::core::Abi for XmlnsDefinition {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows::core::RuntimeType for XmlnsDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Markup.XmlnsDefinition;string;string)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(from.clone())
    }
}
impl ::core::cmp::PartialEq for XmlnsDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.XmlNamespace == other.XmlNamespace && self.Namespace == other.Namespace
    }
}
impl ::core::cmp::Eq for XmlnsDefinition {}
impl ::core::default::Default for XmlnsDefinition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
