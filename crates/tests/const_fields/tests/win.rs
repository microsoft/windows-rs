use windows::{Win32::Storage::CloudFilters::*, Win32::System::CorrelationVector::*};

#[test]
fn can_assign_const() {
    let v: CORRELATION_VECTOR = Default::default();
    let mut i: CF_OPERATION_INFO = Default::default();

    // Note that the rhs expression does not need to be mutable.
    i.CorrelationVector = &v;
}
