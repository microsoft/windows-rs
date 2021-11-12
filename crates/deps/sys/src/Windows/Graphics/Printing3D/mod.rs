#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct Print3DTaskCompletion(i32);
#[repr(C)]
pub struct Print3DTaskDetail(i32);
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
#[repr(C)]
pub struct Printing3DBufferFormat(i32);
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
#[repr(C)]
pub struct Printing3DMeshVerificationMode(i32);
#[repr(transparent)]
pub struct Printing3DMeshVerificationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DModel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DModelTexture(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct Printing3DModelUnit(i32);
#[repr(transparent)]
pub struct Printing3DMultiplePropertyMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DMultiplePropertyMaterialGroup(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct Printing3DObjectType(i32);
#[repr(C)]
pub struct Printing3DPackageCompression(i32);
#[repr(transparent)]
pub struct Printing3DTexture2CoordMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Printing3DTexture2CoordMaterialGroup(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct Printing3DTextureEdgeBehavior(i32);
#[repr(transparent)]
pub struct Printing3DTextureResource(pub *mut ::core::ffi::c_void);
