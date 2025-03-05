use windows_core::*;

#[interface("cccccccc-0000-0000-0000-000000000001")]
unsafe trait IBase: IUnknown {
    fn get_x(&self) -> u32;
}

#[implement(IBase)]
struct Base {
    x: u32,
}

impl IBase_Impl for Base_Impl {
    unsafe fn get_x(&self) -> u32 {
        self.x
    }
}

/*
#[interface("cccccccc-0000-0000-0000-000000000002")]
unsafe trait IBase2: IBase {
    fn get_x_plus_one(&self) -> u32;
}
*/

#[interface("cccccccc-0000-0000-0000-000000000003")]
unsafe trait IDerived: IUnknown {
    fn get_y(&self) -> u32;
}

#[implement(IDerived)]
struct Derived {
    #[base]
    base: Base,

    y: u32,
}

/*
impl IBase2_Impl for Derived_Impl {
    unsafe fn get_x_plus_one(&self) -> u32 {
        self.base().x + 1
    }
}
    */

impl IDerived_Impl for Derived_Impl {
    unsafe fn get_y(&self) -> u32 {
        self.y
    }
}

#[test]
fn make_base_class() {
    let base = ComObject::new(Base { x: 42 });
    let ibase: IBase = base.to_interface();
    unsafe {
        assert_eq!(ibase.get_x(), 42);
    }
}

#[test]
fn make_derived_class() {
    let base = ComObject::new(Base { x: 42 });
    let derived = ComObject::new_aggregated(Derived { y: 100 }, base);

    // let ibase: IBase = derived.to_interface();
    let ibase: IBase = derived.cast().unwrap();
    let iderived: IDerived = derived.to_interface();

    unsafe {
        assert_eq!(ibase.get_x(), 42);
        assert_eq!(iderived.get_y(), 100);
    }
}
