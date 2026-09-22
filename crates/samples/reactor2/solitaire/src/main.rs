#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use windows_reactor2 as reactor2;
use windows_reactor2::{App, AppContext};

const PILES: usize = 7;
const FOUNDATIONS: usize = 4;
const DECK_SIZE: usize = 52;
const CARD_W: f64 = 56.0;
const CARD_H: f64 = 80.0;
const CARD_GAP_X: f64 = 14.0;
const FACE_UP_OFFSET: f64 = 22.0;
const FACE_DOWN_OFFSET: f64 = 8.0;
const TOP_ROW_Y: f64 = 20.0;
const TABLEAU_Y: f64 = 130.0;
const BOARD_LEFT: f64 = 20.0;
const BOARD_W: f64 = BOARD_LEFT + (CARD_W + CARD_GAP_X) * PILES as f64 + 20.0;
const BOARD_H: f64 = 620.0;

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
            1 => "A".into(),
            11 => "J".into(),
            12 => "Q".into(),
            13 => "K".into(),
            rank => rank.to_string(),
        };
        format!("{rank}{}", self.suit.symbol())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Click {
    Stock,
    Waste,
    Foundation(usize),
    Tableau(usize, usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Source {
    Waste,
    Foundation(usize),
    Tableau(usize, usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Destination {
    Tableau(usize),
    Foundation(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LastMove {
    ToFoundation(usize),
    ToTableau(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FailedMove {
    Tableau(usize, usize),
    Waste,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Game {
    tableau: [Vec<Card>; PILES],
    face_up: [usize; PILES],
    foundations: [Vec<Card>; FOUNDATIONS],
    stock: Vec<Card>,
    waste: Vec<Card>,
    last_move: Option<LastMove>,
    failed_move: Option<FailedMove>,
    moves: u32,
}

impl Game {
    fn new(seed: u64) -> Self {
        let mut deck = full_deck();
        shuffle(&mut deck, seed);
        let mut tableau: [Vec<Card>; PILES] = std::array::from_fn(|_| Vec::new());
        for pile in 0..PILES {
            for destination in tableau.iter_mut().skip(pile) {
                destination.push(deck.pop().unwrap());
            }
        }
        let face_up = std::array::from_fn(|pile| tableau[pile].len() - 1);
        for (pile, &face_up) in tableau.iter_mut().zip(&face_up) {
            for card in pile.iter_mut().take(face_up) {
                if card.rank == 1
                    && let Some(index) = deck.iter().position(|card| card.rank != 1)
                {
                    std::mem::swap(card, &mut deck[index]);
                }
            }
        }
        Self {
            tableau,
            face_up,
            foundations: std::array::from_fn(|_| Vec::new()),
            stock: deck,
            waste: Vec::new(),
            last_move: None,
            failed_move: None,
            moves: 0,
        }
    }

    fn is_won(&self) -> bool {
        self.foundations
            .iter()
            .all(|foundation| foundation.len() == 13)
    }
}

fn full_deck() -> Vec<Card> {
    Suit::all()
        .into_iter()
        .flat_map(|suit| (1..=13).map(move |rank| Card { rank, suit }))
        .collect()
}

fn shuffle(deck: &mut [Card], seed: u64) {
    let mut state = seed.max(1);
    for index in (1..deck.len()).rev() {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        deck.swap(index, state as usize % (index + 1));
    }
}

fn can_stack_on_tableau(moving: Card, top: Option<Card>) -> bool {
    top.map_or(moving.rank == 13, |top| {
        top.rank == moving.rank + 1 && top.suit.is_red() != moving.suit.is_red()
    })
}

fn can_place_on_foundation(moving: Card, foundation: &[Card], index: usize) -> bool {
    moving.suit as usize == index
        && foundation
            .last()
            .map_or(moving.rank == 1, |top| top.rank + 1 == moving.rank)
}

fn moving_cards(game: &Game, source: Source) -> Option<Vec<Card>> {
    match source {
        Source::Waste => game.waste.last().copied().map(|card| vec![card]),
        Source::Foundation(index) => game.foundations[index]
            .last()
            .copied()
            .map(|card| vec![card]),
        Source::Tableau(pile, index) => (index >= game.face_up[pile]
            && index < game.tableau[pile].len())
        .then(|| game.tableau[pile][index..].to_vec()),
    }
}

fn try_apply(game: &Game, source: Source, destination: Destination) -> Option<Game> {
    let cards = moving_cards(game, source)?;
    if matches!((source, destination), (Source::Tableau(a, _), Destination::Tableau(b)) if a == b)
        || matches!((source, destination), (Source::Foundation(a), Destination::Foundation(b)) if a == b)
    {
        return None;
    }
    let first = cards[0];
    let valid = match destination {
        Destination::Foundation(index) => {
            cards.len() == 1 && can_place_on_foundation(first, &game.foundations[index], index)
        }
        Destination::Tableau(pile) => {
            can_stack_on_tableau(first, game.tableau[pile].last().copied())
        }
    };
    if !valid {
        return None;
    }

    let mut next = game.clone();
    match source {
        Source::Waste => {
            next.waste.pop();
        }
        Source::Foundation(index) => {
            next.foundations[index].pop();
        }
        Source::Tableau(pile, index) => {
            next.tableau[pile].truncate(index);
            next.face_up[pile] = next.tableau[pile]
                .len()
                .checked_sub(1)
                .map_or(0, |last| next.face_up[pile].min(last));
        }
    }
    match destination {
        Destination::Tableau(pile) => {
            next.tableau[pile].extend(cards);
            next.last_move = Some(LastMove::ToTableau(pile));
        }
        Destination::Foundation(index) => {
            next.foundations[index].push(first);
            next.last_move = Some(LastMove::ToFoundation(index));
        }
    }
    next.moves = next.moves.saturating_add(1);
    Some(next)
}

fn draw_stock(game: &Game) -> Game {
    let mut next = game.clone();
    if let Some(card) = next.stock.pop() {
        next.waste.push(card);
        next.moves = next.moves.saturating_add(1);
    } else if !next.waste.is_empty() {
        next.stock.extend(next.waste.drain(..).rev());
        next.moves = next.moves.saturating_add(1);
    }
    next.last_move = None;
    next.failed_move = None;
    next
}

fn auto_move(game: &Game, source: Source) -> Option<Game> {
    let cards = moving_cards(game, source)?;
    let first = cards[0];
    if cards.len() == 1
        && let Some(next) = try_apply(game, source, Destination::Foundation(first.suit as usize))
    {
        return Some(next);
    }
    let source_pile = match source {
        Source::Tableau(pile, _) => Some(pile),
        _ => None,
    };
    let mut best = None;
    for (pile, cards) in game.tableau.iter().enumerate() {
        if Some(pile) == source_pile || cards.is_empty() {
            continue;
        }
        if let Some(next) = try_apply(game, source, Destination::Tableau(pile))
            && best
                .as_ref()
                .is_none_or(|(face_down, _)| *face_down < game.face_up[pile])
        {
            best = Some((game.face_up[pile], next));
        }
    }
    if let Some((_, next)) = best {
        return Some(next);
    }
    if first.rank == 13 && !matches!(source, Source::Tableau(pile, 0) if game.face_up[pile] == 0) {
        for (pile, cards) in game.tableau.iter().enumerate() {
            if Some(pile) != source_pile
                && cards.is_empty()
                && let Some(next) = try_apply(game, source, Destination::Tableau(pile))
            {
                return Some(next);
            }
        }
    }
    None
}

fn handle_click(game: &Game, click: Click) -> Game {
    let source = match click {
        Click::Stock => return draw_stock(game),
        Click::Waste if !game.waste.is_empty() => Source::Waste,
        Click::Foundation(index) if !game.foundations[index].is_empty() => {
            Source::Foundation(index)
        }
        Click::Tableau(pile, index)
            if index >= game.face_up[pile] && index < game.tableau[pile].len() =>
        {
            Source::Tableau(pile, index)
        }
        _ => return game.clone(),
    };
    if let Some(mut next) = auto_move(game, source) {
        next.failed_move = None;
        next
    } else {
        let mut next = game.clone();
        next.last_move = None;
        next.failed_move = match source {
            Source::Waste => Some(FailedMove::Waste),
            Source::Tableau(pile, index) => Some(FailedMove::Tableau(pile, index)),
            Source::Foundation(_) => None,
        };
        next
    }
}

fn current_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0xDEAD_BEEF, |duration| duration.as_nanos() as u64)
}

fn pile_x(pile: usize) -> f64 {
    BOARD_LEFT + pile as f64 * (CARD_W + CARD_GAP_X)
}

fn foundation_x(index: usize) -> f64 {
    pile_x(PILES - FOUNDATIONS + index)
}

#[derive(Clone, PartialEq)]
struct BoardCard {
    card: Card,
    x: f64,
    y: f64,
    face_up: bool,
    highlighted: bool,
    failed: bool,
    click: Click,
}

fn board_cards(game: &Game) -> Vec<BoardCard> {
    let mut cards = Vec::with_capacity(DECK_SIZE);
    cards.extend(game.stock.iter().copied().map(|card| BoardCard {
        card,
        x: pile_x(0),
        y: TOP_ROW_Y,
        face_up: false,
        highlighted: false,
        failed: false,
        click: Click::Stock,
    }));
    cards.extend(
        game.waste
            .iter()
            .copied()
            .enumerate()
            .map(|(index, card)| BoardCard {
                card,
                x: pile_x(1),
                y: TOP_ROW_Y,
                face_up: true,
                highlighted: false,
                failed: index + 1 == game.waste.len()
                    && matches!(game.failed_move, Some(FailedMove::Waste)),
                click: Click::Waste,
            }),
    );
    for (foundation, pile) in game.foundations.iter().enumerate() {
        cards.extend(
            pile.iter()
                .copied()
                .enumerate()
                .map(|(index, card)| BoardCard {
                    card,
                    x: foundation_x(foundation),
                    y: TOP_ROW_Y,
                    face_up: true,
                    highlighted: index + 1 == pile.len()
                        && matches!(
                            game.last_move,
                            Some(LastMove::ToFoundation(value)) if value == foundation
                        ),
                    failed: false,
                    click: Click::Foundation(foundation),
                }),
        );
    }
    for pile in 0..PILES {
        let mut y = TABLEAU_Y;
        for (index, card) in game.tableau[pile].iter().copied().enumerate() {
            let face_up = index >= game.face_up[pile];
            cards.push(BoardCard {
                card,
                x: pile_x(pile),
                y,
                face_up,
                highlighted: face_up
                    && index + 1 == game.tableau[pile].len()
                    && matches!(
                        game.last_move,
                        Some(LastMove::ToTableau(value)) if value == pile
                    ),
                failed: face_up
                    && matches!(
                        game.failed_move,
                        Some(FailedMove::Tableau(value, card)) if value == pile && card == index
                    ),
                click: Click::Tableau(pile, index),
            });
            y += if face_up {
                FACE_UP_OFFSET
            } else {
                FACE_DOWN_OFFSET
            };
        }
    }
    cards
}

fn card_key(card: Card) -> u64 {
    u64::from(card.suit as u8) * 13 + u64::from(card.rank)
}

struct CardStyle {
    background: reactor2::Color,
    border: reactor2::Color,
    border_thickness: f64,
    padding: reactor2::Thickness,
    opacity: f64,
}

fn positioned_border(
    content: impl Into<reactor2::Visual>,
    style: CardStyle,
    x: f64,
    y: f64,
    pointer_released: reactor2::Callback<reactor2::PointerEventInfo>,
) -> reactor2::Border {
    reactor2::Border::new()
        .width(CARD_W)
        .height(CARD_H)
        .margin(reactor2::Thickness::new(x, y, 0.0, 0.0))
        .horizontal_alignment(reactor2::HorizontalAlignment::Left)
        .vertical_alignment(reactor2::VerticalAlignment::Top)
        .background(style.background)
        .border_brush(style.border)
        .border_thickness(reactor2::Thickness::uniform(style.border_thickness))
        .corner_radius(reactor2::CornerRadius::uniform(4.0))
        .padding(style.padding)
        .opacity(style.opacity)
        .transitions([reactor2::ThemeTransition::Reposition])
        .content(content)
        .on_pointer_released_callback(pointer_released)
}

fn positioned_slot(
    label: impl Into<Rc<str>>,
    foreground: reactor2::Color,
    x: f64,
    y: f64,
    pointer_released: reactor2::Callback<reactor2::PointerEventInfo>,
) -> reactor2::Border {
    positioned_border(
        reactor2::TextBlock::new()
            .text(label)
            .font_size(18.0)
            .foreground(foreground)
            .horizontal_alignment(reactor2::HorizontalAlignment::Center)
            .vertical_alignment(reactor2::VerticalAlignment::Center),
        CardStyle {
            background: reactor2::Color::rgb(40, 90, 55),
            border: reactor2::Color::rgb(50, 110, 70),
            border_thickness: 1.5,
            padding: reactor2::Thickness::uniform(0.0),
            opacity: 0.7,
        },
        x,
        y,
        pointer_released,
    )
}

#[derive(Clone, PartialEq)]
struct SlotPlacement {
    label: Rc<str>,
    foreground: reactor2::Color,
    x: f64,
    y: f64,
    click: Click,
}

#[derive(Clone)]
struct SlotInput {
    on_click: reactor2::Callback<Click>,
    placement: SlotPlacement,
}

impl PartialEq for SlotInput {
    fn eq(&self, other: &Self) -> bool {
        self.placement == other.placement
    }
}

struct SlotView {
    input: SlotInput,
    pointer_released: reactor2::Callback<reactor2::PointerEventInfo>,
}

impl reactor2::Component for SlotView {
    type Input = SlotInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        let click = input.placement.click;
        let on_click = input.on_click.clone();
        Self {
            input: input.clone(),
            pointer_released: reactor2::Callback::new(move |_| on_click.call(click)),
        }
    }

    fn input_changed(
        &mut self,
        input: &Self::Input,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        let click = input.placement.click;
        let on_click = input.on_click.clone();
        self.input = input.clone();
        self.pointer_released = reactor2::Callback::new(move |_| on_click.call(click));
    }

    fn view(
        &self,
        _input: &Self::Input,
        _context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        let placement = &self.input.placement;
        positioned_slot(
            placement.label.clone(),
            placement.foreground,
            placement.x,
            placement.y,
            self.pointer_released.clone(),
        )
        .into()
    }
}

#[derive(Clone)]
struct CardInput {
    on_click: reactor2::Callback<Click>,
    placement: BoardCard,
}

impl PartialEq for CardInput {
    fn eq(&self, other: &Self) -> bool {
        self.placement == other.placement
    }
}

struct CardView {
    input: CardInput,
    pointer_released: reactor2::Callback<reactor2::PointerEventInfo>,
}

impl reactor2::Component for CardView {
    type Input = CardInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        let click = input.placement.click;
        let on_click = input.on_click.clone();
        Self {
            input: input.clone(),
            pointer_released: reactor2::Callback::new(move |_| on_click.call(click)),
        }
    }

    fn input_changed(
        &mut self,
        input: &Self::Input,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        let click = input.placement.click;
        let on_click = input.on_click.clone();
        self.input = input.clone();
        self.pointer_released = reactor2::Callback::new(move |_| on_click.call(click));
    }

    fn view(
        &self,
        _input: &Self::Input,
        _context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        let card = &self.input.placement;
        let (content, background, border, padding): (reactor2::Visual, _, _, _) = if card.face_up {
            let foreground = if card.card.suit.is_red() {
                reactor2::Color::rgb(192, 0, 32)
            } else {
                reactor2::Color::rgb(20, 20, 20)
            };
            (
                reactor2::StackPanel::new()
                    .children([
                        reactor2::TextBlock::new()
                            .text(card.card.label())
                            .font_size(13.0)
                            .foreground(foreground)
                            .horizontal_alignment(reactor2::HorizontalAlignment::Left)
                            .into(),
                        reactor2::TextBlock::new()
                            .text(card.card.suit.symbol())
                            .font_size(22.0)
                            .foreground(foreground)
                            .horizontal_alignment(reactor2::HorizontalAlignment::Center)
                            .vertical_alignment(reactor2::VerticalAlignment::Center)
                            .into(),
                    ])
                    .into(),
                if card.failed {
                    reactor2::Color::rgb(255, 220, 220)
                } else if card.highlighted {
                    reactor2::Color::rgb(220, 245, 220)
                } else {
                    reactor2::Color::rgb(255, 255, 255)
                },
                if card.failed {
                    reactor2::Color::rgb(220, 80, 80)
                } else {
                    reactor2::Color::rgb(180, 180, 180)
                },
                reactor2::Thickness::new(4.0, 3.0, 2.0, 2.0),
            )
        } else {
            (
                reactor2::TextBlock::new()
                    .text("🂠")
                    .font_size(18.0)
                    .foreground(reactor2::Color::rgb(240, 240, 255))
                    .horizontal_alignment(reactor2::HorizontalAlignment::Center)
                    .vertical_alignment(reactor2::VerticalAlignment::Center)
                    .into(),
                reactor2::Color::rgb(40, 80, 160),
                reactor2::Color::rgb(30, 60, 130),
                reactor2::Thickness::uniform(0.0),
            )
        };
        positioned_border(
            content,
            CardStyle {
                background,
                border,
                border_thickness: 1.0,
                padding,
                opacity: 1.0,
            },
            card.x,
            card.y,
            self.pointer_released.clone(),
        )
        .into()
    }
}

#[derive(Clone)]
struct BoardInput {
    game: Game,
    on_click: reactor2::Callback<Click>,
}

impl PartialEq for BoardInput {
    fn eq(&self, other: &Self) -> bool {
        self.game == other.game
    }
}

struct Board(BoardInput);

impl reactor2::Component for Board {
    type Input = BoardInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self(input.clone())
    }

    fn input_changed(
        &mut self,
        input: &Self::Input,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        self.0 = input.clone();
    }

    fn view(
        &self,
        _input: &Self::Input,
        _context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        build_board(&self.0.game, &self.0.on_click).into()
    }
}

fn build_board(game: &Game, on_click: &reactor2::Callback<Click>) -> reactor2::Grid {
    let mut children = Vec::new();
    if game.stock.is_empty() {
        children.push(
            reactor2::component::<SlotView>(
                "stock-slot",
                SlotInput {
                    on_click: on_click.clone(),
                    placement: SlotPlacement {
                        label: if game.waste.is_empty() { "·" } else { "↻" }.into(),
                        foreground: reactor2::Color::rgb(220, 230, 220),
                        x: pile_x(0),
                        y: TOP_ROW_Y,
                        click: Click::Stock,
                    },
                },
            )
            .keyed(),
        );
    }
    if game.waste.is_empty() {
        children.push(
            reactor2::component::<SlotView>(
                "waste-slot",
                SlotInput {
                    on_click: on_click.clone(),
                    placement: SlotPlacement {
                        label: "·".into(),
                        foreground: reactor2::Color::rgb(220, 230, 220),
                        x: pile_x(1),
                        y: TOP_ROW_Y,
                        click: Click::Waste,
                    },
                },
            )
            .keyed(),
        );
    }
    for (index, foundation) in game.foundations.iter().enumerate() {
        if foundation.is_empty() {
            let suit = Suit::all()[index];
            children.push(
                reactor2::component::<SlotView>(
                    format!("foundation-slot-{index}"),
                    SlotInput {
                        on_click: on_click.clone(),
                        placement: SlotPlacement {
                            label: suit.symbol().into(),
                            foreground: if suit.is_red() {
                                reactor2::Color::rgb(180, 120, 120)
                            } else {
                                reactor2::Color::rgb(180, 180, 180)
                            },
                            x: foundation_x(index),
                            y: TOP_ROW_Y,
                            click: Click::Foundation(index),
                        },
                    },
                )
                .keyed(),
            );
        }
    }
    for (pile, cards) in game.tableau.iter().enumerate() {
        if cards.is_empty() {
            children.push(
                reactor2::component::<SlotView>(
                    format!("tableau-slot-{pile}"),
                    SlotInput {
                        on_click: on_click.clone(),
                        placement: SlotPlacement {
                            label: "K".into(),
                            foreground: reactor2::Color::rgb(180, 200, 180),
                            x: pile_x(pile),
                            y: TABLEAU_Y,
                            click: Click::Tableau(pile, 0),
                        },
                    },
                )
                .keyed(),
            );
        }
    }
    for card in board_cards(game) {
        children.push(
            reactor2::component::<CardView>(
                card_key(card.card),
                CardInput {
                    on_click: on_click.clone(),
                    placement: card,
                },
            )
            .keyed(),
        );
    }
    reactor2::Grid::new()
        .width(BOARD_W)
        .height(BOARD_H)
        .horizontal_alignment(reactor2::HorizontalAlignment::Center)
        .children(children)
}

#[derive(Clone)]
enum Message {
    Click(Click),
    NewGame,
}

struct Solitaire {
    game: Game,
}

impl reactor2::Component for Solitaire {
    type Input = ();
    type Message = Message;

    fn create(_input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self {
            game: Game::new(current_seed()),
        }
    }

    fn update(
        &mut self,
        message: Self::Message,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        match message {
            Message::Click(click) => self.game = handle_click(&self.game, click),
            Message::NewGame => self.game = Game::new(current_seed()),
        }
    }

    fn view(
        &self,
        _input: &Self::Input,
        context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        let new_game = context.sender();
        let click = context.sender();
        let status = if self.game.is_won() {
            "You won! Press New Game to deal again.".into()
        } else {
            format!(
                "Moves: {}    Stock: {}    Waste: {}",
                self.game.moves,
                self.game.stock.len(),
                self.game.waste.len()
            )
        };
        reactor2::Border::new()
            .background(reactor2::Color::rgb(20, 100, 60))
            .content(
                reactor2::StackPanel::new().children([
                    reactor2::TitleBar::new()
                        .preferred_height(reactor2::WindowTitleBarHeight::Tall)
                        .title("Solitaire")
                        .subtitle(if self.game.is_won() { "You win!" } else { "" })
                        .into(),
                    reactor2::StackPanel::new()
                        .orientation(reactor2::Orientation::Horizontal)
                        .spacing(12.0)
                        .margin(reactor2::Thickness::new(12.0, 8.0, 12.0, 4.0))
                        .children([
                            reactor2::Button::new()
                                .content("New Game")
                                .on_click(move || {
                                    _ = new_game.send(Message::NewGame);
                                })
                                .into(),
                            reactor2::TextBlock::new()
                                .text(status)
                                .foreground(reactor2::Color::rgb(255, 255, 255))
                                .vertical_alignment(reactor2::VerticalAlignment::Center)
                                .into(),
                        ])
                        .into(),
                    reactor2::Viewbox::new()
                        .height(520.0)
                        .child(reactor2::component::<Board>(
                            "board",
                            BoardInput {
                                game: self.game.clone(),
                                on_click: reactor2::Callback::new(move |value| {
                                    _ = click.send(Message::Click(value));
                                }),
                            },
                        ))
                        .into(),
                ]),
            )
            .into()
    }
}

struct Host {
    host: reactor2::ComponentHost<reactor2::native::WinUiAdapter>,
    _window: reactor2::native::NativeWindow,
}

impl Host {
    fn new(context: &AppContext) -> windows_core::Result<Rc<RefCell<Option<Self>>>> {
        let state = Rc::new(RefCell::new(None::<Self>));
        let drain_state = Rc::clone(&state);
        let drain = context.callback(move || {
            let mut state = drain_state.borrow_mut();
            let host = &mut state.as_mut().unwrap().host;
            host.drain(usize::MAX)?;
            host.runtime()
                .adapter()
                .validate_graph(host.runtime().graph())
                .map_err(Into::into)
        });
        let mut host = reactor2::ComponentHost::mount_with_services(
            reactor2::native::WinUiAdapter::default(),
            context.component_services(),
            [reactor2::component::<Solitaire>("solitaire", ())],
        )?;
        let wake = drain.clone();
        host.set_waker(move || {
            _ = wake.invoke();
        });
        let wake = drain;
        host.set_native_event_waker(move || {
            _ = wake.invoke();
        });
        let root = host.runtime().graph().root().unwrap();
        let policy = reactor2::WindowPolicy::new()
            .title("Solitaire")
            .theme(reactor2::WindowTheme::Dark)
            .client_size(800.0, 600.0)
            .minimum_client_size(800.0, 600.0);
        let mut window = host
            .runtime()
            .adapter()
            .open_window_with_policy(root, &policy)?;
        let application = context.proxy();
        window.set_closed(move || application.exit())?;
        *state.borrow_mut() = Some(Self {
            host,
            _window: window,
        });
        Ok(state)
    }
}

fn main() -> windows_core::Result<()> {
    App::run_with(Host::new)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reactor2::{EventPayload, EventValue, Mutation, ObjectId, PropertyId, RelationId};

    fn card(rank: u8, suit: Suit) -> Card {
        Card { rank, suit }
    }

    fn empty_game() -> Game {
        Game {
            tableau: std::array::from_fn(|_| Vec::new()),
            face_up: [0; PILES],
            foundations: std::array::from_fn(|_| Vec::new()),
            stock: Vec::new(),
            waste: Vec::new(),
            last_move: None,
            failed_move: None,
            moves: 0,
        }
    }

    fn pointer_callback(
        host: &reactor2::ComponentHost<reactor2::RecordingAdapter>,
        object: ObjectId,
    ) -> reactor2::Callback<reactor2::PointerEventInfo> {
        host.runtime()
            .graph()
            .events(object)
            .unwrap()
            .iter()
            .find_map(|event| match (&event.id, &event.value) {
                (reactor2::EventId::PointerReleased, EventValue::PointerEventInfo(callback)) => {
                    Some(callback.clone())
                }
                _ => None,
            })
            .unwrap()
    }

    #[test]
    fn deal_uses_every_card_once_and_keeps_aces_face_up() {
        let game = Game::new(0xABCDEF);
        let mut seen = std::collections::HashSet::new();
        for pile in 0..PILES {
            assert_eq!(game.tableau[pile].len(), pile + 1);
            for (index, card) in game.tableau[pile].iter().enumerate() {
                assert!(seen.insert((card.rank, card.suit as u8)));
                if index < game.face_up[pile] {
                    assert_ne!(card.rank, 1);
                }
            }
        }
        for card in &game.stock {
            assert!(seen.insert((card.rank, card.suit as u8)));
        }
        assert_eq!(seen.len(), DECK_SIZE);
        assert_eq!(game.stock.len(), 24);
    }

    #[test]
    fn tableau_and_foundation_rules_match_the_original() {
        assert!(can_stack_on_tableau(card(13, Suit::Spades), None));
        assert!(can_stack_on_tableau(
            card(6, Suit::Hearts),
            Some(card(7, Suit::Clubs))
        ));
        assert!(!can_stack_on_tableau(
            card(6, Suit::Hearts),
            Some(card(7, Suit::Diamonds))
        ));
        assert!(can_place_on_foundation(
            card(1, Suit::Spades),
            &[],
            Suit::Spades as usize
        ));
        assert!(can_place_on_foundation(
            card(2, Suit::Spades),
            &[card(1, Suit::Spades)],
            Suit::Spades as usize
        ));
    }

    #[test]
    fn board_contains_all_cards_with_stable_keys() {
        let game = Game::new(42);
        let cards = board_cards(&game);
        assert_eq!(cards.len(), DECK_SIZE);
        let keys = cards
            .iter()
            .map(|placement| card_key(placement.card))
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(keys.len(), DECK_SIZE);
    }

    #[test]
    fn stock_click_changes_only_the_game_turn() {
        let game = Game::new(42);
        let next = handle_click(&game, Click::Stock);
        assert_eq!(next.stock.len(), game.stock.len() - 1);
        assert_eq!(next.waste.len(), 1);
        assert_eq!(next.moves, 1);
        assert_eq!(next.tableau, game.tableau);
    }

    #[test]
    fn drawing_preserves_the_moved_card_component_identity() {
        let game = Game::new(42);
        let moved = *game.stock.last().unwrap();
        let callback = reactor2::Callback::new(|_| {});
        let mut host = reactor2::ComponentHost::mount(
            reactor2::RecordingAdapter::default(),
            [reactor2::component::<Board>(
                "board",
                BoardInput {
                    game: game.clone(),
                    on_click: callback.clone(),
                },
            )],
        )
        .unwrap();
        let path = [
            reactor2::Key::from("board"),
            reactor2::Key::from(card_key(moved)),
        ];
        let object = host.reference_at(&path).unwrap().get();

        host.update_input::<Board>(
            &reactor2::Key::from("board"),
            BoardInput {
                game: draw_stock(&game),
                on_click: callback,
            },
        )
        .unwrap();

        assert_eq!(host.reference_at(&path).unwrap().get(), object);
    }

    #[test]
    fn overlapping_face_up_release_targets_the_top_card_message() {
        let mut game = empty_game();
        let lower = card(9, Suit::Clubs);
        let upper = card(8, Suit::Hearts);
        game.tableau[0] = vec![lower, upper];
        let clicked = Rc::new(RefCell::new(None));
        let clicked_for_callback = Rc::clone(&clicked);
        let mut host = reactor2::ComponentHost::mount(
            reactor2::RecordingAdapter::default(),
            [reactor2::component::<Board>(
                "board",
                BoardInput {
                    game,
                    on_click: reactor2::Callback::new(move |click| {
                        *clicked_for_callback.borrow_mut() = Some(click);
                    }),
                },
            )],
        )
        .unwrap();
        let board = host
            .reference(&reactor2::Key::from("board"))
            .unwrap()
            .get()
            .unwrap();
        let lower_object = host
            .reference_at(&[
                reactor2::Key::from("board"),
                reactor2::Key::from(card_key(lower)),
            ])
            .unwrap()
            .get()
            .unwrap();
        let upper_object = host
            .reference_at(&[
                reactor2::Key::from("board"),
                reactor2::Key::from(card_key(upper)),
            ])
            .unwrap()
            .get()
            .unwrap();
        let children = host
            .runtime()
            .graph()
            .children(board, RelationId::Children)
            .unwrap();
        assert!(
            children.iter().position(|object| *object == lower_object)
                < children.iter().position(|object| *object == upper_object)
        );

        let callback = pointer_callback(&host, upper_object);
        host.queue_event(reactor2::EventDispatch::new(
            upper_object,
            reactor2::EventId::PointerReleased,
            EventValue::PointerEventInfo(callback),
            EventPayload::PointerEventInfo(reactor2::PointerEventInfo {
                x: 10.0,
                y: 8.0,
                window_x: pile_x(0) + 10.0,
                window_y: TABLEAU_Y + FACE_UP_OFFSET + 8.0,
                pointer_id: 17,
                ..Default::default()
            }),
        ));
        host.drain(usize::MAX).unwrap();

        assert_eq!(*clicked.borrow(), Some(Click::Tableau(0, 1)));
    }

    #[test]
    fn tableau_move_preserves_card_scope_and_limits_recorded_mutations() {
        let mut game = empty_game();
        let destination = card(7, Suit::Clubs);
        let unrelated = card(13, Suit::Spades);
        let moved = card(6, Suit::Hearts);
        game.tableau[0].push(destination);
        game.tableau[1].push(unrelated);
        game.tableau[2].push(moved);
        let callback = reactor2::Callback::new(|_| {});
        let mut host = reactor2::ComponentHost::mount(
            reactor2::RecordingAdapter::default(),
            [reactor2::component::<Board>(
                "board",
                BoardInput {
                    game: game.clone(),
                    on_click: callback.clone(),
                },
            )],
        )
        .unwrap();
        let board = host
            .reference(&reactor2::Key::from("board"))
            .unwrap()
            .get()
            .unwrap();
        let moved_path = [
            reactor2::Key::from("board"),
            reactor2::Key::from(card_key(moved)),
        ];
        let moved_reference = host.reference_at(&moved_path).unwrap();
        let moved_object = moved_reference.get().unwrap();
        let moved_sender = host.sender_at::<CardView>(&moved_path).unwrap();
        let unrelated_object = host
            .reference_at(&[
                reactor2::Key::from("board"),
                reactor2::Key::from(card_key(unrelated)),
            ])
            .unwrap()
            .get()
            .unwrap();
        host.record_batches(true);

        let next = handle_click(&game, Click::Tableau(2, 0));
        let mutations = host
            .update_input::<Board>(
                &reactor2::Key::from("board"),
                BoardInput {
                    game: next,
                    on_click: callback,
                },
            )
            .unwrap();
        let slot_object = host
            .reference_at(&[
                reactor2::Key::from("board"),
                reactor2::Key::from("tableau-slot-2"),
            ])
            .unwrap()
            .get()
            .unwrap();
        let slot_content = host
            .runtime()
            .graph()
            .child(slot_object, RelationId::Content)
            .unwrap();

        assert_eq!(moved_reference.get(), Some(moved_object));
        assert_eq!(
            host.reference_at(&moved_path).unwrap().get(),
            Some(moved_object)
        );
        assert!(moved_sender.send(()));
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        let is_card_property_update = |set: &[reactor2::Property]| {
            set.len() == 2
                && set
                    .iter()
                    .any(|property| property.id == PropertyId::Background)
                && set.iter().any(|property| property.id == PropertyId::Margin)
        };
        assert!(!mutations.iter().any(|mutation| match mutation {
            Mutation::Create { object, .. }
            | Mutation::Replace { object, .. }
            | Mutation::SetProperties { object, .. }
            | Mutation::SetEvents { object, .. }
            | Mutation::Destroy { object } => *object == unrelated_object,
            Mutation::Attach { parent, child, .. }
            | Mutation::Detach { parent, child, .. }
            | Mutation::Insert { parent, child, .. }
            | Mutation::Remove { parent, child, .. } => {
                *parent == unrelated_object || *child == unrelated_object
            }
            Mutation::Reorder { parent, .. } => *parent == unrelated_object,
            _ => false,
        }));
        assert_eq!(
            mutations
                .iter()
                .filter(|mutation| matches!(
                    mutation,
                    Mutation::SetProperties { object, set, clear }
                        if *object == moved_object
                            && clear.is_empty()
                            && is_card_property_update(set)
                ))
                .count(),
            1
        );
        assert_eq!(
            mutations
                .iter()
                .filter(|mutation| matches!(
                    mutation,
                    Mutation::SetEvents { object, set, clear }
                        if *object == moved_object
                            && clear.is_empty()
                            && set.len() == 1
                            && set[0].id == reactor2::EventId::PointerReleased
                ))
                .count(),
            1
        );
        assert!(matches!(
                mutations.as_slice(),
                [
                    Mutation::Create {
                        object: created_slot,
                        kind: reactor2::ObjectType::Border,
                    },
                    Mutation::SetProperties {
                        object: slot_properties,
                        clear: slot_property_clear,
                        ..
                    },
                    Mutation::SetEvents {
                        object: slot_events,
                        set: slot_event_set,
                        clear: slot_event_clear,
                    },
                    Mutation::Create {
                        object: created_content,
                        kind: reactor2::ObjectType::TextBlock,
                    },
                    Mutation::SetProperties {
                        object: content_properties,
                        clear: content_property_clear,
                        ..
                    },
                    Mutation::Attach {
                        parent: attached_slot,
                        relation: RelationId::Content,
                        child: attached_content,
                    },
                    Mutation::Insert {
                        parent: inserted_parent,
                        relation: RelationId::Children,
                        child: inserted_slot,
                        index: 6,
                    },
                    Mutation::SetProperties {
                        object: updated_card,
                        set: updated_properties,
                        clear: updated_property_clear,
                    },
                    Mutation::SetEvents {
                        object: updated_event_card,
                        set: updated_events,
                        clear: updated_event_clear,
                    },
                    Mutation::Reorder {
                        parent: reordered_parent,
                        relation: RelationId::Children,
                        ..
                    },
                ] if *created_slot == slot_object
                    && *slot_properties == slot_object
                    && slot_property_clear.is_empty()
                    && *slot_events == slot_object
                    && slot_event_set.len() == 1
                    && slot_event_set[0].id == reactor2::EventId::PointerReleased
                    && slot_event_clear.is_empty()
                    && *created_content == slot_content
                    && *content_properties == slot_content
                    && content_property_clear.is_empty()
                    && *attached_slot == slot_object
                    && *attached_content == slot_content
                    && *inserted_parent == board
                    && *inserted_slot == slot_object
                    && *updated_card == moved_object
                    && is_card_property_update(updated_properties)
                    && updated_property_clear.is_empty()
                    && *updated_event_card == moved_object
                    && updated_events.len() == 1
                    && updated_events[0].id == reactor2::EventId::PointerReleased
                    && updated_event_clear.is_empty()
                    && *reordered_parent == board
        ));
        assert_eq!(
            mutations
                .iter()
                .filter(|mutation| matches!(
                    mutation,
                    Mutation::Reorder {
                        parent,
                        relation: RelationId::Children,
                        ..
                    } if *parent == board
                ))
                .count(),
            1
        );
        assert!(!mutations.iter().any(|mutation| matches!(
            mutation,
            Mutation::SetProperties { object, set, .. }
                if *object == moved_object
                    && set.iter().any(|property| property.id == PropertyId::Transitions)
        )));
    }
}
