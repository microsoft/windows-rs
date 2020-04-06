use crate::*;

// TODO: this should return `Result<&I>` e.g. a reference pointing to the factory cache.
// So this function needs to be implemented as some sort of atomic/singleton where RoGetActivationFactory
// is only called once and the result is then cached. Here's how I do it in C++ - it's critical
// that this is super fast. Also, load RoGetActivationFactory dynamically and fall back to LoadLibrary
// and implement DLL garbage collection for those. Version 0.1 can probably just pin everything.
// https://github.com/microsoft/cppwinrt/blob/master/strings/base_activation.h
pub fn factory<C: TypeName, I: TypeGuid>() -> Result<I> {
    let mut ptr = std::ptr::null_mut();
    unsafe {
        let mut code = RoGetActivationFactory(
            HString::from(C::type_name()).abi(),
            I::type_guid(),
            &mut ptr,
        );

        if code == ErrorCode::NOT_INITIALIZED {
            let mut _cookie = std::ptr::null_mut();
            CoIncrementMTAUsage(&mut _cookie);

            code = RoGetActivationFactory(
                HString::from(C::type_name()).abi(),
                I::type_guid(),
                &mut ptr,
            );
        }

        code.and_then(|| std::mem::transmute_copy(&ptr))
    }
}

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct IActivationFactory {
    ptr: IUnknown,
}

impl IActivationFactory {
    pub fn activate_instance<I: TypeGuid>(&self) -> Result<I> {
        let mut object = Object::default();
        unsafe {
            ((*(*(self.ptr.get() as *const *const abi_IActivationFactory))).activate_instance)(
                self.ptr.get(),
                object.set_abi(),
            )
            .and_then(|| safe_query(&object))
        }
    }
}

impl TypeGuid for IActivationFactory {
    fn type_guid() -> &'static Guid {
        static GUID: Guid = Guid::from_values(
            0x0000_0035,
            0x0000,
            0x0000,
            [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
        );
        &GUID
    }
}

#[repr(C)]
struct abi_IActivationFactory {
    __base: [usize; 6],
    activate_instance: extern "system" fn(RawPtr, *mut RawPtr) -> ErrorCode,
}
