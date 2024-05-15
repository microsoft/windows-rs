use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::sync::Arc;
use windows_core::{implement, interface, ComObject, IUnknown, IUnknown_Vtbl};

#[interface("818f2fd1-d479-4398-b286-a93c4c7904d1")]
unsafe trait IFoo: IUnknown {
    fn get_x(&self) -> u32;
}

#[implement(IFoo)]
struct MyApp {
    x: u32,
    tombstone: Arc<Tombstone>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            x: 0,
            tombstone: Arc::new(Tombstone::default()),
        }
    }
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
    fn new(x: u32) -> ComObject<Self> {
        ComObject::new(Self {
            x,
            tombstone: Arc::new(Tombstone::default()),
        })
    }

    fn get_x_direct(&self) -> u32 {
        self.x
    }

    fn set_x(&mut self, x: u32) {
        self.x = x;
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
    let app: ComObject<MyApp> = MyApp::new(42);
    let iunknown: IUnknown = app.cast().unwrap();
    let ifoo: IFoo = app.cast().unwrap();
    assert_eq!(unsafe { ifoo.get_x() }, 42);

    // check lifetimes
    let tombstone = app.get().tombstone.clone();
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
    let app: ComObject<MyApp> = MyApp::new(42);
    let tombstone = app.get().tombstone.clone();

    let ifoo: IFoo = app.cast().unwrap();
    assert_eq!(unsafe { app.get().get_x() }, 42);

    // check lifetimes
    assert!(!tombstone.is_dead());

    drop(app);
    assert!(!tombstone.is_dead());

    drop(ifoo);
    assert!(tombstone.is_dead());
}

#[test]
fn clone() {
    let app: ComObject<MyApp> = MyApp::new(42);
    let ifoo: IFoo = app.cast().unwrap();
    let ifoo2 = ifoo.clone();

    drop(ifoo);
    drop(app);
    assert_eq!(unsafe { ifoo2.get_x() }, 42);
}

#[test]
fn get_mut() {
    let mut app: ComObject<MyApp> = MyApp::new(42);
    assert_eq!(app.get().get_x_direct(), 42);

    // refcount = 1
    app.get_mut().unwrap().set_x(50);

    assert_eq!(app.get().get_x_direct(), 50);

    let app2 = app.clone();
    // refcount = 2
    assert!(app.get_mut().is_none());

    drop(app2);

    // refcount = 1 again
    app.get_mut().unwrap().set_x(60);
}

#[test]
fn try_take() {
    let app: ComObject<MyApp> = MyApp::new(42);
    let tombstone = app.get().tombstone.clone();
    // refcount = 1

    let app2 = app.clone();
    // refcount = 2

    let app2_rejected: ComObject<MyApp> = match app2.try_take() {
        Ok(_unexpected) => panic!("try_take should have failed"),
        Err(e) => e,
    };
    // refcount = 1

    drop(app2_rejected);
    // refcount = 1

    match app.try_take() {
        Ok(unwrapped_app) => {
            // box destroyed
            assert!(!tombstone.is_dead());
            assert_eq!(unwrapped_app.x, 42);
            drop(unwrapped_app);
            assert!(tombstone.is_dead());
        }

        Err(_unexpected) => {
            panic!("try_take should have succeeded");
        }
    }
}

#[test]
fn object_interfaces() {
    let app = MyApp::new(42);
    let ifoo_ref = app.borrow_interface::<IFoo>();
    let ifoo = ifoo_ref.ok().unwrap();
    assert_eq!(unsafe { ifoo.get_x() }, 42);
}

#[test]
fn construct_with_com_object_new() {
    // Tests that we can construct using ComObject::new().
    let app: ComObject<MyApp> = ComObject::new(MyApp::default());
    let _ = app;
}

#[test]
fn construct_with_com_object_from() {
    // Tests that we can construct using ComObject::from().
    let app: ComObject<MyApp> = ComObject::from(MyApp::default());
    let _ = app;
}

#[test]
fn construct_with_into() {
    // Tests that we can construct using ComObject::from().
    fn consume(_: ComObject<MyApp>) {}

    consume(MyApp::default().into())
}
