use crate::*;

pub unsafe trait Abi: Sized {
    type Abi;
    unsafe fn get_abi(&self) -> Self::Abi; // just get
    unsafe fn set_abi(&mut self) -> *mut Self::Abi; // get set
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self>; // just result
}

