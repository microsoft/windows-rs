use windows_core::*;

#[interface("cccccccc-0000-0000-0000-000000000001")]
unsafe trait IFoo: IUnknown {}

#[interface("cccccccc-0000-0000-0000-000000000002")]
unsafe trait IFoo2: IFoo {}

#[interface("cccccccc-0000-0000-0000-000000000003")]
unsafe trait IFoo3: IFoo2 {}

// ObjectA implements a single interface chain, which consists of 3 different
// interfaces: IFoo3, IFoo2, and IFoo. You do not need to explicitly list all
// of the interfaces in the interface chain. Listing all of the interfaces is
// less efficient because it generates redundant interface chains (pointer
// fields in the the generated ObjectA_Impl type), which will never be used.
#[implement(IFoo3)]
struct ObjectWithChains {}

impl IFoo_Impl for ObjectWithChains_Impl {}
impl IFoo2_Impl for ObjectWithChains_Impl {}
impl IFoo3_Impl for ObjectWithChains_Impl {}

#[test]
fn interface_chain_query() {
    let object = ComObject::new(ObjectWithChains {});
    let unknown: IUnknown = object.to_interface();
    let _foo: IFoo = unknown.cast().expect("QueryInterface for IFoo");
    let _foo2: IFoo2 = unknown.cast().expect("QueryInterface for IFoo2");
    let _foo3: IFoo3 = unknown.cast().expect("QueryInterface for IFoo3");
}

// ObjectRedundantChains implements the same interfaces as ObjectWithChains,
// but it defines more than one interface chain. This is unnecessary because it
// is redundant, but we are verifying that this works.
#[implement(IFoo3, IFoo2, IFoo)]
struct ObjectRedundantChains {}

impl IFoo_Impl for ObjectRedundantChains_Impl {}
impl IFoo2_Impl for ObjectRedundantChains_Impl {}
impl IFoo3_Impl for ObjectRedundantChains_Impl {}

#[test]
fn redundant_interface_chains() {
    let object = ComObject::new(ObjectRedundantChains {});
    let unknown: IUnknown = object.to_interface();
    let _foo: IFoo = unknown.cast().expect("QueryInterface for IFoo");
    let _foo2: IFoo2 = unknown.cast().expect("QueryInterface for IFoo2");
    let _foo3: IFoo3 = unknown.cast().expect("QueryInterface for IFoo3");
}
