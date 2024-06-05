use windows_core::*;

#[interface("cccccccc-0000-0000-0000-000000000001")]
unsafe trait IFoo: IUnknown {
    fn hello(&self);
    fn self_as_bar(&self) -> IBar;
}

#[interface("cccccccc-0000-0000-0000-000000000002")]
unsafe trait IFoo2: IFoo {
    fn hello(&self);
}

#[interface("cccccccc-0000-0000-0000-000000000003")]
unsafe trait IFoo3: IFoo2 {
    fn hello(&self);
}

#[interface("cccccccc-0000-0000-0000-000000000004")]
unsafe trait IBar: IUnknown {
    fn goodbye(&self);
}

// This tests that we can compile a COM object that has some COM interfaces implemented on the
// outer object and some on the inner object.
#[implement(IFoo3 @ Outer, IBar)]
struct MyApp {}

impl IFoo_Impl for ComObject<MyApp> {
    unsafe fn hello(&self) {
        println!("MyApp as IFoo: hello");
    }

    unsafe fn self_as_bar(&self) -> IBar {
        println!("MyApp as IFoo::self_as_bar");
        self.to_interface()
    }
}
impl IFoo2_Impl for ComObject<MyApp> {
    unsafe fn hello(&self) {
        println!("MyApp as IFoo2: hello");
    }
}
impl IFoo3_Impl for ComObject<MyApp> {
    unsafe fn hello(&self) {
        println!("MyApp as IFoo3: hello");
    }
}

impl IBar_Impl for MyApp {
    unsafe fn goodbye(&self) {
        println!("MyApp as IBar: goodbye");
    }
}

#[test]
fn basic() {
    let app = ComObject::new(MyApp {});
    let ifoo3: IFoo3 = app.cast().unwrap();
    let ifoo2: IFoo2 = app.cast().unwrap();
    let ifoo: IFoo = app.cast().unwrap();
    let ibar: IBar = app.cast().unwrap();
    unsafe {
        ifoo.hello();
        ifoo2.hello();
        ifoo3.hello();
        ibar.goodbye();
    }

    unsafe {
        let ibar = ifoo3.self_as_bar();
        ibar.goodbye();
    }
}
