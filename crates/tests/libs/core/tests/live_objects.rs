// Runs only when built with `RUSTFLAGS=--cfg windows_cast_diagnostics`. This is the only test in
// the binary so that the process-wide counts are not shared with concurrent tests.
#![cfg(all(windows, windows_cast_diagnostics))]

use windows::Foundation::*;
use windows::core::*;

#[implement(IStringable)]
struct Test;

impl IStringable_Impl for Test_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("test".into())
    }
}

fn live() -> imp::LiveObjects {
    imp::live_objects()
}

#[test]
fn counts_live_objects() -> Result<()> {
    assert_eq!(live(), imp::LiveObjects::default());

    let stringable: IStringable = Test.into();
    let weak = stringable.downgrade()?;
    let handler = EventHandler::<IInspectable>::new(|_, _| Ok(()));
    let copy = handler.clone();

    assert_eq!(
        live(),
        imp::LiveObjects {
            objects: 1,
            delegates: 1,
            weak_references: 1,
        }
    );

    drop((handler, copy));
    assert_eq!(live().delegates, 0);

    // The tear-off outlives the object while a weak reference remains.
    drop(stringable);
    assert_eq!(live().objects, 0);
    assert_eq!(live().weak_references, 1);
    assert!(weak.upgrade().is_none());
    drop(weak);

    let object = ComObject::new(Test);
    assert_eq!(live().objects, 1);
    assert!(object.take().is_ok());

    assert_eq!(live(), imp::LiveObjects::default());
    Ok(())
}
