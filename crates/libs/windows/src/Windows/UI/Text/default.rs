impl ::core::default::Default for CaretType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CaretType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CaretType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ContentLinkInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentLinkInfo {}
impl ::core::fmt::Debug for ContentLinkInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentLinkInfo").field(&self.0).finish()
    }
}
impl ::core::default::Default for FindOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FindOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FindOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FindOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FindOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FindOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FindOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for FontStretch {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FontStretch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FontStretch").field(&self.0).finish()
    }
}
impl ::core::default::Default for FontStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FontStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FontStyle").field(&self.0).finish()
    }
}
impl ::core::default::Default for FontWeight {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FontWeight {
    fn eq(&self, other: &Self) -> bool {
        self.Weight == other.Weight
    }
}
impl ::core::cmp::Eq for FontWeight {}
impl ::core::fmt::Debug for FontWeight {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FontWeight").field("Weight", &self.Weight).finish()
    }
}
impl ::core::cmp::PartialEq for FontWeights {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FontWeights {}
impl ::core::fmt::Debug for FontWeights {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FontWeights").field(&self.0).finish()
    }
}
impl ::core::default::Default for FormatEffect {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FormatEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FormatEffect").field(&self.0).finish()
    }
}
impl ::core::default::Default for HorizontalCharacterAlignment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HorizontalCharacterAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HorizontalCharacterAlignment").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextCharacterFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextCharacterFormat {}
impl ::core::fmt::Debug for ITextCharacterFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextCharacterFormat").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextDocument {}
impl ::core::fmt::Debug for ITextDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextDocument").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextParagraphFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextParagraphFormat {}
impl ::core::fmt::Debug for ITextParagraphFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextParagraphFormat").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextRange {}
impl ::core::fmt::Debug for ITextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextRange").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextSelection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextSelection {}
impl ::core::fmt::Debug for ITextSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextSelection").field(&self.0).finish()
    }
}
impl ::core::default::Default for LetterCase {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LetterCase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LetterCase").field(&self.0).finish()
    }
}
impl ::core::default::Default for LineSpacingRule {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LineSpacingRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineSpacingRule").field(&self.0).finish()
    }
}
impl ::core::default::Default for LinkType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LinkType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LinkType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MarkerAlignment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MarkerAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MarkerAlignment").field(&self.0).finish()
    }
}
impl ::core::default::Default for MarkerStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MarkerStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MarkerStyle").field(&self.0).finish()
    }
}
impl ::core::default::Default for MarkerType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MarkerType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MarkerType").field(&self.0).finish()
    }
}
impl ::core::default::Default for ParagraphAlignment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ParagraphAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ParagraphAlignment").field(&self.0).finish()
    }
}
impl ::core::default::Default for ParagraphStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ParagraphStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ParagraphStyle").field(&self.0).finish()
    }
}
impl ::core::default::Default for PointOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PointOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PointOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PointOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PointOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PointOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PointOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for RangeGravity {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RangeGravity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RangeGravity").field(&self.0).finish()
    }
}
impl ::core::default::Default for RichEditMathMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RichEditMathMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RichEditMathMode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for RichEditTextDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RichEditTextDocument {}
impl ::core::fmt::Debug for RichEditTextDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RichEditTextDocument").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for RichEditTextRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RichEditTextRange {}
impl ::core::fmt::Debug for RichEditTextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RichEditTextRange").field(&self.0).finish()
    }
}
impl ::core::default::Default for SelectionOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SelectionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SelectionOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SelectionOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SelectionOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SelectionOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SelectionOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SelectionOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SelectionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SelectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SelectionType").field(&self.0).finish()
    }
}
impl ::core::default::Default for TabAlignment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TabAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TabAlignment").field(&self.0).finish()
    }
}
impl ::core::default::Default for TabLeader {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TabLeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TabLeader").field(&self.0).finish()
    }
}
impl ::core::default::Default for TextDecorations {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TextDecorations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextDecorations").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TextDecorations {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TextDecorations {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TextDecorations {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TextDecorations {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TextDecorations {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TextGetOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TextGetOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextGetOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TextGetOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TextGetOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TextGetOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TextGetOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TextGetOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TextRangeUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TextRangeUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextRangeUnit").field(&self.0).finish()
    }
}
impl ::core::default::Default for TextScript {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TextScript {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextScript").field(&self.0).finish()
    }
}
impl ::core::default::Default for TextSetOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TextSetOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextSetOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TextSetOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TextSetOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TextSetOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TextSetOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TextSetOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for UnderlineType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UnderlineType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnderlineType").field(&self.0).finish()
    }
}
impl ::core::default::Default for VerticalCharacterAlignment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VerticalCharacterAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VerticalCharacterAlignment").field(&self.0).finish()
    }
}
