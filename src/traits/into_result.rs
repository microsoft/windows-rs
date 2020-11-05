use crate::*;

pub unsafe trait IntoResult: Sized {
    type Abi;

    unsafe fn into_result(abi: Self::Abi) -> Result<Self>;
}