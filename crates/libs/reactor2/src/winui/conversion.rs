use super::*;

pub(super) fn native_stretch(value: Stretch) -> bindings::Stretch {
    match value {
        Stretch::None => bindings::Stretch::None,
        Stretch::Fill => bindings::Stretch::Fill,
        Stretch::Uniform => bindings::Stretch::Uniform,
        Stretch::UniformToFill => bindings::Stretch::UniformToFill,
    }
}

pub(super) fn native_orientation(value: Orientation) -> bindings::Orientation {
    match value {
        Orientation::Horizontal => bindings::Orientation::Horizontal,
        Orientation::Vertical => bindings::Orientation::Vertical,
    }
}

pub(super) fn native_password_reveal_mode(
    value: PasswordRevealMode,
) -> bindings::PasswordRevealMode {
    match value {
        PasswordRevealMode::Peek => bindings::PasswordRevealMode::Peek,
        PasswordRevealMode::Hidden => bindings::PasswordRevealMode::Hidden,
        PasswordRevealMode::Visible => bindings::PasswordRevealMode::Visible,
    }
}

pub(super) fn native_grid_length(value: GridLength) -> bindings::GridLength {
    match value {
        GridLength::Auto => bindings::GridLength {
            value: 0.0,
            grid_unit_type: bindings::GridUnitType::Auto,
        },
        GridLength::Pixel(value) => bindings::GridLength {
            value,
            grid_unit_type: bindings::GridUnitType::Pixel,
        },
        GridLength::Star(value) => bindings::GridLength {
            value,
            grid_unit_type: bindings::GridUnitType::Star,
        },
    }
}

pub(super) fn native_scroll_bar_visibility(
    value: ScrollBarVisibility,
) -> bindings::ScrollBarVisibility {
    match value {
        ScrollBarVisibility::Disabled => bindings::ScrollBarVisibility::Disabled,
        ScrollBarVisibility::Auto => bindings::ScrollBarVisibility::Auto,
        ScrollBarVisibility::Hidden => bindings::ScrollBarVisibility::Hidden,
        ScrollBarVisibility::Visible => bindings::ScrollBarVisibility::Visible,
    }
}

pub(super) fn native_scroll_view_bar_visibility(
    value: ScrollViewBarVisibility,
) -> bindings::ScrollingScrollBarVisibility {
    match value {
        ScrollViewBarVisibility::Auto => bindings::ScrollingScrollBarVisibility::Auto,
        ScrollViewBarVisibility::Visible => bindings::ScrollingScrollBarVisibility::Visible,
        ScrollViewBarVisibility::Hidden => bindings::ScrollingScrollBarVisibility::Hidden,
    }
}

pub(super) fn native_scroll_orientation(
    value: ScrollOrientation,
) -> bindings::ScrollingContentOrientation {
    match value {
        ScrollOrientation::Vertical => bindings::ScrollingContentOrientation::Vertical,
        ScrollOrientation::Horizontal => bindings::ScrollingContentOrientation::Horizontal,
        ScrollOrientation::None => bindings::ScrollingContentOrientation::None,
        ScrollOrientation::Both => bindings::ScrollingContentOrientation::Both,
    }
}

pub(super) fn native_split_view_display_mode(
    value: SplitViewDisplayMode,
) -> bindings::SplitViewDisplayMode {
    match value {
        SplitViewDisplayMode::Overlay => bindings::SplitViewDisplayMode::Overlay,
        SplitViewDisplayMode::Inline => bindings::SplitViewDisplayMode::Inline,
        SplitViewDisplayMode::CompactOverlay => bindings::SplitViewDisplayMode::CompactOverlay,
        SplitViewDisplayMode::CompactInline => bindings::SplitViewDisplayMode::CompactInline,
    }
}

pub(super) fn scroll_viewer_event(
    control: &bindings::ScrollViewer,
    activity: ScrollActivity,
) -> ScrollEvent {
    ScrollEvent {
        horizontal_offset: control.HorizontalOffset().unwrap(),
        vertical_offset: control.VerticalOffset().unwrap(),
        zoom_factor: control.ZoomFactor().unwrap(),
        activity,
    }
}

pub(super) fn scroll_view_event(control: &bindings::ScrollView) -> ScrollEvent {
    let activity = match control.State().unwrap() {
        bindings::ScrollingInteractionState::Idle => ScrollActivity::Idle,
        bindings::ScrollingInteractionState::Interaction => ScrollActivity::Interaction,
        bindings::ScrollingInteractionState::Inertia => ScrollActivity::Inertia,
        bindings::ScrollingInteractionState::Animation => ScrollActivity::Animation,
        _ => ScrollActivity::Unknown,
    };
    ScrollEvent {
        horizontal_offset: control.HorizontalOffset().unwrap(),
        vertical_offset: control.VerticalOffset().unwrap(),
        zoom_factor: control.ZoomFactor().unwrap(),
        activity,
    }
}

pub(super) fn application_resource_value(
    value: &ApplicationResource,
) -> WindowsResult<windows_core::IInspectable> {
    match value {
        ApplicationResource::String(value) => {
            windows_reference::IReference::from(value.as_str()).cast()
        }

        ApplicationResource::Number(value) => {
            Ok(windows_reference::IReference::from(*value).into())
        }
        ApplicationResource::Thickness(value) => {
            Ok(windows_reference::IReference::from(bindings::Thickness {
                left: value.left,
                top: value.top,
                right: value.right,
                bottom: value.bottom,
            })
            .into())
        }
        ApplicationResource::CornerRadius(value) => {
            Ok(windows_reference::IReference::from(bindings::CornerRadius {
                top_left: value.top_left,
                top_right: value.top_right,
                bottom_right: value.bottom_right,
                bottom_left: value.bottom_left,
            })
            .into())
        }
        ApplicationResource::SolidColorBrush(value) => {
            let brush = bindings::SolidColorBrush::new()?;
            brush.SetColor(bindings::Color {
                a: value.a,
                r: value.r,
                g: value.g,
                b: value.b,
            })?;
            brush.cast()
        }
    }
}

pub(super) fn native_brush(value: &Brush) -> WindowsResult<bindings::Brush> {
    match value {
        Brush::Solid(value) => {
            let brush = bindings::SolidColorBrush::new()?;
            brush.SetColor(bindings::Color {
                a: value.a,
                r: value.r,
                g: value.g,
                b: value.b,
            })?;
            brush.cast()
        }

        Brush::Theme(value) => {
            let resources = bindings::Application::Current()?.Resources()?;
            let map =
                resources.cast::<windows_collections::IMap<
                    windows_core::IInspectable,
                    windows_core::IInspectable,
                >>()?;
            let key = windows_reference::IReference::from(windows_core::HSTRING::from(
                value.resource_key(),
            ));
            map.Lookup(&key)?.cast()
        }
    }
}

pub(super) fn native_flyout_placement(value: FlyoutPlacement) -> bindings::FlyoutPlacementMode {
    match value {
        FlyoutPlacement::Top => bindings::FlyoutPlacementMode::Top,
        FlyoutPlacement::Bottom => bindings::FlyoutPlacementMode::Bottom,
        FlyoutPlacement::Left => bindings::FlyoutPlacementMode::Left,
        FlyoutPlacement::Right => bindings::FlyoutPlacementMode::Right,
        FlyoutPlacement::Full => bindings::FlyoutPlacementMode::Full,
        FlyoutPlacement::TopEdgeAlignedLeft => bindings::FlyoutPlacementMode::TopEdgeAlignedLeft,
        FlyoutPlacement::TopEdgeAlignedRight => bindings::FlyoutPlacementMode::TopEdgeAlignedRight,
        FlyoutPlacement::BottomEdgeAlignedLeft => {
            bindings::FlyoutPlacementMode::BottomEdgeAlignedLeft
        }

        FlyoutPlacement::BottomEdgeAlignedRight => {
            bindings::FlyoutPlacementMode::BottomEdgeAlignedRight
        }
        FlyoutPlacement::LeftEdgeAlignedTop => bindings::FlyoutPlacementMode::LeftEdgeAlignedTop,
        FlyoutPlacement::LeftEdgeAlignedBottom => {
            bindings::FlyoutPlacementMode::LeftEdgeAlignedBottom
        }
        FlyoutPlacement::RightEdgeAlignedTop => bindings::FlyoutPlacementMode::RightEdgeAlignedTop,
        FlyoutPlacement::RightEdgeAlignedBottom => {
            bindings::FlyoutPlacementMode::RightEdgeAlignedBottom
        }
        FlyoutPlacement::Auto => bindings::FlyoutPlacementMode::Auto,
    }
}

pub(super) fn native_tooltip_placement(value: TooltipPlacement) -> bindings::PlacementMode {
    match value {
        TooltipPlacement::Top => bindings::PlacementMode::Top,
        TooltipPlacement::Bottom => bindings::PlacementMode::Bottom,
        TooltipPlacement::Left => bindings::PlacementMode::Left,
        TooltipPlacement::Right => bindings::PlacementMode::Right,
        TooltipPlacement::Mouse => bindings::PlacementMode::Mouse,
    }
}

pub(super) fn dimension(value: Dimension, default: f64) -> f64 {
    match value {
        Dimension::Default => default,
        Dimension::Pixels(value) => value,
    }
}

pub(super) fn native_thickness(value: Thickness) -> bindings::Thickness {
    bindings::Thickness {
        left: value.left,
        top: value.top,
        right: value.right,
        bottom: value.bottom,
    }
}

pub(super) fn native_corner_radius(value: CornerRadius) -> bindings::CornerRadius {
    bindings::CornerRadius {
        top_left: value.top_left,
        top_right: value.top_right,
        bottom_right: value.bottom_right,
        bottom_left: value.bottom_left,
    }
}

pub(super) fn native_horizontal_alignment(
    value: HorizontalAlignment,
) -> bindings::HorizontalAlignment {
    match value {
        HorizontalAlignment::Left => bindings::HorizontalAlignment::Left,
        HorizontalAlignment::Center => bindings::HorizontalAlignment::Center,
        HorizontalAlignment::Right => bindings::HorizontalAlignment::Right,
        HorizontalAlignment::Stretch => bindings::HorizontalAlignment::Stretch,
    }
}

pub(super) fn native_vertical_alignment(value: VerticalAlignment) -> bindings::VerticalAlignment {
    match value {
        VerticalAlignment::Top => bindings::VerticalAlignment::Top,
        VerticalAlignment::Center => bindings::VerticalAlignment::Center,
        VerticalAlignment::Bottom => bindings::VerticalAlignment::Bottom,
        VerticalAlignment::Stretch => bindings::VerticalAlignment::Stretch,
    }
}

pub(super) fn native_visibility(value: Visibility) -> bindings::Visibility {
    match value {
        Visibility::Visible => bindings::Visibility::Visible,
        Visibility::Collapsed => bindings::Visibility::Collapsed,
    }
}

#[cfg(test)]
pub(super) use testing::*;

#[cfg(test)]
mod testing {
    use super::*;

    pub(in crate::winui) fn reactor_grid_length(value: bindings::GridLength) -> GridLength {
        match value.grid_unit_type {
            bindings::GridUnitType::Auto => GridLength::Auto,
            bindings::GridUnitType::Pixel => GridLength::Pixel(value.value),
            bindings::GridUnitType::Star => GridLength::Star(value.value),
            _ => panic!("Grid definition has an unknown unit type"),
        }
    }

    pub(in crate::winui) fn definition_grid_length<T: Interface>(
        definition: &T,
    ) -> WindowsResult<bindings::GridLength> {
        #[repr(C)]
        struct DefinitionVtable {
            base__: windows_core::IInspectable_Vtbl,
            get_value: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut bindings::GridLength,
            ) -> windows_core::HRESULT,
        }

        unsafe {
            let vtable = &*(Interface::vtable(definition) as *const _ as *const DefinitionVtable);
            let mut value = bindings::GridLength::default();
            (vtable.get_value)(Interface::as_raw(definition), &mut value).ok()?;
            Ok(value)
        }
    }

    pub(in crate::winui) fn public_visibility(value: bindings::Visibility) -> Visibility {
        if value == bindings::Visibility::Visible {
            Visibility::Visible
        } else if value == bindings::Visibility::Collapsed {
            Visibility::Collapsed
        } else {
            panic!("unknown Visibility value")
        }
    }

    pub(in crate::winui) fn public_horizontal_alignment(
        value: bindings::HorizontalAlignment,
    ) -> HorizontalAlignment {
        if value == bindings::HorizontalAlignment::Left {
            HorizontalAlignment::Left
        } else if value == bindings::HorizontalAlignment::Center {
            HorizontalAlignment::Center
        } else if value == bindings::HorizontalAlignment::Right {
            HorizontalAlignment::Right
        } else if value == bindings::HorizontalAlignment::Stretch {
            HorizontalAlignment::Stretch
        } else {
            panic!("unknown HorizontalAlignment value")
        }
    }

    pub(in crate::winui) fn public_vertical_alignment(
        value: bindings::VerticalAlignment,
    ) -> VerticalAlignment {
        if value == bindings::VerticalAlignment::Top {
            VerticalAlignment::Top
        } else if value == bindings::VerticalAlignment::Center {
            VerticalAlignment::Center
        } else if value == bindings::VerticalAlignment::Bottom {
            VerticalAlignment::Bottom
        } else if value == bindings::VerticalAlignment::Stretch {
            VerticalAlignment::Stretch
        } else {
            panic!("unknown VerticalAlignment value")
        }
    }
}
