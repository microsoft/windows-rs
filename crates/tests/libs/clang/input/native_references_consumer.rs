#![allow(non_snake_case, non_camel_case_types, dead_code)]
include!("bindings.rs");

unsafe extern "system" fn callback(value: *const ReferenceAlias, total: *mut u32) -> u32 {
    unsafe {
        *total += (*value).low + (*value).high as u32;
        *total
    }
}

unsafe extern "C" fn anonymous(value: *const ReferenceAlias, total: *mut u32, extra: i32) -> u32 {
    unsafe {
        *total += extra as u32;
        callback(value, total)
    }
}

fn main() {
    let value = ReferenceValue { low: 5, high: 7 };
    let mut total = 9;
    #[cfg(raw_bindings)]
    unsafe {
        let _: unsafe extern "system" fn(*const ReferenceAlias, *mut u32) -> u32 = ApplyReference;
        let _: ReferenceCallback = Some(callback);
        assert_eq!(ApplyReference(&value, &mut total), 21);
        assert_eq!(InvokeReference(Some(callback), &value, &mut total), 33);
        assert_eq!(InvokeAnonymous(Some(anonymous), &value, &mut total), 48);
        assert_eq!(total, 48);
    }
    #[cfg(not(raw_bindings))]
    unsafe {
        use windows_core::Interface;
        unsafe extern "system" {
            fn ReferenceObject() -> *mut core::ffi::c_void;
            fn InvokeObject(
                object: *mut core::ffi::c_void,
                value: *const ReferenceAlias,
                total: *mut u32,
            ) -> u32;
        }
        let object = ReferenceObject();
        let vtable = &**object.cast::<*const IReferences_Vtbl>();
        let _: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *const ReferenceAlias,
            *mut u32,
        ) -> u32 = vtable.Apply;
        assert_eq!((vtable.Apply)(object, &value, &mut total), 21);
        let interface = IReferences::from_raw(object);
        assert_eq!(interface.Apply(&value, &mut total), 33);
        struct Producer;
        impl IReferences_Impl for Producer {
            fn Apply(&self, value: *const ReferenceAlias, total: *mut u32) -> u32 {
                unsafe { callback(value, total) }
            }
        }
        let producer = IReferences::new(&Producer);
        assert_eq!(InvokeObject(producer.as_raw(), &value, &mut total), 45);
        assert_eq!(total, 45);
    }
}
