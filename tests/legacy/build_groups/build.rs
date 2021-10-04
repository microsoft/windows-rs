fn main() {
    windows::build! {
        Windows::Foundation::{
            Collections::{IMap, IVector},
            IStringable,
        },

        // Test for https://github.com/microsoft/windows-rs/issues/699
        Windows::Win32::System::Diagnostics::{
            Debug::GetLastError, ProcessSnapshotting::PssCaptureSnapshot,
        },
    };
}
