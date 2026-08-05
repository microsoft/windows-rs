windows_link::link!("windows.ui.xaml.dll" "C" fn InitializeXamlDiagnosticsEx(endpointname : windows_sys::core::PCWSTR, pid : u32, wszdllxamldiagnostics : windows_sys::core::PCWSTR, wsztapdllname : windows_sys::core::PCWSTR, tapclsid : windows_sys::core::GUID, wszinitializationdata : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
pub const Add: VisualMutationType = 0;
pub const Animation: BaseValueSource = 12;
pub type BaseValueSource = i32;
pub const BaseValueSourceBuiltInStyle: BaseValueSource = 2;
pub const BaseValueSourceDefault: BaseValueSource = 1;
pub const BaseValueSourceLocal: BaseValueSource = 4;
pub const BaseValueSourceStyle: BaseValueSource = 3;
pub const BaseValueSourceUnknown: BaseValueSource = 0;
pub const BaseValueSourceVisualState: BaseValueSource = 14;
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct BitmapDescription {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::DXGI_FORMAT,
    pub AlphaMode: super::DXGI_ALPHA_MODE,
}
pub const Coercion: BaseValueSource = 13;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CollectionElementValue {
    pub Index: u32,
    pub ValueType: windows_sys::core::BSTR,
    pub Value: windows_sys::core::BSTR,
    pub MetadataBits: i64,
}
impl Default for CollectionElementValue {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DefaultStyleTrigger: BaseValueSource = 6;
pub const E_UNKNOWNTYPE: i32 = -2144665560;
#[repr(C)]
#[cfg(feature = "oaidl")]
#[derive(Clone, Copy)]
pub struct EnumType {
    pub Name: windows_sys::core::BSTR,
    pub ValueInts: *mut super::SAFEARRAY,
    pub ValueStrings: *mut super::SAFEARRAY,
}
#[cfg(feature = "oaidl")]
impl Default for EnumType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ErrorInvalidResource: VisualElementState = 2;
pub const ErrorResolved: VisualElementState = 0;
pub const ErrorResourceNotFound: VisualElementState = 1;
pub const ImplicitStyleReference: BaseValueSource = 9;
pub const Inherited: BaseValueSource = 5;
pub type InstanceHandle = u64;
pub const IsPropertyReadOnly: MetadataBit = 2;
pub const IsValueBindingExpression: MetadataBit = 16;
pub const IsValueCollection: MetadataBit = 4;
pub const IsValueCollectionReadOnly: MetadataBit = 8;
pub const IsValueHandle: MetadataBit = 1;
pub const IsValueHandleAndEvaluatedValue: MetadataBit = 64;
pub const IsValueNull: MetadataBit = 32;
pub type MetadataBit = i32;
pub const None: MetadataBit = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ParentChildRelation {
    pub Parent: InstanceHandle,
    pub Child: InstanceHandle,
    pub ChildIndex: u32,
}
pub const ParentTemplate: BaseValueSource = 10;
pub const ParentTemplateTrigger: BaseValueSource = 11;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PropertyChainSource {
    pub Handle: InstanceHandle,
    pub TargetType: windows_sys::core::BSTR,
    pub Name: windows_sys::core::BSTR,
    pub Source: BaseValueSource,
    pub SrcInfo: SourceInfo,
}
impl Default for PropertyChainSource {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PropertyChainValue {
    pub Index: u32,
    pub Type: windows_sys::core::BSTR,
    pub DeclaringType: windows_sys::core::BSTR,
    pub ValueType: windows_sys::core::BSTR,
    pub ItemType: windows_sys::core::BSTR,
    pub Value: windows_sys::core::BSTR,
    pub Overridden: windows_sys::core::BOOL,
    pub MetadataBits: i64,
    pub PropertyName: windows_sys::core::BSTR,
    pub PropertyChainIndex: u32,
}
impl Default for PropertyChainValue {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const Remove: VisualMutationType = 1;
pub const RenderTarget: RenderTargetBitmapOptions = 0;
pub const RenderTargetAndChildren: RenderTargetBitmapOptions = 1;
pub type RenderTargetBitmapOptions = i32;
pub type ResourceType = i32;
pub const ResourceTypeStatic: ResourceType = 0;
pub const ResourceTypeTheme: ResourceType = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SourceInfo {
    pub FileName: windows_sys::core::BSTR,
    pub LineNumber: u32,
    pub ColumnNumber: u32,
    pub CharPosition: u32,
    pub Hash: windows_sys::core::BSTR,
}
impl Default for SourceInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const StyleTrigger: BaseValueSource = 8;
pub const TemplateTrigger: BaseValueSource = 7;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VisualElement {
    pub Handle: InstanceHandle,
    pub SrcInfo: SourceInfo,
    pub Type: windows_sys::core::BSTR,
    pub Name: windows_sys::core::BSTR,
    pub NumChildren: u32,
}
impl Default for VisualElement {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VisualElementState = i32;
pub type VisualMutationType = i32;
