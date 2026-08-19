use super::*;
use crate::element::props::{PasswordBoxProps, TextBlockProps, TextBoxProps};

struct TextBlockChanges {
    text: bool,
    padding: bool,
}

struct TextBoxChanges {
    text: bool,
    header: bool,
    placeholder: bool,
    accepts_return: bool,
    chrome: bool,
}

struct PasswordBoxChanges {
    password: bool,
    header: bool,
    placeholder: bool,
    reveal_mode: bool,
}

pub(super) fn mount_text_block<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: TextBlockProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::TextBlock)?;
    engine.set_text(id, &props.text)?;
    if props.padding.is_some() {
        engine.set_padding(id, props.padding)?;
    }
    set_mounted(engine, id, key, MountedKind::TextBlock(props));
    Ok(id)
}

pub(super) fn mount_text_box<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: TextBoxProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::TextBox)?;
    engine.set_text(id, &props.text)?;
    if props.header.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::TextBox(Box::new(TextBoxUpdate::Header(props.header.clone()))),
        )?;
    }
    if props.placeholder.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::TextBox(Box::new(TextBoxUpdate::Placeholder(
                props.placeholder.clone(),
            ))),
        )?;
    }
    if props.accepts_return {
        engine.queue_control_update(
            id,
            ControlUpdate::TextBox(Box::new(TextBoxUpdate::AcceptsReturn(true))),
        )?;
    }
    if props.background.is_some()
        || props.border_brush.is_some()
        || props.border_thickness.is_some()
    {
        engine.queue_control_update(
            id,
            ControlUpdate::TextBox(Box::new(TextBoxUpdate::Chrome(Box::new(control_chrome(
                &props,
            ))))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::TextBox(Box::new(props)));
    Ok(id)
}

pub(super) fn mount_password_box<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: PasswordBoxProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::PasswordBox)?;
    engine.set_password(id, &props.password)?;
    if props.header.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::PasswordBox(Box::new(PasswordBoxUpdate::Header(props.header.clone()))),
        )?;
    }
    if props.placeholder.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::PasswordBox(Box::new(PasswordBoxUpdate::Placeholder(
                props.placeholder.clone(),
            ))),
        )?;
    }
    if props.reveal_mode != crate::element::PasswordRevealMode::default() {
        engine.queue_control_update(
            id,
            ControlUpdate::PasswordBox(Box::new(PasswordBoxUpdate::RevealMode(props.reveal_mode))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::PasswordBox(props));
    Ok(id)
}

pub(super) fn reconcile_text_block<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: TextBlockProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::TextBlock(old) => TextBlockChanges {
            text: old.text != props.text,
            padding: old.padding != props.padding,
        },
        _ => unreachable!(),
    };
    if changes.text {
        engine.set_text(id, &props.text)?;
    }
    if changes.padding {
        engine.set_padding(id, props.padding)?;
    }
    set_mounted(engine, id, key, MountedKind::TextBlock(props));
    Ok(())
}

pub(super) fn reconcile_text_box<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: TextBoxProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::TextBox(old) => TextBoxChanges {
            text: old.text != props.text,
            header: old.header != props.header,
            placeholder: old.placeholder != props.placeholder,
            accepts_return: old.accepts_return != props.accepts_return,
            chrome: old.background != props.background
                || old.border_brush != props.border_brush
                || old.border_thickness != props.border_thickness,
        },
        _ => unreachable!(),
    };
    if changes.text {
        engine.set_text(id, &props.text)?;
    }
    if changes.header {
        engine.queue_control_update(
            id,
            ControlUpdate::TextBox(Box::new(TextBoxUpdate::Header(props.header.clone()))),
        )?;
    }
    if changes.placeholder {
        engine.queue_control_update(
            id,
            ControlUpdate::TextBox(Box::new(TextBoxUpdate::Placeholder(
                props.placeholder.clone(),
            ))),
        )?;
    }
    if changes.accepts_return {
        engine.queue_control_update(
            id,
            ControlUpdate::TextBox(Box::new(TextBoxUpdate::AcceptsReturn(props.accepts_return))),
        )?;
    }
    if changes.chrome {
        engine.queue_control_update(
            id,
            ControlUpdate::TextBox(Box::new(TextBoxUpdate::Chrome(Box::new(control_chrome(
                &props,
            ))))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::TextBox(Box::new(props)));
    Ok(())
}

fn control_chrome(props: &TextBoxProps) -> ControlChromeUpdate {
    ControlChromeUpdate {
        background: props.background.clone(),
        border_brush: props.border_brush.clone(),
        border_thickness: props.border_thickness,
    }
}

pub(super) fn reconcile_password_box<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: PasswordBoxProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::PasswordBox(old) => PasswordBoxChanges {
            password: old.password != props.password,
            header: old.header != props.header,
            placeholder: old.placeholder != props.placeholder,
            reveal_mode: old.reveal_mode != props.reveal_mode,
        },
        _ => unreachable!(),
    };
    if changes.password {
        engine.set_password(id, &props.password)?;
    }
    if changes.header {
        engine.queue_control_update(
            id,
            ControlUpdate::PasswordBox(Box::new(PasswordBoxUpdate::Header(props.header.clone()))),
        )?;
    }
    if changes.placeholder {
        engine.queue_control_update(
            id,
            ControlUpdate::PasswordBox(Box::new(PasswordBoxUpdate::Placeholder(
                props.placeholder.clone(),
            ))),
        )?;
    }
    if changes.reveal_mode {
        engine.queue_control_update(
            id,
            ControlUpdate::PasswordBox(Box::new(PasswordBoxUpdate::RevealMode(props.reveal_mode))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::PasswordBox(props));
    Ok(())
}

fn set_mounted<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    kind: MountedKind,
) {
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted { key, kind });
}
