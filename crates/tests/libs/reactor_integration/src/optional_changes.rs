use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;
use windows_core::{HRESULT, Result};
use windows_reactor::{App, OptionalChanges};

const ICON_NO_GRID_OPTIMIZATION: i32 = 61276805;
const ILLEGAL_STATE_CHANGE: HRESULT = HRESULT(0x8000000D_u32 as i32);

fn main() -> Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(30));
        eprintln!("Reactor optional changes fixture timed out");
        std::process::exit(1);
    });

    // Exercise each method as the first WinRT call in a fresh process.
    match std::env::args().nth(1).as_deref() {
        Some("enable") => assert!(OptionalChanges::enable(ICON_NO_GRID_OPTIMIZATION)?),
        Some("disable") => assert!(OptionalChanges::disable(ICON_NO_GRID_OPTIMIZATION)?),
        Some("query") => {
            OptionalChanges::is_enabled(ICON_NO_GRID_OPTIMIZATION)?;
        }
        _ => panic!("expected enable, disable, or query"),
    }

    for _ in 0..2 {
        assert!(OptionalChanges::enable(ICON_NO_GRID_OPTIMIZATION)?);
        assert!(OptionalChanges::is_enabled(ICON_NO_GRID_OPTIMIZATION)?);
        assert!(OptionalChanges::disable(ICON_NO_GRID_OPTIMIZATION)?);
        assert!(!OptionalChanges::is_enabled(ICON_NO_GRID_OPTIMIZATION)?);
    }

    for id in [i32::MIN, -1, i32::MAX] {
        assert!(!OptionalChanges::enable(id)?);
        assert!(!OptionalChanges::disable(id)?);
        assert!(!OptionalChanges::is_enabled(id)?);
    }
    assert!(OptionalChanges::enable(ICON_NO_GRID_OPTIMIZATION)?);

    let ran = Rc::new(Cell::new(false));
    let startup_ran = Rc::clone(&ran);
    App::run_with(move |app| {
        assert!(OptionalChanges::is_enabled(ICON_NO_GRID_OPTIMIZATION)?);
        for id in [ICON_NO_GRID_OPTIMIZATION, -1] {
            assert_eq!(
                OptionalChanges::enable(id).unwrap_err().code(),
                ILLEGAL_STATE_CHANGE
            );
            assert_eq!(
                OptionalChanges::disable(id).unwrap_err().code(),
                ILLEGAL_STATE_CHANGE
            );
        }
        assert!(!OptionalChanges::is_enabled(-1)?);
        assert!(
            std::thread::spawn(|| OptionalChanges::is_enabled(ICON_NO_GRID_OPTIMIZATION))
                .join()
                .unwrap()?
        );
        startup_ran.set(true);
        app.exit()
    })?;
    assert!(ran.get());
    assert!(OptionalChanges::is_enabled(ICON_NO_GRID_OPTIMIZATION)?);
    Ok(())
}
