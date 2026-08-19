// Declaration order is the mount and update command order for independent scalar properties.
macro_rules! scalar_framework_properties {
    ($callback:ident) => {
        $callback! {
            (
                Visibility,
                set_visibility,
                Visibility,
                visual,
                visibility,
                all,
                "set visibility"
            ),
            (Opacity, set_opacity, f32, visual, opacity, all, "set opacity"),
            (
                FontSize,
                set_font_size,
                f32,
                text_style,
                font_size,
                text,
                "set font size"
            ),
            (
                CharacterSpacing,
                set_character_spacing,
                i32,
                text_style,
                character_spacing,
                text,
                "set character spacing"
            ),
            (
                FontWeight,
                set_font_weight,
                FontWeight,
                text_style,
                font_weight,
                text,
                "set font weight"
            ),
            (
                FontStyle,
                set_font_style,
                FontStyle,
                text_style,
                font_style,
                text,
                "set font style"
            ),
            (
                FontStretch,
                set_font_stretch,
                FontStretch,
                text_style,
                font_stretch,
                text,
                "set font stretch"
            ),
            (
                TextWrapping,
                set_text_wrapping,
                TextWrapping,
                text_block_style,
                text_wrapping,
                text_block,
                "set text wrapping"
            ),
            (
                TextTrimming,
                set_text_trimming,
                TextTrimming,
                text_block_style,
                text_trimming,
                text_block,
                "set text trimming"
            ),
            (
                TextSelectionEnabled,
                set_text_selection_enabled,
                bool,
                text_block_style,
                text_selection_enabled,
                text_block,
                "set text selection enabled"
            )
        }
    };
}

use crate::element::*;
use crate::framework_state::*;

#[derive(Default)]
pub(crate) struct FrameworkProps {
    pub data: Option<Box<FrameworkData>>,
}

#[derive(Default)]
pub struct FrameworkData {
    size: SizeProps,
    layout: LayoutProps,
    scalar: ScalarProps,
    heap: Option<Box<HeapProps>>,
}

#[derive(Clone, Copy)]
pub struct ScalarProps {
    opacity: f32,
    font_size: f32,
    character_spacing: i32,
    packed: u32,
}

#[derive(PartialEq)]
pub struct HeapProps {
    strings: Box<str>,
    lengths: [u32; 4],
    foreground: Option<Brush>,
    resources: Option<ElementResources>,
    transitions: ImplicitTransitions,
    scale: Option<f32>,
    input: Option<Box<InputProps>>,
}

#[derive(Default, PartialEq, Eq)]
struct InputProps {
    keyboard_accelerators: Option<KeyboardAcceleratorList>,
    pointer: Option<Box<PointerHandlers>>,
    drop: Option<Box<DropHandler>>,
}

#[derive(Default, PartialEq, Eq)]
pub(crate) struct PointerHandlers {
    pub pressed: Option<Callback<PointerEvent>>,
    pub moved: Option<Callback<PointerEvent>>,
    pub released: Option<Callback<PointerEvent>>,
    pub capture_lost: Option<Callback<PointerEvent>>,
    pub canceled: Option<Callback<PointerEvent>>,
    pub entered: Option<Callback<PointerEvent>>,
    pub exited: Option<Callback<PointerEvent>>,
    pub tapped: Option<Callback<()>>,
    pub right_tapped: Option<Callback<()>>,
    pub capture_on_press: bool,
}

impl PointerHandlers {
    pub fn is_empty(&self) -> bool {
        self.pressed.is_none()
            && self.moved.is_none()
            && self.released.is_none()
            && self.capture_lost.is_none()
            && self.canceled.is_none()
            && self.entered.is_none()
            && self.exited.is_none()
            && self.tapped.is_none()
            && self.right_tapped.is_none()
            && !self.capture_on_press
    }
}

pub(crate) struct DropHandler {
    pub target: DropTarget,
    pub callback: Callback<windows_core::Result<DropEvent>>,
}

impl DropHandler {
    pub fn target(&self) -> DropTarget {
        self.target
    }

    pub fn callback(&self) -> &Callback<windows_core::Result<DropEvent>> {
        &self.callback
    }
}

impl PartialEq for DropHandler {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target && self.callback == other.callback
    }
}

impl Eq for DropHandler {}

#[derive(PartialEq, Eq)]
pub enum KeyboardAcceleratorList {
    One(KeyboardAccelerator),
    Many(Box<[KeyboardAccelerator]>),
}

impl KeyboardAcceleratorList {
    pub fn from_vec(mut values: Vec<KeyboardAccelerator>) -> Option<Self> {
        match values.len() {
            0 => None,
            1 => Some(Self::One(values.pop().unwrap())),
            _ => Some(Self::Many(values.into_boxed_slice())),
        }
    }

    pub fn as_slice(&self) -> &[KeyboardAccelerator] {
        match self {
            Self::One(value) => std::slice::from_ref(value),
            Self::Many(values) => values,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(usize)]
enum HeapString {
    AutomationName,
    AutomationId,
    HelpText,
    FontFamily,
}

impl FrameworkProps {
    pub fn size(&self) -> SizeProps {
        self.data
            .as_deref()
            .map_or_else(SizeProps::default, |data| data.size)
    }

    pub fn layout(&self) -> LayoutProps {
        self.data
            .as_deref()
            .map_or_else(LayoutProps::default, |data| data.layout)
    }

    pub fn visual(&self) -> VisualProps {
        self.data
            .as_deref()
            .map_or_else(VisualProps::default, |data| data.scalar.visual())
    }

    pub fn control(&self) -> ControlProps {
        self.data
            .as_deref()
            .map_or_else(ControlProps::default, |data| data.scalar.control())
    }

    pub fn text_style(&self) -> TextStyleProps {
        self.data
            .as_deref()
            .map_or_else(TextStyleProps::default, |data| data.scalar.text_style())
    }

    pub fn text_block_style(&self) -> TextBlockStyleProps {
        self.data
            .as_deref()
            .map_or_else(TextBlockStyleProps::default, |data| {
                data.scalar.text_block_style()
            })
    }

    pub fn automation_name(&self) -> Option<&str> {
        self.data
            .as_deref()
            .and_then(|data| data.heap.as_deref())
            .and_then(|heap| heap.string(HeapString::AutomationName))
    }

    pub fn automation_id(&self) -> Option<&str> {
        self.data
            .as_deref()
            .and_then(|data| data.heap.as_deref())
            .and_then(|heap| heap.string(HeapString::AutomationId))
    }

    pub fn heading_level(&self) -> Option<AutomationHeadingLevel> {
        self.data
            .as_deref()
            .and_then(|data| data.scalar.heading_level())
    }

    pub fn help_text(&self) -> Option<&str> {
        self.data
            .as_deref()
            .and_then(|data| data.heap.as_deref())
            .and_then(|heap| heap.string(HeapString::HelpText))
    }

    pub fn font_family(&self) -> Option<&str> {
        self.data
            .as_deref()
            .and_then(|data| data.heap.as_deref())
            .and_then(|heap| heap.string(HeapString::FontFamily))
    }

    pub fn foreground(&self) -> Option<&Brush> {
        self.data
            .as_deref()
            .and_then(|data| data.heap.as_deref())
            .and_then(|heap| heap.foreground.as_ref())
    }

    pub fn resources(&self) -> Option<&ElementResources> {
        self.data
            .as_deref()
            .and_then(|data| data.heap.as_deref())
            .and_then(|heap| heap.resources.as_ref())
    }

    pub fn transitions(&self) -> ImplicitTransitions {
        self.data
            .as_deref()
            .and_then(|data| data.heap.as_deref())
            .map_or_else(ImplicitTransitions::default, |heap| heap.transitions)
    }

    pub fn scale(&self) -> Option<f32> {
        self.data
            .as_deref()
            .and_then(|data| data.heap.as_deref())
            .and_then(|heap| heap.scale)
    }

    pub fn keyboard_accelerators(&self) -> &[KeyboardAccelerator] {
        self.data
            .as_deref()
            .and_then(|data| data.heap.as_deref())
            .and_then(|heap| heap.input.as_deref())
            .and_then(|input| input.keyboard_accelerators.as_ref())
            .map_or(&[], KeyboardAcceleratorList::as_slice)
    }

    pub(crate) fn pointer_handlers(&self) -> Option<&PointerHandlers> {
        self.data
            .as_deref()
            .and_then(|data| data.heap.as_deref())
            .and_then(|heap| heap.input.as_deref())
            .and_then(|input| input.pointer.as_deref())
            .filter(|pointer| !pointer.is_empty())
    }

    pub(crate) fn drop_handler(&self) -> Option<&DropHandler> {
        self.data
            .as_deref()
            .and_then(|data| data.heap.as_deref())
            .and_then(|heap| heap.input.as_deref())
            .and_then(|input| input.drop.as_deref())
    }

    pub(crate) fn set_width(&mut self, value: Option<f64>) {
        self.set_size(value, |size, value| size.width = value);
    }

    pub(crate) fn set_height(&mut self, value: Option<f64>) {
        self.set_size(value, |size, value| size.height = value);
    }

    pub(crate) fn set_min_width(&mut self, value: Option<f64>) {
        self.set_size(value, |size, value| size.min_width = value);
    }

    pub(crate) fn set_max_width(&mut self, value: Option<f64>) {
        self.set_size(value, |size, value| size.max_width = value);
    }

    pub(crate) fn set_min_height(&mut self, value: Option<f64>) {
        self.set_size(value, |size, value| size.min_height = value);
    }

    pub(crate) fn set_max_height(&mut self, value: Option<f64>) {
        self.set_size(value, |size, value| size.max_height = value);
    }

    pub(crate) fn set_margin(&mut self, value: Option<Thickness>) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        self.data_mut().layout.margin = value;
        self.compact();
    }

    pub(crate) fn set_horizontal_alignment(&mut self, value: Option<HorizontalAlignment>) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        self.data_mut().layout.horizontal_alignment = value;
        self.compact();
    }

    pub(crate) fn set_vertical_alignment(&mut self, value: Option<VerticalAlignment>) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        self.data_mut().layout.vertical_alignment = value;
        self.compact();
    }

    pub(crate) fn set_visibility(&mut self, value: Option<Visibility>) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        self.data_mut().scalar.set_visibility(value);
        self.compact();
    }

    pub(crate) fn set_opacity(&mut self, value: Option<f32>) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        self.data_mut().scalar.opacity = value.unwrap_or(f32::NAN);
        self.compact();
    }

    pub(crate) fn set_font_size(&mut self, value: Option<f32>) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        self.data_mut().scalar.font_size = value.unwrap_or(f32::NAN);
        self.compact();
    }

    pub(crate) fn set_character_spacing(&mut self, value: Option<i32>) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        self.data_mut().scalar.set_character_spacing(value);
        self.compact();
    }

    pub(crate) fn set_font_weight(&mut self, value: Option<FontWeight>) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        self.data_mut().scalar.set_font_weight(value);
        self.compact();
    }

    pub(crate) fn set_font_style(&mut self, value: Option<FontStyle>) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        self.data_mut().scalar.set_font_style(value);
        self.compact();
    }

    pub(crate) fn set_font_stretch(&mut self, value: Option<FontStretch>) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        self.data_mut().scalar.set_font_stretch(value);
        self.compact();
    }

    pub(crate) fn set_text_wrapping(&mut self, value: Option<TextWrapping>) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        self.data_mut().scalar.set_text_wrapping(value);
        self.compact();
    }

    pub(crate) fn set_text_trimming(&mut self, value: Option<TextTrimming>) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        self.data_mut().scalar.set_text_trimming(value);
        self.compact();
    }

    pub(crate) fn set_text_selection_enabled(&mut self, value: Option<bool>) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        self.data_mut().scalar.set_text_selection_enabled(value);
        self.compact();
    }

    pub(crate) fn set_enabled(&mut self, value: Option<bool>) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        self.data_mut().scalar.set_enabled(value);
        self.compact();
    }

    pub(crate) fn set_automation_name(&mut self, value: Option<String>) {
        self.set_heap_value(value, |heap, value| {
            heap.set_string(HeapString::AutomationName, value);
        });
    }

    pub(crate) fn set_automation_id(&mut self, value: Option<String>) {
        self.set_heap_value(value, |heap, value| {
            heap.set_string(HeapString::AutomationId, value);
        });
    }

    pub(crate) fn set_heading_level(&mut self, value: Option<AutomationHeadingLevel>) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        self.data_mut().scalar.set_heading_level(value);
        self.compact();
    }

    pub(crate) fn set_help_text(&mut self, value: Option<String>) {
        self.set_heap_value(value, |heap, value| {
            heap.set_string(HeapString::HelpText, value);
        });
    }

    pub(crate) fn set_font_family(&mut self, value: Option<String>) {
        self.set_heap_value(value, |heap, value| {
            heap.set_string(HeapString::FontFamily, value);
        });
    }

    pub(crate) fn set_foreground(&mut self, value: Option<Brush>) {
        self.set_heap_value(value, |heap, value| {
            heap.foreground = value;
        });
    }

    pub(crate) fn set_resources(&mut self, value: Option<ElementResources>) {
        self.set_heap_value(value, |heap, value| {
            heap.resources = value;
        });
    }

    pub(crate) fn set_opacity_transition(&mut self, value: Option<std::time::Duration>) {
        self.set_heap_value(value, |heap, value| {
            heap.transitions.opacity = value;
        });
    }

    pub(crate) fn set_scale_transition(&mut self, value: Option<std::time::Duration>) {
        self.set_heap_value(value, |heap, value| {
            heap.transitions.scale = value;
        });
    }

    pub(crate) fn set_scale(&mut self, value: Option<f32>) {
        self.set_heap_value(value, |heap, value| {
            heap.scale = value;
        });
    }

    pub(crate) fn set_keyboard_accelerators(&mut self, value: Vec<KeyboardAccelerator>) {
        for (index, accelerator) in value.iter().enumerate() {
            assert!(
                !value[..index].iter().any(|existing| {
                    existing.key() == accelerator.key()
                        && existing.modifiers() == accelerator.modifiers()
                }),
                "duplicate keyboard accelerator"
            );
        }
        if value.is_empty() && self.data.is_none() {
            return;
        }
        self.input_mut().keyboard_accelerators = KeyboardAcceleratorList::from_vec(value);
        self.compact_input();
    }

    pub(crate) fn push_keyboard_accelerator(&mut self, value: KeyboardAccelerator) {
        if self.keyboard_accelerators().is_empty() {
            self.input_mut().keyboard_accelerators = Some(KeyboardAcceleratorList::One(value));
            return;
        }
        let mut values = self.keyboard_accelerators().to_vec();
        values.push(value);
        self.set_keyboard_accelerators(values);
    }

    pub(crate) fn set_pointer_pressed(&mut self, value: Callback<PointerEvent>) {
        self.pointer_mut().pressed = Some(value);
    }

    pub(crate) fn set_pointer_moved(&mut self, value: Callback<PointerEvent>) {
        self.pointer_mut().moved = Some(value);
    }

    pub(crate) fn set_pointer_released(&mut self, value: Callback<PointerEvent>) {
        self.pointer_mut().released = Some(value);
    }

    pub(crate) fn set_pointer_capture_lost(&mut self, value: Callback<PointerEvent>) {
        self.pointer_mut().capture_lost = Some(value);
    }

    pub(crate) fn set_pointer_canceled(&mut self, value: Callback<PointerEvent>) {
        self.pointer_mut().canceled = Some(value);
    }

    pub(crate) fn set_pointer_entered(&mut self, value: Callback<PointerEvent>) {
        self.pointer_mut().entered = Some(value);
    }

    pub(crate) fn set_pointer_exited(&mut self, value: Callback<PointerEvent>) {
        self.pointer_mut().exited = Some(value);
    }

    pub(crate) fn set_tapped(&mut self, value: Callback<()>) {
        self.pointer_mut().tapped = Some(value);
    }

    pub(crate) fn set_right_tapped(&mut self, value: Callback<()>) {
        self.pointer_mut().right_tapped = Some(value);
    }

    pub(crate) fn set_capture_pointer_on_press(&mut self) {
        self.pointer_mut().capture_on_press = true;
    }

    pub(crate) fn set_drop_handler(
        &mut self,
        target: DropTarget,
        handler: Callback<windows_core::Result<DropEvent>>,
    ) {
        self.input_mut().drop = Some(Box::new(DropHandler {
            target,
            callback: handler,
        }));
    }

    fn set_heap_value<T>(&mut self, value: Option<T>, set: impl FnOnce(&mut HeapProps, Option<T>)) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        let data = self.data_mut();
        if value.is_some() {
            set(
                data.heap
                    .get_or_insert_with(|| Box::new(HeapProps::default())),
                value,
            );
        } else if let Some(heap) = data.heap.as_deref_mut() {
            set(heap, None);
        }
        if data.heap.as_deref().is_some_and(HeapProps::is_default) {
            data.heap = None;
        }
        self.compact();
    }

    pub(crate) fn set_size(&mut self, value: Option<f64>, set: impl FnOnce(&mut SizeProps, f64)) {
        if value.is_none() && self.data.is_none() {
            return;
        }
        set(&mut self.data_mut().size, value.unwrap_or(f64::NAN));
        self.compact();
    }

    fn data_mut(&mut self) -> &mut FrameworkData {
        self.data
            .get_or_insert_with(|| Box::new(FrameworkData::default()))
    }

    fn input_mut(&mut self) -> &mut InputProps {
        self.data_mut()
            .heap
            .get_or_insert_with(|| Box::new(HeapProps::default()))
            .input
            .get_or_insert_with(|| Box::new(InputProps::default()))
    }

    pub fn pointer_mut(&mut self) -> &mut PointerHandlers {
        self.input_mut()
            .pointer
            .get_or_insert_with(|| Box::new(PointerHandlers::default()))
    }

    pub fn compact_input(&mut self) {
        if let Some(data) = self.data.as_deref_mut()
            && let Some(heap) = data.heap.as_deref_mut()
        {
            if let Some(input) = heap.input.as_deref_mut() {
                if input
                    .pointer
                    .as_deref()
                    .is_some_and(PointerHandlers::is_empty)
                {
                    input.pointer = None;
                }
                if input.is_default() {
                    heap.input = None;
                }
            }
            if heap.is_default() {
                data.heap = None;
            }
        }
        self.compact();
    }

    fn compact(&mut self) {
        if self.data.as_deref().is_some_and(FrameworkData::is_default) {
            self.data = None;
        }
    }
}

impl FrameworkData {
    fn is_default(&self) -> bool {
        self.size.is_default()
            && self.layout == LayoutProps::default()
            && self.scalar.is_default()
            && self.heap.is_none()
    }
}

impl InputProps {
    fn is_default(&self) -> bool {
        self.keyboard_accelerators.is_none() && self.pointer.is_none() && self.drop.is_none()
    }
}

impl ScalarProps {
    const VISIBILITY_SHIFT: u32 = 0;
    const ENABLED_SHIFT: u32 = 2;
    const FONT_STYLE_SHIFT: u32 = 4;
    const FONT_STRETCH_SHIFT: u32 = 6;
    const CHARACTER_SPACING_SHIFT: u32 = 10;
    const TEXT_WRAPPING_SHIFT: u32 = 11;
    const TEXT_TRIMMING_SHIFT: u32 = 13;
    const FONT_WEIGHT_SHIFT: u32 = 16;
    const TEXT_SELECTION_ENABLED_SHIFT: u32 = 26;
    const HEADING_LEVEL_SHIFT: u32 = 28;

    pub fn visual(&self) -> VisualProps {
        VisualProps {
            opacity: self.opacity,
            visibility: match self.get_bits(Self::VISIBILITY_SHIFT, 0b11) {
                0 => None,
                1 => Some(Visibility::Visible),
                2 => Some(Visibility::Collapsed),
                _ => unreachable!(),
            },
        }
    }

    pub fn text_style(&self) -> TextStyleProps {
        TextStyleProps {
            font_size: self.font_size,
            character_spacing: (self.get_bits(Self::CHARACTER_SPACING_SHIFT, 0b1) != 0)
                .then_some(self.character_spacing),
            font_weight: self.get_bits(Self::FONT_WEIGHT_SHIFT, 0b11_1111_1111) as u16,
            font_style: self.get_bits(Self::FONT_STYLE_SHIFT, 0b11) as u8,
            font_stretch: self.get_bits(Self::FONT_STRETCH_SHIFT, 0b1111) as u8,
        }
    }

    pub fn text_block_style(&self) -> TextBlockStyleProps {
        TextBlockStyleProps {
            text_wrapping: self.get_bits(Self::TEXT_WRAPPING_SHIFT, 0b11) as u8,
            text_trimming: self.get_bits(Self::TEXT_TRIMMING_SHIFT, 0b111) as u8,
            text_selection_enabled: self.get_bits(Self::TEXT_SELECTION_ENABLED_SHIFT, 0b11) as u8,
        }
    }

    pub fn control(&self) -> ControlProps {
        ControlProps {
            enabled: match self.get_bits(Self::ENABLED_SHIFT, 0b11) {
                0 => None,
                1 => Some(false),
                2 => Some(true),
                _ => unreachable!(),
            },
        }
    }

    pub(crate) fn set_visibility(&mut self, value: Option<Visibility>) {
        self.set_bits(
            Self::VISIBILITY_SHIFT,
            0b11,
            match value {
                None => 0,
                Some(Visibility::Visible) => 1,
                Some(Visibility::Collapsed) => 2,
            },
        );
    }

    pub(crate) fn set_enabled(&mut self, value: Option<bool>) {
        self.set_bits(
            Self::ENABLED_SHIFT,
            0b11,
            match value {
                None => 0,
                Some(false) => 1,
                Some(true) => 2,
            },
        );
    }

    pub(crate) fn set_font_style(&mut self, value: Option<FontStyle>) {
        self.set_bits(
            Self::FONT_STYLE_SHIFT,
            0b11,
            value.map_or(0, |value| value as u32 + 1),
        );
    }

    pub(crate) fn set_font_stretch(&mut self, value: Option<FontStretch>) {
        self.set_bits(
            Self::FONT_STRETCH_SHIFT,
            0b1111,
            value.map_or(0, |value| value as u32 + 1),
        );
    }

    pub(crate) fn set_character_spacing(&mut self, value: Option<i32>) {
        if let Some(value) = value {
            self.character_spacing = value;
        }
        self.set_bits(
            Self::CHARACTER_SPACING_SHIFT,
            0b1,
            u32::from(value.is_some()),
        );
    }

    pub(crate) fn set_font_weight(&mut self, value: Option<FontWeight>) {
        self.set_bits(
            Self::FONT_WEIGHT_SHIFT,
            0b11_1111_1111,
            value.map_or(0, FontWeight::weight) as u32,
        );
    }

    pub(crate) fn set_text_wrapping(&mut self, value: Option<TextWrapping>) {
        self.set_bits(
            Self::TEXT_WRAPPING_SHIFT,
            0b11,
            value.map_or(0, |value| value as u32),
        );
    }

    pub(crate) fn set_text_trimming(&mut self, value: Option<TextTrimming>) {
        self.set_bits(
            Self::TEXT_TRIMMING_SHIFT,
            0b111,
            value.map_or(0, |value| value as u32 + 1),
        );
    }

    pub(crate) fn set_text_selection_enabled(&mut self, value: Option<bool>) {
        self.set_bits(
            Self::TEXT_SELECTION_ENABLED_SHIFT,
            0b11,
            match value {
                None => 0,
                Some(false) => 1,
                Some(true) => 2,
            },
        );
    }

    pub fn get_bits(&self, shift: u32, mask: u32) -> u32 {
        (self.packed >> shift) & mask
    }

    pub(crate) fn set_bits(&mut self, shift: u32, mask: u32, value: u32) {
        self.packed = (self.packed & !(mask << shift)) | (value << shift);
    }

    pub fn heading_level(&self) -> Option<AutomationHeadingLevel> {
        match self.get_bits(Self::HEADING_LEVEL_SHIFT, 0b1111) {
            0 => None,
            1 => Some(AutomationHeadingLevel::Level1),
            2 => Some(AutomationHeadingLevel::Level2),
            3 => Some(AutomationHeadingLevel::Level3),
            4 => Some(AutomationHeadingLevel::Level4),
            5 => Some(AutomationHeadingLevel::Level5),
            6 => Some(AutomationHeadingLevel::Level6),
            7 => Some(AutomationHeadingLevel::Level7),
            8 => Some(AutomationHeadingLevel::Level8),
            9 => Some(AutomationHeadingLevel::Level9),
            _ => unreachable!(),
        }
    }

    pub(crate) fn set_heading_level(&mut self, value: Option<AutomationHeadingLevel>) {
        self.set_bits(
            Self::HEADING_LEVEL_SHIFT,
            0b1111,
            value.map_or(0, |value| value as u32),
        );
    }

    pub fn is_default(&self) -> bool {
        self.opacity.is_nan() && self.font_size.is_nan() && self.packed == 0
    }
}

impl Default for ScalarProps {
    fn default() -> Self {
        Self {
            opacity: f32::NAN,
            font_size: f32::NAN,
            character_spacing: 0,
            packed: 0,
        }
    }
}

impl HeapProps {
    const NONE: u32 = u32::MAX;

    fn string(&self, slot: HeapString) -> Option<&str> {
        let slot = slot as usize;
        let len = self.lengths[slot];
        if len == Self::NONE {
            return None;
        }
        let start = self.lengths[..slot]
            .iter()
            .filter(|len| **len != Self::NONE)
            .map(|len| *len as usize)
            .sum::<usize>();
        Some(&self.strings[start..start + len as usize])
    }

    fn set_string(&mut self, slot: HeapString, value: Option<String>) {
        let slot_index = slot as usize;
        if self
            .lengths
            .iter()
            .enumerate()
            .all(|(index, len)| index == slot_index || *len == Self::NONE)
        {
            if let Some(value) = value {
                self.lengths = [Self::NONE; 4];
                self.lengths[slot_index] = string_length(value.len());
                self.strings = value.into_boxed_str();
            } else {
                self.lengths = [Self::NONE; 4];
                self.strings = Box::default();
            }
            return;
        }

        let value = value.as_deref();
        let values = [
            if matches!(slot, HeapString::AutomationName) {
                value
            } else {
                self.string(HeapString::AutomationName)
            },
            if matches!(slot, HeapString::AutomationId) {
                value
            } else {
                self.string(HeapString::AutomationId)
            },
            if matches!(slot, HeapString::HelpText) {
                value
            } else {
                self.string(HeapString::HelpText)
            },
            if matches!(slot, HeapString::FontFamily) {
                value
            } else {
                self.string(HeapString::FontFamily)
            },
        ];
        let mut strings =
            String::with_capacity(values.iter().flatten().map(|value| value.len()).sum());
        let mut lengths = [Self::NONE; 4];
        for (index, value) in values.into_iter().enumerate() {
            if let Some(value) = value {
                lengths[index] = string_length(value.len());
                strings.push_str(value);
            }
        }
        self.strings = strings.into_boxed_str();
        self.lengths = lengths;
    }

    fn is_default(&self) -> bool {
        self.lengths == [Self::NONE; 4]
            && self.foreground.is_none()
            && self.resources.is_none()
            && self.transitions.is_empty()
            && self.scale.is_none()
            && self.input.is_none()
    }
}

impl Default for HeapProps {
    fn default() -> Self {
        Self {
            strings: Box::default(),
            lengths: [Self::NONE; 4],
            foreground: None,
            resources: None,
            transitions: ImplicitTransitions::default(),
            scale: None,
            input: None,
        }
    }
}

fn string_length(value: usize) -> u32 {
    u32::try_from(value).unwrap_or_else(|_| panic!("framework string exceeds 4 GiB"))
}
