#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for CompositionFrameDisplayInstance {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for CompositionFrameDisplayInstance {
    fn eq(&self, other: &Self) -> bool {
        self.displayAdapterLUID == other.displayAdapterLUID && self.displayVidPnSourceId == other.displayVidPnSourceId && self.displayUniqueId == other.displayUniqueId && self.renderAdapterLUID == other.renderAdapterLUID && self.instanceKind == other.instanceKind && self.finalTransform == other.finalTransform && self.requiredCrossAdapterCopy == other.requiredCrossAdapterCopy && self.colorSpace == other.colorSpace
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for CompositionFrameDisplayInstance {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for CompositionFrameDisplayInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CompositionFrameDisplayInstance").field("displayAdapterLUID", &self.displayAdapterLUID).field("displayVidPnSourceId", &self.displayVidPnSourceId).field("displayUniqueId", &self.displayUniqueId).field("renderAdapterLUID", &self.renderAdapterLUID).field("instanceKind", &self.instanceKind).field("finalTransform", &self.finalTransform).field("requiredCrossAdapterCopy", &self.requiredCrossAdapterCopy).field("colorSpace", &self.colorSpace).finish()
    }
}
impl ::core::default::Default for CompositionFrameInstanceKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CompositionFrameInstanceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionFrameInstanceKind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICompositionFramePresentStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositionFramePresentStatistics {}
impl ::core::fmt::Debug for ICompositionFramePresentStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositionFramePresentStatistics").field(&self.0).finish()
    }
}
impl ICompositionFramePresentStatistics {
    pub unsafe fn GetPresentId(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.GetPresentId)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        (::windows::core::Vtable::vtable(self).base__.GetKind)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IIndependentFlipFramePresentStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIndependentFlipFramePresentStatistics {}
impl ::core::fmt::Debug for IIndependentFlipFramePresentStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIndependentFlipFramePresentStatistics").field(&self.0).finish()
    }
}
impl IIndependentFlipFramePresentStatistics {
    pub unsafe fn GetPresentId(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.GetPresentId)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        (::windows::core::Vtable::vtable(self).base__.GetKind)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IPresentStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPresentStatistics {}
impl ::core::fmt::Debug for IPresentStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPresentStatistics").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPresentStatusPresentStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPresentStatusPresentStatistics {}
impl ::core::fmt::Debug for IPresentStatusPresentStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPresentStatusPresentStatistics").field(&self.0).finish()
    }
}
impl IPresentStatusPresentStatistics {
    pub unsafe fn GetPresentId(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.GetPresentId)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        (::windows::core::Vtable::vtable(self).base__.GetKind)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IPresentationBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPresentationBuffer {}
impl ::core::fmt::Debug for IPresentationBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPresentationBuffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPresentationContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPresentationContent {}
impl ::core::fmt::Debug for IPresentationContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPresentationContent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPresentationFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPresentationFactory {}
impl ::core::fmt::Debug for IPresentationFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPresentationFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPresentationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPresentationManager {}
impl ::core::fmt::Debug for IPresentationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPresentationManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPresentationSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPresentationSurface {}
impl ::core::fmt::Debug for IPresentationSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPresentationSurface").field(&self.0).finish()
    }
}
impl IPresentationSurface {
    pub unsafe fn SetTag(&self, tag: usize) {
        (::windows::core::Vtable::vtable(self).base__.SetTag)(::windows::core::Vtable::as_raw(self), tag)
    }
}
impl ::core::default::Default for PresentStatisticsKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PresentStatisticsKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PresentStatisticsKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for PresentStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PresentStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PresentStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for PresentationTransform {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PresentationTransform {
    fn eq(&self, other: &Self) -> bool {
        self.M11 == other.M11 && self.M12 == other.M12 && self.M21 == other.M21 && self.M22 == other.M22 && self.M31 == other.M31 && self.M32 == other.M32
    }
}
impl ::core::cmp::Eq for PresentationTransform {}
impl ::core::fmt::Debug for PresentationTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PresentationTransform").field("M11", &self.M11).field("M12", &self.M12).field("M21", &self.M21).field("M22", &self.M22).field("M31", &self.M31).field("M32", &self.M32).finish()
    }
}
impl ::core::default::Default for SystemInterruptTime {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SystemInterruptTime {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl ::core::cmp::Eq for SystemInterruptTime {}
impl ::core::fmt::Debug for SystemInterruptTime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SystemInterruptTime").field("value", &self.value).finish()
    }
}
