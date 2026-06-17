use super::*;

/// Describes the content for a Flyout attached to a button.
/// Currently supports text content; element-tree content is planned.
#[derive(Clone, Debug, PartialEq)]
pub struct FlyoutDef {
    pub text: String,
    pub placement: FlyoutPlacementMode,
}
