use crate::controls::*;
use windows_reactor::*;

pub struct ListViewPage {
    selected_contact: i32,
    selected_playlist: i32,
}

#[derive(Clone)]
pub enum Message {
    SelectContact(i32),
    SelectPlaylist(i32),
}

const INBOX_ITEMS: [&str; 6] = [
    "Quarterly planning notes",
    "Design review follow-up",
    "Flight confirmation",
    "Weekly status summary",
    "Welcome to the team",
    "Shared gallery feedback",
];

const CONTACTS: [&str; 5] = ["Avery", "Diego", "Harper", "Mina", "Noah"];

const PLAYLISTS: [&str; 4] = ["Morning Focus", "Deep Work", "Evening Reset", "Weekend Mix"];

impl Component for ListViewPage {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            selected_contact: 1,
            selected_playlist: 0,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::SelectContact(index) => self.selected_contact = index,
            Message::SelectPlaylist(index) => self.selected_playlist = index,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let contact_label = CONTACTS
            .get(self.selected_contact.max(0) as usize)
            .copied()
            .unwrap_or("(none)");
        let playlist_label = PLAYLISTS
            .get(self.selected_playlist.max(0) as usize)
            .copied()
            .unwrap_or("(none)");

        page_content(
            "ListView",
            "Displays items in a vertical scrolling list.",
            [
                KeyedView::new(
                    "basic-list-view",
                    sample_card(
                        "Basic ListView",
                        ListView::new().height(220.0).collection_slot(
                            ListViewSlot::Items,
                            INBOX_ITEMS.into_iter().enumerate().map(|(index, subject)| {
                                KeyedView::new(
                                    subject,
                                    ListViewItem::new().tag(subject).content(
                                        StackPanel::new().spacing(2.0).children((
                                            TextBlock::new().text(subject).font_weight(700),
                                            TextBlock::new()
                                                .text(format!(
                                                    "Message #{} - Updated just now",
                                                    index + 1
                                                ))
                                                .opacity(0.6),
                                        )),
                                    ),
                                )
                            }),
                        ),
                        r#"ListView::new().collection_slot(ListViewSlot::Items, items)"#,
                    ),
                ),
                KeyedView::new(
                    "selection-display",
                    sample_card(
                        "Selection Display",
                        StackPanel::new().spacing(8.0).children((
                            ListView::new()
                                .height(180.0)
                                .selected_index(self.selected_contact)
                                .on_selection_changed(context.callback(Message::SelectContact))
                                .collection_slot(
                                    ListViewSlot::Items,
                                    CONTACTS.into_iter().map(|name| {
                                        KeyedView::new(
                                            name,
                                            ListViewItem::new().tag(name).content(name),
                                        )
                                    }),
                                ),
                            TextBlock::new()
                                .text(format!("Selected contact: {contact_label}"))
                                .opacity(0.6),
                        )),
                        r#"ListView::new().selected_index(selected_contact)
    .on_selection_changed(...)
    .collection_slot(ListViewSlot::Items, contacts)"#,
                    ),
                ),
                KeyedView::new(
                    "playlist-browser",
                    sample_card(
                        "Playlist Browser",
                        StackPanel::new().spacing(8.0).children((
                            ListView::new()
                                .height(200.0)
                                .selected_index(self.selected_playlist)
                                .on_selection_changed(context.callback(Message::SelectPlaylist))
                                .collection_slot(
                                    ListViewSlot::Items,
                                    PLAYLISTS.into_iter().enumerate().map(|(index, name)| {
                                        KeyedView::new(
                                            name,
                                            ListViewItem::new().tag(name).content(
                                                StackPanel::new().spacing(2.0).children((
                                                    TextBlock::new().text(name).font_weight(700),
                                                    TextBlock::new()
                                                        .text(format!(
                                                            "{} tracks ready to play",
                                                            12 + index * 5
                                                        ))
                                                        .opacity(0.6),
                                                )),
                                            ),
                                        )
                                    }),
                                ),
                            TextBlock::new()
                                .text(format!("Now browsing: {playlist_label}"))
                                .opacity(0.6),
                        )),
                        r#"ListView::new().selected_index(selected_playlist)
    .on_selection_changed(...)
    .collection_slot(ListViewSlot::Items, playlists)"#,
                    ),
                ),
            ],
        )
    }
}
