impl ::core::default::Default for ALT_BREAKS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ALT_BREAKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ALT_BREAKS").field(&self.0).finish()
    }
}
impl ::core::default::Default for AppearanceConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppearanceConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppearanceConstants").field(&self.0).finish()
    }
}
impl ::core::default::Default for BorderStyleConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BorderStyleConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BorderStyleConstants").field(&self.0).finish()
    }
}
impl ::core::default::Default for CHARACTER_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHARACTER_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.wcLow == other.wcLow && self.cChars == other.cChars
    }
}
impl ::core::cmp::Eq for CHARACTER_RANGE {}
impl ::core::fmt::Debug for CHARACTER_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHARACTER_RANGE").field("wcLow", &self.wcLow).field("cChars", &self.cChars).finish()
    }
}
impl ::core::default::Default for CONFIDENCE_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONFIDENCE_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONFIDENCE_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for CorrectionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CorrectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorrectionMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for CorrectionPosition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CorrectionPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorrectionPosition").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_Ink {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_Ink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_Ink").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkCollector {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkCollector").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkCollectorEvent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkCollectorEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkCollectorEvent").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkCursor {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkCursor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkCursor").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkCursorButton {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkCursorButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkCursorButton").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkCursorButtons {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkCursorButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkCursorButtons").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkCursors {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkCursors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkCursors").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkCustomStrokes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkCustomStrokes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkCustomStrokes").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkDivider {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkDivider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkDivider").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkDivisionResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkDivisionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkDivisionResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkDivisionUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkDivisionUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkDivisionUnit").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkDivisionUnits {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkDivisionUnits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkDivisionUnits").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkDrawingAttributes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkDrawingAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkDrawingAttributes").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkEdit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkEdit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkEdit").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkEditEvents {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkEditEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkEditEvents").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkEvent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkEvent").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkExtendedProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkExtendedProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkExtendedProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkExtendedProperty {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkExtendedProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkExtendedProperty").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkGesture {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkGesture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkGesture").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkRecoAlternate {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkRecoAlternate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecoAlternate").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkRecoContext {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkRecoContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecoContext").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkRecoContext2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkRecoContext2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecoContext2").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkRecognitionAlternates {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkRecognitionAlternates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecognitionAlternates").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkRecognitionEvent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkRecognitionEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecognitionEvent").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkRecognitionResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkRecognitionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecognitionResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkRecognizer {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecognizer").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkRecognizer2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkRecognizer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecognizer2").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkRecognizerGuide {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkRecognizerGuide {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecognizerGuide").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkRecognizers {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkRecognizers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecognizers").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkRectangle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkRectangle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRectangle").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkRenderer {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkRenderer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRenderer").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkStrokeDisp {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkStrokeDisp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkStrokeDisp").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkStrokes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkStrokes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkStrokes").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkTablet {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkTablet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkTablet").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkTablet2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkTablet2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkTablet2").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkTablet3 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkTablet3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkTablet3").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkTablets {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkTablets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkTablets").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkTransform {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkTransform").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkWordList {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkWordList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkWordList").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_InkWordList2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_InkWordList2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkWordList2").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_MathInputControlEvents {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_MathInputControlEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_MathInputControlEvents").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_PenInputPanel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_PenInputPanel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_PenInputPanel").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_PenInputPanelEvents {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_PenInputPanelEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_PenInputPanelEvents").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPID_StrokeEvent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPID_StrokeEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_StrokeEvent").field(&self.0).finish()
    }
}
impl ::core::default::Default for DYNAMIC_RENDERER_CACHED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DYNAMIC_RENDERER_CACHED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.strokeId == other.strokeId && self.dynamicRenderer == other.dynamicRenderer
    }
}
impl ::core::cmp::Eq for DYNAMIC_RENDERER_CACHED_DATA {}
impl ::core::fmt::Debug for DYNAMIC_RENDERER_CACHED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DYNAMIC_RENDERER_CACHED_DATA").field("strokeId", &self.strokeId).field("dynamicRenderer", &self.dynamicRenderer).finish()
    }
}
impl ::core::default::Default for EventMask {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EventMask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EventMask").field(&self.0).finish()
    }
}
impl ::core::default::Default for FLICKACTION_COMMANDCODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FLICKACTION_COMMANDCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FLICKACTION_COMMANDCODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FLICKDIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FLICKDIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FLICKDIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for FLICKMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FLICKMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FLICKMODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FLICK_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FLICK_DATA {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for FLICK_DATA {}
impl ::core::fmt::Debug for FLICK_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLICK_DATA").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for FLICK_POINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FLICK_POINT {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for FLICK_POINT {}
impl ::core::fmt::Debug for FLICK_POINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLICK_POINT").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for GESTURE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GESTURE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.gestureId == other.gestureId && self.recoConfidence == other.recoConfidence && self.strokeCount == other.strokeCount
    }
}
impl ::core::cmp::Eq for GESTURE_DATA {}
impl ::core::fmt::Debug for GESTURE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GESTURE_DATA").field("gestureId", &self.gestureId).field("recoConfidence", &self.recoConfidence).field("strokeCount", &self.strokeCount).finish()
    }
}
impl ::core::default::Default for GET_DANDIDATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_DANDIDATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_DANDIDATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDynamicRenderer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDynamicRenderer {}
impl ::core::fmt::Debug for IDynamicRenderer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDynamicRenderer").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Controls"))]
impl ::core::default::Default for IEC_GESTUREINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
impl ::core::default::Default for IEC_RECOGNITIONRESULTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
impl ::core::cmp::PartialEq for IEC_RECOGNITIONRESULTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.RecognitionResult == other.RecognitionResult
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
impl ::core::cmp::Eq for IEC_RECOGNITIONRESULTINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
impl ::core::fmt::Debug for IEC_RECOGNITIONRESULTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IEC_RECOGNITIONRESULTINFO").field("nmhdr", &self.nmhdr).field("RecognitionResult", &self.RecognitionResult).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
impl ::core::default::Default for IEC_STROKEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
impl ::core::cmp::PartialEq for IEC_STROKEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.Cursor == other.Cursor && self.Stroke == other.Stroke
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
impl ::core::cmp::Eq for IEC_STROKEINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
impl ::core::fmt::Debug for IEC_STROKEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IEC_STROKEINFO").field("nmhdr", &self.nmhdr).field("Cursor", &self.Cursor).field("Stroke", &self.Stroke).finish()
    }
}
impl ::core::cmp::PartialEq for IGestureRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGestureRecognizer {}
impl ::core::fmt::Debug for IGestureRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGestureRecognizer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IHandwrittenTextInsertion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHandwrittenTextInsertion {}
impl ::core::fmt::Debug for IHandwrittenTextInsertion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHandwrittenTextInsertion").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInk {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInk").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkCollector {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkCollector").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkCursor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkCursor {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkCursor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkCursor").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkCursorButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkCursorButton {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkCursorButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkCursorButton").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkCursorButtons {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkCursorButtons {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkCursorButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkCursorButtons").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkCursors {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkCursors {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkCursors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkCursors").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkCustomStrokes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkCustomStrokes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkCustomStrokes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkCustomStrokes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkDisp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkDisp {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkDisp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkDisp").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkDivider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkDivider {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkDivider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkDivider").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkDivisionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkDivisionResult {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkDivisionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkDivisionResult").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkDivisionUnit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkDivisionUnit {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkDivisionUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkDivisionUnit").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkDivisionUnits {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkDivisionUnits {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkDivisionUnits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkDivisionUnits").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkDrawingAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkDrawingAttributes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkDrawingAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkDrawingAttributes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkEdit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkEdit {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkEdit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkEdit").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkExtendedProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkExtendedProperties {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkExtendedProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkExtendedProperties").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkExtendedProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkExtendedProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkExtendedProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkExtendedProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkGesture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkGesture {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkGesture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkGesture").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInkLineInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkLineInfo {}
impl ::core::fmt::Debug for IInkLineInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkLineInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkOverlay {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkOverlay {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkOverlay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkOverlay").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkPicture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkPicture {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkPicture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkPicture").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkRecognitionAlternate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkRecognitionAlternate {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkRecognitionAlternate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognitionAlternate").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkRecognitionAlternates {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkRecognitionAlternates {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkRecognitionAlternates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognitionAlternates").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkRecognitionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkRecognitionResult {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkRecognitionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognitionResult").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkRecognizer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognizer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkRecognizer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkRecognizer2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkRecognizer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognizer2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkRecognizerContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkRecognizerContext {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkRecognizerContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognizerContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkRecognizerContext2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkRecognizerContext2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkRecognizerContext2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognizerContext2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkRecognizerGuide {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkRecognizerGuide {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkRecognizerGuide {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognizerGuide").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkRecognizers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkRecognizers {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkRecognizers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognizers").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkRectangle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkRectangle {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkRectangle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRectangle").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkRenderer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkRenderer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkRenderer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRenderer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkStrokeDisp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkStrokeDisp {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkStrokeDisp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkStrokeDisp").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkStrokes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkStrokes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkStrokes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkStrokes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkTablet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkTablet {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkTablet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkTablet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkTablet2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkTablet2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkTablet2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkTablet2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkTablet3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkTablet3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkTablet3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkTablet3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkTablets {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkTablets {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkTablets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkTablets").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkTransform {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkTransform").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkWordList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkWordList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkWordList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkWordList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInkWordList2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInkWordList2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInkWordList2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkWordList2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInputPanelWindowHandle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputPanelWindowHandle {}
impl ::core::fmt::Debug for IInputPanelWindowHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputPanelWindowHandle").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMathInputControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMathInputControl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMathInputControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMathInputControl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INKMETRIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INKMETRIC {
    fn eq(&self, other: &Self) -> bool {
        self.iHeight == other.iHeight && self.iFontAscent == other.iFontAscent && self.iFontDescent == other.iFontDescent && self.dwFlags == other.dwFlags && self.color == other.color
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INKMETRIC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INKMETRIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INKMETRIC").field("iHeight", &self.iHeight).field("iFontAscent", &self.iFontAscent).field("iFontDescent", &self.iFontDescent).field("dwFlags", &self.dwFlags).field("color", &self.color).finish()
    }
}
impl ::core::default::Default for INK_METRIC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INK_METRIC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INK_METRIC_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPenInputPanel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPenInputPanel {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPenInputPanel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPenInputPanel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRealTimeStylus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRealTimeStylus {}
impl ::core::fmt::Debug for IRealTimeStylus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRealTimeStylus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRealTimeStylus2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRealTimeStylus2 {}
impl ::core::fmt::Debug for IRealTimeStylus2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRealTimeStylus2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRealTimeStylus3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRealTimeStylus3 {}
impl ::core::fmt::Debug for IRealTimeStylus3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRealTimeStylus3").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRealTimeStylusSynchronization {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRealTimeStylusSynchronization {}
impl ::core::fmt::Debug for IRealTimeStylusSynchronization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRealTimeStylusSynchronization").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISketchInk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISketchInk {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISketchInk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISketchInk").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IStrokeBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStrokeBuilder {}
impl ::core::fmt::Debug for IStrokeBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStrokeBuilder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IStylusAsyncPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStylusAsyncPlugin {}
impl ::core::fmt::Debug for IStylusAsyncPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStylusAsyncPlugin").field(&self.0).finish()
    }
}
impl IStylusAsyncPlugin {
    pub unsafe fn RealTimeStylusEnabled<P0>(&self, pirtssrc: P0, ptcids: &[u32]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RealTimeStylusEnabled)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), ptcids.len() as _, ::core::mem::transmute(ptcids.as_ptr())).ok()
    }
    pub unsafe fn RealTimeStylusDisabled<P0>(&self, pirtssrc: P0, ptcids: &[u32]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RealTimeStylusDisabled)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), ptcids.len() as _, ::core::mem::transmute(ptcids.as_ptr())).ok()
    }
    pub unsafe fn StylusInRange<P0>(&self, pirtssrc: P0, tcid: u32, sid: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StylusInRange)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), tcid, sid).ok()
    }
    pub unsafe fn StylusOutOfRange<P0>(&self, pirtssrc: P0, tcid: u32, sid: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StylusOutOfRange)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), tcid, sid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StylusDown<P0>(&self, pirtssrc: P0, pstylusinfo: *const StylusInfo, ppacket: &[i32], ppinoutpkt: *mut *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StylusDown)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), pstylusinfo, ppacket.len() as _, ::core::mem::transmute(ppacket.as_ptr()), ppinoutpkt).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StylusUp<P0>(&self, pirtssrc: P0, pstylusinfo: *const StylusInfo, ppacket: &[i32], ppinoutpkt: *mut *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StylusUp)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), pstylusinfo, ppacket.len() as _, ::core::mem::transmute(ppacket.as_ptr()), ppinoutpkt).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StylusButtonDown<P0>(&self, pirtssrc: P0, sid: u32, pguidstylusbutton: *const ::windows::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StylusButtonDown)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), sid, pguidstylusbutton, pstyluspos).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StylusButtonUp<P0>(&self, pirtssrc: P0, sid: u32, pguidstylusbutton: *const ::windows::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StylusButtonUp)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), sid, pguidstylusbutton, pstyluspos).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InAirPackets<P0>(&self, pirtssrc: P0, pstylusinfo: *const StylusInfo, cpktcount: u32, ppackets: &[i32], pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InAirPackets)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), pstylusinfo, cpktcount, ppackets.len() as _, ::core::mem::transmute(ppackets.as_ptr()), pcinoutpkts, ppinoutpkts).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Packets<P0>(&self, pirtssrc: P0, pstylusinfo: *const StylusInfo, cpktcount: u32, ppackets: &[i32], pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Packets)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), pstylusinfo, cpktcount, ppackets.len() as _, ::core::mem::transmute(ppackets.as_ptr()), pcinoutpkts, ppinoutpkts).ok()
    }
    pub unsafe fn CustomStylusDataAdded<P0>(&self, pirtssrc: P0, pguidid: *const ::windows::core::GUID, pbdata: ::core::option::Option<&[u8]>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CustomStylusDataAdded)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), pguidid, pbdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pbdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    pub unsafe fn SystemEvent<P0>(&self, pirtssrc: P0, tcid: u32, sid: u32, event: u16, eventdata: SYSTEM_EVENT_DATA) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SystemEvent)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), tcid, sid, event, ::core::mem::transmute(eventdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TabletAdded<P0, P1>(&self, pirtssrc: P0, pitablet: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
        P1: ::std::convert::Into<::windows::core::InParam<IInkTablet>>,
    {
        (::windows::core::Vtable::vtable(self).base__.TabletAdded)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), pitablet.into().abi()).ok()
    }
    pub unsafe fn TabletRemoved<P0>(&self, pirtssrc: P0, itabletindex: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.TabletRemoved)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), itabletindex).ok()
    }
    pub unsafe fn Error<P0, P1>(&self, pirtssrc: P0, piplugin: P1, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows::core::HRESULT, lptrkey: *mut isize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
        P1: ::std::convert::Into<::windows::core::InParam<IStylusPlugin>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Error)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), piplugin.into().abi(), datainterest, hrerrorcode, lptrkey).ok()
    }
    pub unsafe fn UpdateMapping<P0>(&self, pirtssrc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UpdateMapping)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi()).ok()
    }
    pub unsafe fn DataInterest(&self) -> ::windows::core::Result<RealTimeStylusDataInterest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DataInterest)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IStylusPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStylusPlugin {}
impl ::core::fmt::Debug for IStylusPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStylusPlugin").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IStylusSyncPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStylusSyncPlugin {}
impl ::core::fmt::Debug for IStylusSyncPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStylusSyncPlugin").field(&self.0).finish()
    }
}
impl IStylusSyncPlugin {
    pub unsafe fn RealTimeStylusEnabled<P0>(&self, pirtssrc: P0, ptcids: &[u32]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RealTimeStylusEnabled)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), ptcids.len() as _, ::core::mem::transmute(ptcids.as_ptr())).ok()
    }
    pub unsafe fn RealTimeStylusDisabled<P0>(&self, pirtssrc: P0, ptcids: &[u32]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RealTimeStylusDisabled)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), ptcids.len() as _, ::core::mem::transmute(ptcids.as_ptr())).ok()
    }
    pub unsafe fn StylusInRange<P0>(&self, pirtssrc: P0, tcid: u32, sid: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StylusInRange)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), tcid, sid).ok()
    }
    pub unsafe fn StylusOutOfRange<P0>(&self, pirtssrc: P0, tcid: u32, sid: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StylusOutOfRange)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), tcid, sid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StylusDown<P0>(&self, pirtssrc: P0, pstylusinfo: *const StylusInfo, ppacket: &[i32], ppinoutpkt: *mut *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StylusDown)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), pstylusinfo, ppacket.len() as _, ::core::mem::transmute(ppacket.as_ptr()), ppinoutpkt).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StylusUp<P0>(&self, pirtssrc: P0, pstylusinfo: *const StylusInfo, ppacket: &[i32], ppinoutpkt: *mut *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StylusUp)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), pstylusinfo, ppacket.len() as _, ::core::mem::transmute(ppacket.as_ptr()), ppinoutpkt).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StylusButtonDown<P0>(&self, pirtssrc: P0, sid: u32, pguidstylusbutton: *const ::windows::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StylusButtonDown)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), sid, pguidstylusbutton, pstyluspos).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StylusButtonUp<P0>(&self, pirtssrc: P0, sid: u32, pguidstylusbutton: *const ::windows::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StylusButtonUp)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), sid, pguidstylusbutton, pstyluspos).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InAirPackets<P0>(&self, pirtssrc: P0, pstylusinfo: *const StylusInfo, cpktcount: u32, ppackets: &[i32], pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InAirPackets)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), pstylusinfo, cpktcount, ppackets.len() as _, ::core::mem::transmute(ppackets.as_ptr()), pcinoutpkts, ppinoutpkts).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Packets<P0>(&self, pirtssrc: P0, pstylusinfo: *const StylusInfo, cpktcount: u32, ppackets: &[i32], pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Packets)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), pstylusinfo, cpktcount, ppackets.len() as _, ::core::mem::transmute(ppackets.as_ptr()), pcinoutpkts, ppinoutpkts).ok()
    }
    pub unsafe fn CustomStylusDataAdded<P0>(&self, pirtssrc: P0, pguidid: *const ::windows::core::GUID, pbdata: ::core::option::Option<&[u8]>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CustomStylusDataAdded)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), pguidid, pbdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pbdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    pub unsafe fn SystemEvent<P0>(&self, pirtssrc: P0, tcid: u32, sid: u32, event: u16, eventdata: SYSTEM_EVENT_DATA) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SystemEvent)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), tcid, sid, event, ::core::mem::transmute(eventdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TabletAdded<P0, P1>(&self, pirtssrc: P0, pitablet: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
        P1: ::std::convert::Into<::windows::core::InParam<IInkTablet>>,
    {
        (::windows::core::Vtable::vtable(self).base__.TabletAdded)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), pitablet.into().abi()).ok()
    }
    pub unsafe fn TabletRemoved<P0>(&self, pirtssrc: P0, itabletindex: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.TabletRemoved)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), itabletindex).ok()
    }
    pub unsafe fn Error<P0, P1>(&self, pirtssrc: P0, piplugin: P1, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows::core::HRESULT, lptrkey: *mut isize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
        P1: ::std::convert::Into<::windows::core::InParam<IStylusPlugin>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Error)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi(), piplugin.into().abi(), datainterest, hrerrorcode, lptrkey).ok()
    }
    pub unsafe fn UpdateMapping<P0>(&self, pirtssrc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRealTimeStylus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UpdateMapping)(::windows::core::Vtable::as_raw(self), pirtssrc.into().abi()).ok()
    }
    pub unsafe fn DataInterest(&self) -> ::windows::core::Result<RealTimeStylusDataInterest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DataInterest)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITextInputPanel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextInputPanel {}
impl ::core::fmt::Debug for ITextInputPanel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextInputPanel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextInputPanelEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextInputPanelEventSink {}
impl ::core::fmt::Debug for ITextInputPanelEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextInputPanelEventSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextInputPanelRunInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextInputPanelRunInfo {}
impl ::core::fmt::Debug for ITextInputPanelRunInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextInputPanelRunInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITipAutoCompleteClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITipAutoCompleteClient {}
impl ::core::fmt::Debug for ITipAutoCompleteClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITipAutoCompleteClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITipAutoCompleteProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITipAutoCompleteProvider {}
impl ::core::fmt::Debug for ITipAutoCompleteProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITipAutoCompleteProvider").field(&self.0).finish()
    }
}
impl ::core::default::Default for InPlaceDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InPlaceDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InPlaceDirection").field(&self.0).finish()
    }
}
impl ::core::default::Default for InPlaceState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InPlaceState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InPlaceState").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkApplicationGesture {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkApplicationGesture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkApplicationGesture").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkBoundingBoxMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkBoundingBoxMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkBoundingBoxMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkClipboardFormats {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkClipboardFormats {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkClipboardFormats").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkClipboardModes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkClipboardModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkClipboardModes").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkCollectionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkCollectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkCollectionMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkCollectorEventInterest {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkCollectorEventInterest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkCollectorEventInterest").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkCursorButtonState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkCursorButtonState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkCursorButtonState").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkDisplayMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkDisplayMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkDisplayMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkDivisionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkDivisionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkDivisionType").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkEditStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkEditStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkEditStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkExtractFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkExtractFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkExtractFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkInsertMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkInsertMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkInsertMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkMouseButton {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkMouseButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkMouseButton").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkMousePointer {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkMousePointer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkMousePointer").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkOverlayAttachMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkOverlayAttachMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkOverlayAttachMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkOverlayEditingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkOverlayEditingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkOverlayEditingMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkOverlayEraserMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkOverlayEraserMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkOverlayEraserMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkPenTip {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkPenTip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPenTip").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkPersistenceCompressionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkPersistenceCompressionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPersistenceCompressionMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkPersistenceFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkPersistenceFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPersistenceFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkPictureSizeMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkPictureSizeMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPictureSizeMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkRasterOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkRasterOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRasterOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for InkRecoGuide {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for InkRecoGuide {
    fn eq(&self, other: &Self) -> bool {
        self.rectWritingBox == other.rectWritingBox && self.rectDrawnBox == other.rectDrawnBox && self.cRows == other.cRows && self.cColumns == other.cColumns && self.midline == other.midline
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for InkRecoGuide {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for InkRecoGuide {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("InkRecoGuide").field("rectWritingBox", &self.rectWritingBox).field("rectDrawnBox", &self.rectDrawnBox).field("cRows", &self.cRows).field("cColumns", &self.cColumns).field("midline", &self.midline).finish()
    }
}
impl ::core::default::Default for InkRecognitionAlternatesSelection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkRecognitionAlternatesSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognitionAlternatesSelection").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkRecognitionConfidence {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkRecognitionConfidence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognitionConfidence").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkRecognitionModes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkRecognitionModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognitionModes").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkRecognitionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkRecognitionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognitionStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkRecognizerCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkRecognizerCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognizerCapabilities").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkRecognizerCharacterAutoCompletionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkRecognizerCharacterAutoCompletionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognizerCharacterAutoCompletionMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkSelectionConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkSelectionConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkSelectionConstants").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkShiftKeyModifierFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkShiftKeyModifierFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkShiftKeyModifierFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for InkSystemGesture {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InkSystemGesture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkSystemGesture").field(&self.0).finish()
    }
}
impl ::core::default::Default for InteractionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InteractionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for KEYMODIFIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KEYMODIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KEYMODIFIER").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LATTICE_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LATTICE_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.lsBaseline == other.lsBaseline && self.iMidlineOffset == other.iMidlineOffset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LATTICE_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LATTICE_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LATTICE_METRICS").field("lsBaseline", &self.lsBaseline).field("iMidlineOffset", &self.iMidlineOffset).finish()
    }
}
impl ::core::default::Default for LINE_METRICS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LINE_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LINE_METRICS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LINE_SEGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LINE_SEGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.PtA == other.PtA && self.PtB == other.PtB
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LINE_SEGMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LINE_SEGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LINE_SEGMENT").field("PtA", &self.PtA).field("PtB", &self.PtB).finish()
    }
}
impl ::core::default::Default for MICUIELEMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MICUIELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MICUIELEMENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for MICUIELEMENTSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MICUIELEMENTSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MICUIELEMENTSTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MouseButton {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MouseButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MouseButton").field(&self.0).finish()
    }
}
impl ::core::default::Default for PACKET_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PACKET_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.cbPacketSize == other.cbPacketSize && self.cPacketProperties == other.cPacketProperties && self.pPacketProperties == other.pPacketProperties && self.cButtons == other.cButtons && self.pguidButtons == other.pguidButtons
    }
}
impl ::core::cmp::Eq for PACKET_DESCRIPTION {}
impl ::core::fmt::Debug for PACKET_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PACKET_DESCRIPTION").field("cbPacketSize", &self.cbPacketSize).field("cPacketProperties", &self.cPacketProperties).field("pPacketProperties", &self.pPacketProperties).field("cButtons", &self.cButtons).field("pguidButtons", &self.pguidButtons).finish()
    }
}
impl ::core::default::Default for PACKET_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PACKET_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.PropertyMetrics == other.PropertyMetrics
    }
}
impl ::core::cmp::Eq for PACKET_PROPERTY {}
impl ::core::fmt::Debug for PACKET_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PACKET_PROPERTY").field("guid", &self.guid).field("PropertyMetrics", &self.PropertyMetrics).finish()
    }
}
impl ::core::default::Default for PROPERTY_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROPERTY_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.nLogicalMin == other.nLogicalMin && self.nLogicalMax == other.nLogicalMax && self.Units == other.Units && self.fResolution == other.fResolution
    }
}
impl ::core::cmp::Eq for PROPERTY_METRICS {}
impl ::core::fmt::Debug for PROPERTY_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROPERTY_METRICS").field("nLogicalMin", &self.nLogicalMin).field("nLogicalMax", &self.nLogicalMax).field("Units", &self.Units).field("fResolution", &self.fResolution).finish()
    }
}
impl ::core::default::Default for PROPERTY_UNITS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROPERTY_UNITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPERTY_UNITS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PanelInputArea {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PanelInputArea {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PanelInputArea").field(&self.0).finish()
    }
}
impl ::core::default::Default for PanelType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PanelType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PanelType").field(&self.0).finish()
    }
}
impl ::core::default::Default for RECO_ATTRS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RECO_ATTRS {
    fn eq(&self, other: &Self) -> bool {
        self.dwRecoCapabilityFlags == other.dwRecoCapabilityFlags && self.awcVendorName == other.awcVendorName && self.awcFriendlyName == other.awcFriendlyName && self.awLanguageId == other.awLanguageId
    }
}
impl ::core::cmp::Eq for RECO_ATTRS {}
impl ::core::fmt::Debug for RECO_ATTRS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECO_ATTRS").field("dwRecoCapabilityFlags", &self.dwRecoCapabilityFlags).field("awcVendorName", &self.awcVendorName).field("awcFriendlyName", &self.awcFriendlyName).field("awLanguageId", &self.awLanguageId).finish()
    }
}
impl ::core::default::Default for RECO_GUIDE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RECO_GUIDE {
    fn eq(&self, other: &Self) -> bool {
        self.xOrigin == other.xOrigin && self.yOrigin == other.yOrigin && self.cxBox == other.cxBox && self.cyBox == other.cyBox && self.cxBase == other.cxBase && self.cyBase == other.cyBase && self.cHorzBox == other.cHorzBox && self.cVertBox == other.cVertBox && self.cyMid == other.cyMid
    }
}
impl ::core::cmp::Eq for RECO_GUIDE {}
impl ::core::fmt::Debug for RECO_GUIDE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECO_GUIDE").field("xOrigin", &self.xOrigin).field("yOrigin", &self.yOrigin).field("cxBox", &self.cxBox).field("cyBox", &self.cyBox).field("cxBase", &self.cxBase).field("cyBase", &self.cyBase).field("cHorzBox", &self.cHorzBox).field("cVertBox", &self.cVertBox).field("cyMid", &self.cyMid).finish()
    }
}
impl ::core::default::Default for RECO_LATTICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RECO_LATTICE {
    fn eq(&self, other: &Self) -> bool {
        self.ulColumnCount == other.ulColumnCount && self.pLatticeColumns == other.pLatticeColumns && self.ulPropertyCount == other.ulPropertyCount && self.pGuidProperties == other.pGuidProperties && self.ulBestResultColumnCount == other.ulBestResultColumnCount && self.pulBestResultColumns == other.pulBestResultColumns && self.pulBestResultIndexes == other.pulBestResultIndexes
    }
}
impl ::core::cmp::Eq for RECO_LATTICE {}
impl ::core::fmt::Debug for RECO_LATTICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECO_LATTICE").field("ulColumnCount", &self.ulColumnCount).field("pLatticeColumns", &self.pLatticeColumns).field("ulPropertyCount", &self.ulPropertyCount).field("pGuidProperties", &self.pGuidProperties).field("ulBestResultColumnCount", &self.ulBestResultColumnCount).field("pulBestResultColumns", &self.pulBestResultColumns).field("pulBestResultIndexes", &self.pulBestResultIndexes).finish()
    }
}
impl ::core::default::Default for RECO_LATTICE_COLUMN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RECO_LATTICE_COLUMN {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.cpProp == other.cpProp && self.cStrokes == other.cStrokes && self.pStrokes == other.pStrokes && self.cLatticeElements == other.cLatticeElements && self.pLatticeElements == other.pLatticeElements
    }
}
impl ::core::cmp::Eq for RECO_LATTICE_COLUMN {}
impl ::core::fmt::Debug for RECO_LATTICE_COLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECO_LATTICE_COLUMN").field("key", &self.key).field("cpProp", &self.cpProp).field("cStrokes", &self.cStrokes).field("pStrokes", &self.pStrokes).field("cLatticeElements", &self.cLatticeElements).field("pLatticeElements", &self.pLatticeElements).finish()
    }
}
impl ::core::default::Default for RECO_LATTICE_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RECO_LATTICE_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.r#type == other.r#type && self.pData == other.pData && self.ulNextColumn == other.ulNextColumn && self.ulStrokeNumber == other.ulStrokeNumber && self.epProp == other.epProp
    }
}
impl ::core::cmp::Eq for RECO_LATTICE_ELEMENT {}
impl ::core::fmt::Debug for RECO_LATTICE_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECO_LATTICE_ELEMENT").field("score", &self.score).field("type", &self.r#type).field("pData", &self.pData).field("ulNextColumn", &self.ulNextColumn).field("ulStrokeNumber", &self.ulStrokeNumber).field("epProp", &self.epProp).finish()
    }
}
impl ::core::default::Default for RECO_LATTICE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RECO_LATTICE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.cProperties == other.cProperties && self.apProps == other.apProps
    }
}
impl ::core::cmp::Eq for RECO_LATTICE_PROPERTIES {}
impl ::core::fmt::Debug for RECO_LATTICE_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECO_LATTICE_PROPERTIES").field("cProperties", &self.cProperties).field("apProps", &self.apProps).finish()
    }
}
impl ::core::default::Default for RECO_LATTICE_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RECO_LATTICE_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.guidProperty == other.guidProperty && self.cbPropertyValue == other.cbPropertyValue && self.pPropertyValue == other.pPropertyValue
    }
}
impl ::core::cmp::Eq for RECO_LATTICE_PROPERTY {}
impl ::core::fmt::Debug for RECO_LATTICE_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECO_LATTICE_PROPERTY").field("guidProperty", &self.guidProperty).field("cbPropertyValue", &self.cbPropertyValue).field("pPropertyValue", &self.pPropertyValue).finish()
    }
}
impl ::core::default::Default for RECO_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RECO_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.iwcBegin == other.iwcBegin && self.cCount == other.cCount
    }
}
impl ::core::cmp::Eq for RECO_RANGE {}
impl ::core::fmt::Debug for RECO_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECO_RANGE").field("iwcBegin", &self.iwcBegin).field("cCount", &self.cCount).finish()
    }
}
impl ::core::default::Default for RECO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RECO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RECO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RealTimeStylusDataInterest {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RealTimeStylusDataInterest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RealTimeStylusDataInterest").field(&self.0).finish()
    }
}
impl ::core::default::Default for RealTimeStylusLockType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RealTimeStylusLockType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RealTimeStylusLockType").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCROLLDIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCROLLDIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCROLLDIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for STROKE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STROKE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.iStrokeBegin == other.iStrokeBegin && self.iStrokeEnd == other.iStrokeEnd
    }
}
impl ::core::cmp::Eq for STROKE_RANGE {}
impl ::core::fmt::Debug for STROKE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STROKE_RANGE").field("iStrokeBegin", &self.iStrokeBegin).field("iStrokeEnd", &self.iStrokeEnd).finish()
    }
}
impl ::core::default::Default for SYSTEM_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.bModifier == other.bModifier && self.wKey == other.wKey && self.xPos == other.xPos && self.yPos == other.yPos && self.bCursorMode == other.bCursorMode && self.dwButtonState == other.dwButtonState
    }
}
impl ::core::cmp::Eq for SYSTEM_EVENT_DATA {}
impl ::core::fmt::Debug for SYSTEM_EVENT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_EVENT_DATA").field("bModifier", &self.bModifier).field("wKey", &self.wKey).field("xPos", &self.xPos).field("yPos", &self.yPos).field("bCursorMode", &self.bCursorMode).field("dwButtonState", &self.dwButtonState).finish()
    }
}
impl ::core::default::Default for ScrollBarsConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ScrollBarsConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScrollBarsConstants").field(&self.0).finish()
    }
}
impl ::core::default::Default for SelAlignmentConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SelAlignmentConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SelAlignmentConstants").field(&self.0).finish()
    }
}
impl ::core::default::Default for SelectionHitResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SelectionHitResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SelectionHitResult").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for StylusInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for StylusInfo {
    fn eq(&self, other: &Self) -> bool {
        self.tcid == other.tcid && self.cid == other.cid && self.bIsInvertedCursor == other.bIsInvertedCursor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for StylusInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for StylusInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StylusInfo").field("tcid", &self.tcid).field("cid", &self.cid).field("bIsInvertedCursor", &self.bIsInvertedCursor).finish()
    }
}
impl ::core::default::Default for StylusQueue {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for StylusQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StylusQueue").field(&self.0).finish()
    }
}
impl ::core::default::Default for TabletDeviceKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TabletDeviceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TabletDeviceKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for TabletHardwareCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TabletHardwareCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TabletHardwareCapabilities").field(&self.0).finish()
    }
}
impl ::core::default::Default for TabletPropertyMetricUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TabletPropertyMetricUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TabletPropertyMetricUnit").field(&self.0).finish()
    }
}
impl ::core::default::Default for VisualState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VisualState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualState").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _IInkCollectorEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _IInkCollectorEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _IInkCollectorEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IInkCollectorEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _IInkEditEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _IInkEditEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _IInkEditEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IInkEditEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _IInkEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _IInkEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _IInkEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IInkEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _IInkOverlayEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _IInkOverlayEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _IInkOverlayEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IInkOverlayEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _IInkPictureEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _IInkPictureEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _IInkPictureEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IInkPictureEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _IInkRecognitionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _IInkRecognitionEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _IInkRecognitionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IInkRecognitionEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _IInkStrokesEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _IInkStrokesEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _IInkStrokesEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IInkStrokesEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _IMathInputControlEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _IMathInputControlEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _IMathInputControlEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IMathInputControlEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _IPenInputPanelEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _IPenInputPanelEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _IPenInputPanelEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IPenInputPanelEvents").field(&self.0).finish()
    }
}
