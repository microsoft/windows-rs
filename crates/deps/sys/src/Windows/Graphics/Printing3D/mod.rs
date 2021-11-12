#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPrint3DManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrint3DManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrint3DTask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrint3DTaskCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrint3DTaskRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrint3DTaskRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrint3DTaskSourceChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrint3DTaskSourceRequestedArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3D3MFPackage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3D3MFPackage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3D3MFPackageStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DBaseMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DBaseMaterialGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DBaseMaterialGroupFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DBaseMaterialStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DColorMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DColorMaterial2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DColorMaterialGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DColorMaterialGroupFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DComponent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DComponentWithMatrix(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DCompositeMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DCompositeMaterialGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DCompositeMaterialGroup2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DCompositeMaterialGroupFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DFaceReductionOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DMesh(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DMeshVerificationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DModel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DModel2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DModelTexture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DMultiplePropertyMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DMultiplePropertyMaterialGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DMultiplePropertyMaterialGroupFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DTexture2CoordMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DTexture2CoordMaterialGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DTexture2CoordMaterialGroup2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DTexture2CoordMaterialGroupFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DTextureResource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Print3DManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Print3DTask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Print3DTaskCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Print3DTaskCompletion(pub i32);
impl Print3DTaskCompletion {
    pub const Abandoned: Print3DTaskCompletion = Print3DTaskCompletion(0i32);
    pub const Canceled: Print3DTaskCompletion = Print3DTaskCompletion(1i32);
    pub const Failed: Print3DTaskCompletion = Print3DTaskCompletion(2i32);
    pub const Slicing: Print3DTaskCompletion = Print3DTaskCompletion(3i32);
    pub const Submitted: Print3DTaskCompletion = Print3DTaskCompletion(4i32);
}
#[repr(transparent)]
pub struct Print3DTaskDetail(pub i32);
impl Print3DTaskDetail {
    pub const Unknown: Print3DTaskDetail = Print3DTaskDetail(0i32);
    pub const ModelExceedsPrintBed: Print3DTaskDetail = Print3DTaskDetail(1i32);
    pub const UploadFailed: Print3DTaskDetail = Print3DTaskDetail(2i32);
    pub const InvalidMaterialSelection: Print3DTaskDetail = Print3DTaskDetail(3i32);
    pub const InvalidModel: Print3DTaskDetail = Print3DTaskDetail(4i32);
    pub const ModelNotManifold: Print3DTaskDetail = Print3DTaskDetail(5i32);
    pub const InvalidPrintTicket: Print3DTaskDetail = Print3DTaskDetail(6i32);
}
#[repr(transparent)]
pub struct Print3DTaskRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Print3DTaskRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Print3DTaskSourceChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Print3DTaskSourceRequestedArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Print3DTaskSourceRequestedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3D3MFPackage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DBaseMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DBaseMaterialGroup(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct Printing3DBufferDescription(i32);
#[repr(transparent)]
pub struct Printing3DBufferFormat(pub i32);
impl Printing3DBufferFormat {
    pub const Unknown: Printing3DBufferFormat = Printing3DBufferFormat(0i32);
    pub const R32G32B32A32Float: Printing3DBufferFormat = Printing3DBufferFormat(2i32);
    pub const R32G32B32A32UInt: Printing3DBufferFormat = Printing3DBufferFormat(3i32);
    pub const R32G32B32Float: Printing3DBufferFormat = Printing3DBufferFormat(6i32);
    pub const R32G32B32UInt: Printing3DBufferFormat = Printing3DBufferFormat(7i32);
    pub const Printing3DDouble: Printing3DBufferFormat = Printing3DBufferFormat(500i32);
    pub const Printing3DUInt: Printing3DBufferFormat = Printing3DBufferFormat(501i32);
}
#[repr(transparent)]
pub struct Printing3DColorMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DColorMaterialGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DComponent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DComponentWithMatrix(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DCompositeMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DCompositeMaterialGroup(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct Printing3DContract(i32);
#[repr(transparent)]
pub struct Printing3DFaceReductionOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DMesh(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DMeshVerificationMode(pub i32);
impl Printing3DMeshVerificationMode {
    pub const FindFirstError: Printing3DMeshVerificationMode = Printing3DMeshVerificationMode(0i32);
    pub const FindAllErrors: Printing3DMeshVerificationMode = Printing3DMeshVerificationMode(1i32);
}
#[repr(transparent)]
pub struct Printing3DMeshVerificationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DModel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DModelTexture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DModelUnit(pub i32);
impl Printing3DModelUnit {
    pub const Meter: Printing3DModelUnit = Printing3DModelUnit(0i32);
    pub const Micron: Printing3DModelUnit = Printing3DModelUnit(1i32);
    pub const Millimeter: Printing3DModelUnit = Printing3DModelUnit(2i32);
    pub const Centimeter: Printing3DModelUnit = Printing3DModelUnit(3i32);
    pub const Inch: Printing3DModelUnit = Printing3DModelUnit(4i32);
    pub const Foot: Printing3DModelUnit = Printing3DModelUnit(5i32);
}
#[repr(transparent)]
pub struct Printing3DMultiplePropertyMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DMultiplePropertyMaterialGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DObjectType(pub i32);
impl Printing3DObjectType {
    pub const Model: Printing3DObjectType = Printing3DObjectType(0i32);
    pub const Support: Printing3DObjectType = Printing3DObjectType(1i32);
    pub const Others: Printing3DObjectType = Printing3DObjectType(2i32);
}
#[repr(transparent)]
pub struct Printing3DPackageCompression(pub i32);
impl Printing3DPackageCompression {
    pub const Low: Printing3DPackageCompression = Printing3DPackageCompression(0i32);
    pub const Medium: Printing3DPackageCompression = Printing3DPackageCompression(1i32);
    pub const High: Printing3DPackageCompression = Printing3DPackageCompression(2i32);
}
#[repr(transparent)]
pub struct Printing3DTexture2CoordMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DTexture2CoordMaterialGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DTextureEdgeBehavior(pub i32);
impl Printing3DTextureEdgeBehavior {
    pub const None: Printing3DTextureEdgeBehavior = Printing3DTextureEdgeBehavior(0i32);
    pub const Wrap: Printing3DTextureEdgeBehavior = Printing3DTextureEdgeBehavior(1i32);
    pub const Mirror: Printing3DTextureEdgeBehavior = Printing3DTextureEdgeBehavior(2i32);
    pub const Clamp: Printing3DTextureEdgeBehavior = Printing3DTextureEdgeBehavior(3i32);
}
#[repr(transparent)]
pub struct Printing3DTextureResource(pub *mut ::core::ffi::c_void);
