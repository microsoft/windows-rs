use super::*;

pub(super) fn calendar_view_properties(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(Vec<DateTime>, CalendarSelectionMode, bool, bool)> {
    let Handle::CalendarView { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a CalendarView");
    };
    let view: bindings::ICalendarView = value.cast()?;
    let dates = view.SelectedDates()?;
    let mut selected = Vec::with_capacity(dates.Size()? as usize);
    for index in 0..dates.Size()? {
        selected.push(dates.GetAt(index)?);
    }
    selected.sort_unstable();
    let mode = match view.SelectionMode()? {
        bindings::CalendarViewSelectionMode::None => CalendarSelectionMode::None,
        bindings::CalendarViewSelectionMode::Single => CalendarSelectionMode::Single,
        bindings::CalendarViewSelectionMode::Multiple => CalendarSelectionMode::Multiple,
        _ => unreachable!(),
    };
    Ok((
        selected,
        mode,
        view.IsTodayHighlighted()?,
        view.IsGroupLabelVisible()?,
    ))
}

pub(super) fn set_calendar_view_dates(
    runtime: &WinUiRuntime,
    id: NodeId,
    values: &[DateTime],
) -> WindowsResult<()> {
    let Handle::CalendarView { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a CalendarView");
    };
    let dates = value.cast::<bindings::ICalendarView>()?.SelectedDates()?;
    dates.Clear()?;
    for value in values {
        dates.Append(*value)?;
    }
    Ok(())
}
