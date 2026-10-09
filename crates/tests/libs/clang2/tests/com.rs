#![cfg(target_env = "msvc")]

#[allow(
    non_snake_case,
    non_camel_case_types,
    dead_code,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/com.rs"));
}
#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    clippy::upper_case_acronyms
)]
mod sys {
    include!(concat!(env!("OUT_DIR"), "/com_sys.rs"));
}
use bindings::{IClassFactory, IProperties};
use sys::ComStats;
use windows_core::{HRESULT, IInspectable, IUnknown, Interface};

const E_NOINTERFACE: HRESULT = HRESULT(0x80004002_u32 as i32);
const CLASS_E_NOAGGREGATION: HRESULT = HRESULT(0x80040110_u32 as i32);

#[test]
fn shared_interface_aliases_preserve_identity_clone_and_release() {
    let mut factory_stats = ComStats::default();
    let mut instance_stats = ComStats::default();
    let factory: bindings::FirstFactory = unsafe {
        IClassFactory::from_raw(sys::ComFactory(
            &raw mut factory_stats,
            &raw mut instance_stats,
        ))
    };
    let other: bindings::SecondFactory = factory.clone();
    assert_eq!(bindings::FirstFactory::IID, bindings::SecondFactory::IID);
    assert_eq!(factory.as_raw(), other.as_raw());
    assert_eq!(counts(&factory_stats), [2, 1, 0, 0, 1, 0]);
    drop(other);
    assert_eq!(counts(&factory_stats), [1, 1, 1, 0, 1, 0]);
    drop(factory);
    assert_eq!(counts(&factory_stats), [0, 1, 2, 0, 1, 1]);
}

#[test]
fn generated_handle_setters_and_strings_reach_native_methods() {
    let properties = unsafe { IProperties::from_raw(sys::ComProperties()) };
    for value in [0, 0x1234, usize::MAX] {
        let window = core::ptr::without_provenance_mut(value);
        unsafe {
            properties.put_Window(window).unwrap();
            assert_eq!(properties.get_Window().unwrap(), window);
        }
    }
    unsafe {
        properties
            .SetText(windows_core::w!("native contract"))
            .unwrap();
        assert_eq!(
            properties
                .MatchText(windows_core::w!("native contract"))
                .unwrap(),
            1
        );
        assert_eq!(properties.MatchText(windows_core::w!("other")).unwrap(), 0);
    }
}

fn counts(stats: &ComStats) -> [u32; 6] {
    [
        stats.references,
        stats.adds,
        stats.releases,
        stats.queries,
        stats.created,
        stats.destroyed,
    ]
}

#[test]
fn generated_object_borrows_reach_native_methods() {
    fn _implementation_signature<T: bindings::IProperties_Impl>() {
        let _: fn(&T, windows_core::Ref<IUnknown>) -> windows_core::Result<()> = T::Inspect;
    }
    let properties = unsafe { IProperties::from_raw(sys::ComProperties()) };
    let mut factory_stats = ComStats::default();
    let mut instance_stats = ComStats::default();
    let factory = unsafe {
        IClassFactory::from_raw(sys::ComFactory(
            &raw mut factory_stats,
            &raw mut instance_stats,
        ))
    };
    unsafe {
        properties.Inspect(&factory).unwrap();
    }
    assert_eq!(counts(&factory_stats), [1, 1, 1, 1, 1, 0]);
    drop(factory);
    assert_eq!(counts(&factory_stats), [0, 1, 2, 1, 1, 1]);
}

#[test]
fn query_clone_and_drop_preserve_identity_and_ownership() {
    let mut factory_stats = ComStats::default();
    let mut instance_stats = ComStats::default();
    let factory = unsafe {
        IClassFactory::from_raw(sys::ComFactory(
            &raw mut factory_stats,
            &raw mut instance_stats,
        ))
    };
    assert_eq!(counts(&factory_stats), [1, 0, 0, 0, 1, 0]);
    let clone = factory.clone();
    assert_eq!(clone.as_raw(), factory.as_raw());
    assert_eq!(counts(&factory_stats), [2, 1, 0, 0, 1, 0]);

    let unknown = factory.cast::<IUnknown>().unwrap();
    assert_eq!(unknown.as_raw(), factory.as_raw());
    assert_eq!(counts(&factory_stats), [3, 2, 0, 1, 1, 0]);
    let queried = unknown.cast::<IClassFactory>().unwrap();
    assert_eq!(queried.as_raw(), factory.as_raw());
    assert_eq!(counts(&factory_stats), [4, 3, 0, 2, 1, 0]);

    assert_eq!(
        factory.cast::<IInspectable>().unwrap_err().code(),
        E_NOINTERFACE
    );
    assert_eq!(counts(&factory_stats), [4, 3, 0, 3, 1, 0]);
    drop(queried);
    drop(unknown);
    drop(clone);
    assert_eq!(counts(&factory_stats), [1, 3, 3, 3, 1, 0]);
    drop(factory);
    assert_eq!(counts(&factory_stats), [0, 3, 4, 3, 1, 1]);
    assert_eq!(counts(&instance_stats), [0; 6]);
}

#[test]
fn create_instance_transfers_one_reference() {
    let mut factory_stats = ComStats::default();
    let mut instance_stats = ComStats::default();
    let factory = unsafe {
        IClassFactory::from_raw(sys::ComFactory(
            &raw mut factory_stats,
            &raw mut instance_stats,
        ))
    };
    let instance: IUnknown = unsafe { factory.CreateInstance(None).unwrap() };
    assert_eq!(counts(&factory_stats), [1, 0, 0, 0, 1, 0]);
    assert_eq!(counts(&instance_stats), [1, 1, 1, 1, 1, 0]);
    let clone = instance.clone();
    assert_eq!(clone.as_raw(), instance.as_raw());
    assert_eq!(counts(&instance_stats), [2, 2, 1, 1, 1, 0]);
    assert_eq!(
        instance.cast::<IClassFactory>().unwrap_err().code(),
        E_NOINTERFACE
    );
    assert_eq!(counts(&instance_stats), [2, 2, 1, 2, 1, 0]);
    drop(factory);
    assert_eq!(counts(&factory_stats), [0, 0, 1, 0, 1, 1]);
    drop(instance);
    assert_eq!(counts(&instance_stats), [1, 2, 2, 2, 1, 0]);
    drop(clone);
    assert_eq!(counts(&instance_stats), [0, 2, 3, 2, 1, 1]);
}

#[test]
fn failed_outputs_do_not_create_owned_objects() {
    let mut factory_stats = ComStats::default();
    let mut instance_stats = ComStats::default();
    let factory = unsafe {
        IClassFactory::from_raw(sys::ComFactory(
            &raw mut factory_stats,
            &raw mut instance_stats,
        ))
    };
    let failure = unsafe { factory.CreateInstance::<_, IClassFactory>(None) };
    assert_eq!(failure.unwrap_err().code(), E_NOINTERFACE);
    assert_eq!(counts(&instance_stats), [0; 6]);

    let outer = factory.cast::<IUnknown>().unwrap();
    let failure = unsafe { factory.CreateInstance::<_, IUnknown>(&outer) };
    assert_eq!(failure.unwrap_err().code(), CLASS_E_NOAGGREGATION);
    assert_eq!(counts(&factory_stats), [2, 1, 0, 1, 1, 0]);
    assert_eq!(counts(&instance_stats), [0; 6]);
    drop(outer);
    drop(factory);
    assert_eq!(counts(&factory_stats), [0, 1, 2, 1, 1, 1]);
}
