// Generates the Volume Shadow Copy backup API from the in-house Win32 metadata
// with `windows-bindgen` and saves the Backup Components Document as XML.
// `IVssBackupComponents` is defined in the C++-only `vsbackup.h` header. Run elevated.

#![allow(unused_qualifications, nonstandard_style, clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use windows_core::*;

fn main() -> Result<()> {
    unsafe {
        CoIncrementMTAUsage()?;

        let backup = CreateVssBackupComponentsInternal()?;
        backup.InitializeForBackup(&BSTR::new()).ok()?;
        backup.SetBackupState(true, true, VSS_BT_FULL, false).ok()?;

        let mut xml = BSTR::new();
        backup.SaveAsXML(&mut xml).ok()?;
        println!("{}", xml.display());
        Ok(())
    }
}
