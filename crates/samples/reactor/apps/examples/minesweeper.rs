#![windows_subsystem = "windows"]

//! A `windows-reactor` port of <https://github.com/robmikh/minesweeper-rs>.
//!
//! The board is a grid of `button` widgets with all state in `cx.use_state`.
//! Left-tap reveals, right-tap cycles flag/question. Defaults to 9×9 / 10 mines.

use std::time::{SystemTime, UNIX_EPOCH};

use windows_reactor::*;

// ---------------------------------------------------------------------------
// Game model
// ---------------------------------------------------------------------------

const WIDTH: usize = 9;
const HEIGHT: usize = 9;
const TOTAL: usize = WIDTH * HEIGHT;
const MINES: usize = 10;

/// Per-tile UI state, mirroring `MineState` in the original sample.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TileState {
    Hidden,
    Flag,
    Question,
    Revealed,
}

impl TileState {
    /// Cycle through flag/question/hidden on a flag-mode click.
    fn cycle(self) -> Self {
        match self {
            TileState::Hidden => TileState::Flag,
            TileState::Flag => TileState::Question,
            TileState::Question => TileState::Hidden,
            // Revealed tiles cannot be flagged; callers must check first.
            TileState::Revealed => TileState::Revealed,
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
    /// Per-tile UI state.
    tiles: Vec<TileState>,
    /// True iff the tile is a mine. Filled in lazily on the first reveal so
    /// the first click is always safe (matching the original sample).
    mines: Vec<bool>,
    /// Neighbour mine counts for non-mine tiles; `-1` for mine tiles. Filled
    /// in alongside `mines`.
    neighbors: Vec<i8>,
    /// True once `mines`/`neighbors` have been populated.
    generated: bool,
    /// Set to `Some(index)` when the player triggers a mine, so the UI can
    /// highlight the fatal tile.
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

    /// Count of `TileState::Flag` tiles. Used to derive "mines remaining".
    fn flag_count(&self) -> usize {
        self.tiles.iter().filter(|t| **t == TileState::Flag).count()
    }
}

/// Iterate the (up to) eight neighbours of `(x, y)`.
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

/// Populate `game.mines` and `game.neighbors`, avoiding the tile at
/// `(exclude_x, exclude_y)` so the first click is never a mine.
fn generate_mines(game: &mut Game, exclude_x: usize, exclude_y: usize) {
    let exclude = Game::index(exclude_x, exclude_y);
    let mut rng = XorShift64::new(game.seed);

    let mut placed = 0;
    while placed < MINES {
        let idx = (rng.next_u64() as usize) % TOTAL;
        if idx == exclude || game.mines[idx] {
            continue;
        }
        game.mines[idx] = true;
        placed += 1;
    }

    for i in 0..TOTAL {
        let (x, y) = Game::xy(i);
        if game.mines[i] {
            game.neighbors[i] = -1;
        } else {
            let count = neighbors(x, y)
                .filter(|(nx, ny)| game.mines[Game::index(*nx, *ny)])
                .count();
            game.neighbors[i] = count as i8;
        }
    }

    game.generated = true;
}

/// Try to reveal `(x, y)`. Returns `None` if the click is a no-op (e.g.
/// game over, already revealed, or flagged); otherwise returns the new
/// `Game`. Mirrors `Minesweeper::sweep` + reveal bookkeeping in the original.
fn apply_reveal(game: &Game, x: usize, y: usize) -> Option<Game> {
    if x >= WIDTH || y >= HEIGHT {
        return None;
    }
    if game.status != Status::Playing {
        return None;
    }
    let idx = Game::index(x, y);
    match game.tiles[idx] {
        TileState::Revealed | TileState::Flag | TileState::Question => return None,
        TileState::Hidden => {}
    }

    let mut next = game.clone();
    if !next.generated {
        generate_mines(&mut next, x, y);
    }

    if next.mines[idx] {
        // Reveal every mine so the player can see the layout, then mark
        // the fatal tile and lose.
        for i in 0..TOTAL {
            if next.mines[i] {
                next.tiles[i] = TileState::Revealed;
            }
        }
        next.hit_mine = Some(idx);
        next.status = Status::Lost;
        return Some(next);
    }

    // BFS flood fill: reveal this tile and, for every revealed 0-neighbour
    // tile, expand to its hidden neighbours.
    let mut queue: Vec<usize> = Vec::new();
    queue.push(idx);
    next.tiles[idx] = TileState::Revealed;
    while let Some(cur) = queue.pop() {
        if next.neighbors[cur] != 0 {
            continue;
        }
        let (cx, cy) = Game::xy(cur);
        for (nx, ny) in neighbors(cx, cy) {
            let nidx = Game::index(nx, ny);
            // Only reveal hidden tiles — flag/question stay as the player
            // left them, which matches typical minesweeper behaviour.
            if next.tiles[nidx] == TileState::Hidden && !next.mines[nidx] {
                next.tiles[nidx] = TileState::Revealed;
                queue.push(nidx);
            }
        }
    }

    if check_won(&next) {
        next.status = Status::Won;
    }
    Some(next)
}

/// Cycle hidden → flag → question → hidden on `(x, y)`. Returns `None` if
/// the click is a no-op (game over or tile already revealed).
fn apply_flag(game: &Game, x: usize, y: usize) -> Option<Game> {
    if x >= WIDTH || y >= HEIGHT {
        return None;
    }
    if game.status != Status::Playing {
        return None;
    }
    let idx = Game::index(x, y);
    if game.tiles[idx] == TileState::Revealed {
        return None;
    }
    let mut next = game.clone();
    next.tiles[idx] = next.tiles[idx].cycle();
    Some(next)
}

/// Win = every non-mine tile is revealed.
fn check_won(game: &Game) -> bool {
    if !game.generated {
        return false;
    }
    for i in 0..TOTAL {
        if !game.mines[i] && game.tiles[i] != TileState::Revealed {
            return false;
        }
    }
    true
}

// ---------------------------------------------------------------------------
// Tiny seedable PRNG
// ---------------------------------------------------------------------------
//
// Marsaglia xorshift64. Plenty good enough to scatter ten mines across an
// 81-tile board and lets us seed deterministically from unit tests without
// pulling in `rand`.

struct XorShift64(u64);

impl XorShift64 {
    fn new(seed: u64) -> Self {
        // 0 is a fixed point for xorshift64; substitute a non-zero constant.
        Self(if seed == 0 {
            0xDEAD_BEEF_CAFE_F00D
        } else {
            seed
        })
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

fn seed_from_clock() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0xA5A5_5A5A_A5A5_5A5A, |d| d.as_nanos() as u64)
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

/// Color-coded foreground for a revealed number tile, matching the classic
/// Windows Minesweeper palette.
fn number_color(n: i8) -> Color {
    match n {
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

fn tile_label(game: &Game, idx: usize) -> String {
    match game.tiles[idx] {
        TileState::Hidden => String::new(),
        TileState::Flag => "🚩".to_string(),
        TileState::Question => "?".to_string(),
        TileState::Revealed => {
            if game.mines[idx] {
                "💣".to_string()
            } else {
                let n = game.neighbors[idx];
                if n == 0 { String::new() } else { n.to_string() }
            }
        }
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

/// Per-tile screen-reader label, derived from the same data that drives
/// the visible glyph. Without this, hidden tiles and revealed zero-neighbour
/// tiles have an empty button content and become indistinguishable to
/// assistive tech.
fn tile_automation_name(game: &Game, idx: usize) -> String {
    let (x, y) = Game::xy(idx);
    let position = format!("row {}, column {}", y + 1, x + 1);
    let state = match game.tiles[idx] {
        TileState::Hidden => "hidden".to_string(),
        TileState::Flag => "flagged".to_string(),
        TileState::Question => "question mark".to_string(),
        TileState::Revealed => {
            if game.mines[idx] {
                "mine".to_string()
            } else {
                let n = game.neighbors[idx];
                if n == 0 {
                    "empty".to_string()
                } else if n == 1 {
                    "1 mine nearby".to_string()
                } else {
                    format!("{n} mines nearby")
                }
            }
        }
    };
    format!("Tile {position}, {state}")
}

/// Actions dispatched to the game reducer. Routing clicks through actions
/// (instead of capturing the rendered `Game` in the click handler) lets the
/// reducer always see the latest board state, so two clicks delivered before
/// the next render are both applied in order.
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

fn app(cx: &mut RenderCx) -> Element {
    let (game, dispatch) = cx.use_reducer_fn(reduce, Game::new());

    let reset_handler = {
        let d = dispatch.clone();
        move || d.call(Action::Reset)
    };

    let reveal_handler = make_reveal_handler(dispatch.clone());
    let flag_handler = make_flag_handler(dispatch);

    let header = vstack((
        text_block(status_line(&game))
            .bold()
            .font_size(20.0)
            .horizontal_alignment(HorizontalAlignment::Center),
        button("New Game")
            .on_click(reset_handler)
            .horizontal_alignment(HorizontalAlignment::Center),
    ))
    .spacing(8.0)
    .margin(Thickness {
        top: 12.0,
        bottom: 4.0,
        ..Thickness::default()
    });

    let board = build_board(&game, reveal_handler, flag_handler);

    let title_bar = TitleBar::new("windows_reactor — minesweeper");
    vstack((title_bar, vstack((header, board)).spacing(12.0))).into()
}

fn build_board(
    game: &Game,
    reveal_handler: impl Fn(usize, usize) + Clone + 'static,
    flag_handler: impl Fn(usize, usize) + Clone + 'static,
) -> Element {
    let cells = build_cells(game, reveal_handler, flag_handler);
    grid(cells)
        .rows([GridLength::STAR; HEIGHT])
        .columns([GridLength::STAR; WIDTH])
        .row_spacing(2.0)
        .column_spacing(2.0)
        .width(420.0)
        .height(420.0)
        .horizontal_alignment(HorizontalAlignment::Center)
        .into()
}

fn build_cells(
    game: &Game,
    reveal_handler: impl Fn(usize, usize) + Clone + 'static,
    flag_handler: impl Fn(usize, usize) + Clone + 'static,
) -> Vec<Element> {
    let game_over = game.status != Status::Playing;
    (0..TOTAL)
        .map(|idx| {
            let (x, y) = Game::xy(idx);
            let tile = game.tiles[idx];
            let label = tile_label(game, idx);
            let automation = tile_automation_name(game, idx);
            let reveal = reveal_handler.clone();
            let flag = flag_handler.clone();

            let mut btn = button(label)
                .on_click(move || reveal(x, y))
                .with_key(format!("tile-{idx}"))
                .horizontal_alignment(HorizontalAlignment::Stretch)
                .vertical_alignment(VerticalAlignment::Stretch);

            // Pick a foreground that matches the classic palette for number
            // tiles, and a red highlight for the mine that ended the game.
            if tile == TileState::Revealed && !game.mines[idx] && game.neighbors[idx] > 0 {
                btn = btn.foreground(number_color(game.neighbors[idx]));
            }
            if game.hit_mine == Some(idx) {
                btn = btn.background(Color::rgb(220, 80, 80));
            }

            // Disable every tile once the game ends, and any tile that has
            // already been revealed (revealed tiles, including 0-neighbour
            // blanks, carry no further interaction).
            let is_inert_revealed = tile == TileState::Revealed;
            if game_over || is_inert_revealed {
                btn = btn.enabled(false);
            }

            btn.automation_name(automation)
                .on_right_tapped(move || flag(x, y))
                .grid_row(y as i32)
                .grid_column(x as i32)
                .into()
        })
        .collect()
}

fn make_reveal_handler(dispatch: Dispatch<Action>) -> impl Fn(usize, usize) + Clone + 'static {
    move |x, y| dispatch.call(Action::Reveal(x, y))
}

fn make_flag_handler(dispatch: Dispatch<Action>) -> impl Fn(usize, usize) + Clone + 'static {
    move |x, y| dispatch.call(Action::Flag(x, y))
}

fn main() -> Result<()> {
    App::new()
        .title("windows_reactor — minesweeper")
        .render(app)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh(seed: u64) -> Game {
        Game::new_seeded(seed)
    }

    #[test]
    fn new_game_is_hidden_and_playing() {
        let g = fresh(1);
        assert_eq!(g.status, Status::Playing);
        assert!(!g.generated);
        assert!(g.tiles.iter().all(|t| *t == TileState::Hidden));
        assert_eq!(g.flag_count(), 0);
    }

    #[test]
    fn first_reveal_is_never_a_mine() {
        // Try across many seeds; the first revealed tile must not be a mine
        // for any of them.
        for seed in 1..200u64 {
            let g = fresh(seed);
            let g = apply_reveal(&g, 4, 4).expect("legal reveal");
            assert!(g.generated);
            assert!(
                !g.mines[Game::index(4, 4)],
                "seed {seed} placed mine on first click"
            );
            assert_eq!(g.tiles[Game::index(4, 4)], TileState::Revealed);
        }
    }

    #[test]
    fn generates_exactly_mines_count() {
        let g = apply_reveal(&fresh(42), 0, 0).expect("legal");
        assert_eq!(g.mines.iter().filter(|m| **m).count(), MINES);
    }

    #[test]
    fn flag_cycle_hidden_flag_question_hidden() {
        let g = fresh(7);
        let g = apply_flag(&g, 0, 0).unwrap();
        assert_eq!(g.tiles[0], TileState::Flag);
        let g = apply_flag(&g, 0, 0).unwrap();
        assert_eq!(g.tiles[0], TileState::Question);
        let g = apply_flag(&g, 0, 0).unwrap();
        assert_eq!(g.tiles[0], TileState::Hidden);
    }

    #[test]
    fn flagged_tile_cannot_be_revealed() {
        let g = fresh(7);
        let g = apply_flag(&g, 3, 3).unwrap();
        assert_eq!(g.tiles[Game::index(3, 3)], TileState::Flag);
        assert!(apply_reveal(&g, 3, 3).is_none());
    }

    #[test]
    fn revealed_tile_cannot_be_flagged_or_revealed_again() {
        let g = apply_reveal(&fresh(7), 0, 0).expect("legal");
        // The corner is now revealed (and safe because of first-click safety).
        assert_eq!(g.tiles[0], TileState::Revealed);
        assert!(apply_flag(&g, 0, 0).is_none());
        assert!(apply_reveal(&g, 0, 0).is_none());
    }

    #[test]
    fn bfs_flood_reveals_connected_zero_region() {
        // A 9×9 board with 10 mines has plenty of 0-neighbour tiles, so a
        // first reveal must expand into more than one tile for at least
        // some seed/position combinations. Try several until we find one.
        let mut found = false;
        'outer: for seed in 1..200u64 {
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    let g = apply_reveal(&fresh(seed), x, y).expect("legal");
                    let revealed = g
                        .tiles
                        .iter()
                        .filter(|t| **t == TileState::Revealed)
                        .count();
                    if revealed > 1 {
                        // All revealed tiles must be non-mines.
                        for i in 0..TOTAL {
                            if g.tiles[i] == TileState::Revealed {
                                assert!(!g.mines[i]);
                            }
                        }
                        found = true;
                        break 'outer;
                    }
                }
            }
        }
        assert!(found, "no seed/position produced a flood-fill expansion");
    }

    #[test]
    fn revealing_a_mine_loses_and_reveals_all_mines() {
        // Seed and generate so we know mine positions, then click one.
        let mut g = apply_reveal(&fresh(9), 0, 0).expect("legal");
        let mine_idx = (0..TOTAL).find(|i| g.mines[*i]).expect("must have a mine");
        let (mx, my) = Game::xy(mine_idx);
        // Clear any flag the BFS-less path didn't set, just in case.
        g.tiles[mine_idx] = TileState::Hidden;
        let g = apply_reveal(&g, mx, my).expect("legal click on hidden mine");
        assert_eq!(g.status, Status::Lost);
        assert_eq!(g.hit_mine, Some(mine_idx));
        for i in 0..TOTAL {
            if g.mines[i] {
                assert_eq!(
                    g.tiles[i],
                    TileState::Revealed,
                    "mine {i} should be revealed on loss"
                );
            }
        }
    }

    #[test]
    fn no_interactions_after_game_over() {
        let mut g = apply_reveal(&fresh(9), 0, 0).expect("legal");
        let mine_idx = (0..TOTAL).find(|i| g.mines[*i]).expect("must have a mine");
        let (mx, my) = Game::xy(mine_idx);
        g.tiles[mine_idx] = TileState::Hidden;
        let g = apply_reveal(&g, mx, my).expect("legal");
        assert_eq!(g.status, Status::Lost);

        // Any further action is a no-op.
        assert!(apply_reveal(&g, 0, 0).is_none());
        assert!(apply_flag(&g, 0, 0).is_none());
    }

    #[test]
    fn revealing_every_non_mine_wins() {
        // Force-reveal every non-mine tile via the engine and assert that
        // the status flips to Won.
        let g = apply_reveal(&fresh(11), 0, 0).expect("legal");
        let mut g = g;
        // Reveal every still-hidden non-mine tile.
        for i in 0..TOTAL {
            if !g.mines[i] && g.tiles[i] != TileState::Revealed {
                let (x, y) = Game::xy(i);
                // Clear any leftover flag/question first.
                if g.tiles[i] != TileState::Hidden {
                    g.tiles[i] = TileState::Hidden;
                }
                if let Some(n) = apply_reveal(&g, x, y) {
                    g = n;
                }
            }
        }
        assert_eq!(g.status, Status::Won);
    }

    #[test]
    fn out_of_bounds_clicks_are_noop() {
        let g = fresh(1);
        assert!(apply_reveal(&g, WIDTH, 0).is_none());
        assert!(apply_reveal(&g, 0, HEIGHT).is_none());
        assert!(apply_reveal(&g, usize::MAX, usize::MAX).is_none());
        assert!(apply_flag(&g, WIDTH, 0).is_none());
        assert!(apply_flag(&g, 0, HEIGHT).is_none());
    }

    #[test]
    fn xorshift_is_not_constant() {
        let mut r = XorShift64::new(1);
        let a = r.next_u64();
        let b = r.next_u64();
        let c = r.next_u64();
        assert_ne!(a, b);
        assert_ne!(b, c);
    }
}
