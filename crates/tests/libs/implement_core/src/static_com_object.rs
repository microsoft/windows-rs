//! Unit tests for `windows_core::StaticComObject`

use std::sync::atomic::{AtomicU32, Ordering::SeqCst};
use windows_core::{
    implement, interface, ComObject, IUnknown, IUnknownImpl, IUnknown_Vtbl, InterfaceRef,
    StaticComObject,
};

#[interface("818f2fd1-d479-4398-b286-a93c4c7904d1")]
unsafe trait INumberFactory: IUnknown {
    fn next(&self) -> u32;

    fn add(&self, x: u32, y: u32) -> u32;
}

#[implement(INumberFactory)]
struct MyFactory {
    x: AtomicU32,
}

impl INumberFactory_Impl for MyFactory_Impl {
    unsafe fn next(&self) -> u32 {
        self.x.fetch_add(1, SeqCst)
    }

    unsafe fn add(&self, x: u32, y: u32) -> u32 {
        x + y
    }
}

static NUMBER_FACTORY_INSTANCE: StaticComObject<MyFactory> = MyFactory {
    x: AtomicU32::new(100),
}
.into_static();

#[test]
fn as_interface() {
    let factory_outer: &MyFactory_Impl = NUMBER_FACTORY_INSTANCE.get();
    let ifactory: InterfaceRef<INumberFactory> = factory_outer.as_interface::<INumberFactory>();

    // Produce the next number. We don't verify the value since tests are multi-threaded.
    // This just demonstrates that you can have shared state with interior mutability (such as
    // atomics) in a static COM object.
    let n = unsafe { ifactory.next() };
    println!("n = {n:?}");

    assert_eq!(unsafe { ifactory.add(333, 444) }, 777);
}

// This tests that we can safely AddRef/Release a StaticComObject.
#[test]
fn to_interface() {
    let factory_outer: &MyFactory_Impl = NUMBER_FACTORY_INSTANCE.get();
    let ifactory: INumberFactory = factory_outer.to_interface::<INumberFactory>();
    assert_eq!(unsafe { ifactory.add(333, 444) }, 777);
    drop(ifactory);
}

#[test]
fn to_object() {
    let factory_outer: &MyFactory_Impl = NUMBER_FACTORY_INSTANCE.get();
    let factory_object: ComObject<MyFactory> = factory_outer.to_object();
    assert_eq!(unsafe { factory_object.add(333, 444) }, 777);
}

// This tests the behavior when dropping a StaticComObject. Since static variables are never
// dropped, this isn't relevant to normal usage. However, if app code constructs a StaticComObject
// in local variables (not statics) and them drops them, then we still need well-defined behavior.
// Basically, we are testing that the reference-count field does not panic when being dropped
// with a non-zero reference count.
#[test]
fn drop_half_constructed() {
    let _static_com_object: StaticComObject<MyFactory> = MyFactory {
        x: AtomicU32::new(0),
    }
    .into_static();
}
