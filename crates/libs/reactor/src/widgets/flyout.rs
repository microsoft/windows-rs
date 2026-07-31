use super::*;

/// Text content for a flyout attached to a button.
#[derive(Clone, Debug, PartialEq)]
pub struct FlyoutDef {
    pub text: String,
    pub placement: FlyoutPlacementMode,
}
