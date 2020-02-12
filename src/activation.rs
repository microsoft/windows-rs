use crate::*;

// TODO: this should return `Result<&I>` e.g. a reference pointing to the factory cache.
// So this function needs to be implemented as some sort of atomic/singleton where RoGetActivationFactory
// is only called once and the result is then cached. Here's how I do it in C++ - it's critical
// that this is super fast. Also, load RoGetActivationFactory dynamically and fall back to LoadLibrary
// and implement DLL garbage collection for those. Version 0.1 can probably just pin everything.
// https://github.com/microsoft/cppwinrt/blob/master/strings/base_activation.h
pub fn factory<C: TypeName, I: QueryType>() -> Result<I> {
    unsafe {
        let mut ptr = std::ptr::null_mut();

        let mut code = RoGetActivationFactory(String::from(C::type_name()).abi(), I::type_guid(), &mut ptr);

        if code == ErrorCode::NOT_INITIALIZED {
            let mut cookie = std::ptr::null_mut();
            CoIncrementMTAUsage(&mut cookie);
            code = RoGetActivationFactory(String::from(C::type_name()).abi(), I::type_guid(), &mut ptr);
        }

        code.ok_or(std::mem::transmute_copy(&ptr))
    }
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct IActivationFactory {
    ptr: ComPtr,
}

impl IActivationFactory {
    pub fn activate_instance<I: QueryType>(&self) -> Result<I> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            // TODO: this is cheating - we need a QI here...
            ((*(*(self.ptr.get() as *const *const abi_IActivationFactory))).activate_instance)(self.ptr.get(), &mut ptr).ok_or(std::mem::transmute_copy(&ptr))
        }
    }
}

impl QueryType for IActivationFactory {
    fn type_guid() -> &'static Guid {
        static GUID: Guid = Guid::from_values(0x00000035, 0x0000, 0x0000, &[0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]);
        &GUID
    }
}

#[repr(C)]
struct abi_IActivationFactory {
    __0: usize,
    __1: usize,
    __2: usize,
    __3: usize,
    __4: usize,
    __5: usize,
    activate_instance: extern "system" fn(RawPtr, *mut RawPtr) -> ErrorCode,
}
