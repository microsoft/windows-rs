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

        let mut code = RoGetActivationFactory(
            HString::from(C::type_name()).abi(),
            I::type_guid(),
            &mut ptr,
        );

        if code == ErrorCode::NOT_INITIALIZED {
            let mut cookie = std::ptr::null_mut();
            CoIncrementMTAUsage(&mut cookie);

            code = RoGetActivationFactory(
                HString::from(C::type_name()).abi(),
                I::type_guid(),
                &mut ptr,
            );
        }

        code.ok_or(std::mem::transmute_copy(&ptr))
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct IActivationFactory {
    ptr: com::ComPtr<dyn abi::IActivationFactory>,
}

impl IActivationFactory {
    pub fn activate_instance<I: QueryType>(&self) -> Result<I> {
        use abi::IActivationFactory;
        let mut instance = std::ptr::null_mut();

        unsafe {
            // TODO: this is cheating - we need a QI here...
            self.ptr
                .activate_instance(&mut instance)
                .ok_or(std::mem::transmute_copy(&instance))
        }
    }
}

impl QueryType for IActivationFactory {
    fn type_guid() -> &'static Guid {
        use com::ComInterface;
        static GUID: Guid = Guid(abi::IActivationFactory::IID);
        &GUID
    }
}

mod abi {
    #[com::com_interface("00000035-0000-0000-C000-000000000046")]
    pub trait IActivationFactory: crate::object::abi::IInspectable {
        unsafe fn activate_instance(&self, instance: *mut crate::RawPtr) -> crate::ErrorCode;
    }
}
