pub unsafe trait GetAbi: Sized {
    type Abi;
    unsafe fn get_abi(&self) -> Self::Abi;
}
