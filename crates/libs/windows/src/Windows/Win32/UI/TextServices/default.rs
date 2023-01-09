impl ::core::default::Default for ANCHOR_CHANGE_HISTORY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ANCHOR_CHANGE_HISTORY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ANCHOR_CHANGE_HISTORY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ANCHOR_CHANGE_HISTORY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ANCHOR_CHANGE_HISTORY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ANCHOR_CHANGE_HISTORY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ANCHOR_CHANGE_HISTORY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ANCHOR_CHANGE_HISTORY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_TEXT_AND_PROPERTY_UPDATES_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for IAccClientDocMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccClientDocMgr {}
impl ::core::fmt::Debug for IAccClientDocMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccClientDocMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAccDictionary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccDictionary {}
impl ::core::fmt::Debug for IAccDictionary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccDictionary").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAccServerDocMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccServerDocMgr {}
impl ::core::fmt::Debug for IAccServerDocMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccServerDocMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAccStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccStore {}
impl ::core::fmt::Debug for IAccStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccStore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAnchor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAnchor {}
impl ::core::fmt::Debug for IAnchor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAnchor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IClonableWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClonableWrapper {}
impl ::core::fmt::Debug for IClonableWrapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClonableWrapper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICoCreateLocally {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoCreateLocally {}
impl ::core::fmt::Debug for ICoCreateLocally {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoCreateLocally").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICoCreatedLocally {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoCreatedLocally {}
impl ::core::fmt::Debug for ICoCreatedLocally {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoCreatedLocally").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDocWrap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDocWrap {}
impl ::core::fmt::Debug for IDocWrap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDocWrap").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumITfCompositionView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumITfCompositionView {}
impl ::core::fmt::Debug for IEnumITfCompositionView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumITfCompositionView").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumSpeechCommands {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSpeechCommands {}
impl ::core::fmt::Debug for IEnumSpeechCommands {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSpeechCommands").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTfCandidates {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfCandidates {}
impl ::core::fmt::Debug for IEnumTfCandidates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfCandidates").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTfContextViews {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfContextViews {}
impl ::core::fmt::Debug for IEnumTfContextViews {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfContextViews").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTfContexts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfContexts {}
impl ::core::fmt::Debug for IEnumTfContexts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfContexts").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTfDisplayAttributeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfDisplayAttributeInfo {}
impl ::core::fmt::Debug for IEnumTfDisplayAttributeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfDisplayAttributeInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTfDocumentMgrs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfDocumentMgrs {}
impl ::core::fmt::Debug for IEnumTfDocumentMgrs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfDocumentMgrs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTfFunctionProviders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfFunctionProviders {}
impl ::core::fmt::Debug for IEnumTfFunctionProviders {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfFunctionProviders").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTfInputProcessorProfiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfInputProcessorProfiles {}
impl ::core::fmt::Debug for IEnumTfInputProcessorProfiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfInputProcessorProfiles").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTfLangBarItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfLangBarItems {}
impl ::core::fmt::Debug for IEnumTfLangBarItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfLangBarItems").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTfLanguageProfiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfLanguageProfiles {}
impl ::core::fmt::Debug for IEnumTfLanguageProfiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfLanguageProfiles").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTfLatticeElements {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfLatticeElements {}
impl ::core::fmt::Debug for IEnumTfLatticeElements {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfLatticeElements").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTfProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfProperties {}
impl ::core::fmt::Debug for IEnumTfProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfProperties").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTfPropertyValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfPropertyValue {}
impl ::core::fmt::Debug for IEnumTfPropertyValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfPropertyValue").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTfRanges {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfRanges {}
impl ::core::fmt::Debug for IEnumTfRanges {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfRanges").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTfUIElements {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfUIElements {}
impl ::core::fmt::Debug for IEnumTfUIElements {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfUIElements").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInternalDocWrap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternalDocWrap {}
impl ::core::fmt::Debug for IInternalDocWrap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternalDocWrap").field(&self.0).finish()
    }
}
impl ::core::default::Default for INSERT_TEXT_AT_SELECTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INSERT_TEXT_AT_SELECTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSERT_TEXT_AT_SELECTION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpeechCommandProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpeechCommandProvider {}
impl ::core::fmt::Debug for ISpeechCommandProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechCommandProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextStoreACP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACP {}
impl ::core::fmt::Debug for ITextStoreACP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreACP").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextStoreACP2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACP2 {}
impl ::core::fmt::Debug for ITextStoreACP2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreACP2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextStoreACPEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACPEx {}
impl ::core::fmt::Debug for ITextStoreACPEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreACPEx").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextStoreACPServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACPServices {}
impl ::core::fmt::Debug for ITextStoreACPServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreACPServices").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextStoreACPSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACPSink {}
impl ::core::fmt::Debug for ITextStoreACPSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreACPSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextStoreACPSinkEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACPSinkEx {}
impl ::core::fmt::Debug for ITextStoreACPSinkEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreACPSinkEx").field(&self.0).finish()
    }
}
impl ITextStoreACPSinkEx {
    pub unsafe fn OnTextChange(&self, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnTextChange)(::windows::core::Vtable::as_raw(self), dwflags, pchange).ok()
    }
    pub unsafe fn OnSelectionChange(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnSelectionChange)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnLayoutChange)(::windows::core::Vtable::as_raw(self), lcode, vcview).ok()
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnStatusChange)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn OnAttrsChange(&self, acpstart: i32, acpend: i32, paattrs: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnAttrsChange)(::windows::core::Vtable::as_raw(self), acpstart, acpend, paattrs.len() as _, ::core::mem::transmute(paattrs.as_ptr())).ok()
    }
    pub unsafe fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnLockGranted)(::windows::core::Vtable::as_raw(self), dwlockflags).ok()
    }
    pub unsafe fn OnStartEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnStartEditTransaction)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnEndEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnEndEditTransaction)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for ITextStoreAnchor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreAnchor {}
impl ::core::fmt::Debug for ITextStoreAnchor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreAnchor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextStoreAnchorEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreAnchorEx {}
impl ::core::fmt::Debug for ITextStoreAnchorEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreAnchorEx").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextStoreAnchorSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreAnchorSink {}
impl ::core::fmt::Debug for ITextStoreAnchorSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreAnchorSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextStoreSinkAnchorEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreSinkAnchorEx {}
impl ::core::fmt::Debug for ITextStoreSinkAnchorEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreSinkAnchorEx").field(&self.0).finish()
    }
}
impl ITextStoreSinkAnchorEx {
    pub unsafe fn OnTextChange<P0, P1>(&self, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: P0, paend: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAnchor>>,
        P1: ::std::convert::Into<::windows::core::InParam<IAnchor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnTextChange)(::windows::core::Vtable::as_raw(self), dwflags, pastart.into().abi(), paend.into().abi()).ok()
    }
    pub unsafe fn OnSelectionChange(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnSelectionChange)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnLayoutChange)(::windows::core::Vtable::as_raw(self), lcode, vcview).ok()
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnStatusChange)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn OnAttrsChange<P0, P1>(&self, pastart: P0, paend: P1, paattrs: &[::windows::core::GUID]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAnchor>>,
        P1: ::std::convert::Into<::windows::core::InParam<IAnchor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnAttrsChange)(::windows::core::Vtable::as_raw(self), pastart.into().abi(), paend.into().abi(), paattrs.len() as _, ::core::mem::transmute(paattrs.as_ptr())).ok()
    }
    pub unsafe fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnLockGranted)(::windows::core::Vtable::as_raw(self), dwlockflags).ok()
    }
    pub unsafe fn OnStartEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnStartEditTransaction)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnEndEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnEndEditTransaction)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for ITfActiveLanguageProfileNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfActiveLanguageProfileNotifySink {}
impl ::core::fmt::Debug for ITfActiveLanguageProfileNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfActiveLanguageProfileNotifySink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfCandidateList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCandidateList {}
impl ::core::fmt::Debug for ITfCandidateList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCandidateList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfCandidateListUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCandidateListUIElement {}
impl ::core::fmt::Debug for ITfCandidateListUIElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCandidateListUIElement").field(&self.0).finish()
    }
}
impl ITfCandidateListUIElement {
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, bshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Show)(::windows::core::Vtable::as_raw(self), bshow.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsShown)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfCandidateListUIElementBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCandidateListUIElementBehavior {}
impl ::core::fmt::Debug for ITfCandidateListUIElementBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCandidateListUIElementBehavior").field(&self.0).finish()
    }
}
impl ITfCandidateListUIElementBehavior {
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, bshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Show)(::windows::core::Vtable::as_raw(self), bshow.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsShown)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUpdatedFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUpdatedFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDocumentMgr(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDocumentMgr)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, uindex: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetString)(::windows::core::Vtable::as_raw(self), uindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPageIndex(&self, pindex: &mut [u32], pupagecnt: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPageIndex)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pindex.as_ptr()), pindex.len() as _, pupagecnt).ok()
    }
    pub unsafe fn SetPageIndex(&self, pindex: &[u32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPageIndex)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pindex.as_ptr()), pindex.len() as _).ok()
    }
    pub unsafe fn GetCurrentPage(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentPage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfCandidateString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCandidateString {}
impl ::core::fmt::Debug for ITfCandidateString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCandidateString").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfCategoryMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCategoryMgr {}
impl ::core::fmt::Debug for ITfCategoryMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCategoryMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfCleanupContextDurationSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCleanupContextDurationSink {}
impl ::core::fmt::Debug for ITfCleanupContextDurationSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCleanupContextDurationSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfCleanupContextSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCleanupContextSink {}
impl ::core::fmt::Debug for ITfCleanupContextSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCleanupContextSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfClientId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfClientId {}
impl ::core::fmt::Debug for ITfClientId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfClientId").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfCompartment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCompartment {}
impl ::core::fmt::Debug for ITfCompartment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCompartment").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfCompartmentEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCompartmentEventSink {}
impl ::core::fmt::Debug for ITfCompartmentEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCompartmentEventSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfCompartmentMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCompartmentMgr {}
impl ::core::fmt::Debug for ITfCompartmentMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCompartmentMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfComposition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfComposition {}
impl ::core::fmt::Debug for ITfComposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfComposition").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfCompositionSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCompositionSink {}
impl ::core::fmt::Debug for ITfCompositionSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCompositionSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfCompositionView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCompositionView {}
impl ::core::fmt::Debug for ITfCompositionView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCompositionView").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfConfigureSystemKeystrokeFeed {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfConfigureSystemKeystrokeFeed {}
impl ::core::fmt::Debug for ITfConfigureSystemKeystrokeFeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfConfigureSystemKeystrokeFeed").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContext {}
impl ::core::fmt::Debug for ITfContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfContextComposition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextComposition {}
impl ::core::fmt::Debug for ITfContextComposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfContextComposition").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfContextKeyEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextKeyEventSink {}
impl ::core::fmt::Debug for ITfContextKeyEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfContextKeyEventSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfContextOwner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextOwner {}
impl ::core::fmt::Debug for ITfContextOwner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfContextOwner").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfContextOwnerCompositionServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextOwnerCompositionServices {}
impl ::core::fmt::Debug for ITfContextOwnerCompositionServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfContextOwnerCompositionServices").field(&self.0).finish()
    }
}
impl ITfContextOwnerCompositionServices {
    pub unsafe fn StartComposition<P0, P1>(&self, ecwrite: u32, pcompositionrange: P0, psink: P1) -> ::windows::core::Result<ITfComposition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfRange>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITfCompositionSink>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StartComposition)(::windows::core::Vtable::as_raw(self), ecwrite, pcompositionrange.into().abi(), psink.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumCompositions(&self) -> ::windows::core::Result<IEnumITfCompositionView> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumCompositions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindComposition<P0>(&self, ecread: u32, ptestrange: P0) -> ::windows::core::Result<IEnumITfCompositionView>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindComposition)(::windows::core::Vtable::as_raw(self), ecread, ptestrange.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TakeOwnership<P0, P1>(&self, ecwrite: u32, pcomposition: P0, psink: P1) -> ::windows::core::Result<ITfComposition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfCompositionView>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITfCompositionSink>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TakeOwnership)(::windows::core::Vtable::as_raw(self), ecwrite, pcomposition.into().abi(), psink.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfContextOwnerCompositionSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextOwnerCompositionSink {}
impl ::core::fmt::Debug for ITfContextOwnerCompositionSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfContextOwnerCompositionSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfContextOwnerServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextOwnerServices {}
impl ::core::fmt::Debug for ITfContextOwnerServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfContextOwnerServices").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfContextView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextView {}
impl ::core::fmt::Debug for ITfContextView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfContextView").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfCreatePropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCreatePropertyStore {}
impl ::core::fmt::Debug for ITfCreatePropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCreatePropertyStore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfDisplayAttributeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfDisplayAttributeInfo {}
impl ::core::fmt::Debug for ITfDisplayAttributeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfDisplayAttributeInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfDisplayAttributeMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfDisplayAttributeMgr {}
impl ::core::fmt::Debug for ITfDisplayAttributeMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfDisplayAttributeMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfDisplayAttributeNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfDisplayAttributeNotifySink {}
impl ::core::fmt::Debug for ITfDisplayAttributeNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfDisplayAttributeNotifySink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfDisplayAttributeProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfDisplayAttributeProvider {}
impl ::core::fmt::Debug for ITfDisplayAttributeProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfDisplayAttributeProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfDocumentMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfDocumentMgr {}
impl ::core::fmt::Debug for ITfDocumentMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfDocumentMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfEditRecord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfEditRecord {}
impl ::core::fmt::Debug for ITfEditRecord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfEditRecord").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfEditSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfEditSession {}
impl ::core::fmt::Debug for ITfEditSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfEditSession").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfEditTransactionSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfEditTransactionSink {}
impl ::core::fmt::Debug for ITfEditTransactionSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfEditTransactionSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfFnAdviseText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnAdviseText {}
impl ::core::fmt::Debug for ITfFnAdviseText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnAdviseText").field(&self.0).finish()
    }
}
impl ITfFnAdviseText {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfFnBalloon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnBalloon {}
impl ::core::fmt::Debug for ITfFnBalloon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnBalloon").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfFnConfigure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnConfigure {}
impl ::core::fmt::Debug for ITfFnConfigure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnConfigure").field(&self.0).finish()
    }
}
impl ITfFnConfigure {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfFnConfigureRegisterEudc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnConfigureRegisterEudc {}
impl ::core::fmt::Debug for ITfFnConfigureRegisterEudc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnConfigureRegisterEudc").field(&self.0).finish()
    }
}
impl ITfFnConfigureRegisterEudc {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfFnConfigureRegisterWord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnConfigureRegisterWord {}
impl ::core::fmt::Debug for ITfFnConfigureRegisterWord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnConfigureRegisterWord").field(&self.0).finish()
    }
}
impl ITfFnConfigureRegisterWord {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfFnCustomSpeechCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnCustomSpeechCommand {}
impl ::core::fmt::Debug for ITfFnCustomSpeechCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnCustomSpeechCommand").field(&self.0).finish()
    }
}
impl ITfFnCustomSpeechCommand {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfFnGetLinguisticAlternates {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnGetLinguisticAlternates {}
impl ::core::fmt::Debug for ITfFnGetLinguisticAlternates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnGetLinguisticAlternates").field(&self.0).finish()
    }
}
impl ITfFnGetLinguisticAlternates {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfFnGetPreferredTouchKeyboardLayout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnGetPreferredTouchKeyboardLayout {}
impl ::core::fmt::Debug for ITfFnGetPreferredTouchKeyboardLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnGetPreferredTouchKeyboardLayout").field(&self.0).finish()
    }
}
impl ITfFnGetPreferredTouchKeyboardLayout {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfFnGetSAPIObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnGetSAPIObject {}
impl ::core::fmt::Debug for ITfFnGetSAPIObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnGetSAPIObject").field(&self.0).finish()
    }
}
impl ITfFnGetSAPIObject {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfFnLMInternal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnLMInternal {}
impl ::core::fmt::Debug for ITfFnLMInternal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnLMInternal").field(&self.0).finish()
    }
}
impl ITfFnLMInternal {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryRange<P0>(&self, prange: P0, ppnewrange: *mut ::core::option::Option<ITfRange>, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfRange>>,
    {
        (::windows::core::Vtable::vtable(self).base__.QueryRange)(::windows::core::Vtable::as_raw(self), prange.into().abi(), ::core::mem::transmute(ppnewrange), pfaccepted).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryLangID(&self, langid: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.QueryLangID)(::windows::core::Vtable::as_raw(self), langid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetReconversion<P0>(&self, prange: P0) -> ::windows::core::Result<ITfCandidateList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetReconversion)(::windows::core::Vtable::as_raw(self), prange.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Reconvert<P0>(&self, prange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfRange>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Reconvert)(::windows::core::Vtable::as_raw(self), prange.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryKey<P0, P1, P2>(&self, fup: P0, vkey: P1, lparamkeydata: P2) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.QueryKey)(::windows::core::Vtable::as_raw(self), fup.into(), vkey.into(), lparamkeydata.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvokeKey<P0, P1, P2>(&self, fup: P0, vkey: P1, lparamkeydata: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.InvokeKey)(::windows::core::Vtable::as_raw(self), fup.into(), vkey.into(), lparamkeydata.into()).ok()
    }
    pub unsafe fn InvokeFunc<P0>(&self, pic: P0, refguidfunc: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfContext>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InvokeFunc)(::windows::core::Vtable::as_raw(self), pic.into().abi(), refguidfunc).ok()
    }
}
impl ::core::cmp::PartialEq for ITfFnLMProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnLMProcessor {}
impl ::core::fmt::Debug for ITfFnLMProcessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnLMProcessor").field(&self.0).finish()
    }
}
impl ITfFnLMProcessor {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfFnLangProfileUtil {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnLangProfileUtil {}
impl ::core::fmt::Debug for ITfFnLangProfileUtil {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnLangProfileUtil").field(&self.0).finish()
    }
}
impl ITfFnLangProfileUtil {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfFnPlayBack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnPlayBack {}
impl ::core::fmt::Debug for ITfFnPlayBack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnPlayBack").field(&self.0).finish()
    }
}
impl ITfFnPlayBack {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfFnPropertyUIStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnPropertyUIStatus {}
impl ::core::fmt::Debug for ITfFnPropertyUIStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnPropertyUIStatus").field(&self.0).finish()
    }
}
impl ITfFnPropertyUIStatus {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfFnReconversion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnReconversion {}
impl ::core::fmt::Debug for ITfFnReconversion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnReconversion").field(&self.0).finish()
    }
}
impl ITfFnReconversion {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfFnSearchCandidateProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnSearchCandidateProvider {}
impl ::core::fmt::Debug for ITfFnSearchCandidateProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnSearchCandidateProvider").field(&self.0).finish()
    }
}
impl ITfFnSearchCandidateProvider {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfFnShowHelp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnShowHelp {}
impl ::core::fmt::Debug for ITfFnShowHelp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnShowHelp").field(&self.0).finish()
    }
}
impl ITfFnShowHelp {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfFunction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFunction {}
impl ::core::fmt::Debug for ITfFunction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFunction").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfFunctionProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFunctionProvider {}
impl ::core::fmt::Debug for ITfFunctionProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFunctionProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfInputProcessorProfileActivationSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputProcessorProfileActivationSink {}
impl ::core::fmt::Debug for ITfInputProcessorProfileActivationSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfInputProcessorProfileActivationSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfInputProcessorProfileMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputProcessorProfileMgr {}
impl ::core::fmt::Debug for ITfInputProcessorProfileMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfInputProcessorProfileMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfInputProcessorProfileSubstituteLayout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputProcessorProfileSubstituteLayout {}
impl ::core::fmt::Debug for ITfInputProcessorProfileSubstituteLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfInputProcessorProfileSubstituteLayout").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfInputProcessorProfiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputProcessorProfiles {}
impl ::core::fmt::Debug for ITfInputProcessorProfiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfInputProcessorProfiles").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfInputProcessorProfilesEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputProcessorProfilesEx {}
impl ::core::fmt::Debug for ITfInputProcessorProfilesEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfInputProcessorProfilesEx").field(&self.0).finish()
    }
}
impl ITfInputProcessorProfilesEx {
    pub unsafe fn Register(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Register)(::windows::core::Vtable::as_raw(self), rclsid).ok()
    }
    pub unsafe fn Unregister(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unregister)(::windows::core::Vtable::as_raw(self), rclsid).ok()
    }
    pub unsafe fn AddLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: &[u16], pchiconfile: &[u16], uiconindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddLanguageProfile)(::windows::core::Vtable::as_raw(self), rclsid, langid, guidprofile, ::core::mem::transmute(pchdesc.as_ptr()), pchdesc.len() as _, ::core::mem::transmute(pchiconfile.as_ptr()), pchiconfile.len() as _, uiconindex).ok()
    }
    pub unsafe fn RemoveLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveLanguageProfile)(::windows::core::Vtable::as_raw(self), rclsid, langid, guidprofile).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumInputProcessorInfo(&self) -> ::windows::core::Result<super::super::System::Com::IEnumGUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumInputProcessorInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDefaultLanguageProfile(&self, langid: u16, catid: *const ::windows::core::GUID, pclsid: *mut ::windows::core::GUID, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDefaultLanguageProfile)(::windows::core::Vtable::as_raw(self), langid, catid, pclsid, pguidprofile).ok()
    }
    pub unsafe fn SetDefaultLanguageProfile(&self, langid: u16, rclsid: *const ::windows::core::GUID, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDefaultLanguageProfile)(::windows::core::Vtable::as_raw(self), langid, rclsid, guidprofiles).ok()
    }
    pub unsafe fn ActivateLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ActivateLanguageProfile)(::windows::core::Vtable::as_raw(self), rclsid, langid, guidprofiles).ok()
    }
    pub unsafe fn GetActiveLanguageProfile(&self, rclsid: *const ::windows::core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetActiveLanguageProfile)(::windows::core::Vtable::as_raw(self), rclsid, plangid, pguidprofile).ok()
    }
    pub unsafe fn GetLanguageProfileDescription(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLanguageProfileDescription)(::windows::core::Vtable::as_raw(self), rclsid, langid, guidprofile, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentLanguage(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentLanguage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ChangeCurrentLanguage(&self, langid: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ChangeCurrentLanguage)(::windows::core::Vtable::as_raw(self), langid).ok()
    }
    pub unsafe fn GetLanguageList(&self, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLanguageList)(::windows::core::Vtable::as_raw(self), pplangid, pulcount).ok()
    }
    pub unsafe fn EnumLanguageProfiles(&self, langid: u16) -> ::windows::core::Result<IEnumTfLanguageProfiles> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumLanguageProfiles)(::windows::core::Vtable::as_raw(self), langid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableLanguageProfile<P0>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnableLanguageProfile)(::windows::core::Vtable::as_raw(self), rclsid, langid, guidprofile, fenable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEnabledLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsEnabledLanguageProfile)(::windows::core::Vtable::as_raw(self), rclsid, langid, guidprofile, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableLanguageProfileByDefault<P0>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnableLanguageProfileByDefault)(::windows::core::Vtable::as_raw(self), rclsid, langid, guidprofile, fenable.into()).ok()
    }
    pub unsafe fn SubstituteKeyboardLayout<P0>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, hkl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<HKL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SubstituteKeyboardLayout)(::windows::core::Vtable::as_raw(self), rclsid, langid, guidprofile, hkl.into()).ok()
    }
}
impl ::core::cmp::PartialEq for ITfInputScope {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputScope {}
impl ::core::fmt::Debug for ITfInputScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfInputScope").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfInputScope2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputScope2 {}
impl ::core::fmt::Debug for ITfInputScope2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfInputScope2").field(&self.0).finish()
    }
}
impl ITfInputScope2 {
    pub unsafe fn GetInputScopes(&self, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInputScopes)(::windows::core::Vtable::as_raw(self), pprginputscopes, pccount).ok()
    }
    pub unsafe fn GetPhrase(&self, ppbstrphrases: *mut *mut ::windows::core::BSTR, pccount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPhrase)(::windows::core::Vtable::as_raw(self), ppbstrphrases, pccount).ok()
    }
    pub unsafe fn GetRegularExpression(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRegularExpression)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSRGS(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSRGS)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetXML(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetXML)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfInsertAtSelection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInsertAtSelection {}
impl ::core::fmt::Debug for ITfInsertAtSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfInsertAtSelection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfIntegratableCandidateListUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfIntegratableCandidateListUIElement {}
impl ::core::fmt::Debug for ITfIntegratableCandidateListUIElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfIntegratableCandidateListUIElement").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfKeyEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfKeyEventSink {}
impl ::core::fmt::Debug for ITfKeyEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfKeyEventSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfKeyTraceEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfKeyTraceEventSink {}
impl ::core::fmt::Debug for ITfKeyTraceEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfKeyTraceEventSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfKeystrokeMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfKeystrokeMgr {}
impl ::core::fmt::Debug for ITfKeystrokeMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfKeystrokeMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfLMLattice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLMLattice {}
impl ::core::fmt::Debug for ITfLMLattice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLMLattice").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfLangBarEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarEventSink {}
impl ::core::fmt::Debug for ITfLangBarEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarEventSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfLangBarItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItem {}
impl ::core::fmt::Debug for ITfLangBarItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfLangBarItemBalloon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemBalloon {}
impl ::core::fmt::Debug for ITfLangBarItemBalloon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarItemBalloon").field(&self.0).finish()
    }
}
impl ITfLangBarItemBalloon {
    pub unsafe fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self), pinfo).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Show)(::windows::core::Vtable::as_raw(self), fshow.into()).ok()
    }
    pub unsafe fn GetTooltipString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTooltipString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfLangBarItemBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemBitmap {}
impl ::core::fmt::Debug for ITfLangBarItemBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarItemBitmap").field(&self.0).finish()
    }
}
impl ITfLangBarItemBitmap {
    pub unsafe fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self), pinfo).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Show)(::windows::core::Vtable::as_raw(self), fshow.into()).ok()
    }
    pub unsafe fn GetTooltipString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTooltipString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfLangBarItemBitmapButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemBitmapButton {}
impl ::core::fmt::Debug for ITfLangBarItemBitmapButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarItemBitmapButton").field(&self.0).finish()
    }
}
impl ITfLangBarItemBitmapButton {
    pub unsafe fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self), pinfo).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Show)(::windows::core::Vtable::as_raw(self), fshow.into()).ok()
    }
    pub unsafe fn GetTooltipString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTooltipString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfLangBarItemButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemButton {}
impl ::core::fmt::Debug for ITfLangBarItemButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarItemButton").field(&self.0).finish()
    }
}
impl ITfLangBarItemButton {
    pub unsafe fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self), pinfo).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Show)(::windows::core::Vtable::as_raw(self), fshow.into()).ok()
    }
    pub unsafe fn GetTooltipString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTooltipString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfLangBarItemMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemMgr {}
impl ::core::fmt::Debug for ITfLangBarItemMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarItemMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfLangBarItemSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemSink {}
impl ::core::fmt::Debug for ITfLangBarItemSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarItemSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfLangBarMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarMgr {}
impl ::core::fmt::Debug for ITfLangBarMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfLanguageProfileNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLanguageProfileNotifySink {}
impl ::core::fmt::Debug for ITfLanguageProfileNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLanguageProfileNotifySink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfMSAAControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMSAAControl {}
impl ::core::fmt::Debug for ITfMSAAControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfMSAAControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfMenu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMenu {}
impl ::core::fmt::Debug for ITfMenu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfMenu").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfMessagePump {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMessagePump {}
impl ::core::fmt::Debug for ITfMessagePump {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfMessagePump").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfMouseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMouseSink {}
impl ::core::fmt::Debug for ITfMouseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfMouseSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfMouseTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMouseTracker {}
impl ::core::fmt::Debug for ITfMouseTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfMouseTracker").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfMouseTrackerACP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMouseTrackerACP {}
impl ::core::fmt::Debug for ITfMouseTrackerACP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfMouseTrackerACP").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfPersistentPropertyLoaderACP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfPersistentPropertyLoaderACP {}
impl ::core::fmt::Debug for ITfPersistentPropertyLoaderACP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfPersistentPropertyLoaderACP").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfPreservedKeyNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfPreservedKeyNotifySink {}
impl ::core::fmt::Debug for ITfPreservedKeyNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfPreservedKeyNotifySink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfProperty {}
impl ::core::fmt::Debug for ITfProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfProperty").field(&self.0).finish()
    }
}
impl ITfProperty {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumRanges<P0>(&self, ec: u32, ppenum: *mut ::core::option::Option<IEnumTfRanges>, ptargetrange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfRange>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnumRanges)(::windows::core::Vtable::as_raw(self), ec, ::core::mem::transmute(ppenum), ptargetrange.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetValue<P0>(&self, ec: u32, prange: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetValue)(::windows::core::Vtable::as_raw(self), ec, prange.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<ITfContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfPropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfPropertyStore {}
impl ::core::fmt::Debug for ITfPropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfPropertyStore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfQueryEmbedded {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfQueryEmbedded {}
impl ::core::fmt::Debug for ITfQueryEmbedded {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfQueryEmbedded").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfRange {}
impl ::core::fmt::Debug for ITfRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfRange").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfRangeACP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfRangeACP {}
impl ::core::fmt::Debug for ITfRangeACP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfRangeACP").field(&self.0).finish()
    }
}
impl ITfRangeACP {
    pub unsafe fn GetText(&self, ec: u32, dwflags: u32, pchtext: &mut [u16], pcch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetText)(::windows::core::Vtable::as_raw(self), ec, dwflags, ::core::mem::transmute(pchtext.as_ptr()), pchtext.len() as _, pcch).ok()
    }
    pub unsafe fn SetText(&self, ec: u32, dwflags: u32, pchtext: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetText)(::windows::core::Vtable::as_raw(self), ec, dwflags, ::core::mem::transmute(pchtext.as_ptr()), pchtext.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self, ec: u32) -> ::windows::core::Result<super::super::System::Com::IDataObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFormattedText)(::windows::core::Vtable::as_raw(self), ec, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetEmbedded(&self, ec: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEmbedded)(::windows::core::Vtable::as_raw(self), ec, rguidservice, riid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbedded<P0>(&self, ec: u32, dwflags: u32, pdataobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InsertEmbedded)(::windows::core::Vtable::as_raw(self), ec, dwflags, pdataobject.into().abi()).ok()
    }
    pub unsafe fn ShiftStart(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ShiftStart)(::windows::core::Vtable::as_raw(self), ec, cchreq, pcch, phalt).ok()
    }
    pub unsafe fn ShiftEnd(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ShiftEnd)(::windows::core::Vtable::as_raw(self), ec, cchreq, pcch, phalt).ok()
    }
    pub unsafe fn ShiftStartToRange<P0>(&self, ec: u32, prange: P0, apos: TfAnchor) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfRange>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ShiftStartToRange)(::windows::core::Vtable::as_raw(self), ec, prange.into().abi(), apos).ok()
    }
    pub unsafe fn ShiftEndToRange<P0>(&self, ec: u32, prange: P0, apos: TfAnchor) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfRange>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ShiftEndToRange)(::windows::core::Vtable::as_raw(self), ec, prange.into().abi(), apos).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShiftStartRegion(&self, ec: u32, dir: TfShiftDir) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ShiftStartRegion)(::windows::core::Vtable::as_raw(self), ec, dir, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShiftEndRegion(&self, ec: u32, dir: TfShiftDir) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ShiftEndRegion)(::windows::core::Vtable::as_raw(self), ec, dir, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEmpty(&self, ec: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsEmpty)(::windows::core::Vtable::as_raw(self), ec, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Collapse(&self, ec: u32, apos: TfAnchor) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Collapse)(::windows::core::Vtable::as_raw(self), ec, apos).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqualStart<P0>(&self, ec: u32, pwith: P0, apos: TfAnchor) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsEqualStart)(::windows::core::Vtable::as_raw(self), ec, pwith.into().abi(), apos, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqualEnd<P0>(&self, ec: u32, pwith: P0, apos: TfAnchor) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsEqualEnd)(::windows::core::Vtable::as_raw(self), ec, pwith.into().abi(), apos, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareStart<P0>(&self, ec: u32, pwith: P0, apos: TfAnchor) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareStart)(::windows::core::Vtable::as_raw(self), ec, pwith.into().abi(), apos, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareEnd<P0>(&self, ec: u32, pwith: P0, apos: TfAnchor) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareEnd)(::windows::core::Vtable::as_raw(self), ec, pwith.into().abi(), apos, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdjustForInsert(&self, ec: u32, cchinsert: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AdjustForInsert)(::windows::core::Vtable::as_raw(self), ec, cchinsert, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGravity(&self, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetGravity)(::windows::core::Vtable::as_raw(self), pgstart, pgend).ok()
    }
    pub unsafe fn SetGravity(&self, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGravity)(::windows::core::Vtable::as_raw(self), ec, gstart, gend).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ITfRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<ITfContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfRangeBackup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfRangeBackup {}
impl ::core::fmt::Debug for ITfRangeBackup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfRangeBackup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfReadOnlyProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfReadOnlyProperty {}
impl ::core::fmt::Debug for ITfReadOnlyProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfReadOnlyProperty").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfReadingInformationUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfReadingInformationUIElement {}
impl ::core::fmt::Debug for ITfReadingInformationUIElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfReadingInformationUIElement").field(&self.0).finish()
    }
}
impl ITfReadingInformationUIElement {
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, bshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Show)(::windows::core::Vtable::as_raw(self), bshow.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsShown)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfReverseConversion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfReverseConversion {}
impl ::core::fmt::Debug for ITfReverseConversion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfReverseConversion").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfReverseConversionList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfReverseConversionList {}
impl ::core::fmt::Debug for ITfReverseConversionList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfReverseConversionList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfReverseConversionMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfReverseConversionMgr {}
impl ::core::fmt::Debug for ITfReverseConversionMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfReverseConversionMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSource {}
impl ::core::fmt::Debug for ITfSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfSourceSingle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSourceSingle {}
impl ::core::fmt::Debug for ITfSourceSingle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfSourceSingle").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfSpeechUIServer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSpeechUIServer {}
impl ::core::fmt::Debug for ITfSpeechUIServer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfSpeechUIServer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfStatusSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfStatusSink {}
impl ::core::fmt::Debug for ITfStatusSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfStatusSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfSystemDeviceTypeLangBarItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSystemDeviceTypeLangBarItem {}
impl ::core::fmt::Debug for ITfSystemDeviceTypeLangBarItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfSystemDeviceTypeLangBarItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfSystemLangBarItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSystemLangBarItem {}
impl ::core::fmt::Debug for ITfSystemLangBarItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfSystemLangBarItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfSystemLangBarItemSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSystemLangBarItemSink {}
impl ::core::fmt::Debug for ITfSystemLangBarItemSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfSystemLangBarItemSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfSystemLangBarItemText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSystemLangBarItemText {}
impl ::core::fmt::Debug for ITfSystemLangBarItemText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfSystemLangBarItemText").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfTextEditSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTextEditSink {}
impl ::core::fmt::Debug for ITfTextEditSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfTextEditSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfTextInputProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTextInputProcessor {}
impl ::core::fmt::Debug for ITfTextInputProcessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfTextInputProcessor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfTextInputProcessorEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTextInputProcessorEx {}
impl ::core::fmt::Debug for ITfTextInputProcessorEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfTextInputProcessorEx").field(&self.0).finish()
    }
}
impl ITfTextInputProcessorEx {
    pub unsafe fn Activate<P0>(&self, ptim: P0, tid: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfThreadMgr>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Activate)(::windows::core::Vtable::as_raw(self), ptim.into().abi(), tid).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Deactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for ITfTextLayoutSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTextLayoutSink {}
impl ::core::fmt::Debug for ITfTextLayoutSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfTextLayoutSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfThreadFocusSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfThreadFocusSink {}
impl ::core::fmt::Debug for ITfThreadFocusSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfThreadFocusSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfThreadMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfThreadMgr {}
impl ::core::fmt::Debug for ITfThreadMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfThreadMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfThreadMgr2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfThreadMgr2 {}
impl ::core::fmt::Debug for ITfThreadMgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfThreadMgr2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfThreadMgrEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfThreadMgrEventSink {}
impl ::core::fmt::Debug for ITfThreadMgrEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfThreadMgrEventSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfThreadMgrEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfThreadMgrEx {}
impl ::core::fmt::Debug for ITfThreadMgrEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfThreadMgrEx").field(&self.0).finish()
    }
}
impl ITfThreadMgrEx {
    pub unsafe fn Activate(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Activate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Deactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CreateDocumentMgr(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDocumentMgr)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumDocumentMgrs(&self) -> ::windows::core::Result<IEnumTfDocumentMgrs> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumDocumentMgrs)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFocus(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFocus<P0>(&self, pdimfocus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITfDocumentMgr>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFocus)(::windows::core::Vtable::as_raw(self), pdimfocus.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AssociateFocus<P0, P1>(&self, hwnd: P0, pdimnew: P1) -> ::windows::core::Result<ITfDocumentMgr>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<ITfDocumentMgr>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AssociateFocus)(::windows::core::Vtable::as_raw(self), hwnd.into(), pdimnew.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsThreadFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsThreadFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFunctionProvider(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfFunctionProvider> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFunctionProvider)(::windows::core::Vtable::as_raw(self), clsid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumFunctionProviders(&self) -> ::windows::core::Result<IEnumTfFunctionProviders> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumFunctionProviders)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGlobalCompartment(&self) -> ::windows::core::Result<ITfCompartmentMgr> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGlobalCompartment)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfToolTipUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfToolTipUIElement {}
impl ::core::fmt::Debug for ITfToolTipUIElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfToolTipUIElement").field(&self.0).finish()
    }
}
impl ITfToolTipUIElement {
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, bshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Show)(::windows::core::Vtable::as_raw(self), bshow.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsShown)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfTransitoryExtensionSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTransitoryExtensionSink {}
impl ::core::fmt::Debug for ITfTransitoryExtensionSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfTransitoryExtensionSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfTransitoryExtensionUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTransitoryExtensionUIElement {}
impl ::core::fmt::Debug for ITfTransitoryExtensionUIElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfTransitoryExtensionUIElement").field(&self.0).finish()
    }
}
impl ITfTransitoryExtensionUIElement {
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, bshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Show)(::windows::core::Vtable::as_raw(self), bshow.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsShown)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITfUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfUIElement {}
impl ::core::fmt::Debug for ITfUIElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfUIElement").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfUIElementMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfUIElementMgr {}
impl ::core::fmt::Debug for ITfUIElementMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfUIElementMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITfUIElementSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfUIElementSink {}
impl ::core::fmt::Debug for ITfUIElementSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfUIElementSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIManagerEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIManagerEventSink {}
impl ::core::fmt::Debug for IUIManagerEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIManagerEventSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVersionInfo {}
impl ::core::fmt::Debug for IVersionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVersionInfo").field(&self.0).finish()
    }
}
impl ::core::default::Default for InputScope {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InputScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputScope").field(&self.0).finish()
    }
}
impl ::core::default::Default for LANG_BAR_ITEM_ICON_MODE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LANG_BAR_ITEM_ICON_MODE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LANG_BAR_ITEM_ICON_MODE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TEXT_STORE_CHANGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TEXT_STORE_CHANGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEXT_STORE_CHANGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TEXT_STORE_CHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TEXT_STORE_CHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TEXT_STORE_CHANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TEXT_STORE_CHANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TEXT_STORE_CHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TEXT_STORE_LOCK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TEXT_STORE_LOCK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEXT_STORE_LOCK_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TEXT_STORE_TEXT_CHANGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TEXT_STORE_TEXT_CHANGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEXT_STORE_TEXT_CHANGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TEXT_STORE_TEXT_CHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TEXT_STORE_TEXT_CHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TEXT_STORE_TEXT_CHANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TEXT_STORE_TEXT_CHANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TEXT_STORE_TEXT_CHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TF_CONTEXT_EDIT_CONTEXT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TF_DA_ATTR_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TF_DA_ATTR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TF_DA_ATTR_INFO").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_DA_COLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TF_DA_COLORTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TF_DA_COLORTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TF_DA_COLORTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TF_DA_LINESTYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TF_DA_LINESTYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TF_DA_LINESTYLE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_DISPLAYATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TF_HALTCOND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TF_HALTCOND {
    fn eq(&self, other: &Self) -> bool {
        self.pHaltRange == other.pHaltRange && self.aHaltPos == other.aHaltPos && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for TF_HALTCOND {}
impl ::core::fmt::Debug for TF_HALTCOND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_HALTCOND").field("pHaltRange", &self.pHaltRange).field("aHaltPos", &self.aHaltPos).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for TF_INPUTPROCESSORPROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TF_INPUTPROCESSORPROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.dwProfileType == other.dwProfileType && self.langid == other.langid && self.clsid == other.clsid && self.guidProfile == other.guidProfile && self.catid == other.catid && self.hklSubstitute == other.hklSubstitute && self.dwCaps == other.dwCaps && self.hkl == other.hkl && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for TF_INPUTPROCESSORPROFILE {}
impl ::core::fmt::Debug for TF_INPUTPROCESSORPROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_INPUTPROCESSORPROFILE").field("dwProfileType", &self.dwProfileType).field("langid", &self.langid).field("clsid", &self.clsid).field("guidProfile", &self.guidProfile).field("catid", &self.catid).field("hklSubstitute", &self.hklSubstitute).field("dwCaps", &self.dwCaps).field("hkl", &self.hkl).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for TF_LANGBARITEMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TF_LANGBARITEMINFO {
    fn eq(&self, other: &Self) -> bool {
        self.clsidService == other.clsidService && self.guidItem == other.guidItem && self.dwStyle == other.dwStyle && self.ulSort == other.ulSort && self.szDescription == other.szDescription
    }
}
impl ::core::cmp::Eq for TF_LANGBARITEMINFO {}
impl ::core::fmt::Debug for TF_LANGBARITEMINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_LANGBARITEMINFO").field("clsidService", &self.clsidService).field("guidItem", &self.guidItem).field("dwStyle", &self.dwStyle).field("ulSort", &self.ulSort).field("szDescription", &self.szDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_LANGUAGEPROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_LANGUAGEPROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.clsid == other.clsid && self.langid == other.langid && self.catid == other.catid && self.fActive == other.fActive && self.guidProfile == other.guidProfile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_LANGUAGEPROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TF_LANGUAGEPROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_LANGUAGEPROFILE").field("clsid", &self.clsid).field("langid", &self.langid).field("catid", &self.catid).field("fActive", &self.fActive).field("guidProfile", &self.guidProfile).finish()
    }
}
impl ::core::default::Default for TF_LBBALLOONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TF_LBBALLOONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.style == other.style && self.bstrText == other.bstrText
    }
}
impl ::core::cmp::Eq for TF_LBBALLOONINFO {}
impl ::core::fmt::Debug for TF_LBBALLOONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_LBBALLOONINFO").field("style", &self.style).field("bstrText", &self.bstrText).finish()
    }
}
impl ::core::default::Default for TF_LMLATTELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    fn eq(&self, other: &Self) -> bool {
        self.guidType == other.guidType && self.ichStart == other.ichStart && self.cch == other.cch && self.cb == other.cb && self.dwPrivate == other.dwPrivate && self.clsidTIP == other.clsidTIP
    }
}
impl ::core::cmp::Eq for TF_PERSISTENT_PROPERTY_HEADER_ACP {}
impl ::core::fmt::Debug for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_PERSISTENT_PROPERTY_HEADER_ACP").field("guidType", &self.guidType).field("ichStart", &self.ichStart).field("cch", &self.cch).field("cb", &self.cb).field("dwPrivate", &self.dwPrivate).field("clsidTIP", &self.clsidTIP).finish()
    }
}
impl ::core::default::Default for TF_PRESERVEDKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TF_PRESERVEDKEY {
    fn eq(&self, other: &Self) -> bool {
        self.uVKey == other.uVKey && self.uModifiers == other.uModifiers
    }
}
impl ::core::cmp::Eq for TF_PRESERVEDKEY {}
impl ::core::fmt::Debug for TF_PRESERVEDKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_PRESERVEDKEY").field("uVKey", &self.uVKey).field("uModifiers", &self.uModifiers).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for TF_PROPERTYVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_SELECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_SELECTION {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range && self.style == other.style
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_SELECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TF_SELECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_SELECTION").field("range", &self.range).field("style", &self.style).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_SELECTIONSTYLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_SELECTIONSTYLE {
    fn eq(&self, other: &Self) -> bool {
        self.ase == other.ase && self.fInterimChar == other.fInterimChar
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_SELECTIONSTYLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TF_SELECTIONSTYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_SELECTIONSTYLE").field("ase", &self.ase).field("fInterimChar", &self.fInterimChar).finish()
    }
}
impl ::core::default::Default for TKBLayoutType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TKBLayoutType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TKBLayoutType").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for TS_ATTRVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TS_RUNINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TS_RUNINFO {
    fn eq(&self, other: &Self) -> bool {
        self.uCount == other.uCount && self.r#type == other.r#type
    }
}
impl ::core::cmp::Eq for TS_RUNINFO {}
impl ::core::fmt::Debug for TS_RUNINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TS_RUNINFO").field("uCount", &self.uCount).field("type", &self.r#type).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TS_SELECTIONSTYLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TS_SELECTIONSTYLE {
    fn eq(&self, other: &Self) -> bool {
        self.ase == other.ase && self.fInterimChar == other.fInterimChar
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TS_SELECTIONSTYLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TS_SELECTIONSTYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TS_SELECTIONSTYLE").field("ase", &self.ase).field("fInterimChar", &self.fInterimChar).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TS_SELECTION_ACP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TS_SELECTION_ACP {
    fn eq(&self, other: &Self) -> bool {
        self.acpStart == other.acpStart && self.acpEnd == other.acpEnd && self.style == other.style
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TS_SELECTION_ACP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TS_SELECTION_ACP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TS_SELECTION_ACP").field("acpStart", &self.acpStart).field("acpEnd", &self.acpEnd).field("style", &self.style).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TS_SELECTION_ANCHOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TS_SELECTION_ANCHOR {
    fn eq(&self, other: &Self) -> bool {
        self.paStart == other.paStart && self.paEnd == other.paEnd && self.style == other.style
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TS_SELECTION_ANCHOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TS_SELECTION_ANCHOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TS_SELECTION_ANCHOR").field("paStart", &self.paStart).field("paEnd", &self.paEnd).field("style", &self.style).finish()
    }
}
impl ::core::default::Default for TS_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TS_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwDynamicFlags == other.dwDynamicFlags && self.dwStaticFlags == other.dwStaticFlags
    }
}
impl ::core::cmp::Eq for TS_STATUS {}
impl ::core::fmt::Debug for TS_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TS_STATUS").field("dwDynamicFlags", &self.dwDynamicFlags).field("dwStaticFlags", &self.dwStaticFlags).finish()
    }
}
impl ::core::default::Default for TS_TEXTCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TS_TEXTCHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.acpStart == other.acpStart && self.acpOldEnd == other.acpOldEnd && self.acpNewEnd == other.acpNewEnd
    }
}
impl ::core::cmp::Eq for TS_TEXTCHANGE {}
impl ::core::fmt::Debug for TS_TEXTCHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TS_TEXTCHANGE").field("acpStart", &self.acpStart).field("acpOldEnd", &self.acpOldEnd).field("acpNewEnd", &self.acpNewEnd).finish()
    }
}
impl ::core::default::Default for TfActiveSelEnd {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TfActiveSelEnd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfActiveSelEnd").field(&self.0).finish()
    }
}
impl ::core::default::Default for TfAnchor {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TfAnchor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfAnchor").field(&self.0).finish()
    }
}
impl ::core::default::Default for TfCandidateResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TfCandidateResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfCandidateResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for TfGravity {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TfGravity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfGravity").field(&self.0).finish()
    }
}
impl ::core::default::Default for TfIntegratableCandidateListSelectionStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TfIntegratableCandidateListSelectionStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfIntegratableCandidateListSelectionStyle").field(&self.0).finish()
    }
}
impl ::core::default::Default for TfLBBalloonStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TfLBBalloonStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfLBBalloonStyle").field(&self.0).finish()
    }
}
impl ::core::default::Default for TfLBIClick {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TfLBIClick {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfLBIClick").field(&self.0).finish()
    }
}
impl ::core::default::Default for TfLayoutCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TfLayoutCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfLayoutCode").field(&self.0).finish()
    }
}
impl ::core::default::Default for TfSapiObject {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TfSapiObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfSapiObject").field(&self.0).finish()
    }
}
impl ::core::default::Default for TfShiftDir {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TfShiftDir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfShiftDir").field(&self.0).finish()
    }
}
impl ::core::default::Default for TsActiveSelEnd {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TsActiveSelEnd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TsActiveSelEnd").field(&self.0).finish()
    }
}
impl ::core::default::Default for TsGravity {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TsGravity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TsGravity").field(&self.0).finish()
    }
}
impl ::core::default::Default for TsLayoutCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TsLayoutCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TsLayoutCode").field(&self.0).finish()
    }
}
impl ::core::default::Default for TsRunType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TsRunType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TsRunType").field(&self.0).finish()
    }
}
impl ::core::default::Default for TsShiftDir {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TsShiftDir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TsShiftDir").field(&self.0).finish()
    }
}
