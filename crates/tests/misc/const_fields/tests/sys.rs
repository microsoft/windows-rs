use windows_sys::{Win32::Storage::CloudFilters::*, Win32::System::CorrelationVector::*};

#[test]
fn can_assign_const() {
    unsafe {
        let v: CORRELATION_VECTOR = std::mem::zeroed();
        let mut i: CF_OPERATION_INFO = std::mem::zeroed();

        // Note that the rhs expression does not need to be mutable.
        i.CorrelationVector = &v;
    }
}
