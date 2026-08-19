use super::*;

pub(in crate::winui) fn checked(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<bool> {
    let toggle = match &runtime.node(id)?.handle {
        Handle::CheckBox { value, .. } => value.cast::<bindings::IToggleButton>(),
        Handle::RadioButton { value, .. } => value.cast::<bindings::IToggleButton>(),
        Handle::ToggleButton { value, .. } => value.cast::<bindings::IToggleButton>(),
        _ => panic!("native node is not a toggle control"),
    }?;
    toggle.IsChecked()
}

pub(in crate::winui) fn on(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<bool> {
    let Handle::ToggleSwitch { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a ToggleSwitch");
    };
    value.cast::<bindings::IToggleSwitch>()?.IsOn()
}

pub(in crate::winui) fn text(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<String> {
    let Handle::TextBox { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a TextBox");
    };
    value.Text()
}

pub(in crate::winui) fn password(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<String> {
    let Handle::PasswordBox { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a PasswordBox");
    };
    value.Password()
}

pub(in crate::winui) fn slider(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(f64, f64, f64, Orientation)> {
    let Handle::Slider { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a Slider");
    };
    let range: bindings::IRangeBase = value.cast()?;
    let slider: bindings::ISlider = value.cast()?;
    let orientation = match slider.Orientation()? {
        bindings::Orientation::Horizontal => Orientation::Horizontal,
        bindings::Orientation::Vertical => Orientation::Vertical,
        _ => panic!("Slider returned an unknown orientation"),
    };
    Ok((
        range.Value()?,
        range.Minimum()?,
        range.Maximum()?,
        orientation,
    ))
}

pub(in crate::winui) fn set_slider(
    runtime: &WinUiRuntime,
    id: NodeId,
    value: f64,
) -> WindowsResult<()> {
    let Handle::Slider { value: slider, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a Slider");
    };
    slider.cast::<bindings::IRangeBase>()?.SetValue(value)
}

pub(in crate::winui) fn number_box(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(Option<f64>, f64, f64)> {
    let Handle::NumberBox { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a NumberBox");
    };
    let number: bindings::INumberBox = value.cast()?;
    Ok((
        optional_number(number.Value()?),
        number.Minimum()?,
        number.Maximum()?,
    ))
}

pub(in crate::winui) fn set_number_box(
    runtime: &WinUiRuntime,
    id: NodeId,
    value: Option<f64>,
) -> WindowsResult<()> {
    let Handle::NumberBox { value: number, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a NumberBox");
    };
    number
        .cast::<bindings::INumberBox>()?
        .SetValue(value.unwrap_or(f64::NAN))
}

pub(in crate::winui) fn rating_control(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(Option<f64>, i32, Option<f64>, String, bool)> {
    let Handle::RatingControl { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a RatingControl");
    };
    let rating: bindings::IRatingControl = value.cast()?;
    Ok((
        optional_rating(rating.Value()?),
        rating.MaxRating()?,
        optional_rating(rating.PlaceholderValue()?),
        rating.Caption()?,
        rating.IsReadOnly()?,
    ))
}

pub(in crate::winui) fn set_rating_control(
    runtime: &WinUiRuntime,
    id: NodeId,
    value: Option<f64>,
) -> WindowsResult<()> {
    let Handle::RatingControl { value: rating, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a RatingControl");
    };
    rating
        .cast::<bindings::IRatingControl>()?
        .SetValue(value.unwrap_or(-1.0))
}

pub(in crate::winui) fn set_rating_read_only(
    runtime: &WinUiRuntime,
    id: NodeId,
    value: bool,
) -> WindowsResult<()> {
    let Handle::RatingControl { value: rating, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a RatingControl");
    };
    rating
        .cast::<bindings::IRatingControl>()?
        .SetIsReadOnly(value)
}

pub(in crate::winui) fn color_picker(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(Color, bool, bool, bool, bool)> {
    let picker = runtime.color_picker(id)?;
    let color = picker.Color()?;
    Ok((
        Color {
            a: color.a,
            r: color.r,
            g: color.g,
            b: color.b,
        },
        picker.IsAlphaEnabled()?,
        picker.IsHexInputVisible()?,
        picker.IsColorSliderVisible()?,
        picker.IsColorChannelTextInputVisible()?,
    ))
}

pub(in crate::winui) fn set_color_picker(
    runtime: &WinUiRuntime,
    id: NodeId,
    value: Color,
) -> WindowsResult<()> {
    runtime.color_picker(id)?.SetColor(native_color(value))
}

pub(in crate::winui) fn date_picker(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(Option<DateTime>, bool, bool, bool)> {
    let picker = runtime.date_picker(id)?;
    Ok((
        selected_date(&picker)?,
        picker.DayVisible()?,
        picker.MonthVisible()?,
        picker.YearVisible()?,
    ))
}

pub(in crate::winui) fn set_date_picker(
    runtime: &WinUiRuntime,
    id: NodeId,
    value: Option<DateTime>,
) -> WindowsResult<()> {
    runtime.date_picker(id)?.SetSelectedDate(value)
}
