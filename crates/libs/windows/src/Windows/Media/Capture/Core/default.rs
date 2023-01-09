impl ::core::cmp::PartialEq for VariablePhotoCapturedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VariablePhotoCapturedEventArgs {}
impl ::core::fmt::Debug for VariablePhotoCapturedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VariablePhotoCapturedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for VariablePhotoSequenceCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VariablePhotoSequenceCapture {}
impl ::core::fmt::Debug for VariablePhotoSequenceCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VariablePhotoSequenceCapture").field(&self.0).finish()
    }
}
