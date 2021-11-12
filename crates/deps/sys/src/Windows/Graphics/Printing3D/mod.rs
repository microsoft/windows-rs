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
    pub const Abandoned: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
    pub const Slicing: Self = Self(3i32);
    pub const Submitted: Self = Self(4i32);
}
#[repr(transparent)]
pub struct Print3DTaskDetail(pub i32);
impl Print3DTaskDetail {
    pub const Unknown: Self = Self(0i32);
    pub const ModelExceedsPrintBed: Self = Self(1i32);
    pub const UploadFailed: Self = Self(2i32);
    pub const InvalidMaterialSelection: Self = Self(3i32);
    pub const InvalidModel: Self = Self(4i32);
    pub const ModelNotManifold: Self = Self(5i32);
    pub const InvalidPrintTicket: Self = Self(6i32);
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
    pub const Unknown: Self = Self(0i32);
    pub const R32G32B32A32Float: Self = Self(2i32);
    pub const R32G32B32A32UInt: Self = Self(3i32);
    pub const R32G32B32Float: Self = Self(6i32);
    pub const R32G32B32UInt: Self = Self(7i32);
    pub const Printing3DDouble: Self = Self(500i32);
    pub const Printing3DUInt: Self = Self(501i32);
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
    pub const FindFirstError: Self = Self(0i32);
    pub const FindAllErrors: Self = Self(1i32);
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
    pub const Meter: Self = Self(0i32);
    pub const Micron: Self = Self(1i32);
    pub const Millimeter: Self = Self(2i32);
    pub const Centimeter: Self = Self(3i32);
    pub const Inch: Self = Self(4i32);
    pub const Foot: Self = Self(5i32);
}
#[repr(transparent)]
pub struct Printing3DMultiplePropertyMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DMultiplePropertyMaterialGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DObjectType(pub i32);
impl Printing3DObjectType {
    pub const Model: Self = Self(0i32);
    pub const Support: Self = Self(1i32);
    pub const Others: Self = Self(2i32);
}
#[repr(transparent)]
pub struct Printing3DPackageCompression(pub i32);
impl Printing3DPackageCompression {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
}
#[repr(transparent)]
pub struct Printing3DTexture2CoordMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DTexture2CoordMaterialGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DTextureEdgeBehavior(pub i32);
impl Printing3DTextureEdgeBehavior {
    pub const None: Self = Self(0i32);
    pub const Wrap: Self = Self(1i32);
    pub const Mirror: Self = Self(2i32);
    pub const Clamp: Self = Self(3i32);
}
#[repr(transparent)]
pub struct Printing3DTextureResource(pub *mut ::core::ffi::c_void);
