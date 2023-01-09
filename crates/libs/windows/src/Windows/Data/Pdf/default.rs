impl ::core::cmp::PartialEq for PdfDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PdfDocument {}
impl ::core::fmt::Debug for PdfDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdfDocument").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PdfPage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PdfPage {}
impl ::core::fmt::Debug for PdfPage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdfPage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PdfPageDimensions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PdfPageDimensions {}
impl ::core::fmt::Debug for PdfPageDimensions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdfPageDimensions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PdfPageRenderOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PdfPageRenderOptions {}
impl ::core::fmt::Debug for PdfPageRenderOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdfPageRenderOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for PdfPageRotation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PdfPageRotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdfPageRotation").field(&self.0).finish()
    }
}
