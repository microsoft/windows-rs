/// Placement mode for a [`FlyoutDef`] relative to its target element.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum FlyoutPlacement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    Full,
    TopEdgeAlignedLeft,
    TopEdgeAlignedRight,
    BottomEdgeAlignedLeft,
    BottomEdgeAlignedRight,
    LeftEdgeAlignedTop,
    LeftEdgeAlignedBottom,
    RightEdgeAlignedTop,
    RightEdgeAlignedBottom,
    Auto,
}

/// Describes the content for a Flyout attached to a button.
/// Currently supports text content; element-tree content is planned.
#[derive(Clone, Debug, PartialEq)]
pub struct FlyoutDef {
    pub text: String,
    pub placement: FlyoutPlacement,
}
