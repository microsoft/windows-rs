//! Unit tests for `windows_core::StaticComObject`

use std::sync::atomic::{AtomicU32, Ordering::SeqCst};
use windows_core::{
    ComObject, IUnknown, IUnknownImpl, InterfaceRef, StaticComObject, implement, interface,
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

    // The shared counter is nondeterministic because tests run concurrently.
    let n = unsafe { ifactory.next() };
    println!("n = {n:?}");

    assert_eq!(unsafe { ifactory.add(333, 444) }, 777);
}

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

// Local `StaticComObject` values must tolerate a nonzero reference count on drop.
#[test]
fn drop_half_constructed() {
    let _static_com_object: StaticComObject<MyFactory> = MyFactory {
        x: AtomicU32::new(0),
    }
    .into_static();
}
