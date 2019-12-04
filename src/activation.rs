use crate::*;

pub fn init() {
    let mut cookie = abi::Void::null_mut();
    unsafe { CoIncrementMTAUsage(&mut cookie).unwrap() };
}

// TODO: this should return `Result<&I>` e.g. a reference pointing to the factory cache.
// So this function needs to be implemented as some sort of atomic/singleton where RoGetActivationFactory
// is only called once and the result is then cached. Here's how I do it in C++ - it's critical
// that this is super fast.
// https://github.com/microsoft/cppwinrt/blob/master/strings/base_activation.h
pub fn factory<C: TypeName, I: TypeInterface>() -> Result<I> {
    unsafe {
        let mut ptr = Void::null_mut();
        RoGetActivationFactory(String::from(C::type_name()).as_raw_handle(), I::type_guid(), &mut ptr).ok()?;
        Ok(I::take_ownership(ptr))
    }
}
