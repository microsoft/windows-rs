use super::*;

pub(super) fn time_picker_properties(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(Option<TimeSpan>, i32)> {
    let Handle::TimePicker { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a TimePicker");
    };
    let picker: bindings::ITimePicker = value.cast()?;
    let time = match picker.SelectedTime() {
        Ok(value) => Some(value),
        Err(error) if error.code() == windows_core::HRESULT(0) => None,
        Err(error) => return Err(error),
    };
    Ok((time, picker.MinuteIncrement()?))
}

pub(super) fn set_time_picker_time(
    runtime: &WinUiRuntime,
    id: NodeId,
    value: Option<TimeSpan>,
) -> WindowsResult<()> {
    let Handle::TimePicker { value: picker, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a TimePicker");
    };
    picker
        .cast::<bindings::ITimePicker>()?
        .SetSelectedTime(value)
}
