#![allow(unused_qualifications, nonstandard_style, clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use windows_core::*;

fn main() -> Result<()> {
    init_mta()?;

    unsafe {
        let backup = CreateVssBackupComponentsInternal()?;
        backup.InitializeForBackup(&BSTR::new()).ok()?;
        backup.SetBackupState(true, true, VSS_BT_FULL, false).ok()?;

        let mut xml = BSTR::new();
        backup.SaveAsXML(&mut xml).ok()?;
        println!("{}", xml.display());
        Ok(())
    }
}
