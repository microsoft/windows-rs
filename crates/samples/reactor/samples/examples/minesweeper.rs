#![windows_subsystem = "windows"]

use std::time::{SystemTime, UNIX_EPOCH};

use windows_reactor::{
    Application, Button, Callback, Color, Element, Grid, GridChild, GridLength,
    HorizontalAlignment, RenderCx, TextBlock, Thickness, VerticalAlignment, Window,
    WindowConstraints, vstack,
};

const WIDTH: usize = 9;
const HEIGHT: usize = 9;
const TOTAL: usize = WIDTH * HEIGHT;
const MINES: usize = 10;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TileState {
    Hidden,
    Flag,
    Question,
    Revealed,
}

impl TileState {
    fn cycle(self) -> Self {
        match self {
            Self::Hidden => Self::Flag,
            Self::Flag => Self::Question,
            Self::Question => Self::Hidden,
            Self::Revealed => Self::Revealed,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Status {
    Playing,
    Won,
    Lost,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Game {
    tiles: Vec<TileState>,
    mines: Vec<bool>,
    neighbors: Vec<i8>,
    generated: bool,
    hit_mine: Option<usize>,
    status: Status,
    seed: u64,
}

impl Game {
    fn new() -> Self {
        Self::new_seeded(seed_from_clock())
    }

    fn new_seeded(seed: u64) -> Self {
        Self {
            tiles: vec![TileState::Hidden; TOTAL],
            mines: vec![false; TOTAL],
            neighbors: vec![0; TOTAL],
            generated: false,
            hit_mine: None,
            status: Status::Playing,
            seed,
        }
    }

    fn index(x: usize, y: usize) -> usize {
        y * WIDTH + x
    }

    fn xy(index: usize) -> (usize, usize) {
        (index % WIDTH, index / WIDTH)
    }

    fn flag_count(&self) -> usize {
        self.tiles
            .iter()
            .filter(|tile| **tile == TileState::Flag)
            .count()
    }
}

fn neighbors(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    const OFFSETS: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    OFFSETS.iter().filter_map(move |(dx, dy)| {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx < 0 || ny < 0 || nx >= WIDTH as i32 || ny >= HEIGHT as i32 {
            None
        } else {
            Some((nx as usize, ny as usize))
        }
    })
}

fn generate_mines(game: &mut Game, exclude_x: usize, exclude_y: usize) {
    let exclude = Game::index(exclude_x, exclude_y);
    let mut rng = XorShift64::new(game.seed);
    let mut placed = 0;
    while placed < MINES {
        let index = (rng.next_u64() as usize) % TOTAL;
        if index == exclude || game.mines[index] {
            continue;
        }
        game.mines[index] = true;
        placed += 1;
    }

    for index in 0..TOTAL {
        let (x, y) = Game::xy(index);
        if game.mines[index] {
            game.neighbors[index] = -1;
        } else {
            game.neighbors[index] = neighbors(x, y)
                .filter(|(nx, ny)| game.mines[Game::index(*nx, *ny)])
                .count() as i8;
        }
    }
    game.generated = true;
}

fn apply_reveal(game: &Game, x: usize, y: usize) -> Option<Game> {
    if x >= WIDTH || y >= HEIGHT || game.status != Status::Playing {
        return None;
    }
    let index = Game::index(x, y);
    if game.tiles[index] != TileState::Hidden {
        return None;
    }

    let mut next = game.clone();
    if !next.generated {
        generate_mines(&mut next, x, y);
    }
    if next.mines[index] {
        for index in 0..TOTAL {
            if next.mines[index] {
                next.tiles[index] = TileState::Revealed;
            }
        }
        next.hit_mine = Some(index);
        next.status = Status::Lost;
        return Some(next);
    }

    let mut queue = vec![index];
    next.tiles[index] = TileState::Revealed;
    while let Some(current) = queue.pop() {
        if next.neighbors[current] != 0 {
            continue;
        }
        let (cx, cy) = Game::xy(current);
        for (nx, ny) in neighbors(cx, cy) {
            let neighbor = Game::index(nx, ny);
            if next.tiles[neighbor] == TileState::Hidden && !next.mines[neighbor] {
                next.tiles[neighbor] = TileState::Revealed;
                queue.push(neighbor);
            }
        }
    }
    if check_won(&next) {
        next.status = Status::Won;
    }
    Some(next)
}

fn apply_flag(game: &Game, x: usize, y: usize) -> Option<Game> {
    if x >= WIDTH || y >= HEIGHT || game.status != Status::Playing {
        return None;
    }
    let index = Game::index(x, y);
    if game.tiles[index] == TileState::Revealed {
        return None;
    }
    let mut next = game.clone();
    next.tiles[index] = next.tiles[index].cycle();
    Some(next)
}

fn check_won(game: &Game) -> bool {
    game.generated
        && (0..TOTAL).all(|index| game.mines[index] || game.tiles[index] == TileState::Revealed)
}

struct XorShift64(u64);

impl XorShift64 {
    fn new(seed: u64) -> Self {
        Self(if seed == 0 {
            0xDEAD_BEEF_CAFE_F00D
        } else {
            seed
        })
    }

    fn next_u64(&mut self) -> u64 {
        let mut value = self.0;
        value ^= value << 13;
        value ^= value >> 7;
        value ^= value << 17;
        self.0 = value;
        value
    }
}

fn seed_from_clock() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0xA5A5_5A5A_A5A5_5A5A, |duration| duration.as_nanos() as u64)
}

fn number_color(number: i8) -> Color {
    match number {
        1 => Color::rgb(0, 0, 200),
        2 => Color::rgb(0, 128, 0),
        3 => Color::rgb(200, 0, 0),
        4 => Color::rgb(0, 0, 128),
        5 => Color::rgb(128, 0, 0),
        6 => Color::rgb(0, 128, 128),
        7 => Color::rgb(0, 0, 0),
        _ => Color::rgb(128, 128, 128),
    }
}

fn tile_label(game: &Game, index: usize) -> String {
    match game.tiles[index] {
        TileState::Hidden => String::new(),
        TileState::Flag => "🚩".to_string(),
        TileState::Question => "?".to_string(),
        TileState::Revealed if game.mines[index] => "💣".to_string(),
        TileState::Revealed => match game.neighbors[index] {
            0 => String::new(),
            count => count.to_string(),
        },
    }
}

fn status_line(game: &Game) -> String {
    let remaining = MINES as i32 - game.flag_count() as i32;
    match game.status {
        Status::Playing => format!("Mines remaining: {remaining}"),
        Status::Won => "🎉 You cleared the board!".to_string(),
        Status::Lost => "💥 Boom! Game over.".to_string(),
    }
}

fn tile_automation_name(game: &Game, index: usize) -> String {
    let (x, y) = Game::xy(index);
    let position = format!("row {}, column {}", y + 1, x + 1);
    let state = match game.tiles[index] {
        TileState::Hidden => "hidden".to_string(),
        TileState::Flag => "flagged".to_string(),
        TileState::Question => "question mark".to_string(),
        TileState::Revealed if game.mines[index] => "mine".to_string(),
        TileState::Revealed if game.neighbors[index] == 0 => "empty".to_string(),
        TileState::Revealed if game.neighbors[index] == 1 => "1 mine nearby".to_string(),
        TileState::Revealed => format!("{} mines nearby", game.neighbors[index]),
    };
    format!("Tile {position}, {state}")
}

enum Action {
    Reveal(usize, usize),
    Flag(usize, usize),
    Reset,
}

fn reduce(state: Game, action: Action) -> Game {
    match action {
        Action::Reveal(x, y) => apply_reveal(&state, x, y).unwrap_or(state),
        Action::Flag(x, y) => apply_flag(&state, x, y).unwrap_or(state),
        Action::Reset => Game::new(),
    }
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let open = cx.use_state(|| true);
    let (game, dispatch) = cx.use_reducer(Game::new, reduce);
    let windows = if open.value() {
        vec![
            Window::new("Minesweeper", game_view(&game, dispatch), move || {
                open.set(false);
            })
            .client_size(480.0, 560.0)
            .client_constraints(WindowConstraints {
                min_width: Some(480.0),
                min_height: Some(560.0),
                ..WindowConstraints::default()
            })
            .build()
            .key(0),
        ]
    } else {
        Vec::new()
    };
    Application::new(windows).build()
}

fn game_view(game: &Game, dispatch: Callback<Action>) -> Element {
    let reset = dispatch.clone();
    let header = vstack(
        8.0,
        [
            TextBlock::new(status_line(game))
                .font_size(20.0)
                .horizontal_alignment(HorizontalAlignment::Center)
                .build(),
            Button::new("New Game")
                .on_click(move || reset.call(Action::Reset))
                .horizontal_alignment(HorizontalAlignment::Center)
                .build(),
        ],
    );

    vstack(
        12.0,
        [
            windows_reactor::Border::new(header)
                .margin(Thickness {
                    top: 12.0,
                    bottom: 4.0,
                    ..Thickness::default()
                })
                .build(),
            build_board(game, dispatch),
        ],
    )
}

fn build_board(game: &Game, dispatch: Callback<Action>) -> Element {
    let cells = (0..TOTAL).map(|index| {
        let (x, y) = Game::xy(index);
        let tile = game.tiles[index];
        let reveal = dispatch.clone();
        let flag = dispatch.clone();
        let mut button = Button::new(tile_label(game, index))
            .on_click(move || reveal.call(Action::Reveal(x, y)))
            .on_right_tapped(move || flag.call(Action::Flag(x, y)))
            .horizontal_alignment(HorizontalAlignment::Stretch)
            .vertical_alignment(VerticalAlignment::Stretch)
            .automation_name(tile_automation_name(game, index));
        if tile == TileState::Revealed && !game.mines[index] && game.neighbors[index] > 0 {
            button = button.foreground(number_color(game.neighbors[index]));
        }
        if game.hit_mine == Some(index) {
            button = button.foreground(Color::rgb(220, 80, 80));
        }
        if game.status != Status::Playing || tile == TileState::Revealed {
            button = button.enabled(false);
        }
        GridChild::new(button.build().key(index as u64))
            .row(y as i32)
            .column(x as i32)
    });

    Grid::new(cells)
        .rows([GridLength::STAR; HEIGHT])
        .columns([GridLength::STAR; WIDTH])
        .row_spacing(2.0)
        .column_spacing(2.0)
        .width(420.0)
        .height(420.0)
        .horizontal_alignment(HorizontalAlignment::Center)
        .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run_application(app)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh(seed: u64) -> Game {
        Game::new_seeded(seed)
    }

    #[test]
    fn new_game_is_hidden_and_playing() {
        let game = fresh(1);
        assert_eq!(game.status, Status::Playing);
        assert!(!game.generated);
        assert!(game.tiles.iter().all(|tile| *tile == TileState::Hidden));
        assert_eq!(game.flag_count(), 0);
    }

    #[test]
    fn first_reveal_is_never_a_mine() {
        for seed in 1..200 {
            let game = apply_reveal(&fresh(seed), 4, 4).unwrap();
            assert!(!game.mines[Game::index(4, 4)]);
        }
    }

    #[test]
    fn generates_exactly_mines_count() {
        let game = apply_reveal(&fresh(42), 0, 0).unwrap();
        assert_eq!(game.mines.iter().filter(|mine| **mine).count(), MINES);
    }

    #[test]
    fn flag_cycle_hidden_flag_question_hidden() {
        let game = apply_flag(&fresh(7), 0, 0).unwrap();
        assert_eq!(game.tiles[0], TileState::Flag);
        let game = apply_flag(&game, 0, 0).unwrap();
        assert_eq!(game.tiles[0], TileState::Question);
        let game = apply_flag(&game, 0, 0).unwrap();
        assert_eq!(game.tiles[0], TileState::Hidden);
    }

    #[test]
    fn flagged_tile_cannot_be_revealed() {
        let game = apply_flag(&fresh(7), 3, 3).unwrap();
        assert!(apply_reveal(&game, 3, 3).is_none());
    }

    #[test]
    fn revealed_tile_cannot_be_flagged_or_revealed_again() {
        let game = apply_reveal(&fresh(7), 0, 0).unwrap();
        assert!(apply_flag(&game, 0, 0).is_none());
        assert!(apply_reveal(&game, 0, 0).is_none());
    }

    #[test]
    fn revealing_a_mine_loses_and_reveals_all_mines() {
        let mut game = apply_reveal(&fresh(9), 0, 0).unwrap();
        let mine = (0..TOTAL).find(|index| game.mines[*index]).unwrap();
        let (x, y) = Game::xy(mine);
        game.tiles[mine] = TileState::Hidden;
        let game = apply_reveal(&game, x, y).unwrap();
        assert_eq!(game.status, Status::Lost);
        assert_eq!(game.hit_mine, Some(mine));
        assert!(
            (0..TOTAL).all(|index| !game.mines[index] || game.tiles[index] == TileState::Revealed)
        );
    }

    #[test]
    fn no_interactions_after_game_over() {
        let mut game = apply_reveal(&fresh(9), 0, 0).unwrap();
        let mine = (0..TOTAL).find(|index| game.mines[*index]).unwrap();
        let (x, y) = Game::xy(mine);
        game.tiles[mine] = TileState::Hidden;
        let game = apply_reveal(&game, x, y).unwrap();
        assert!(apply_reveal(&game, 0, 0).is_none());
        assert!(apply_flag(&game, 0, 0).is_none());
    }

    #[test]
    fn revealing_every_non_mine_wins() {
        let mut game = apply_reveal(&fresh(11), 0, 0).unwrap();
        for index in 0..TOTAL {
            if !game.mines[index] && game.tiles[index] != TileState::Revealed {
                let (x, y) = Game::xy(index);
                game.tiles[index] = TileState::Hidden;
                if let Some(next) = apply_reveal(&game, x, y) {
                    game = next;
                }
            }
        }
        assert_eq!(game.status, Status::Won);
    }

    #[test]
    fn out_of_bounds_clicks_are_noop() {
        let game = fresh(1);
        assert!(apply_reveal(&game, WIDTH, 0).is_none());
        assert!(apply_reveal(&game, 0, HEIGHT).is_none());
        assert!(apply_flag(&game, WIDTH, 0).is_none());
        assert!(apply_flag(&game, 0, HEIGHT).is_none());
    }

    #[test]
    fn xorshift_is_not_constant() {
        let mut rng = XorShift64::new(1);
        let first = rng.next_u64();
        let second = rng.next_u64();
        let third = rng.next_u64();
        assert_ne!(first, second);
        assert_ne!(second, third);
    }
}
