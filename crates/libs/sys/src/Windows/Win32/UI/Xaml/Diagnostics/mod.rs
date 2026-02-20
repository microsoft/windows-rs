windows_link::link!("windows.ui.xaml.dll" "system" fn InitializeXamlDiagnostic(endpointname : windows_sys::core::PCWSTR, pid : u32, wszdllxamldiagnostics : windows_sys::core::PCWSTR, wsztapdllname : windows_sys::core::PCWSTR, tapclsid : windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("windows.ui.xaml.dll" "system" fn InitializeXamlDiagnosticsEx(endpointname : windows_sys::core::PCWSTR, pid : u32, wszdllxamldiagnostics : windows_sys::core::PCWSTR, wsztapdllname : windows_sys::core::PCWSTR, tapclsid : windows_sys::core::GUID, wszinitializationdata : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
pub const Add: VisualMutationType = 0i32;
pub const Animation: BaseValueSource = 12i32;
pub type BaseValueSource = i32;
pub const BaseValueSourceBuiltInStyle: BaseValueSource = 2i32;
pub const BaseValueSourceDefault: BaseValueSource = 1i32;
pub const BaseValueSourceLocal: BaseValueSource = 4i32;
pub const BaseValueSourceStyle: BaseValueSource = 3i32;
pub const BaseValueSourceUnknown: BaseValueSource = 0i32;
pub const BaseValueSourceVisualState: BaseValueSource = 14i32;
pub const Coercion: BaseValueSource = 13i32;
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
pub const DefaultStyleTrigger: BaseValueSource = 6i32;
pub const E_UNKNOWNTYPE: windows_sys::core::HRESULT = 0x802B0028_u32 as _;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct EnumType {
    pub Name: windows_sys::core::BSTR,
    pub ValueInts: *mut super::super::super::System::Com::SAFEARRAY,
    pub ValueStrings: *mut super::super::super::System::Com::SAFEARRAY,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for EnumType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ErrorInvalidResource: VisualElementState = 2i32;
pub const ErrorResolved: VisualElementState = 0i32;
pub const ErrorResourceNotFound: VisualElementState = 1i32;
pub const ImplicitStyleReference: BaseValueSource = 9i32;
pub const Inherited: BaseValueSource = 5i32;
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct MetadataBit(pub i32);
impl MetadataBit {
    pub const None: Self = Self(0i32);
    pub const IsValueHandle: Self = Self(1i32);
    pub const IsPropertyReadOnly: Self = Self(2i32);
    pub const IsValueCollection: Self = Self(4i32);
    pub const IsValueCollectionReadOnly: Self = Self(8i32);
    pub const IsValueBindingExpression: Self = Self(16i32);
    pub const IsValueNull: Self = Self(32i32);
    pub const IsValueHandleAndEvaluatedValue: Self = Self(64i32);
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ParentChildRelation {
    pub Parent: u64,
    pub Child: u64,
    pub ChildIndex: u32,
}
pub const ParentTemplate: BaseValueSource = 10i32;
pub const ParentTemplateTrigger: BaseValueSource = 11i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PropertyChainSource {
    pub Handle: u64,
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
pub const Remove: VisualMutationType = 1i32;
pub const RenderTarget: RenderTargetBitmapOptions = 0i32;
pub const RenderTargetAndChildren: RenderTargetBitmapOptions = 1i32;
pub type RenderTargetBitmapOptions = i32;
pub type ResourceType = i32;
pub const ResourceTypeStatic: ResourceType = 0i32;
pub const ResourceTypeTheme: ResourceType = 1i32;
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
pub const StyleTrigger: BaseValueSource = 8i32;
pub const TemplateTrigger: BaseValueSource = 7i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VisualElement {
    pub Handle: u64,
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
