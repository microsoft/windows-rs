use windows_reactor::*;

const PILES: usize = 7;
const FOUNDATIONS: usize = 4;
const DECK_SIZE: usize = 52;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Suit {
    Spades = 0,
    Hearts = 1,
    Diamonds = 2,
    Clubs = 3,
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Card {
    rank: u8,
    suit: Suit,
}

impl Card {
    fn is_red(self) -> bool {
        self.suit.is_red()
    }

    fn rank_label(self) -> &'static str {
        match self.rank {
            1 => "A",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "10",
            11 => "J",
            12 => "Q",
            13 => "K",
            _ => "?",
        }
    }

    fn label(self) -> String {
        format!("{}{}", self.rank_label(), self.suit.symbol())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Click {
    Stock,
    Waste,
    Foundation(usize),
    Tableau(usize, usize),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Source {
    Waste,
    Foundation(usize),
    Tableau(usize, usize),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dest {
    Tableau(usize),
    Foundation(usize),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum LastMove {
    ToFoundation(usize),
    ToTableau(usize),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FailedMove {
    Tableau(usize, usize),
    Waste,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Game {
    tableau: Vec<Vec<Card>>,
    face_up: Vec<usize>,
    foundations: Vec<Vec<Card>>,
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

        let mut tableau: Vec<Vec<Card>> = (0..PILES).map(|_| Vec::new()).collect();
        for p in 0..PILES {
            for pile in tableau.iter_mut().skip(p) {
                pile.push(deck.pop().expect("52-card deck"));
            }
        }
        let face_up: Vec<usize> = tableau.iter().map(|p| p.len() - 1).collect();

        #[allow(clippy::needless_range_loop)]
        for p in 0..PILES {
            for i in 0..face_up[p] {
                if tableau[p][i].rank == 1
                    && let Some(swap_idx) = deck.iter().position(|c| c.rank != 1)
                {
                    std::mem::swap(&mut tableau[p][i], &mut deck[swap_idx]);
                }
            }
        }

        Self {
            tableau,
            face_up,
            foundations: (0..FOUNDATIONS).map(|_| Vec::new()).collect(),
            stock: deck,
            waste: Vec::new(),
            last_move: None,
            failed_move: None,
            moves: 0,
        }
    }

    fn is_won(&self) -> bool {
        self.foundations.iter().all(|f| f.len() == 13)
    }
}

fn full_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(DECK_SIZE);
    for suit in Suit::all() {
        for rank in 1..=13 {
            deck.push(Card { rank, suit });
        }
    }
    deck
}

fn shuffle(deck: &mut [Card], seed: u64) {
    let mut rng = Lcg::new(seed);
    for i in (1..deck.len()).rev() {
        let j = (rng.next() % (i as u64 + 1)) as usize;
        deck.swap(i, j);
    }
}

fn can_stack_on_tableau(moving: Card, top: Option<Card>) -> bool {
    match top {
        None => moving.rank == 13,
        Some(t) => t.rank == moving.rank + 1 && t.is_red() != moving.is_red(),
    }
}

fn can_place_on_foundation(moving: Card, foundation: &[Card], f_idx: usize) -> bool {
    if moving.suit as usize != f_idx {
        return false;
    }
    match foundation.last() {
        None => moving.rank == 1,
        Some(t) => t.rank + 1 == moving.rank,
    }
}

fn moving_cards(game: &Game, src: Source) -> Option<Vec<Card>> {
    match src {
        Source::Waste => game.waste.last().map(|c| vec![*c]),
        Source::Foundation(f) => game
            .foundations
            .get(f)
            .and_then(|v| v.last().map(|c| vec![*c])),
        Source::Tableau(p, i) => {
            let pile = game.tableau.get(p)?;
            if i >= pile.len() || i < game.face_up[p] {
                return None;
            }
            Some(pile[i..].to_vec())
        }
    }
}

fn try_apply(game: &Game, src: Source, dst: Dest) -> Option<Game> {
    let cards = moving_cards(game, src)?;
    if cards.is_empty() {
        return None;
    }
    match (src, dst) {
        (Source::Tableau(p1, _), Dest::Tableau(p2)) if p1 == p2 => return None,
        (Source::Foundation(f1), Dest::Foundation(f2)) if f1 == f2 => return None,
        _ => {}
    }
    let first = cards[0];
    let valid = match dst {
        Dest::Foundation(f) => {
            cards.len() == 1 && can_place_on_foundation(first, &game.foundations[f], f)
        }
        Dest::Tableau(p) => can_stack_on_tableau(first, game.tableau[p].last().copied()),
    };
    if !valid {
        return None;
    }

    let mut next = game.clone();
    match src {
        Source::Waste => {
            next.waste.pop();
        }
        Source::Foundation(f) => {
            next.foundations[f].pop();
        }
        Source::Tableau(p, i) => {
            next.tableau[p].truncate(i);
            if next.tableau[p].is_empty() {
                next.face_up[p] = 0;
            } else if next.face_up[p] >= next.tableau[p].len() {
                next.face_up[p] = next.tableau[p].len() - 1;
            }
        }
    }
    match dst {
        Dest::Tableau(p) => {
            let was_empty = next.tableau[p].is_empty();
            for c in &cards {
                next.tableau[p].push(*c);
            }
            if was_empty {
                next.face_up[p] = 0;
            }
        }
        Dest::Foundation(f) => {
            next.foundations[f].push(first);
        }
    }
    next.moves = next.moves.saturating_add(1);
    next.last_move = Some(match dst {
        Dest::Foundation(f) => LastMove::ToFoundation(f),
        Dest::Tableau(p) => LastMove::ToTableau(p),
    });
    Some(next)
}

fn draw_stock(game: &Game) -> Game {
    let mut next = game.clone();
    if let Some(c) = next.stock.pop() {
        next.waste.push(c);
        next.moves = next.moves.saturating_add(1);
    } else if !next.waste.is_empty() {
        while let Some(c) = next.waste.pop() {
            next.stock.push(c);
        }
        next.moves = next.moves.saturating_add(1);
    }
    next.last_move = None;
    next
}

fn auto_move(game: &Game, src: Source) -> Option<Game> {
    let cards = moving_cards(game, src)?;
    if cards.is_empty() {
        return None;
    }
    let first = cards[0];

    if cards.len() == 1 {
        let f = first.suit as usize;
        if let Some(g) = try_apply(game, src, Dest::Foundation(f)) {
            return Some(g);
        }
    }

    let src_pile = match src {
        Source::Tableau(p, _) => Some(p),
        _ => None,
    };

    let mut best: Option<(usize, usize, Game)> = None;
    for p in 0..PILES {
        if Some(p) == src_pile || game.tableau[p].is_empty() {
            continue;
        }
        if let Some(g) = try_apply(game, src, Dest::Tableau(p)) {
            let face_down = game.face_up[p];
            match &best {
                Some((best_fd, _, _)) if *best_fd >= face_down => {}
                _ => best = Some((face_down, p, g)),
            }
        }
    }
    if let Some((_, _, g)) = best {
        return Some(g);
    }

    if first.rank == 13 {
        let dominated = matches!(src, Source::Tableau(p, 0) if game.face_up[p] == 0);
        if !dominated {
            for p in 0..PILES {
                if Some(p) == src_pile || !game.tableau[p].is_empty() {
                    continue;
                }
                if let Some(g) = try_apply(game, src, Dest::Tableau(p)) {
                    return Some(g);
                }
            }
        }
    }

    None
}

fn handle_click(game: &Game, click: Click) -> Game {
    match click {
        Click::Stock => {
            let mut g = draw_stock(game);
            g.failed_move = None;
            g
        }
        Click::Waste => {
            if game.waste.is_empty() {
                return game.clone();
            }
            if let Some(mut g) = auto_move(game, Source::Waste) {
                g.failed_move = None;
                g
            } else {
                let mut next = game.clone();
                next.last_move = None;
                next.failed_move = Some(FailedMove::Waste);
                next
            }
        }
        Click::Foundation(f) => {
            if game.foundations[f].is_empty() {
                return game.clone();
            }
            if let Some(mut g) = auto_move(game, Source::Foundation(f)) {
                g.failed_move = None;
                g
            } else {
                let mut next = game.clone();
                next.last_move = None;
                next.failed_move = None;
                next
            }
        }
        Click::Tableau(p, i) => {
            let pile = &game.tableau[p];
            if pile.is_empty() || i < game.face_up[p] || i >= pile.len() {
                return game.clone();
            }
            if let Some(mut g) = auto_move(game, Source::Tableau(p, i)) {
                g.failed_move = None;
                g
            } else {
                let mut next = game.clone();
                next.last_move = None;
                next.failed_move = Some(FailedMove::Tableau(p, i));
                next
            }
        }
    }
}

struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed.max(1) }
    }

    fn next(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }
}

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

fn current_seed() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0xDEAD_BEEF, |d| d.as_nanos() as u64)
}

fn pile_x(p: usize) -> f64 {
    BOARD_LEFT + p as f64 * (CARD_W + CARD_GAP_X)
}

fn foundation_x(f: usize) -> f64 {
    pile_x(PILES - FOUNDATIONS + f)
}

#[derive(Clone)]
enum Message {
    Click(Click),
    NewGame,
}

struct Solitaire {
    game: Game,
}

impl Component for Solitaire {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            game: Game::new(current_seed()),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Click(click) => self.game = handle_click(&self.game, click),
            Message::NewGame => self.game = Game::new(current_seed()),
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Solitaire");
        context.window_visuals(WindowVisuals::new().client_size(800.0, 600.0).constraints(
            WindowConstraints {
                min_width: Some(800.0),
                min_height: Some(600.0),
                ..Default::default()
            },
        ));

        let header = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(12.0)
            .margin(Thickness::new(12.0, 8.0, 12.0, 4.0))
            .children((
                Button::new()
                    .on_click(context.message(Message::NewGame))
                    .content("New Game"),
                TextBlock::new()
                    .text(status_line(&self.game))
                    .vertical_alignment(VerticalAlignment::Center)
                    .foreground(ThemeBrush::PrimaryText),
            ));

        let title_bar = TitleBar::new()
            .preferred_height(WindowTitleBarHeight::Tall)
            .title("Solitaire")
            .subtitle(if self.game.is_won() {
                "🎉 You win!".to_string()
            } else {
                String::new()
            });

        Border::new().background(Color::rgb(20, 100, 60)).content(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .children((
                    title_bar,
                    header,
                    Viewbox::new().child(build_board(&self.game, context.callback(Message::Click))),
                )),
        )
    }
}

fn status_line(game: &Game) -> String {
    if game.is_won() {
        "🎉 You won! Press New Game to deal again.".to_string()
    } else {
        format!(
            "Moves: {}    Stock: {}    Waste: {}",
            game.moves,
            game.stock.len(),
            game.waste.len()
        )
    }
}

fn build_board(game: &Game, click: Callback<Click>) -> View {
    let mut children: Vec<KeyedView> = Vec::new();

    if game.stock.is_empty() {
        let stock_click = click.clone();
        let view = if game.waste.is_empty() {
            empty_slot("·", Color::rgb(220, 230, 220), move || {
                _ = stock_click.call(Click::Stock);
            })
        } else {
            recycle_stock(move || {
                _ = stock_click.call(Click::Stock);
            })
        };
        children.push(KeyedView::new(
            "stock-slot",
            positioned(view, pile_x(0), TOP_ROW_Y),
        ));
    }

    if game.waste.is_empty() {
        let waste_click = click.clone();
        children.push(KeyedView::new(
            "waste-slot",
            positioned(
                empty_slot("·", Color::rgb(220, 230, 220), move || {
                    _ = waste_click.call(Click::Waste);
                }),
                pile_x(1),
                TOP_ROW_Y,
            ),
        ));
    }

    for (f, foundation) in game.foundations.iter().enumerate() {
        if foundation.is_empty() {
            let foundation_click = click.clone();
            let suit = Suit::all()[f];
            let foreground = if suit.is_red() {
                Color::rgb(180, 120, 120)
            } else {
                Color::rgb(180, 180, 180)
            };
            children.push(KeyedView::new(
                format!("foundation-slot-{f}"),
                positioned(
                    empty_slot(suit.symbol(), foreground, move || {
                        _ = foundation_click.call(Click::Foundation(f));
                    }),
                    foundation_x(f),
                    TOP_ROW_Y,
                ),
            ));
        }
    }

    for (p, pile) in game.tableau.iter().enumerate() {
        if pile.is_empty() {
            let tableau_click = click.clone();
            children.push(KeyedView::new(
                format!("tableau-slot-{p}"),
                positioned(
                    empty_slot("K", Color::rgb(180, 200, 180), move || {
                        _ = tableau_click.call(Click::Tableau(p, 0));
                    }),
                    pile_x(p),
                    TABLEAU_Y,
                ),
            ));
        }
    }

    for placement in board_cards(game) {
        let card_click = click.clone();
        let view = if placement.face_up {
            card_face(
                placement.card,
                placement.highlighted,
                placement.failed,
                move || {
                    _ = card_click.call(placement.click);
                },
            )
        } else {
            card_back(move || {
                _ = card_click.call(placement.click);
            })
        };
        children.push(KeyedView::new(
            card_key(placement.card),
            positioned_card(view, placement.x, placement.y),
        ));
    }

    Grid::new()
        .width(BOARD_W)
        .height(BOARD_H)
        .horizontal_alignment(HorizontalAlignment::Center)
        .keyed_children(children)
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

    if let Some(card) = game.stock.last().copied() {
        cards.push(BoardCard {
            card,
            x: pile_x(0),
            y: TOP_ROW_Y,
            face_up: false,
            highlighted: false,
            failed: false,
            click: Click::Stock,
        });
    }

    if let Some(card) = game.waste.last().copied() {
        cards.push(BoardCard {
            card,
            x: pile_x(1),
            y: TOP_ROW_Y,
            face_up: true,
            highlighted: false,
            failed: matches!(game.failed_move, Some(FailedMove::Waste)),
            click: Click::Waste,
        });
    }

    for f in 0..FOUNDATIONS {
        if let Some(card) = game.foundations[f].last().copied() {
            cards.push(BoardCard {
                card,
                x: foundation_x(f),
                y: TOP_ROW_Y,
                face_up: true,
                highlighted: matches!(
                    game.last_move,
                    Some(LastMove::ToFoundation(destination)) if destination == f
                ),
                failed: false,
                click: Click::Foundation(f),
            });
        }
    }

    for p in 0..PILES {
        let mut y = 0.0_f64;
        for (i, card) in game.tableau[p].iter().copied().enumerate() {
            let is_face_up = i >= game.face_up[p];
            cards.push(BoardCard {
                card,
                x: pile_x(p),
                y: TABLEAU_Y + y,
                face_up: is_face_up,
                highlighted: is_face_up
                    && matches!(game.last_move, Some(LastMove::ToTableau(destination)) if destination == p)
                    && i == game.tableau[p].len() - 1,
                failed: is_face_up
                    && matches!(game.failed_move, Some(FailedMove::Tableau(pile, index)) if pile == p && index == i),
                click: Click::Tableau(p, i),
            });
            y += if is_face_up {
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

fn recycle_stock(on_click: impl Fn() + 'static) -> View {
    let label = TextBlock::new()
        .text("↻")
        .font_size(22.0)
        .foreground(Color::rgb(255, 255, 255))
        .vertical_alignment(VerticalAlignment::Center)
        .horizontal_alignment(HorizontalAlignment::Center);
    Border::new()
        .corner_radius(4.0)
        .border_brush(Color::rgb(50, 90, 60))
        .border_thickness(Thickness::uniform(1.5))
        .background(Color::rgb(70, 110, 80))
        .width(CARD_W)
        .height(CARD_H)
        .on_pointer_released(move |_| on_click())
        .content(label)
}

fn card_face(card: Card, highlighted: bool, failed: bool, on_click: impl Fn() + 'static) -> View {
    let fg = if card.is_red() {
        Color::rgb(192, 0, 32)
    } else {
        Color::rgb(20, 20, 20)
    };
    let bg = if failed {
        Color::rgb(255, 220, 220)
    } else if highlighted {
        Color::rgb(220, 245, 220)
    } else {
        Color::rgb(255, 255, 255)
    };
    let border_color = if failed {
        Color::rgb(220, 80, 80)
    } else {
        Color::rgb(180, 180, 180)
    };
    let top_label = TextBlock::new()
        .text(card.label())
        .font_size(13.0)
        .foreground(fg)
        .horizontal_alignment(HorizontalAlignment::Left);
    let center_suit = TextBlock::new()
        .text(card.suit.symbol())
        .font_size(22.0)
        .foreground(fg)
        .horizontal_alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Center);
    let content = StackPanel::new()
        .orientation(Orientation::Vertical)
        .children((top_label, center_suit));
    Border::new()
        .corner_radius(4.0)
        .border_brush(border_color)
        .border_thickness(Thickness::uniform(1.0))
        .background(bg)
        .width(CARD_W)
        .height(CARD_H)
        .padding(Thickness::new(4.0, 3.0, 2.0, 2.0))
        .on_pointer_released(move |_| on_click())
        .content(content)
}

fn card_back(on_click: impl Fn() + 'static) -> View {
    let label = TextBlock::new()
        .text("🂠")
        .font_size(18.0)
        .foreground(Color::rgb(240, 240, 255))
        .vertical_alignment(VerticalAlignment::Center)
        .horizontal_alignment(HorizontalAlignment::Center);
    Border::new()
        .corner_radius(4.0)
        .border_brush(Color::rgb(30, 60, 130))
        .border_thickness(Thickness::uniform(1.0))
        .background(Color::rgb(40, 80, 160))
        .width(CARD_W)
        .height(CARD_H)
        .on_pointer_released(move |_| on_click())
        .content(label)
}

fn empty_slot(label: &str, fg: Color, on_click: impl Fn() + 'static) -> View {
    let tb = TextBlock::new()
        .text(label)
        .font_size(16.0)
        .foreground(fg)
        .vertical_alignment(VerticalAlignment::Center)
        .horizontal_alignment(HorizontalAlignment::Center);
    Border::new()
        .corner_radius(4.0)
        .border_brush(Color::rgb(50, 110, 70))
        .border_thickness(Thickness::uniform(1.5))
        .background(Color::rgb(40, 90, 55))
        .width(CARD_W)
        .height(CARD_H)
        .opacity(0.7)
        .on_pointer_released(move |_| on_click())
        .content(tb)
}

fn positioned(content: View, x: f64, y: f64) -> View {
    Border::new()
        .horizontal_alignment(HorizontalAlignment::Left)
        .vertical_alignment(VerticalAlignment::Top)
        .margin(Thickness::new(x, y, 0.0, 0.0))
        .content(content)
}

fn positioned_card(content: View, x: f64, y: f64) -> View {
    Border::new()
        .horizontal_alignment(HorizontalAlignment::Left)
        .vertical_alignment(VerticalAlignment::Top)
        .margin(Thickness::new(x, y, 0.0, 0.0))
        .transitions([ThemeTransition::Reposition])
        .content(content)
}

fn main() {
    App::run_component::<Solitaire>(()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: Suit) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn full_deck_has_52_unique_cards() {
        let deck = full_deck();
        assert_eq!(deck.len(), DECK_SIZE);
        let mut seen = std::collections::HashSet::new();
        for card in &deck {
            assert!(seen.insert((card.rank, card.suit as u8)));
            assert!((1..=13).contains(&card.rank));
        }
        assert_eq!(seen.len(), DECK_SIZE);
    }

    #[test]
    fn new_game_deals_28_tableau_and_24_stock() {
        let g = Game::new(1);
        let tableau_count: usize = g.tableau.iter().map(|p| p.len()).sum();
        assert_eq!(tableau_count, 28);
        for (p, pile) in g.tableau.iter().enumerate() {
            assert_eq!(pile.len(), p + 1);
            assert_eq!(g.face_up[p], pile.len() - 1);
        }
        assert_eq!(g.stock.len(), 24);
        assert!(g.waste.is_empty());
        assert!(g.foundations.iter().all(|f| f.is_empty()));
        assert_eq!(g.moves, 0);
        assert!(g.last_move.is_none());
        assert!(!g.is_won());
    }

    #[test]
    fn deal_uses_every_card_exactly_once_and_has_no_face_down_aces() {
        let g = Game::new(0xABCDEF);
        let mut seen = std::collections::HashSet::new();
        for p in 0..PILES {
            let pile = &g.tableau[p];
            for (i, card) in pile.iter().enumerate() {
                assert!(seen.insert((card.rank, card.suit as u8)));
                if i < g.face_up[p] {
                    assert_ne!(card.rank, 1, "Ace should never be face-down");
                }
            }
        }
        for card in &g.stock {
            assert!(seen.insert((card.rank, card.suit as u8)));
        }
        assert_eq!(seen.len(), DECK_SIZE);
    }

    #[test]
    fn new_game_is_deterministic_for_seed() {
        let a = Game::new(42);
        let b = Game::new(42);
        assert_eq!(a, b);
    }

    #[test]
    fn shuffle_actually_reorders() {
        let mut a = full_deck();
        let unshuffled = a.clone();
        shuffle(&mut a, 7);
        assert_ne!(a, unshuffled);
        let mut sorted_a = a.clone();
        sorted_a.sort_by_key(|c| (c.suit as u8, c.rank));
        let mut sorted_b = unshuffled;
        sorted_b.sort_by_key(|c| (c.suit as u8, c.rank));
        assert_eq!(sorted_a, sorted_b);
    }

    #[test]
    fn tableau_stacking_rules() {
        assert!(can_stack_on_tableau(c(13, Suit::Spades), None));
        assert!(!can_stack_on_tableau(c(12, Suit::Spades), None));
        assert!(can_stack_on_tableau(
            c(6, Suit::Hearts),
            Some(c(7, Suit::Clubs))
        ));
        assert!(!can_stack_on_tableau(
            c(6, Suit::Hearts),
            Some(c(7, Suit::Diamonds))
        ));
        assert!(!can_stack_on_tableau(
            c(5, Suit::Hearts),
            Some(c(7, Suit::Clubs))
        ));
    }

    #[test]
    fn foundation_placement_rules() {
        assert!(can_place_on_foundation(
            c(1, Suit::Spades),
            &[],
            Suit::Spades as usize
        ));
        assert!(!can_place_on_foundation(
            c(1, Suit::Hearts),
            &[],
            Suit::Spades as usize
        ));
        assert!(can_place_on_foundation(
            c(2, Suit::Spades),
            &[c(1, Suit::Spades)],
            Suit::Spades as usize
        ));
        assert!(!can_place_on_foundation(
            c(3, Suit::Spades),
            &[c(1, Suit::Spades)],
            Suit::Spades as usize
        ));
        assert!(!can_place_on_foundation(
            c(2, Suit::Spades),
            &[],
            Suit::Spades as usize
        ));
    }

    fn empty_game() -> Game {
        Game {
            tableau: (0..PILES).map(|_| Vec::new()).collect(),
            face_up: vec![0; PILES],
            foundations: (0..FOUNDATIONS).map(|_| Vec::new()).collect(),
            stock: Vec::new(),
            waste: Vec::new(),
            last_move: None,
            failed_move: None,
            moves: 0,
        }
    }

    #[test]
    fn moving_ace_from_waste_to_foundation_is_legal() {
        let mut g = empty_game();
        g.waste.push(c(1, Suit::Hearts));
        let next =
            try_apply(&g, Source::Waste, Dest::Foundation(Suit::Hearts as usize)).expect("legal");
        assert!(next.waste.is_empty());
        assert_eq!(
            next.foundations[Suit::Hearts as usize],
            vec![c(1, Suit::Hearts)]
        );
        assert_eq!(next.moves, 1);
    }

    #[test]
    fn draw_stock_moves_top_to_waste() {
        let mut g = empty_game();
        g.stock = vec![c(2, Suit::Clubs), c(5, Suit::Hearts)];
        let next = draw_stock(&g);
        assert_eq!(next.stock, vec![c(2, Suit::Clubs)]);
        assert_eq!(next.waste, vec![c(5, Suit::Hearts)]);
        assert_eq!(next.moves, 1);
    }

    #[test]
    fn draw_stock_recycles_waste_when_empty() {
        let mut g = empty_game();
        g.waste = vec![c(2, Suit::Clubs), c(5, Suit::Hearts), c(9, Suit::Diamonds)];
        let next = draw_stock(&g);
        assert!(next.waste.is_empty());
        assert_eq!(
            next.stock,
            vec![c(9, Suit::Diamonds), c(5, Suit::Hearts), c(2, Suit::Clubs)]
        );
        assert_eq!(next.moves, 1);
    }

    #[test]
    fn clicking_waste_ace_auto_moves_to_foundation() {
        let mut g = empty_game();
        g.waste.push(c(1, Suit::Hearts));
        let g = handle_click(&g, Click::Waste);
        assert!(g.waste.is_empty());
        assert_eq!(
            g.foundations[Suit::Hearts as usize],
            vec![c(1, Suit::Hearts)]
        );
        assert_eq!(g.moves, 1);
    }

    #[test]
    fn auto_move_prefers_destination_with_more_face_down_cards() {
        let mut g = empty_game();
        g.waste.push(c(6, Suit::Hearts));
        g.tableau[0] = vec![c(7, Suit::Spades)];
        g.face_up[0] = 0;
        g.tableau[5] = vec![
            c(2, Suit::Diamonds),
            c(4, Suit::Diamonds),
            c(10, Suit::Hearts),
            c(7, Suit::Clubs),
        ];
        g.face_up[5] = 3;
        let g = handle_click(&g, Click::Waste);
        assert!(g.waste.is_empty());
        assert_eq!(g.tableau[0], vec![c(7, Suit::Spades)]);
        assert_eq!(g.tableau[5].last(), Some(&c(6, Suit::Hearts)));
    }

    #[test]
    fn clicking_waste_card_does_nothing_when_no_move() {
        let mut g = empty_game();
        g.waste.push(c(7, Suit::Hearts));
        let next = handle_click(&g, Click::Waste);
        assert_eq!(next.waste, vec![c(7, Suit::Hearts)]);
        assert_eq!(next.moves, 0);
    }

    #[test]
    fn clicking_tableau_ace_auto_moves_to_foundation() {
        let mut g = empty_game();
        g.tableau[0] = vec![c(1, Suit::Spades)];
        g.face_up[0] = 0;
        let g = handle_click(&g, Click::Tableau(0, 0));
        assert!(g.tableau[0].is_empty());
        assert_eq!(
            g.foundations[Suit::Spades as usize],
            vec![c(1, Suit::Spades)]
        );
    }

    #[test]
    fn board_cards_keep_physical_identity_across_tableau_moves() {
        let moving = c(6, Suit::Hearts);
        let mut game = empty_game();
        game.tableau[0] = vec![c(9, Suit::Diamonds), moving];
        game.face_up[0] = 1;
        game.tableau[1] = vec![c(7, Suit::Spades)];
        game.face_up[1] = 0;

        let before = board_cards(&game)
            .into_iter()
            .find(|placement| placement.card == moving)
            .unwrap();
        let game = try_apply(&game, Source::Tableau(0, 1), Dest::Tableau(1)).unwrap();
        let after = board_cards(&game)
            .into_iter()
            .find(|placement| placement.card == moving)
            .unwrap();

        assert_eq!(card_key(before.card), card_key(after.card));
        assert_eq!(before.click, Click::Tableau(0, 1));
        assert_eq!(after.click, Click::Tableau(1, 1));
        assert_eq!(before.x, pile_x(0));
        assert_eq!(after.x, pile_x(1));
    }

    #[test]
    fn visible_board_cards_have_unique_keys() {
        let game = Game::new(42);
        let cards = board_cards(&game);
        let keys = cards
            .iter()
            .map(|placement| card_key(placement.card))
            .collect::<std::collections::HashSet<_>>();

        assert_eq!(keys.len(), cards.len());
    }

    #[test]
    fn king_at_base_does_not_move_to_another_empty() {
        let mut g = empty_game();
        g.tableau[0] = vec![c(13, Suit::Spades)];
        g.face_up[0] = 0;
        let g2 = handle_click(&g, Click::Tableau(0, 0));
        assert_eq!(g2.tableau[0], vec![c(13, Suit::Spades)]);
        assert_eq!(g2.moves, 0);
    }
}
