use windows_core::{implement, interface, BoxRef, IUnknown, IUnknown_Vtbl};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

#[interface("818f2fd1-d479-4398-b286-a93c4c7904d1")]
unsafe trait IFoo: IUnknown {
    fn get_x(&self) -> u32;
}

#[implement(IFoo)]
struct MyApp {
    x: u32,
    tombstone: Arc<Tombstone>
}

#[derive(Default)]
struct Tombstone {
    cell: AtomicBool,
}

impl Tombstone {
    fn is_dead(&self) -> bool {
        self.cell.load(SeqCst)
    }

    fn mark_dead(&self) {
        self.cell.store(true, SeqCst);
    }
}

impl MyApp {
    fn new(x: u32) -> BoxRef<Self> {
        BoxRef::new(Self {
            x,
            tombstone: Arc::new(Tombstone::default()),
        })
    }
}

impl Drop for MyApp {
    fn drop(&mut self) {
        println!("MyApp::drop");
        self.tombstone.mark_dead();
    }
}

impl IFoo_Impl for MyApp {
    unsafe fn get_x(&self) -> u32 {
        self.x
    }
}

#[test]
fn basic() {
    let app: BoxRef<MyApp> = MyApp::new(42);
    let iunknown: IUnknown = app.cast().unwrap();
    let ifoo: IFoo = app.cast().unwrap();
    assert_eq!(unsafe { ifoo.get_x() }, 42);

    // check lifetimes
    let tombstone = app.tombstone.clone();
    assert!(!tombstone.is_dead());

    drop(app);
    assert!(!tombstone.is_dead());

    drop(iunknown);
    assert!(!tombstone.is_dead());

    drop(ifoo);
    assert!(tombstone.is_dead());
}

#[test]
fn casting() {
    let app: BoxRef<MyApp> = MyApp::new(42);
    let ifoo: IFoo = app.cast().unwrap();
    assert_eq!(unsafe { app.get_x() }, 42);

    // check lifetimes
    let tombstone = app.tombstone.clone();
    assert!(!tombstone.is_dead());

    drop(app);
    assert!(!tombstone.is_dead());

    drop(ifoo);
    assert!(tombstone.is_dead());
}

#[test]
fn clone() {
    let app: BoxRef<MyApp> = MyApp::new(42);
    let ifoo: IFoo = app.cast().unwrap();
    let ifoo2 = ifoo.clone();

    drop(ifoo);
    drop(app);
    assert_eq!(unsafe { ifoo2.get_x() }, 42);
}
