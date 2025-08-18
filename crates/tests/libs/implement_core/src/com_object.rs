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

    fn common(&self) -> u32;
}

#[interface("687eb4b2-6df6-41a3-86c7-4b04b94ad2d8")]
unsafe trait IBar: IUnknown {
    fn say_hello(&self);

    fn common(&self) -> u64;
}

#[interface("4351c285-97ad-450a-b445-8795632d2fb9")]
unsafe trait IBar2: IBar {
    fn common(&self) -> f32;
}

const APP_SIGNATURE: [u8; 8] = *b"cafef00d";

// We are intentionally declaring IBar twice (in two different interface chains).
// If you change this, you will need to update tests.
#[implement(IFoo, IBar, IBar2)]
struct MyApp {
    // We use signature to verify field offsets for dynamic casts
    signature: [u8; 8],
    x: u32,
    tombstone: Arc<Tombstone>,
}

impl IFoo_Impl for MyApp_Impl {
    unsafe fn get_x(&self) -> u32 {
        self.x
    }

    unsafe fn get_self_as_bar(&self) -> IBar {
        self.to_interface()
    }

    unsafe fn common(&self) -> u32 {
        100
    }
}

impl IBar_Impl for MyApp_Impl {
    unsafe fn say_hello(&self) {
        println!("Hello!");
    }

    unsafe fn common(&self) -> u64 {
        1_000_000_000_000
    }
}

impl IBar2_Impl for MyApp_Impl {
    unsafe fn common(&self) -> f32 {
        std::f32::consts::PI
    }
}

impl Borrow<u32> for MyApp {
    fn borrow(&self) -> &u32 {
        &self.x
    }
}

impl core::fmt::Debug for MyApp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "x = {}", self.x)
    }
}

impl core::fmt::Display for MyApp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
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
fn com_object_debug() {
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

// Test that we can invoke the correct method in situations where two different
// interfaces declare a method with the same name, including situations where
// one of the interfaces inherits from the other.
#[test]
fn common_method_name() {
    let app = MyApp::new(42);

    let ifoo: IFoo = app.to_interface();
    assert_eq!(unsafe { ifoo.common() }, 100);

    let ibar: IBar = app.to_interface();
    assert_eq!(unsafe { ibar.common() }, 1_000_000_000_000);

    let ibar2: IBar2 = app.to_interface();
    assert_eq!(unsafe { ibar2.common() }, std::f32::consts::PI);
}

#[test]
fn interface_debug_fmt() {
    let app = MyApp::new(42);

    let iunknown: IUnknown = app.to_interface();
    let unknown_dbg = format!("{iunknown:?}");
    assert!(unknown_dbg.starts_with("IUnknown(0x"), "{unknown_dbg:?}");

    let ifoo: IFoo = app.to_interface();
    let foo_dbg = format!("{ifoo:?}");
    assert!(foo_dbg.starts_with("IFoo(0x"), "{foo_dbg:?}");
}

// Test that we always get the same IUnknown pointer for an object, regardless of which
// interface we use to query for it.
#[test]
fn iunknown_identity() {
    let app = MyApp::new(0);

    // iunknown is from the identity vtable slot. It is the canonical IUnknown pointer.
    let iunknown: IUnknown = app.to_interface();

    // Get the most-derived interface of each interface chain.
    let _ifoo: IFoo = app.to_interface();
    let ibar: IBar = app.to_interface();
    let ibar2: IBar2 = app.to_interface();

    // Do a static cast from IBar to IUnknown and verify that the interface pointer for the
    // resulting IUnknown is _not_ the same as the canonical IUnknown pointer.
    {
        let ibar_iunknown_static: IUnknown = (*ibar).clone();
        assert_ne!(
            ibar_iunknown_static.as_raw(),
            iunknown.as_raw(),
            "IBar-to-IUnknown is non-canonical interface chain"
        );

        // The equality implementation uses QueryInterface to check that they are the same pointer.
        assert_eq!(
            ibar_iunknown_static, iunknown,
            "QueryInterface for IUnknown yields same pointer"
        );
    }

    // ibar and ibar2_ibar point to different interface chains, even though they have the same
    // COM interface type.
    let ibar2_ibar: &IBar = &ibar2; // static cast
    assert_ne!(
        ibar.as_raw(),
        ibar2_ibar.as_raw(),
        "IBar from different interface chains have different pointer values"
    );
    assert_eq!(
        ibar, *ibar2_ibar,
        "IBar from different interface chains are equal (using QueryInterface)"
    );
}

// This tests that we can place a type that is not Send in a ComObject.
// Compilation is sufficient to test.
#[implement(IBar)]
struct UnsendableThing {
    cell: core::cell::Cell<u32>,
}

impl IBar_Impl for UnsendableThing_Impl {
    unsafe fn say_hello(&self) {
        println!("{}", self.cell.get());
    }

    unsafe fn common(&self) -> u64 {
        0
    }
}

static_assertions::assert_not_impl_all!(UnsendableThing: Send, Sync);
static_assertions::assert_not_impl_all!(ComObject<UnsendableThing>: Send, Sync);

#[implement(IBar)]
struct SendableThing {
    arc: std::sync::Arc<u32>,
}

impl IBar_Impl for SendableThing_Impl {
    unsafe fn say_hello(&self) {
        println!("{}", *self.arc);
    }

    unsafe fn common(&self) -> u64 {
        0
    }
}

static_assertions::assert_impl_all!(SendableThing: Send, Sync);
static_assertions::assert_impl_all!(ComObject<SendableThing>: Send, Sync);
