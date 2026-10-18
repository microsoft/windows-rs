use super::*;

pub(super) struct KeyboardFixture {
    generation: u32,
    preview: u32,
    child: u32,
    characters: Vec<u16>,
    accelerators: Vec<u32>,
}

#[derive(Clone, Copy)]
pub(super) enum Message {
    Key(KeyEventInfo, bool),
    Character(CharacterEventInfo),
    Accelerator(u32),
    Replace,
}

impl Component for KeyboardFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            generation: 0,
            preview: 0,
            child: 0,
            characters: Vec::new(),
            accelerators: Vec::new(),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Key(info, child) => {
                assert_eq!(info.key, VirtualKey::SPACE);
                assert_eq!(info.original_key, VirtualKey::SPACE);
                assert_eq!(info.modifiers, InputModifiers::CONTROL);
                assert_eq!(info.status.repeat_count, 1);
                assert!(!info.status.was_down);
                assert!(!info.status.is_released);
                if child {
                    self.child += 1;
                } else {
                    self.preview += 1;
                }
            }
            Message::Character(info) => {
                assert_eq!(info.character, u16::from(b' '));
                assert_eq!(info.modifiers, InputModifiers::NONE);
                assert_eq!(info.status.repeat_count, 1);
                assert!(!info.status.is_released);
                self.characters.push(info.character);
            }
            Message::Accelerator(generation) => self.accelerators.push(generation),
            Message::Replace => self.generation += 1,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let generation = self.generation;
        let handled = generation == 0;
        let preview_sender = context.sender();
        let child_sender = context.sender();
        let character_sender = context.sender();
        let input = Border::new()
            .on_preview_key_down(RoutedCallback::new(move |info: KeyEventInfo| {
                if info.key == VirtualKey::SPACE && info.modifiers.contains(InputModifiers::CONTROL)
                {
                    assert!(preview_sender.send(Message::Key(info, false)));
                    handled
                } else {
                    false
                }
            }))
            .content(
                Border::new()
                    .automation_id("input")
                    .automation_name("Keyboard input")
                    .is_tab_stop(true)
                    .padding(12.0)
                    .on_preview_key_down(RoutedCallback::new(move |info: KeyEventInfo| {
                        if info.key == VirtualKey::SPACE
                            && info.modifiers.contains(InputModifiers::CONTROL)
                        {
                            assert!(child_sender.send(Message::Key(info, true)));
                        }
                        false
                    }))
                    .on_character_received(RoutedCallback::new(move |info: CharacterEventInfo| {
                        if info.modifiers == InputModifiers::NONE {
                            assert!(character_sender.send(Message::Character(info)));
                            true
                        } else {
                            false
                        }
                    }))
                    .content(TextBlock::new().text("Keyboard target")),
            );
        context.window_title("Reactor UI fixture: keyboard");
        context.window_visuals(WindowVisuals::new().client_size(500.0, 400.0));
        Grid::new()
            .key_accelerators(KeyAccelerators::new([KeyAccelerator::new(
                AcceleratorKey::Enter,
                AcceleratorModifiers::Control,
                context.message(Message::Accelerator(generation)),
            )]))
            .children([StackPanel::new()
                .margin(20.0)
                .spacing(12.0)
                .children((
                    input,
                    TextBlock::new()
                        .automation_id("keys")
                        .text(format!("Preview {}; child {}", self.preview, self.child)),
                    TextBlock::new()
                        .automation_id("characters")
                        .text(format!("Characters {:?}", self.characters)),
                    TextBlock::new()
                        .automation_id("accelerators")
                        .text(format!("Accelerators {:?}", self.accelerators)),
                    TextBlock::new()
                        .automation_id("state")
                        .text(format!("Handled {handled}; generation {generation}")),
                    Button::new()
                        .automation_id("replace")
                        .content("Replace callbacks")
                        .on_click(context.message(Message::Replace)),
                ))
                .into()])
            .into()
    }
}
