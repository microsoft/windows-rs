use crate::*;

pub unsafe trait IntoResult: Sized {
    unsafe fn into_result(self) -> Result<Self>;
}