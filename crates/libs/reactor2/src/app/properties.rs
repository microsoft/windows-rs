use super::events::{accelerator_spec, pointer_subscription};
use crate::element::tree::ElementKind;
use crate::element::{
    Brush, DropTarget, ElementResources, ImplicitTransitions, KeyboardAccelerator,
};
use crate::engine::{Engine, EngineError};
use crate::framework_properties::*;
use crate::framework_state::*;
use crate::id::NodeId;
use crate::mounted::MountedKind;
use crate::runtime::*;

#[derive(Clone, Copy, Default, PartialEq)]
pub(crate) struct FrameworkValues {
    size: SizeProps,
    layout: LayoutProps,
    visual: VisualProps,
    text_style: TextStyleProps,
    text_block_style: TextBlockStyleProps,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct ControlValues {
    pub(crate) props: ControlProps,
}

#[derive(Default, PartialEq)]
pub(crate) struct HeapChanges {
    automation_name: OptionalChange<String>,
    automation_id: OptionalChange<String>,
    heading_level: OptionalChange<crate::element::AutomationHeadingLevel>,
    help_text: OptionalChange<String>,
    font_family: OptionalChange<String>,
    foreground: OptionalChange<Brush>,
    resources: ValueChange<ElementResources>,
    transitions: ValueChange<ImplicitTransitions>,
    scale: OptionalChange<f32>,
    keyboard_accelerators: ValueChange<Vec<KeyboardAcceleratorSpec>>,
    pointer: ValueChange<PointerSubscription>,
    drop: ValueChange<Option<DropTarget>>,
}

#[derive(Default, PartialEq)]
enum OptionalChange<T> {
    #[default]
    Unchanged,
    Set(T),
    Clear,
}

#[derive(Default, PartialEq)]
enum ValueChange<T> {
    #[default]
    Unchanged,
    Set(T),
}

#[derive(Clone, Copy, Default)]
pub(crate) struct HeapValues<'a> {
    automation_name: Option<&'a str>,
    automation_id: Option<&'a str>,
    heading_level: Option<crate::element::AutomationHeadingLevel>,
    help_text: Option<&'a str>,
    font_family: Option<&'a str>,
    foreground: Option<&'a Brush>,
    resources: Option<&'a ElementResources>,
    transitions: ImplicitTransitions,
    scale: Option<f32>,
    keyboard_accelerators: &'a [KeyboardAccelerator],
    pointer: Option<&'a PointerHandlers>,
    drop: Option<&'a DropHandler>,
}

pub(crate) fn element_framework(kind: &ElementKind) -> FrameworkValues {
    kind.framework_props()
        .map_or_else(FrameworkValues::default, framework_values)
}

pub(crate) fn mounted_framework(kind: &MountedKind) -> FrameworkValues {
    kind.framework_props()
        .map_or_else(FrameworkValues::default, framework_values)
}

fn framework_values(props: &FrameworkProps) -> FrameworkValues {
    FrameworkValues {
        size: props.size(),
        layout: props.layout(),
        visual: props.visual(),
        text_style: props.text_style(),
        text_block_style: props.text_block_style(),
    }
}

macro_rules! define_scalar_framework_reconciliation {
    ($(($variant:ident, $setter:ident, $ty:ty, $group:ident, $getter:ident, $capability:ident, $name:literal)),* $(,)?) => {
        #[inline(always)]
        fn apply_scalar_framework_props<R: NativeRuntime>(
            engine: &mut Engine<R>,
            target: NodeId,
            props: &FrameworkValues,
        ) -> Result<(), EngineError> {
            $(
                if let Some(value) = props.$group.$getter() {
                    engine.queue_framework_update(
                        target,
                        FrameworkUpdate::$variant(Some(value)),
                    )?;
                }
            )*
            Ok(())
        }

        #[inline(always)]
        fn update_scalar_framework_props<R: NativeRuntime>(
            engine: &mut Engine<R>,
            target: NodeId,
            old: FrameworkValues,
            new: FrameworkValues,
        ) -> Result<(), EngineError> {
            $(
                if old.$group.$getter() != new.$group.$getter() {
                    engine.queue_framework_update(
                        target,
                        FrameworkUpdate::$variant(new.$group.$getter()),
                    )?;
                }
            )*
            Ok(())
        }
    };
}

scalar_framework_properties!(define_scalar_framework_reconciliation);

pub(crate) fn apply_framework_props<R: NativeRuntime>(
    engine: &mut Engine<R>,
    target: NodeId,
    props: &FrameworkValues,
) -> Result<(), EngineError> {
    let size = props.size;
    if let Some(width) = size.width() {
        engine.set_width(target, Some(width))?;
    }
    if let Some(height) = size.height() {
        engine.set_height(target, Some(height))?;
    }
    if let Some(min_width) = size.min_width() {
        engine.set_min_width(target, Some(min_width))?;
    }
    if let Some(max_width) = size.max_width() {
        engine.set_max_width(target, Some(max_width))?;
    }
    if let Some(min_height) = size.min_height() {
        engine.set_min_height(target, Some(min_height))?;
    }
    if let Some(max_height) = size.max_height() {
        engine.set_max_height(target, Some(max_height))?;
    }
    let layout = props.layout;
    if let Some(margin) = layout.margin() {
        engine.queue_framework_update(target, FrameworkUpdate::Margin(Some(margin)))?;
    }
    if let Some(alignment) = layout.horizontal_alignment() {
        engine.queue_framework_update(
            target,
            FrameworkUpdate::HorizontalAlignment(Some(alignment)),
        )?;
    }
    if let Some(alignment) = layout.vertical_alignment() {
        engine
            .queue_framework_update(target, FrameworkUpdate::VerticalAlignment(Some(alignment)))?;
    }
    apply_scalar_framework_props(engine, target, props)
}

pub(crate) fn update_framework_props<R: NativeRuntime>(
    engine: &mut Engine<R>,
    target: NodeId,
    old: FrameworkValues,
    new: FrameworkValues,
) -> Result<(), EngineError> {
    if old.size.width() != new.size.width() {
        engine.set_width(target, new.size.width())?;
    }
    if old.size.height() != new.size.height() {
        engine.set_height(target, new.size.height())?;
    }
    if old.size.min_width() != new.size.min_width() {
        engine.set_min_width(target, new.size.min_width())?;
    }
    if old.size.max_width() != new.size.max_width() {
        engine.set_max_width(target, new.size.max_width())?;
    }
    if old.size.min_height() != new.size.min_height() {
        engine.set_min_height(target, new.size.min_height())?;
    }
    if old.size.max_height() != new.size.max_height() {
        engine.set_max_height(target, new.size.max_height())?;
    }
    if old.layout.margin() != new.layout.margin() {
        engine.queue_framework_update(target, FrameworkUpdate::Margin(new.layout.margin()))?;
    }
    if old.layout.horizontal_alignment() != new.layout.horizontal_alignment() {
        engine.queue_framework_update(
            target,
            FrameworkUpdate::HorizontalAlignment(new.layout.horizontal_alignment()),
        )?;
    }
    if old.layout.vertical_alignment() != new.layout.vertical_alignment() {
        engine.queue_framework_update(
            target,
            FrameworkUpdate::VerticalAlignment(new.layout.vertical_alignment()),
        )?;
    }
    update_scalar_framework_props(engine, target, old, new)
}

pub(crate) fn element_control(kind: &ElementKind) -> ControlValues {
    ControlValues {
        props: kind
            .framework_props()
            .map_or_else(ControlProps::default, FrameworkProps::control),
    }
}

pub(crate) fn mounted_control(kind: &MountedKind) -> ControlValues {
    ControlValues {
        props: kind
            .framework_props()
            .map_or_else(ControlProps::default, FrameworkProps::control),
    }
}

pub(crate) fn update_control_props<R: NativeRuntime>(
    engine: &mut Engine<R>,
    target: NodeId,
    old: ControlValues,
    new: ControlValues,
) -> Result<(), EngineError> {
    if old.props.enabled() != new.props.enabled() {
        engine.queue_framework_update(
            target,
            FrameworkUpdate::Enabled(new.props.enabled().unwrap_or(true)),
        )?;
    }
    Ok(())
}

fn framework_heap(props: &FrameworkProps) -> HeapValues<'_> {
    HeapValues {
        automation_name: props.automation_name(),
        automation_id: props.automation_id(),
        heading_level: props.heading_level(),
        help_text: props.help_text(),
        font_family: props.font_family(),
        foreground: props.foreground(),
        resources: props.resources(),
        transitions: props.transitions(),
        scale: props.scale(),
        keyboard_accelerators: props.keyboard_accelerators(),
        pointer: props.pointer_handlers(),
        drop: props.drop_handler(),
    }
}

pub(crate) fn element_heap(kind: &ElementKind) -> HeapValues<'_> {
    kind.framework_props()
        .map_or_else(HeapValues::default, framework_heap)
}

pub(crate) fn mounted_heap(kind: &MountedKind) -> HeapValues<'_> {
    kind.framework_props()
        .map_or_else(HeapValues::default, framework_heap)
}

pub(crate) fn diff_heap(old: HeapValues<'_>, new: HeapValues<'_>) -> HeapChanges {
    HeapChanges {
        automation_name: optional_string_change(old.automation_name, new.automation_name),
        automation_id: optional_string_change(old.automation_id, new.automation_id),
        heading_level: optional_change(old.heading_level, new.heading_level),
        help_text: optional_string_change(old.help_text, new.help_text),
        font_family: optional_string_change(old.font_family, new.font_family),
        foreground: optional_change(old.foreground.cloned(), new.foreground.cloned()),
        resources: if old.resources == new.resources {
            ValueChange::Unchanged
        } else {
            ValueChange::Set(new.resources.cloned().unwrap_or_default())
        },
        transitions: value_change(old.transitions, new.transitions),
        scale: optional_change(old.scale, new.scale),
        keyboard_accelerators: if same_accelerators(
            old.keyboard_accelerators,
            new.keyboard_accelerators,
        ) {
            ValueChange::Unchanged
        } else {
            ValueChange::Set(
                new.keyboard_accelerators
                    .iter()
                    .map(accelerator_spec)
                    .collect(),
            )
        },
        pointer: value_change(
            pointer_subscription(old.pointer),
            pointer_subscription(new.pointer),
        ),
        drop: value_change(
            old.drop.map(DropHandler::target),
            new.drop.map(DropHandler::target),
        ),
    }
}

impl HeapChanges {
    pub(crate) fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

pub(crate) fn apply_heap_changes<R: NativeRuntime>(
    engine: &mut Engine<R>,
    target: NodeId,
    changes: HeapChanges,
) -> Result<(), EngineError> {
    match changes.automation_name {
        OptionalChange::Unchanged => {}
        OptionalChange::Set(value) => engine.set_automation_name(target, Some(value))?,
        OptionalChange::Clear => engine.set_automation_name(target, None)?,
    }
    match changes.automation_id {
        OptionalChange::Unchanged => {}
        OptionalChange::Set(value) => engine.set_automation_id(target, Some(value))?,
        OptionalChange::Clear => engine.set_automation_id(target, None)?,
    }
    match changes.heading_level {
        OptionalChange::Unchanged => {}
        OptionalChange::Set(value) => engine
            .queue_accessibility_update(target, AccessibilityUpdate::HeadingLevel(Some(value)))?,
        OptionalChange::Clear => {
            engine.queue_accessibility_update(target, AccessibilityUpdate::HeadingLevel(None))?;
        }
    }
    match changes.help_text {
        OptionalChange::Unchanged => {}
        OptionalChange::Set(value) => engine.set_help_text(target, Some(value))?,
        OptionalChange::Clear => engine.set_help_text(target, None)?,
    }
    match changes.font_family {
        OptionalChange::Unchanged => {}
        OptionalChange::Set(value) => {
            engine.queue_text_style_update(target, TextStyleUpdate::FontFamily(Some(value)))?;
        }
        OptionalChange::Clear => {
            engine.queue_text_style_update(target, TextStyleUpdate::FontFamily(None))?;
        }
    }
    match changes.foreground {
        OptionalChange::Unchanged => {}
        OptionalChange::Set(value) => {
            engine.queue_text_style_update(target, TextStyleUpdate::Foreground(Some(value)))?;
        }
        OptionalChange::Clear => {
            engine.queue_text_style_update(target, TextStyleUpdate::Foreground(None))?;
        }
    }
    if let ValueChange::Set(value) = changes.resources {
        engine.queue_resources_update(target, value)?;
    }
    if let ValueChange::Set(value) = changes.transitions {
        engine.queue_visual_update(target, VisualUpdate::ImplicitTransitions(value))?;
    }
    match changes.scale {
        OptionalChange::Unchanged => {}
        OptionalChange::Set(value) => {
            engine.queue_visual_update(target, VisualUpdate::Scale(Some(value)))?;
        }
        OptionalChange::Clear => engine.queue_visual_update(target, VisualUpdate::Scale(None))?,
    }
    if let ValueChange::Set(value) = changes.keyboard_accelerators {
        engine.queue_input_update(target, InputUpdate::KeyboardAccelerators(value))?;
    }
    if let ValueChange::Set(value) = changes.pointer {
        engine.queue_input_update(target, InputUpdate::Pointer(value))?;
    }
    if let ValueChange::Set(value) = changes.drop {
        engine.queue_input_update(target, InputUpdate::Drop(value))?;
    }
    Ok(())
}

fn optional_string_change(old: Option<&str>, new: Option<&str>) -> OptionalChange<String> {
    if old == new {
        OptionalChange::Unchanged
    } else if let Some(value) = new {
        OptionalChange::Set(value.to_owned())
    } else {
        OptionalChange::Clear
    }
}

fn optional_change<T: PartialEq>(old: Option<T>, new: Option<T>) -> OptionalChange<T> {
    if old == new {
        OptionalChange::Unchanged
    } else if let Some(value) = new {
        OptionalChange::Set(value)
    } else {
        OptionalChange::Clear
    }
}

fn value_change<T: PartialEq>(old: T, new: T) -> ValueChange<T> {
    if old == new {
        ValueChange::Unchanged
    } else {
        ValueChange::Set(new)
    }
}

fn same_accelerators(old: &[KeyboardAccelerator], new: &[KeyboardAccelerator]) -> bool {
    old.len() == new.len()
        && old
            .iter()
            .zip(new)
            .all(|(old, new)| accelerator_spec(old) == accelerator_spec(new))
}
