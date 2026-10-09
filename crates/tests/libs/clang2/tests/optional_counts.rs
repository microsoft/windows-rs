#[cfg(target_env = "msvc")]
#[allow(non_snake_case, dead_code)]
mod native {
    include!(concat!(env!("OUT_DIR"), "/optional_counts.rs"));
}

#[test]
#[cfg(target_env = "msvc")]
fn native_optional_counts_keep_null_counts_zero_capacity_and_partial_output() {
    let _: unsafe extern "C" fn(*mut u8, u32, *mut u32) = native::FillRequired;
    unsafe {
        for fill in [
            native::FillRequired,
            native::FillOptional,
            native::FillInout,
        ] {
            let mut buffer = [0xff; 8];
            let mut written = 8;
            fill(buffer.as_mut_ptr(), 8, &mut written);
            assert_eq!(written, 3);
            assert_eq!(buffer, [1, 2, 3, 0xff, 0xff, 0xff, 0xff, 0xff]);
            buffer.fill(0xff);
            fill(buffer.as_mut_ptr(), 8, std::ptr::null_mut());
            assert_eq!(buffer, [1, 2, 3, 0xff, 0xff, 0xff, 0xff, 0xff]);
            fill(buffer.as_mut_ptr(), 0, &mut written);
            assert_eq!(written, 0);
            assert_eq!(buffer, [1, 2, 3, 0xff, 0xff, 0xff, 0xff, 0xff]);
            fill(buffer.as_mut_ptr(), 0, std::ptr::null_mut());
        }
        let mut written = 99;
        native::FillOptional(std::ptr::null_mut(), 0, &mut written);
        assert_eq!(written, 0);
        native::FillOptional(std::ptr::null_mut(), 8, std::ptr::null_mut());
        let mut buffer = [0xff; 8];
        written = 2;
        native::FillInout(buffer.as_mut_ptr(), 8, &mut written);
        assert_eq!(written, 2);
        assert_eq!(buffer, [1, 2, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    }
}

#[test]
#[cfg(target_env = "msvc")]
fn native_optional_counts_cross_callback_boundary_with_and_without_count() {
    unsafe extern "C" fn callback(buffer: *mut u8, capacity: u32, written: *mut u32) {
        unsafe {
            if capacity != 0 {
                buffer.write(42);
            }
            if !written.is_null() {
                written.write(u32::from(capacity != 0));
            }
        }
    }
    unsafe {
        let mut buffer = [0xff; 2];
        let mut written = 99;
        native::InvokeOptional(Some(callback), buffer.as_mut_ptr(), 2, &mut written);
        assert_eq!(buffer, [42, 0xff]);
        assert_eq!(written, 1);
        buffer.fill(0xff);
        native::InvokeOptional(Some(callback), buffer.as_mut_ptr(), 2, std::ptr::null_mut());
        assert_eq!(buffer, [42, 0xff]);
        native::InvokeOptional(Some(callback), buffer.as_mut_ptr(), 0, &mut written);
        assert_eq!(written, 0);
    }
}
