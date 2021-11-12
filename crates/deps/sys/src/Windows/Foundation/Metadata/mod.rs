#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AttributeTargets(pub u32);
impl AttributeTargets {
    pub const All: AttributeTargets = AttributeTargets(4294967295u32);
    pub const Delegate: AttributeTargets = AttributeTargets(1u32);
    pub const Enum: AttributeTargets = AttributeTargets(2u32);
    pub const Event: AttributeTargets = AttributeTargets(4u32);
    pub const Field: AttributeTargets = AttributeTargets(8u32);
    pub const Interface: AttributeTargets = AttributeTargets(16u32);
    pub const Method: AttributeTargets = AttributeTargets(64u32);
    pub const Parameter: AttributeTargets = AttributeTargets(128u32);
    pub const Property: AttributeTargets = AttributeTargets(256u32);
    pub const RuntimeClass: AttributeTargets = AttributeTargets(512u32);
    pub const Struct: AttributeTargets = AttributeTargets(1024u32);
    pub const InterfaceImpl: AttributeTargets = AttributeTargets(2048u32);
    pub const ApiContract: AttributeTargets = AttributeTargets(8192u32);
}
#[repr(transparent)]
pub struct CompositionType(pub i32);
impl CompositionType {
    pub const Protected: CompositionType = CompositionType(1i32);
    pub const Public: CompositionType = CompositionType(2i32);
}
#[repr(transparent)]
pub struct DeprecationType(pub i32);
impl DeprecationType {
    pub const Deprecate: DeprecationType = DeprecationType(0i32);
    pub const Remove: DeprecationType = DeprecationType(1i32);
}
#[repr(transparent)]
pub struct FeatureStage(pub i32);
impl FeatureStage {
    pub const AlwaysDisabled: FeatureStage = FeatureStage(0i32);
    pub const DisabledByDefault: FeatureStage = FeatureStage(1i32);
    pub const EnabledByDefault: FeatureStage = FeatureStage(2i32);
    pub const AlwaysEnabled: FeatureStage = FeatureStage(3i32);
}
#[repr(transparent)]
pub struct GCPressureAmount(pub i32);
impl GCPressureAmount {
    pub const Low: GCPressureAmount = GCPressureAmount(0i32);
    pub const Medium: GCPressureAmount = GCPressureAmount(1i32);
    pub const High: GCPressureAmount = GCPressureAmount(2i32);
}
#[repr(transparent)]
pub struct IApiInformationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MarshalingType(pub i32);
impl MarshalingType {
    pub const None: MarshalingType = MarshalingType(1i32);
    pub const Agile: MarshalingType = MarshalingType(2i32);
    pub const Standard: MarshalingType = MarshalingType(3i32);
    pub const InvalidMarshaling: MarshalingType = MarshalingType(0i32);
}
#[repr(transparent)]
pub struct Platform(pub i32);
impl Platform {
    pub const Windows: Platform = Platform(0i32);
    pub const WindowsPhone: Platform = Platform(1i32);
}
#[repr(transparent)]
pub struct ThreadingModel(pub i32);
impl ThreadingModel {
    pub const STA: ThreadingModel = ThreadingModel(1i32);
    pub const MTA: ThreadingModel = ThreadingModel(2i32);
    pub const Both: ThreadingModel = ThreadingModel(3i32);
    pub const InvalidThreading: ThreadingModel = ThreadingModel(0i32);
}
