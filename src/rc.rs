use crate::*;

#[derive(Default, Clone)]
pub struct Rc<T: ?Sized> {
    ptr: ComPtr,
    __t: std::marker::PhantomData<T>,
}

// impl<T: RuntimeType + ?Sized> Rc<T> {
//     fn new() -> Rc<T> {
//         Rc { placeholder: std::marker::PhantomData }
//     }
//     fn into<I: RuntimeType + ?Sized>(&self) -> Rc<I> {
//         Rc { placeholder: std::marker::PhantomData }
//     }
// }

impl<T: ?Sized> RuntimeType for Rc<T> {
    type Abi = RawPtr;

    fn abi(&self) -> Self::Abi {
        panic!();
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        panic!();
    }
}
