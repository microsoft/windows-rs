use super::*;
use crate::{IUnknown_Vtbl, IUnknownImpl, Interface};
use core::any::Any;
use core::ffi::c_void;
use core::mem::MaybeUninit;

define_interface!(
    IRustObject,
    IRustObject_Vtbl,
    0xb48fa9c9_a2be_48b3_9849_0e86dba71671
);

impl IRustObject {
    pub(crate) fn get_any(&self) -> &dyn Any {
        unsafe {
            let mut result = MaybeUninit::<&dyn Any>::uninit();
            (self.vtable().GetAny)(self.as_raw(), result.as_mut_ptr().cast());
            result.assume_init()
        }
    }
}

#[repr(C)]
pub struct IRustObject_Vtbl {
    base__: IUnknown_Vtbl,
    GetAny: unsafe extern "system" fn(*mut c_void, *mut c_void),
}

impl IRustObject_Vtbl {
    pub const fn new<T: IUnknownImpl + Any, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAny<T: IUnknownImpl + Any, const OFFSET: isize>(
            this: *mut c_void,
            result: *mut c_void,
        ) {
            unsafe {
                let this = (this as *mut *mut c_void).offset(OFFSET).cast::<T>();
                let any: &dyn Any = &*this;
                result.cast::<&dyn Any>().write(any);
            }
        }
        Self {
            base__: IUnknown_Vtbl::new::<T, OFFSET>(),
            GetAny: GetAny::<T, OFFSET>,
        }
    }
}
