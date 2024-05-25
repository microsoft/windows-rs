//! Unit tests for `windows_core::ComObject`

use std::borrow::Borrow;
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::sync::Arc;
use windows_core::{
    implement, interface, ComObject, IUnknown, IUnknownImpl, IUnknown_Vtbl, Interface, InterfaceRef,
};

#[interface("818f2fd1-d479-4398-b286-a93c4c7904d1")]
unsafe trait IFoo: IUnknown {
    fn get_x(&self) -> u32;

    fn get_self_as_bar(&self) -> IBar;
}

#[interface("687eb4b2-6df6-41a3-86c7-4b04b94ad2d8")]
unsafe trait IBar: IUnknown {
    fn say_hello(&self);
}

const APP_SIGNATURE: [u8; 8] = *b"cafef00d";

#[implement(IFoo, IBar)]
struct MyApp {
    // We use signature to verify field offsets for dynamic casts
    signature: [u8; 8],
    x: u32,
    tombstone: Arc<Tombstone>,
}

impl IFoo_Impl for MyApp {
    unsafe fn get_x(&self) -> u32 {
        self.x
    }

    unsafe fn get_self_as_bar(&self) -> IBar {
        let outer = MyApp_Impl::from_inner_ref(self);
        outer.to_interface()
    }
}

impl IBar_Impl for MyApp {
    unsafe fn say_hello(&self) {
        println!("Hello!");
    }
}

impl Borrow<u32> for MyApp {
    fn borrow(&self) -> &u32 {
        &self.x
    }
}

impl core::fmt::Debug for MyApp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "x = {}", self.x)
    }
}

impl core::fmt::Display for MyApp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "x = {}", self.x)
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            signature: APP_SIGNATURE,
            x: 0,
            tombstone: Arc::new(Tombstone::default()),
        }
    }
}

impl std::hash::Hash for MyApp {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
    }
}

impl PartialEq<u32> for MyApp {
    fn eq(&self, other: &u32) -> bool {
        self.x == *other
    }
}

impl PartialEq for MyApp {
    fn eq(&self, other: &MyApp) -> bool {
        self.x == other.x
    }
}

impl Eq for MyApp {}

/// This lets us detect when an object has been dropped.
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
            signature: APP_SIGNATURE,
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

#[test]
fn basic() {
    let app: ComObject<MyApp> = MyApp::new(42);
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
    let app: ComObject<MyApp> = MyApp::new(42);
    let tombstone = app.tombstone.clone();

    let ifoo: IFoo = app.cast().unwrap();
    assert_eq!(unsafe { app.get_x() }, 42);

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
    assert_eq!(app.get_x_direct(), 42);

    // refcount = 1
    app.get_mut().unwrap().set_x(50);

    assert_eq!(app.get_x_direct(), 50);

    let app2 = app.clone();
    // refcount = 2
    assert!(app.get_mut().is_none());

    drop(app2);

    // refcount = 1 again
    app.get_mut().unwrap().set_x(60);
}

#[test]
fn take() {
    let app: ComObject<MyApp> = MyApp::new(42);
    let tombstone = app.tombstone.clone();
    // refcount = 1

    let app2 = app.clone();
    // refcount = 2

    let app2_rejected: ComObject<MyApp> = match app2.take() {
        Ok(_unexpected) => panic!("take() should have failed"),
        Err(e) => e,
    };
    // refcount = 1

    drop(app2_rejected);
    // refcount = 1

    match app.take() {
        Ok(unwrapped_app) => {
            // box destroyed
            assert!(!tombstone.is_dead());
            assert_eq!(unwrapped_app.x, 42);
            drop(unwrapped_app);
            assert!(tombstone.is_dead());
        }

        Err(_unexpected) => {
            panic!("take() should have succeeded");
        }
    }
}

#[test]
fn as_interface() {
    let app = MyApp::new(42);
    let tombstone = app.tombstone.clone();

    // All ComObject<T> implement IUnknown.
    let _ = app.as_interface::<IUnknown>();

    let ifoo = app.as_interface::<IFoo>();
    assert_eq!(unsafe { ifoo.get_x() }, 42);
    assert!(!tombstone.is_dead());

    drop(app);
    assert!(tombstone.is_dead());
}

#[test]
fn to_interface() {
    let app = MyApp::new(42);
    let tombstone = app.tombstone.clone();

    // All ComObject<T> implement IUnknown.
    drop(app.to_interface::<IUnknown>());

    let ifoo = app.to_interface::<IFoo>();
    assert_eq!(unsafe { ifoo.get_x() }, 42);
    assert!(!tombstone.is_dead());

    drop(app);
    assert!(!tombstone.is_dead());

    drop(ifoo);
    assert!(tombstone.is_dead());
}

#[test]
fn into_interface() {
    let app = MyApp::new(42);
    let tombstone = app.tombstone.clone();

    let ifoo = app.into_interface::<IFoo>();
    assert_eq!(unsafe { ifoo.get_x() }, 42);
    assert!(!tombstone.is_dead());

    drop(ifoo);
    assert!(tombstone.is_dead());
}

#[test]
fn construct_with_com_object_new() {
    // Test that we can construct using ComObject::new().
    let app: ComObject<MyApp> = ComObject::new(MyApp::default());
    let _ = app;
}

#[test]
fn construct_with_com_object_from() {
    // Test that we can construct using ComObject::from().
    let app: ComObject<MyApp> = ComObject::from(MyApp::default());
    let _ = app;
}

#[test]
fn construct_with_into() {
    // Test that we can construct using ComObject::from().
    fn consume(_: ComObject<MyApp>) {}

    consume(MyApp::default().into())
}

#[test]
fn debug() {
    let app = MyApp::new(100);
    let s = format!("{:?}", app);
    assert_eq!(s, "x = 100");
}

#[test]
fn display() {
    let app = MyApp::new(200);
    let s = format!("{}", app);
    assert_eq!(s, "x = 200");
}

#[test]
fn hashable() {
    use std::collections::HashMap;

    let mut map: HashMap<ComObject<MyApp>, &'static str> = HashMap::new();
    map.insert(MyApp::new(100), "hello");
    map.insert(MyApp::new(200), "world");
}

#[test]
fn from_inner_ref() {
    let app = MyApp::new(42);
    let ifoo: InterfaceRef<IFoo> = app.as_interface();
    let ibar: IBar = unsafe { ifoo.get_self_as_bar() };
    unsafe { ibar.say_hello() };
}

#[test]
fn to_object() {
    let app = MyApp::new(42);
    let tombstone = app.tombstone.clone();
    let app_outer: &MyApp_Impl = &app;

    let second_app = app_outer.to_object();
    assert!(!tombstone.is_dead());
    assert_eq!(second_app.signature, APP_SIGNATURE);

    println!("x = {}", unsafe { second_app.get_x() });

    drop(second_app);
    assert!(!tombstone.is_dead());

    drop(app);
    assert!(tombstone.is_dead());
}

#[test]
fn dynamic_cast() {
    let app = MyApp::new(42);
    let unknown = app.to_interface::<IUnknown>();

    assert!(!unknown.is_object::<SendableThing>());
    assert!(unknown.is_object::<MyApp>());

    let dyn_app_ref: &MyApp_Impl = unknown.cast_object_ref::<MyApp>().unwrap();
    assert_eq!(dyn_app_ref.signature, APP_SIGNATURE);

    let dyn_app_owned: ComObject<MyApp> = unknown.cast_object().unwrap();
    assert_eq!(dyn_app_owned.signature, APP_SIGNATURE);

    let dyn_app_owned_2: ComObject<MyApp> = ComObject::cast_from(&unknown).unwrap();
    assert_eq!(dyn_app_owned_2.signature, APP_SIGNATURE);
}

// This tests that we can place a type that is not Send in a ComObject.
// Compilation is sufficient to test.
#[implement(IBar)]
struct UnsendableThing {
    cell: core::cell::Cell<u32>,
}

impl IBar_Impl for UnsendableThing {
    unsafe fn say_hello(&self) {
        println!("{}", self.cell.get());
    }
}

static_assertions::assert_not_impl_all!(UnsendableThing: Send, Sync);
static_assertions::assert_not_impl_all!(ComObject<UnsendableThing>: Send, Sync);

#[implement(IBar)]
struct SendableThing {
    arc: std::sync::Arc<u32>,
}

impl IBar_Impl for SendableThing {
    unsafe fn say_hello(&self) {
        println!("{}", *self.arc);
    }
}

static_assertions::assert_impl_all!(SendableThing: Send, Sync);
static_assertions::assert_impl_all!(ComObject<SendableThing>: Send, Sync);
