use test_weak::Windows::{Foundation::Uri, Win32::Foundation::E_NOINTERFACE};
use windows::{factory, IActivationFactory, Interface, Result};

// The Uri class supports weak references, so it is used to test a successful downgrade and upgrade.
#[test]
fn test_success() -> Result<()> {
    let strong = Uri::CreateUri("http://kennykerr.ca")?;
    let weak = strong.downgrade()?;

    assert_eq!(weak.upgrade().unwrap(), strong);
    drop(strong);
    assert_eq!(weak.upgrade(), None);

    Ok(())
}

// The Uri class factory does not support weak references, so it is used to test an unsuccessful downgrade.
#[test]
fn test_failure() -> Result<()> {
    let strong = factory::<Uri, IActivationFactory>()?;
    let result = strong.downgrade();

    assert_eq!(result.is_err(), false);

    // E_NOINTERFACE is returned because IWeakReferenceSource is not implemented.
    assert_eq!(result.unwrap_err().code(), E_NOINTERFACE);

    Ok(())
}
