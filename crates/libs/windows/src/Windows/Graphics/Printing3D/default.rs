impl ::core::cmp::PartialEq for Print3DManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DManager {}
impl ::core::fmt::Debug for Print3DManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Print3DTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DTask {}
impl ::core::fmt::Debug for Print3DTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTask").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Print3DTaskCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DTaskCompletedEventArgs {}
impl ::core::fmt::Debug for Print3DTaskCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTaskCompletedEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for Print3DTaskCompletion {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Print3DTaskCompletion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTaskCompletion").field(&self.0).finish()
    }
}
impl ::core::default::Default for Print3DTaskDetail {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Print3DTaskDetail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTaskDetail").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Print3DTaskRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DTaskRequest {}
impl ::core::fmt::Debug for Print3DTaskRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTaskRequest").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Print3DTaskRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DTaskRequestedEventArgs {}
impl ::core::fmt::Debug for Print3DTaskRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTaskRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Print3DTaskSourceChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DTaskSourceChangedEventArgs {}
impl ::core::fmt::Debug for Print3DTaskSourceChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTaskSourceChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Print3DTaskSourceRequestedArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DTaskSourceRequestedArgs {}
impl ::core::fmt::Debug for Print3DTaskSourceRequestedArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTaskSourceRequestedArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Print3DTaskSourceRequestedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DTaskSourceRequestedHandler {}
impl ::core::fmt::Debug for Print3DTaskSourceRequestedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTaskSourceRequestedHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3D3MFPackage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3D3MFPackage {}
impl ::core::fmt::Debug for Printing3D3MFPackage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3D3MFPackage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DBaseMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DBaseMaterial {}
impl ::core::fmt::Debug for Printing3DBaseMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DBaseMaterial").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DBaseMaterialGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DBaseMaterialGroup {}
impl ::core::fmt::Debug for Printing3DBaseMaterialGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DBaseMaterialGroup").field(&self.0).finish()
    }
}
impl ::core::default::Default for Printing3DBufferDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Printing3DBufferDescription {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.Stride == other.Stride
    }
}
impl ::core::cmp::Eq for Printing3DBufferDescription {}
impl ::core::fmt::Debug for Printing3DBufferDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Printing3DBufferDescription").field("Format", &self.Format).field("Stride", &self.Stride).finish()
    }
}
impl ::core::default::Default for Printing3DBufferFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Printing3DBufferFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DBufferFormat").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DColorMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DColorMaterial {}
impl ::core::fmt::Debug for Printing3DColorMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DColorMaterial").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DColorMaterialGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DColorMaterialGroup {}
impl ::core::fmt::Debug for Printing3DColorMaterialGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DColorMaterialGroup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DComponent {}
impl ::core::fmt::Debug for Printing3DComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DComponent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DComponentWithMatrix {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DComponentWithMatrix {}
impl ::core::fmt::Debug for Printing3DComponentWithMatrix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DComponentWithMatrix").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DCompositeMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DCompositeMaterial {}
impl ::core::fmt::Debug for Printing3DCompositeMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DCompositeMaterial").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DCompositeMaterialGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DCompositeMaterialGroup {}
impl ::core::fmt::Debug for Printing3DCompositeMaterialGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DCompositeMaterialGroup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DFaceReductionOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DFaceReductionOptions {}
impl ::core::fmt::Debug for Printing3DFaceReductionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DFaceReductionOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DMaterial {}
impl ::core::fmt::Debug for Printing3DMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DMaterial").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DMesh {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DMesh {}
impl ::core::fmt::Debug for Printing3DMesh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DMesh").field(&self.0).finish()
    }
}
impl ::core::default::Default for Printing3DMeshVerificationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Printing3DMeshVerificationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DMeshVerificationMode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DMeshVerificationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DMeshVerificationResult {}
impl ::core::fmt::Debug for Printing3DMeshVerificationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DMeshVerificationResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DModel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DModel {}
impl ::core::fmt::Debug for Printing3DModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DModel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DModelTexture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DModelTexture {}
impl ::core::fmt::Debug for Printing3DModelTexture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DModelTexture").field(&self.0).finish()
    }
}
impl ::core::default::Default for Printing3DModelUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Printing3DModelUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DModelUnit").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DMultiplePropertyMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DMultiplePropertyMaterial {}
impl ::core::fmt::Debug for Printing3DMultiplePropertyMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DMultiplePropertyMaterial").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DMultiplePropertyMaterialGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DMultiplePropertyMaterialGroup {}
impl ::core::fmt::Debug for Printing3DMultiplePropertyMaterialGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DMultiplePropertyMaterialGroup").field(&self.0).finish()
    }
}
impl ::core::default::Default for Printing3DObjectType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Printing3DObjectType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DObjectType").field(&self.0).finish()
    }
}
impl ::core::default::Default for Printing3DPackageCompression {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Printing3DPackageCompression {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DPackageCompression").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DTexture2CoordMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DTexture2CoordMaterial {}
impl ::core::fmt::Debug for Printing3DTexture2CoordMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DTexture2CoordMaterial").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DTexture2CoordMaterialGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DTexture2CoordMaterialGroup {}
impl ::core::fmt::Debug for Printing3DTexture2CoordMaterialGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DTexture2CoordMaterialGroup").field(&self.0).finish()
    }
}
impl ::core::default::Default for Printing3DTextureEdgeBehavior {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Printing3DTextureEdgeBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DTextureEdgeBehavior").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Printing3DTextureResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DTextureResource {}
impl ::core::fmt::Debug for Printing3DTextureResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DTextureResource").field(&self.0).finish()
    }
}
