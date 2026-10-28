const _: unsafe extern "system" fn() = Clear;
const _: unsafe extern "system" fn(ConstAddress) = Set;
const _: unsafe extern "system" fn() -> Address = Get;

unsafe extern "system" fn invoke(_: Address) {}

const _: Callback = Some(invoke);

fn check(value: Holder, vtable: IConsumer_Vtbl) {
    let _: *mut Again = value.direct_value;
    let _: *const Nothing = value.qualified_value;
    let _: unsafe extern "system" fn(*mut core::ffi::c_void) = vtable.Clear;
    let _: unsafe extern "system" fn(*mut core::ffi::c_void, ConstAddress) = vtable.Set;
}
