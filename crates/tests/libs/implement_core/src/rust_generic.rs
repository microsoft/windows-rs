use windows_core::{implement, interface, ComObject, IUnknown, IUnknown_Vtbl};

#[interface("91617cc4-df1f-42e5-b6c8-7dd820a2698c")]
unsafe trait INumberFactory: IUnknown {
    fn get(&self) -> u32;
}

#[implement(INumberFactory)]
struct MyFactory<T: Copy + Into<u32> + 'static>(T);

impl<T: Copy + Into<u32> + 'static> INumberFactory_Impl for MyFactory_Impl<T> {
    unsafe fn get(&self) -> u32 {
        self.0.into()
    }
}

#[test]
fn basic() {
    let object = ComObject::new(MyFactory(42u8));
    let ifactory: INumberFactory = object.cast().unwrap();
    assert_eq!(unsafe { ifactory.get() }, 42);
}
