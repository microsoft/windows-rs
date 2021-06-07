fn main() {
    windows::build! {
        // TODO: IKeyValuePair should not need to be needed here https://github.com/microsoft/windows-rs/issues/772
        Windows::Foundation::{
            Collections::{IKeyValuePair, IMap, IVector},
            IStringable,
        },

        // Test for https://github.com/microsoft/windows-rs/issues/699
        Windows::Win32::System::Diagnostics::{
            Debug::GetLastError, ProcessSnapshotting::PssCaptureSnapshot,
        },
    };
}
