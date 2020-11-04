pub unsafe trait SetAbi: Sized {
    type Abi;
    unsafe fn set_abi(&mut self) -> Self::Abi;
}
