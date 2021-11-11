#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IVariablePhotoCapturedEventArgs();
    fn IVariablePhotoSequenceCapture();
    fn IVariablePhotoSequenceCapture2();
    fn VariablePhotoCapturedEventArgs();
    fn VariablePhotoSequenceCapture();
}
