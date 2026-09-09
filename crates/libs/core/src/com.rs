use crate::{GUID, Interface, Result, imp};

/// Creates an instance of an in-process COM class and returns the requested interface.
///
/// The calling thread must already be initialized for COM in an apartment supported by the
/// class.
pub fn create_instance<T: Interface>(class: &GUID) -> Result<T> {
    unsafe {
        let mut value = core::ptr::null_mut();
        imp::CoCreateInstance(
            class,
            core::ptr::null_mut(),
            imp::CLSCTX_INPROC_SERVER,
            &T::IID,
            &mut value,
        )
        .and_then(|| imp::Type::from_abi(value))
    }
}
