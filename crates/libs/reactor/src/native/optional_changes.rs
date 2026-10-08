use super::*;
use bindings::{XamlChangeId, XamlOptionalChanges};

/// Process-wide optional WinUI changes, addressed by numeric IDs.
///
/// Runtime bootstrapping is automatic. Call `enable` and `disable` on the main thread before
/// WinUI starts; later mutations return an error. Queries work before and after startup.
pub struct OptionalChanges;

impl OptionalChanges {
    /// Enables a change, returning false if the runtime does not recognize the ID.
    pub fn enable(id: i32) -> windows_core::Result<bool> {
        bootstrap_runtime()?;
        XamlOptionalChanges::EnableChange(XamlChangeId(id))
    }

    /// Disables a change, returning false if the runtime cannot disable it.
    pub fn disable(id: i32) -> windows_core::Result<bool> {
        bootstrap_runtime()?;
        XamlOptionalChanges::DisableChange(XamlChangeId(id))
    }

    /// Returns whether a change is enabled. Unrecognized IDs return false.
    pub fn is_enabled(id: i32) -> windows_core::Result<bool> {
        bootstrap_runtime()?;
        XamlOptionalChanges::IsChangeEnabled(XamlChangeId(id))
    }
}
