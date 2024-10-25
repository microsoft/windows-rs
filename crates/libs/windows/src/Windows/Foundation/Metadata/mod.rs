windows_core::imp::define_interface!(IApiInformationStatics, IApiInformationStatics_Vtbl, 0x997439fe_f681_4a11_b416_c13a47e8ba36);
impl windows_core::RuntimeType for IApiInformationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApiInformationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsTypePresent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub IsMethodPresent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub IsMethodPresentWithArity: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *mut bool) -> windows_core::HRESULT,
    pub IsEventPresent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub IsPropertyPresent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub IsReadOnlyPropertyPresent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub IsWriteablePropertyPresent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub IsEnumNamedValuePresent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub IsApiContractPresentByMajor: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u16, *mut bool) -> windows_core::HRESULT,
    pub IsApiContractPresentByMajorAndMinor: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u16, u16, *mut bool) -> windows_core::HRESULT,
}
pub struct ApiInformation;
impl ApiInformation {}
impl windows_core::RuntimeName for ApiInformation {
    const NAME: &'static str = "Windows.Foundation.Metadata.ApiInformation";
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AttributeTargets(pub u32);
impl AttributeTargets {
    pub const All: Self = Self(4294967295u32);
    pub const Delegate: Self = Self(1u32);
    pub const Enum: Self = Self(2u32);
    pub const Event: Self = Self(4u32);
    pub const Field: Self = Self(8u32);
    pub const Interface: Self = Self(16u32);
    pub const Method: Self = Self(64u32);
    pub const Parameter: Self = Self(128u32);
    pub const Property: Self = Self(256u32);
    pub const RuntimeClass: Self = Self(512u32);
    pub const Struct: Self = Self(1024u32);
    pub const InterfaceImpl: Self = Self(2048u32);
    pub const ApiContract: Self = Self(8192u32);
}
impl windows_core::TypeKind for AttributeTargets {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AttributeTargets {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.AttributeTargets;u4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompositionType(pub i32);
impl CompositionType {
    pub const Protected: Self = Self(1i32);
    pub const Public: Self = Self(2i32);
}
impl windows_core::TypeKind for CompositionType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CompositionType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.CompositionType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeprecationType(pub i32);
impl DeprecationType {
    pub const Deprecate: Self = Self(0i32);
    pub const Remove: Self = Self(1i32);
}
impl windows_core::TypeKind for DeprecationType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DeprecationType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.DeprecationType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FeatureStage(pub i32);
impl FeatureStage {
    pub const AlwaysDisabled: Self = Self(0i32);
    pub const DisabledByDefault: Self = Self(1i32);
    pub const EnabledByDefault: Self = Self(2i32);
    pub const AlwaysEnabled: Self = Self(3i32);
}
impl windows_core::TypeKind for FeatureStage {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for FeatureStage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.FeatureStage;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GCPressureAmount(pub i32);
impl GCPressureAmount {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
}
impl windows_core::TypeKind for GCPressureAmount {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GCPressureAmount {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.GCPressureAmount;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MarshalingType(pub i32);
impl MarshalingType {
    pub const None: Self = Self(1i32);
    pub const Agile: Self = Self(2i32);
    pub const Standard: Self = Self(3i32);
    pub const InvalidMarshaling: Self = Self(0i32);
}
impl windows_core::TypeKind for MarshalingType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for MarshalingType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.MarshalingType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Platform(pub i32);
impl Platform {
    pub const Windows: Self = Self(0i32);
    pub const WindowsPhone: Self = Self(1i32);
}
impl windows_core::TypeKind for Platform {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Platform {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.Platform;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ThreadingModel(pub i32);
impl ThreadingModel {
    pub const STA: Self = Self(1i32);
    pub const MTA: Self = Self(2i32);
    pub const Both: Self = Self(3i32);
    pub const InvalidThreading: Self = Self(0i32);
}
impl windows_core::TypeKind for ThreadingModel {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ThreadingModel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.ThreadingModel;i4)");
}
