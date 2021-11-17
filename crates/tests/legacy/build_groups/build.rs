fn main() {
    windows::core::build_legacy! {
        Windows::Foundation::{
            Collections::{IMap, IVector},
            IStringable,
        },
        Windows::Win32::Foundation::GetLastError,
        Windows::Win32::System::Diagnostics::ProcessSnapshotting::PssCaptureSnapshot,
    };
}
