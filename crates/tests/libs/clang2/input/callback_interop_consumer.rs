mod bindings;
use bindings::*;

unsafe extern "system" fn system(value: i32) -> i32 {
    value + 1
}

unsafe extern "C" fn cdecl(value: i32) -> i32 {
    value + 2
}

unsafe extern "C" fn factory(callback: SystemCallback) -> SystemCallback {
    callback
}

unsafe extern "C" fn accept(callback: SystemCallback) {
    unsafe { assert_eq!(callback.unwrap()(41), 42) };
}

fn check_selection(value: CallbackSelection) {
    let _: NamedSystem = value.system;
    let _: NamedCdecl = value.cdecl;
    let _: NestedSystem = value.nested_system;
    let _: NestedCdecl = value.nested_cdecl;
}

fn main() {
    let mut system = SystemCallbacks {
        named: Some(system),
        structural: None,
        factory: Some(factory),
        accept: Some(accept),
    };
    system.structural = system.named;
    system.named = system.structural;
    let mut cdecl = CdeclCallbacks {
        named: Some(cdecl),
        structural: None,
    };
    cdecl.structural = cdecl.named;
    cdecl.named = cdecl.structural;
    unsafe {
        assert_eq!(system.named.unwrap()(41), 42);
        assert_eq!(system.structural.unwrap()(41), 42);
        let callback = system.factory.unwrap()(system.named);
        assert_eq!(callback.unwrap()(41), 42);
        system.accept.unwrap()(callback);
        assert_eq!(cdecl.named.unwrap()(40), 42);
        assert_eq!(cdecl.structural.unwrap()(40), 42);
    }
    check_selection(CallbackSelection {
        system: None,
        cdecl: None,
        nested_system: None,
        nested_cdecl: None,
    });
}
