#![windows_subsystem = "windows"]

use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use windows_reactor::{
    App, Component as AppComponent, ComponentContext as AppComponentContext, TextBlock, View,
    ViewContext,
};
use windows_reactor2 as reactor2;

const PILES: usize = 7;
const FOUNDATIONS: usize = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    fn all() -> [Self; 4] {
        [Self::Spades, Self::Hearts, Self::Diamonds, Self::Clubs]
    }

    fn is_red(self) -> bool {
        matches!(self, Self::Hearts | Self::Diamonds)
    }

    fn symbol(self) -> &'static str {
        match self {
            Self::Spades => "♠",
            Self::Hearts => "♥",
            Self::Diamonds => "♦",
            Self::Clubs => "♣",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Card {
    rank: u8,
    suit: Suit,
}

impl Card {
    fn label(self) -> String {
        let rank = match self.rank {
            1 => "A",
            11 => "J",
            12 => "Q",
            13 => "K",
            rank => return format!("{rank}{}", self.suit.symbol()),
        };
        format!("{rank}{}", self.suit.symbol())
    }
}

#[derive(Clone)]
struct Game {
    tableau: [Vec<Card>; PILES],
    face_up: [usize; PILES],
    foundations: [Vec<Card>; FOUNDATIONS],
    stock: Vec<Card>,
    waste: Vec<Card>,
    moves: u32,
}

impl Game {
    fn new(seed: u64) -> Self {
        let mut deck = Vec::with_capacity(52);
        for suit in Suit::all() {
            for rank in 1..=13 {
                deck.push(Card { rank, suit });
            }
        }
        shuffle(&mut deck, seed);

        let mut tableau: [Vec<Card>; PILES] = std::array::from_fn(|_| Vec::new());
        for pile in 0..PILES {
            for destination in tableau.iter_mut().skip(pile) {
                destination.push(deck.pop().unwrap());
            }
        }

        Self {
            face_up: std::array::from_fn(|pile| tableau[pile].len() - 1),
            foundations: std::array::from_fn(|_| Vec::new()),
            stock: deck,
            tableau,
            waste: Vec::new(),
            moves: 0,
        }
    }

    fn draw(&mut self) {
        if let Some(card) = self.stock.pop() {
            self.waste.push(card);
            self.moves += 1;
        } else if !self.waste.is_empty() {
            self.stock.extend(self.waste.drain(..).rev());
            self.moves += 1;
        }
    }

    fn move_waste(&mut self) {
        let Some(card) = self.waste.last().copied() else {
            return;
        };
        if self.move_to_foundation(card) || self.move_to_tableau(card, None) {
            self.waste.pop();
            self.moves += 1;
        }
    }

    fn move_tableau(&mut self, source: usize) {
        let Some(card) = self.tableau[source].last().copied() else {
            return;
        };
        if self.tableau[source].len() <= self.face_up[source] {
            return;
        }
        if self.move_to_foundation(card) || self.move_to_tableau(card, Some(source)) {
            self.tableau[source].pop();
            if !self.tableau[source].is_empty()
                && self.face_up[source] >= self.tableau[source].len()
            {
                self.face_up[source] = self.tableau[source].len() - 1;
            }
            self.moves += 1;
        }
    }

    fn move_to_foundation(&mut self, card: Card) -> bool {
        let foundation = &mut self.foundations[card.suit as usize];
        let valid = foundation
            .last()
            .is_none_or(|top| top.rank + 1 == card.rank)
            && foundation.last().is_some()
            || foundation.is_empty() && card.rank == 1;
        if valid {
            foundation.push(card);
        }
        valid
    }

    fn move_to_tableau(&mut self, card: Card, source: Option<usize>) -> bool {
        let destination = self.tableau.iter().enumerate().position(|(index, pile)| {
            Some(index) != source
                && pile.last().is_none_or(|top| {
                    top.rank == card.rank + 1 && top.suit.is_red() != card.suit.is_red()
                })
                && (!pile.is_empty() || card.rank == 13)
        });
        if let Some(destination) = destination {
            self.tableau[destination].push(card);
            true
        } else {
            false
        }
    }

    fn lines(&self) -> Vec<String> {
        let mut lines = Vec::with_capacity(11);
        lines.push(format!(
            "Moves: {}    Stock: {}    Waste: {}",
            self.moves,
            self.stock.len(),
            self.waste
                .last()
                .map_or_else(|| "-".into(), |card| card.label())
        ));
        lines.push(format!(
            "Foundations: {}",
            self.foundations
                .iter()
                .map(|foundation| foundation
                    .last()
                    .map_or_else(|| "-".into(), |card| card.label()))
                .collect::<Vec<_>>()
                .join("  ")
        ));
        lines.push(String::new());
        for (index, pile) in self.tableau.iter().enumerate() {
            let cards = pile
                .iter()
                .enumerate()
                .map(|(card_index, card)| {
                    if card_index < self.face_up[index] {
                        "[]".to_string()
                    } else {
                        card.label()
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            lines.push(format!("T{}: {cards}", index + 1));
        }
        lines
    }
}

fn shuffle(cards: &mut [Card], seed: u64) {
    let mut state = seed.max(1);
    for index in (1..cards.len()).rev() {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        cards.swap(index, state as usize % (index + 1));
    }
}

fn seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0xDEAD_BEEF, |duration| duration.as_nanos() as u64)
}

enum Message {
    Draw,
    Input(String),
    NewGame,
    Waste,
}

struct Solitaire {
    command: Rc<str>,
    game: Game,
}

impl reactor2::Component for Solitaire {
    type Input = ();
    type Message = Message;

    fn create(_input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self {
            command: Rc::from(""),
            game: Game::new(seed()),
        }
    }

    fn update(
        &mut self,
        message: Self::Message,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        let Message::Input(value) = message else {
            match message {
                Message::Draw => self.game.draw(),
                Message::NewGame => self.game = Game::new(seed()),
                Message::Waste => self.game.move_waste(),
                Message::Input(_) => unreachable!(),
            }
            return;
        };
        self.command = Rc::from(value.as_str());
        let command = value.trim().to_ascii_lowercase();
        let handled = match command.as_str() {
            "draw" => {
                self.game.draw();
                true
            }
            "waste" => {
                self.game.move_waste();
                true
            }
            "new" => {
                self.game = Game::new(seed());
                true
            }
            _ => command
                .strip_prefix('t')
                .and_then(|value| value.parse::<usize>().ok())
                .filter(|pile| (1..=PILES).contains(pile))
                .is_some_and(|pile| {
                    self.game.move_tableau(pile - 1);
                    true
                }),
        };
        if handled {
            self.command = Rc::from("");
        }
    }

    fn view(
        &self,
        _input: &Self::Input,
        context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        let sender = context.sender();
        let draw = context.sender();
        let waste = context.sender();
        let new_game = context.sender();
        let mut children = vec![
            reactor2::TextBlock::new("Reactor2 Solitaire").into(),
            reactor2::TextBlock::new("Commands: draw, waste, t1 ... t7, new").into(),
            reactor2::Button::new()
                .content(reactor2::TextBlock::new("Draw"))
                .on_click(move || {
                    _ = draw.send(Message::Draw);
                })
                .into(),
            reactor2::Button::new()
                .content(reactor2::TextBlock::new("Move waste"))
                .on_click(move || {
                    _ = waste.send(Message::Waste);
                })
                .into(),
            reactor2::Button::new()
                .content(reactor2::TextBlock::new("New game"))
                .on_click(move || {
                    _ = new_game.send(Message::NewGame);
                })
                .into(),
            reactor2::TextBox::new(Rc::clone(&self.command))
                .on_text_changed(move |value| {
                    _ = sender.send(Message::Input(value.to_string()));
                })
                .into(),
        ];
        children.extend(
            self.game
                .lines()
                .into_iter()
                .map(|line| reactor2::TextBlock::new(line).into()),
        );
        reactor2::StackPanel::new().children(children).into()
    }
}

struct Host {
    host: reactor2::ComponentHost<reactor2::native::WinUiAdapter>,
    _window: reactor2::native::NativeWindow,
}

impl AppComponent for Host {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, context: &AppComponentContext<Self>) -> Self {
        let mut host = reactor2::ComponentHost::mount(
            reactor2::native::WinUiAdapter::default(),
            [reactor2::component::<Solitaire>("solitaire", ())],
        )
        .unwrap();
        let wake = context.sender();
        host.runtime_mut().adapter_mut().set_event_waker(move || {
            _ = wake.send(());
        });
        let root = host.runtime().graph().root().unwrap();
        let window = host.runtime().adapter().open_window(root).unwrap();
        Self {
            host,
            _window: window,
        }
    }

    fn update(&mut self, (): (), _context: &AppComponentContext<Self>) {
        self.host.drain(usize::MAX).unwrap();
        self.host
            .runtime()
            .adapter()
            .validate_graph(self.host.runtime().graph())
            .unwrap();
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor2 bootstrap");
        TextBlock::new()
            .text("This window hosts the Reactor2 Solitaire window.")
            .into()
    }
}

fn main() -> windows_core::Result<()> {
    App::run_component::<Host>(())
}
