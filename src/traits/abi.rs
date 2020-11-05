pub unsafe trait Abi: Sized {
    type Abi;
    unsafe fn get_abi(&self) -> Self::Abi;
    unsafe fn set_abi(&mut self) -> *mut Self::Abi;
}

