#![windows_subsystem = "windows"]

//! `dotsweeper` — a Reactor port of classic Minesweeper.
//!
//! Pure game logic (`Board`, `reveal`, `toggle_flag`, `chord`, `new_game`)
//! lives below in plain Rust. The view wires state to Reactor DSL via
//! `use_reducer` + a `DispatcherTimer` for the elapsed-seconds counter.
//!
//! Input: left-tap reveals/chords, right-tap cycles marks, right-press
//! on a revealed cell shows chord preview (release commits, drag cancels).

use std::rc::Rc;
use std::time::Duration;

use windows_reactor::*;

// ─── Difficulty ────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum DifficultyKind {
    Beginner,
    Intermediate,
    Expert,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Difficulty {
    kind: DifficultyKind,
    rows: usize,
    cols: usize,
    mines: usize,
}

impl Difficulty {
    const BEGINNER: Difficulty = Difficulty {
        kind: DifficultyKind::Beginner,
        rows: 9,
        cols: 9,
        mines: 10,
    };
    const INTERMEDIATE: Difficulty = Difficulty {
        kind: DifficultyKind::Intermediate,
        rows: 16,
        cols: 16,
        mines: 40,
    };
    const EXPERT: Difficulty = Difficulty {
        kind: DifficultyKind::Expert,
        rows: 16,
        cols: 30,
        mines: 99,
    };

    fn display_name(self) -> &'static str {
        match self.kind {
            DifficultyKind::Beginner => "Beginner",
            DifficultyKind::Intermediate => "Intermediate",
            DifficultyKind::Expert => "Expert",
        }
    }
}

// ─── Cells & board ────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CellMark {
    None,
    Flag,
    Question,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct BoardCell {
    is_mine: bool,
    adjacent_mines: u8,
    is_revealed: bool,
    mark: CellMark,
}

impl BoardCell {
    const EMPTY_HIDDEN: BoardCell = BoardCell {
        is_mine: false,
        adjacent_mines: 0,
        is_revealed: false,
        mark: CellMark::None,
    };
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum GamePhase {
    NotStarted,
    Playing,
    Won,
    Lost,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Board {
    difficulty: Difficulty,
    cells: Vec<BoardCell>,
    phase: GamePhase,
    exploded_at: Option<(usize, usize)>,
    revealed_safe: usize,
    flag_count: usize,
}

impl Board {
    fn new_game(difficulty: Difficulty) -> Self {
        Board {
            cells: vec![BoardCell::EMPTY_HIDDEN; difficulty.rows * difficulty.cols],
            difficulty,
            phase: GamePhase::NotStarted,
            exploded_at: None,
            revealed_safe: 0,
            flag_count: 0,
        }
    }

    fn rows(&self) -> usize {
        self.difficulty.rows
    }
    fn cols(&self) -> usize {
        self.difficulty.cols
    }
    fn idx(&self, r: usize, c: usize) -> usize {
        r * self.cols() + c
    }
    fn cell(&self, r: usize, c: usize) -> BoardCell {
        self.cells[self.idx(r, c)]
    }
    fn in_bounds(&self, r: isize, c: isize) -> bool {
        r >= 0 && c >= 0 && (r as usize) < self.rows() && (c as usize) < self.cols()
    }
    fn mines_remaining(&self) -> i32 {
        self.difficulty.mines as i32 - self.flag_count as i32
    }
    fn total_safe_cells(&self) -> usize {
        self.rows() * self.cols() - self.difficulty.mines
    }

    fn neighbors(&self, r: usize, c: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        let r = r as isize;
        let c = c as isize;
        (-1..=1).flat_map(move |dr| {
            (-1..=1).filter_map(move |dc| {
                if dr == 0 && dc == 0 {
                    return None;
                }
                let nr = r + dr;
                let nc = c + dc;
                if self.in_bounds(nr, nc) {
                    Some((nr as usize, nc as usize))
                } else {
                    None
                }
            })
        })
    }
}

// ─── Pure reducer functions ───────────────────────────────────────────

/// Reveal `(row, col)`. On the first reveal of a fresh board, mines are
/// placed avoiding the click and its 8 neighbors (first-click safety).
/// Reveals cascade through empty (zero-adjacent) regions. Hitting a mine
/// transitions to `GamePhase::Lost`; clearing the last safe cell
/// transitions to `GamePhase::Won`.
fn reveal(mut board: Board, row: usize, col: usize, rng: &mut Lcg) -> Board {
    if matches!(board.phase, GamePhase::Won | GamePhase::Lost) {
        return board;
    }
    if row >= board.rows() || col >= board.cols() {
        return board;
    }
    let c = board.cell(row, col);
    if c.is_revealed || c.mark == CellMark::Flag {
        return board;
    }

    if board.phase == GamePhase::NotStarted {
        board = place_mines_avoiding(board, row, col, rng);
    }

    if board.cell(row, col).is_mine {
        return reveal_all_mines(board, row, col);
    }

    cascade_reveal(board, row, col)
}

/// Cycle the mark on a hidden cell: None → Flag → Question → None.
fn toggle_flag(mut board: Board, row: usize, col: usize) -> Board {
    if matches!(board.phase, GamePhase::Won | GamePhase::Lost) {
        return board;
    }
    if row >= board.rows() || col >= board.cols() {
        return board;
    }
    let idx = board.idx(row, col);
    let cell = board.cells[idx];
    if cell.is_revealed {
        return board;
    }

    let next_mark = match cell.mark {
        CellMark::None => CellMark::Flag,
        CellMark::Flag => CellMark::Question,
        CellMark::Question => CellMark::None,
    };

    let flag_delta = (next_mark == CellMark::Flag) as i32 - (cell.mark == CellMark::Flag) as i32;
    board.cells[idx].mark = next_mark;
    board.flag_count = (board.flag_count as i32 + flag_delta).max(0) as usize;
    board
}

/// Chord reveal at a revealed numbered cell: if the count of flagged
/// neighbors equals the cell's `adjacent_mines`, reveal every
/// non-flagged neighbor.
fn chord(mut board: Board, row: usize, col: usize, rng: &mut Lcg) -> Board {
    if board.phase != GamePhase::Playing {
        return board;
    }
    if row >= board.rows() || col >= board.cols() {
        return board;
    }
    let cell = board.cell(row, col);
    if !cell.is_revealed || cell.is_mine || cell.adjacent_mines == 0 {
        return board;
    }

    let adj_flags = board
        .neighbors(row, col)
        .filter(|&(nr, nc)| board.cell(nr, nc).mark == CellMark::Flag)
        .count() as u8;
    if adj_flags != cell.adjacent_mines {
        return board;
    }

    let targets: Vec<(usize, usize)> = board
        .neighbors(row, col)
        .filter(|&(nr, nc)| {
            let n = board.cell(nr, nc);
            !n.is_revealed && n.mark != CellMark::Flag
        })
        .collect();
    for (nr, nc) in targets {
        board = reveal(board, nr, nc, rng);
        if board.phase == GamePhase::Lost {
            return board;
        }
    }
    board
}

fn place_mines_avoiding(
    mut board: Board,
    safe_row: usize,
    safe_col: usize,
    rng: &mut Lcg,
) -> Board {
    let total = board.rows() * board.cols();
    let mut forbidden = vec![false; total];
    for dr in -1_isize..=1 {
        for dc in -1_isize..=1 {
            let r = safe_row as isize + dr;
            let c = safe_col as isize + dc;
            if board.in_bounds(r, c) {
                forbidden[r as usize * board.cols() + c as usize] = true;
            }
        }
    }

    let mut candidates: Vec<usize> = (0..total).filter(|&i| !forbidden[i]).collect();
    let picks = board.difficulty.mines.min(candidates.len());

    // Partial Fisher–Yates: shuffle the first `picks` slots.
    for i in 0..picks {
        let j = rng.range(i, candidates.len() - 1);
        candidates.swap(i, j);
    }

    let cols = board.cols();
    // Preserve any pre-first-click marks; only set mine bits and recompute counts.
    let mut cells = board.cells.clone();
    if cells.len() != total {
        cells = vec![BoardCell::EMPTY_HIDDEN; total];
    }
    for cell in &mut cells {
        cell.is_mine = false;
        cell.adjacent_mines = 0;
    }
    for &idx in &candidates[..picks] {
        cells[idx].is_mine = true;
    }

    // Adjacency counts.
    for r in 0..board.rows() {
        for c in 0..cols {
            let i = r * cols + c;
            if cells[i].is_mine {
                continue;
            }
            let mut count = 0_u8;
            for dr in -1_isize..=1 {
                for dc in -1_isize..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr >= 0
                        && nc >= 0
                        && (nr as usize) < board.rows()
                        && (nc as usize) < cols
                        && cells[nr as usize * cols + nc as usize].is_mine
                    {
                        count += 1;
                    }
                }
            }
            cells[i].adjacent_mines = count;
        }
    }

    board.cells = cells;
    board.phase = GamePhase::Playing;
    board
}

fn cascade_reveal(mut board: Board, start_row: usize, start_col: usize) -> Board {
    let mut new_reveals = 0_usize;
    let mut queue: std::collections::VecDeque<(usize, usize)> = std::collections::VecDeque::new();
    queue.push_back((start_row, start_col));

    while let Some((r, c)) = queue.pop_front() {
        let idx = board.idx(r, c);
        let cell = board.cells[idx];
        if cell.is_revealed || cell.mark == CellMark::Flag || cell.is_mine {
            continue;
        }
        board.cells[idx] = BoardCell {
            is_revealed: true,
            mark: CellMark::None,
            ..cell
        };
        new_reveals += 1;

        if cell.adjacent_mines == 0 {
            for n in board.neighbors(r, c) {
                queue.push_back(n);
            }
        }
    }

    board.revealed_safe += new_reveals;
    if board.revealed_safe >= board.total_safe_cells() {
        board.phase = GamePhase::Won;
        // Auto-flag remaining mines so the mines-remaining display reads 0.
        for cell in &mut board.cells {
            if cell.is_mine && cell.mark != CellMark::Flag {
                cell.mark = CellMark::Flag;
                board.flag_count += 1;
            }
        }
    } else {
        board.phase = GamePhase::Playing;
    }
    board
}

fn reveal_all_mines(mut board: Board, exploded_row: usize, exploded_col: usize) -> Board {
    for cell in &mut board.cells {
        if cell.is_mine && cell.mark != CellMark::Flag {
            cell.is_revealed = true;
        }
    }
    board.phase = GamePhase::Lost;
    board.exploded_at = Some((exploded_row, exploded_col));
    board
}

// ─── RNG (LCG; no external crate) ─────────────────────────────────────

#[derive(Clone, Debug)]
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed.max(1) }
    }
    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.state
    }
    /// Inclusive range `[lo, hi]`.
    fn range(&mut self, lo: usize, hi: usize) -> usize {
        debug_assert!(hi >= lo);
        let span = (hi - lo + 1) as u64;
        lo + (self.next_u64() % span) as usize
    }
}

fn fresh_seed() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0xCAFE_BABE_DEAD_BEEF, |d| d.as_nanos() as u64)
}

/// Golden-ratio-derived constant used to perturb the RNG seed between
/// games so successive boards don't all share the same mine layout.
const RNG_SEED_INCREMENT: u64 = 0x9E37_79B9_7F4A_7C15;

// ─── App state + reducer ──────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq)]
struct AppState {
    board: Board,
    elapsed_seconds: u32,
    /// Center cell of an in-flight chord preview, set when the user
    /// right-presses a revealed numbered cell and cleared on release /
    /// pointer-exit. Drives the `CoveredPreview` skin on the 8 neighbors.
    chord_preview: Option<(usize, usize)>,
    rng_seed: u64,
}

impl AppState {
    fn initial(difficulty: Difficulty, seed: u64) -> Self {
        Self {
            board: Board::new_game(difficulty),
            elapsed_seconds: 0,
            chord_preview: None,
            rng_seed: seed,
        }
    }
}

// ─── Cell visuals ─────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CellSkin {
    Covered,
    /// Non-flagged covered cell rendered with a flattened, lighter
    /// background while the user is right-pressing the central revealed
    /// numbered cell of a chord — see `chord_preview` in `AppState`.
    CoveredPreview,
    Revealed,
    ExplodedMine,
    RevealedMine,
    WrongFlag,
}

fn compute_skin(cell: BoardCell, is_lost: bool, is_exploded: bool, is_preview: bool) -> CellSkin {
    if is_lost {
        if cell.is_mine && is_exploded {
            return CellSkin::ExplodedMine;
        }
        if cell.is_mine && cell.is_revealed && cell.mark != CellMark::Flag {
            return CellSkin::RevealedMine;
        }
        if !cell.is_mine && cell.mark == CellMark::Flag {
            return CellSkin::WrongFlag;
        }
    }
    if cell.is_revealed {
        CellSkin::Revealed
    } else if is_preview && cell.mark != CellMark::Flag {
        CellSkin::CoveredPreview
    } else {
        CellSkin::Covered
    }
}

fn cell_glyph(skin: CellSkin, cell: BoardCell) -> String {
    match skin {
        CellSkin::ExplodedMine | CellSkin::RevealedMine => "💣".to_string(),
        CellSkin::WrongFlag => "✗".to_string(),
        CellSkin::Revealed => {
            if cell.adjacent_mines > 0 {
                cell.adjacent_mines.to_string()
            } else {
                String::new()
            }
        }
        CellSkin::CoveredPreview | CellSkin::Covered => match cell.mark {
            CellMark::Flag => "🚩".to_string(),
            CellMark::Question => "?".to_string(),
            CellMark::None => String::new(),
        },
    }
}

/// Classic-palette number color. Tuned to read against `LayerFill` in
/// both light and dark themes. `adjacent_mines` is at most 8 on a
/// standard board; values >= 7 share a neutral light-gray.
fn number_color(n: u8) -> Color {
    match n {
        1 => Color::rgb(0x42, 0x9B, 0xE6),
        2 => Color::rgb(0x4C, 0xAF, 0x50),
        3 => Color::rgb(0xE5, 0x57, 0x4B),
        4 => Color::rgb(0x7E, 0x57, 0xC2),
        5 => Color::rgb(0xAB, 0x47, 0xBC),
        6 => Color::rgb(0x26, 0xA6, 0xA4),
        7 => Color::rgb(0x9E, 0x9E, 0x9E),
        // n >= 8 (max possible is 8 on a standard grid).
        _ => Color::rgb(0xBD, 0xBD, 0xBD),
    }
}

const EXPLOSION_RED: Color = Color::rgb(0xD3, 0x2F, 0x2F);
const COVERED_FILL: Color = Color::rgb(0xB0, 0xB0, 0xB0);
const COVERED_PREVIEW_FILL: Color = Color::rgb(0xD8, 0xD8, 0xD8);

fn cell_size(difficulty: Difficulty) -> f64 {
    match difficulty.kind {
        DifficultyKind::Beginner => 36.0,
        DifficultyKind::Intermediate => 30.0,
        DifficultyKind::Expert => 26.0,
    }
}

fn smiley(phase: GamePhase) -> &'static str {
    match phase {
        GamePhase::Won => "😎",
        GamePhase::Lost => "😵",
        _ => "🙂",
    }
}

// ─── View helpers ─────────────────────────────────────────────────────

/// Per-cell input handlers, passed down so the cell view can wire the
/// pointer events without owning the reducer.
#[derive(Clone)]
struct CellHandlers {
    on_reveal: Rc<dyn Fn(usize, usize)>,
    on_flag: Rc<dyn Fn(usize, usize)>,
    on_chord: Rc<dyn Fn(usize, usize)>,
    on_begin_chord_preview: Rc<dyn Fn(usize, usize)>,
    on_end_chord_preview: Rc<dyn Fn(bool)>,
}

fn build_cell(
    board: &Board,
    chord_preview_center: Option<(usize, usize)>,
    r: usize,
    c: usize,
    size: f64,
    handlers: CellHandlers,
) -> Element {
    let cell = board.cell(r, c);
    let is_exploded = board.exploded_at == Some((r, c));
    let is_lost = board.phase == GamePhase::Lost;
    let is_won = board.phase == GamePhase::Won;
    let is_preview = match chord_preview_center {
        Some((pr, pc)) => {
            (r as isize - pr as isize).abs() <= 1 && (c as isize - pc as isize).abs() <= 1
        }
        None => false,
    };
    let skin = compute_skin(cell, is_lost, is_exploded, is_preview);
    let glyph = cell_glyph(skin, cell);

    let fg: BrushBinding = match skin {
        CellSkin::ExplodedMine => Color::rgb(0xFF, 0xFF, 0xFF).into(),
        CellSkin::WrongFlag => EXPLOSION_RED.into(),
        CellSkin::Revealed if cell.adjacent_mines > 0 => number_color(cell.adjacent_mines).into(),
        CellSkin::Covered | CellSkin::CoveredPreview if cell.mark == CellMark::Question => {
            ThemeRef::Accent.into()
        }
        _ => ThemeRef::PrimaryText.into(),
    };
    let bg: BrushBinding = match skin {
        CellSkin::ExplodedMine => EXPLOSION_RED.into(),
        CellSkin::RevealedMine | CellSkin::WrongFlag => ThemeRef::SubtleFill.into(),
        CellSkin::Revealed => ThemeRef::LayerFill.into(),
        CellSkin::CoveredPreview => COVERED_PREVIEW_FILL.into(),
        CellSkin::Covered => COVERED_FILL.into(),
    };

    let chordable_here = cell.is_revealed && cell.adjacent_mines > 0;
    let covered_here = !cell.is_revealed && cell.mark != CellMark::Flag;

    // Press: right-press on a revealed number → begin chord preview.
    let pp_handler = {
        let begin = handlers.on_begin_chord_preview.clone();
        move |info: PointerEventInfo| {
            if is_lost || is_won {
                return;
            }
            if info.is_right_button_pressed && chordable_here {
                begin(r, c);
            }
        }
    };

    // Release: commit chord only when right button is actually released.
    let pr_handler = {
        let end = handlers.on_end_chord_preview.clone();
        move |info: PointerEventInfo| {
            if is_lost || is_won {
                return;
            }
            if !info.is_right_button_pressed {
                end(true);
            }
        }
    };

    // Exit: pointer leaves the cell while a preview is in flight → cancel.
    let pe_handler = {
        let end = handlers.on_end_chord_preview.clone();
        move || end(false)
    };

    // Left-tap: chord shortcut on revealed numbers, otherwise reveal.
    let tap_handler = {
        let reveal_cb = handlers.on_reveal.clone();
        let chord_cb = handlers.on_chord.clone();
        move || {
            if is_lost || is_won {
                return;
            }
            if chordable_here {
                chord_cb(r, c);
            } else if !cell.is_revealed {
                reveal_cb(r, c);
            }
        }
    };

    // Right-tap: flag toggle on covered cells (swallow if chord just resolved).
    let rtap_handler = {
        let flag_cb = handlers.on_flag.clone();
        move || {
            if is_lost || is_won || chordable_here {
                return;
            }
            if covered_here || matches!(cell.mark, CellMark::Flag | CellMark::Question) {
                flag_cb(r, c);
            }
        }
    };

    let btn = button(glyph)
        .with_key(format!("cell-{r}-{c}"))
        .width(size)
        .height(size)
        .horizontal_alignment(HorizontalAlignment::Stretch)
        .vertical_alignment(VerticalAlignment::Stretch)
        .foreground(fg)
        .background(bg)
        .on_tapped(tap_handler)
        .on_right_tapped(rtap_handler)
        .on_pointer_pressed(pp_handler)
        .on_pointer_released(pr_handler)
        .on_pointer_exited(pe_handler);

    btn.grid_row(r as i32).grid_column(c as i32).into()
}

fn build_board(
    board: &Board,
    chord_preview_center: Option<(usize, usize)>,
    handlers: CellHandlers,
) -> Element {
    let size = cell_size(board.difficulty);
    let rows = board.rows();
    let cols = board.cols();

    let children: Vec<Element> = (0..(rows * cols))
        .map(|pos| {
            let r = pos / cols;
            let c = pos % cols;
            build_cell(board, chord_preview_center, r, c, size, handlers.clone())
        })
        .collect();

    let mut g = grid(children);
    let mut row_tracks: Vec<GridLength> = Vec::with_capacity(rows);
    for _ in 0..rows {
        row_tracks.push(GridLength::Pixel(size));
    }
    let mut col_tracks: Vec<GridLength> = Vec::with_capacity(cols);
    for _ in 0..cols {
        col_tracks.push(GridLength::Pixel(size));
    }
    g = g.rows(row_tracks).columns(col_tracks);

    border(g)
        .background(ThemeRef::LayerFill)
        .padding(Thickness::uniform(1.0))
        .horizontal_alignment(HorizontalAlignment::Center)
        .into()
}

/// Red-on-black three-digit LED-style display. Negative values render as
/// `-NN` (sign + 2 digits) to fit in the 3-character slot.
fn led_display(value: i32) -> Element {
    let clamped = value.clamp(-99, 999);
    let text = if clamped < 0 {
        format!("-{:02}", -clamped)
    } else {
        format!("{clamped:03}")
    };
    border(
        text_block(text)
            .font_size(22.0)
            .bold()
            .foreground(Color::rgb(0xFF, 0x3B, 0x30))
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center),
    )
    .background(Color::rgb(0x1A, 0x00, 0x00))
    .padding(Thickness::xy(8.0, 2.0))
    .min_width(64.0)
    .into()
}

fn status_subtitle(state: &AppState) -> String {
    match state.board.phase {
        GamePhase::Won => format!("You won in {}s!", state.elapsed_seconds),
        GamePhase::Lost => "Boom! Press 🙂 to try again.".to_string(),
        GamePhase::Playing => format!(
            "{} — {}s",
            state.board.difficulty.display_name(),
            state.elapsed_seconds
        ),
        GamePhase::NotStarted => state.board.difficulty.display_name().to_string(),
    }
}

// ─── The component ────────────────────────────────────────────────────

fn app(cx: &mut RenderCx) -> Element {
    let (state, update) = cx.use_reducer(AppState::initial(Difficulty::BEGINNER, fresh_seed()));

    // Left-tap on a covered cell → reveal.
    let on_reveal: Rc<dyn Fn(usize, usize)> = {
        let u = update.clone();
        Rc::new(move |r: usize, c: usize| {
            u.call(move |mut s| {
                let mut rng = Lcg::new(s.rng_seed);
                s.board = reveal(s.board, r, c, &mut rng);
                s.rng_seed = rng.state;
                s.chord_preview = None;
                s
            });
        })
    };

    // Right-tap on a covered cell → cycle the flag/question mark.
    let on_flag: Rc<dyn Fn(usize, usize)> = {
        let u = update.clone();
        Rc::new(move |r: usize, c: usize| {
            u.call(move |mut s| {
                s.board = toggle_flag(s.board, r, c);
                s
            });
        })
    };

    // Left-tap on a revealed number → chord.
    let on_chord: Rc<dyn Fn(usize, usize)> = {
        let u = update.clone();
        Rc::new(move |r: usize, c: usize| {
            u.call(move |mut s| {
                let mut rng = Lcg::new(s.rng_seed);
                s.board = chord(s.board, r, c, &mut rng);
                s.rng_seed = rng.state;
                s.chord_preview = None;
                s
            });
        })
    };

    // Right-press on a revealed number → start chord preview.
    let on_begin_chord_preview: Rc<dyn Fn(usize, usize)> = {
        let u = update.clone();
        Rc::new(move |r: usize, c: usize| {
            u.call(move |mut s| {
                s.chord_preview = Some((r, c));
                s
            });
        })
    };

    // Pointer release / exit → commit the preview as a chord if
    // `commit` is true (release inside the cell), otherwise just
    // cancel it.
    let on_end_chord_preview: Rc<dyn Fn(bool)> = {
        let u = update.clone();
        Rc::new(move |commit: bool| {
            u.call(move |mut s| {
                if let Some((r, c)) = s.chord_preview.take()
                    && commit
                {
                    let mut rng = Lcg::new(s.rng_seed);
                    s.board = chord(s.board, r, c, &mut rng);
                    s.rng_seed = rng.state;
                }
                s
            });
        })
    };

    let on_reset = {
        let u = update.clone();
        move || {
            u.call(move |s| {
                let diff = s.board.difficulty;
                AppState {
                    board: Board::new_game(diff),
                    elapsed_seconds: 0,
                    chord_preview: None,
                    rng_seed: s.rng_seed.wrapping_add(RNG_SEED_INCREMENT),
                }
            });
        }
    };

    let on_new_game = {
        let u = update.clone();
        move |d: Difficulty| {
            u.call(move |s| AppState {
                board: Board::new_game(d),
                elapsed_seconds: 0,
                chord_preview: None,
                rng_seed: s.rng_seed.wrapping_add(RNG_SEED_INCREMENT),
            });
        }
    };

    // ── Timer effect ───────────────────────────────────────────────
    let should_tick = state.board.phase == GamePhase::Playing;
    let timer_update = update;
    cx.use_effect_with_cleanup((should_tick,), move || {
        if !should_tick {
            return None;
        }
        let u = timer_update;
        let timer = DispatcherTimer::new(Duration::from_secs(1), move || {
            u.call(|mut s| {
                if s.board.phase == GamePhase::Playing {
                    s.elapsed_seconds = s.elapsed_seconds.saturating_add(1).min(999);
                }
                s
            });
        })
        .ok();
        // Drop on cleanup stops the timer.
        Some(move || drop(timer))
    });

    // ── Header: mines-remaining LED · 🙂 · timer LED ────────────────
    let smiley_button = button(smiley(state.board.phase))
        .on_click(on_reset)
        .width(56.0)
        .height(40.0);

    let status_panel = hstack((
        led_display(state.board.mines_remaining()).horizontal_alignment(HorizontalAlignment::Left),
        smiley_button.horizontal_alignment(HorizontalAlignment::Center),
        led_display(state.elapsed_seconds as i32).horizontal_alignment(HorizontalAlignment::Right),
    ))
    .spacing(16.0)
    .padding(Thickness::xy(12.0, 8.0))
    .horizontal_alignment(HorizontalAlignment::Center);

    let status_card = border(status_panel)
        .background(ThemeRef::LayerFill)
        .padding(Thickness::xy(2.0, 2.0))
        .horizontal_alignment(HorizontalAlignment::Center);

    // ── Toolbar: difficulty buttons ────────────────────────────────
    let mk_diff = |label: &'static str, d: Difficulty| -> Element {
        let h = on_new_game.clone();
        let mut b = button(label).on_click(move || h(d));
        if state.board.difficulty.kind == d.kind {
            b = b.foreground(ThemeRef::Accent);
        }
        b.into()
    };

    let toolbar = hstack((
        mk_diff("Beginner", Difficulty::BEGINNER),
        mk_diff("Intermediate", Difficulty::INTERMEDIATE),
        mk_diff("Expert", Difficulty::EXPERT),
    ))
    .spacing(8.0)
    .horizontal_alignment(HorizontalAlignment::Center);

    // ── Board ──────────────────────────────────────────────────────
    let cell_handlers = CellHandlers {
        on_reveal,
        on_flag,
        on_chord,
        on_begin_chord_preview,
        on_end_chord_preview,
    };
    let board_view = build_board(&state.board, state.chord_preview, cell_handlers);

    // ── Compose ────────────────────────────────────────────────────
    let title_bar = TitleBar::new("windows_reactor — dotsweeper").subtitle(status_subtitle(&state));

    vstack((
        title_bar,
        vstack((
            status_card.margin(Thickness {
                top: 12.0,
                bottom: 4.0,
                ..Thickness::default()
            }),
            toolbar,
            board_view.margin(Thickness {
                top: 8.0,
                bottom: 12.0,
                ..Thickness::default()
            }),
        ))
        .spacing(8.0),
    ))
    .into()
}

fn main() -> Result<()> {
    bootstrap()?;
    App::new().title("windows_reactor — dotsweeper").render(app)
}

// ─── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn rng() -> Lcg {
        Lcg::new(0xDEAD_BEEF)
    }

    #[test]
    fn new_game_has_no_mines_no_reveals_no_flags() {
        let b = Board::new_game(Difficulty::BEGINNER);
        assert_eq!(b.phase, GamePhase::NotStarted);
        assert_eq!(b.revealed_safe, 0);
        assert_eq!(b.flag_count, 0);
        assert!(b.cells.iter().all(|c| !c.is_mine && !c.is_revealed));
        assert_eq!(b.mines_remaining(), Difficulty::BEGINNER.mines as i32);
    }

    #[test]
    fn first_reveal_is_always_safe_and_clears_a_pocket() {
        for seed in 0..32_u64 {
            let mut r = Lcg::new(seed.wrapping_mul(RNG_SEED_INCREMENT).max(1));
            let b = Board::new_game(Difficulty::BEGINNER);
            let next = reveal(b, 4, 4, &mut r);
            // The clicked cell must be revealed and non-mine.
            let clicked = next.cell(4, 4);
            assert!(clicked.is_revealed, "seed={seed} click hidden");
            assert!(
                !clicked.is_mine,
                "seed={seed} clicked a mine on first reveal"
            );
            // First-click pocket: 8 neighbors also non-mine.
            for (nr, nc) in next.neighbors(4, 4) {
                assert!(!next.cell(nr, nc).is_mine, "seed={seed} neighbor is mine");
            }
            assert_eq!(next.phase, GamePhase::Playing);
        }
    }

    #[test]
    fn mine_count_matches_difficulty() {
        let mut r = rng();
        let b = reveal(Board::new_game(Difficulty::INTERMEDIATE), 0, 0, &mut r);
        let mines = b.cells.iter().filter(|c| c.is_mine).count();
        assert_eq!(mines, Difficulty::INTERMEDIATE.mines);
    }

    #[test]
    fn toggle_flag_cycles_none_flag_question_none() {
        let mut b = Board::new_game(Difficulty::BEGINNER);
        b = toggle_flag(b, 0, 0);
        assert_eq!(b.cell(0, 0).mark, CellMark::Flag);
        assert_eq!(b.flag_count, 1);
        b = toggle_flag(b, 0, 0);
        assert_eq!(b.cell(0, 0).mark, CellMark::Question);
        assert_eq!(b.flag_count, 0);
        b = toggle_flag(b, 0, 0);
        assert_eq!(b.cell(0, 0).mark, CellMark::None);
        assert_eq!(b.flag_count, 0);
    }

    #[test]
    fn flagged_cells_cannot_be_revealed() {
        let mut r = rng();
        let mut b = Board::new_game(Difficulty::BEGINNER);
        b = toggle_flag(b, 0, 0);
        let before = b.clone();
        b = reveal(b, 0, 0, &mut r);
        assert_eq!(b, before, "reveal on a flagged cell must be a no-op");
    }

    #[test]
    fn revealing_a_mine_loses_the_game() {
        let mut b = Board::new_game(Difficulty::BEGINNER);
        // Hand-place a single mine far from the click.
        b.phase = GamePhase::Playing;
        let mines_idx = b.idx(8, 8);
        b.cells[mines_idx].is_mine = true;
        let mut r = rng();
        let next = reveal(b, 8, 8, &mut r);
        assert_eq!(next.phase, GamePhase::Lost);
        assert_eq!(next.exploded_at, Some((8, 8)));
    }

    #[test]
    fn winning_auto_flags_remaining_mines() {
        // Tiny hand-built board: 2x2, 1 mine at (0,0). Reveal the three
        // safe cells and the game should be Won with the mine auto-flagged.
        let diff = Difficulty {
            kind: DifficultyKind::Beginner,
            rows: 2,
            cols: 2,
            mines: 1,
        };
        let mut b = Board::new_game(diff);
        b.phase = GamePhase::Playing;
        // (0,0) is the only mine; pre-fill adjacency counts so every
        // safe cell sees it as a 1-neighbor.
        b.cells[0].is_mine = true;
        b.cells[1].adjacent_mines = 1;
        b.cells[2].adjacent_mines = 1;
        b.cells[3].adjacent_mines = 1;

        let mut r = rng();
        b = reveal(b, 0, 1, &mut r);
        b = reveal(b, 1, 0, &mut r);
        b = reveal(b, 1, 1, &mut r);

        assert_eq!(b.phase, GamePhase::Won);
        assert_eq!(b.cell(0, 0).mark, CellMark::Flag);
        assert_eq!(b.flag_count, 1);
        assert_eq!(b.mines_remaining(), 0);
    }

    #[test]
    fn cascade_reveals_empty_region() {
        // 3x3, single mine in a corner — the opposite corner reveal
        // should cascade through every empty cell.
        let diff = Difficulty {
            kind: DifficultyKind::Beginner,
            rows: 3,
            cols: 3,
            mines: 1,
        };
        let mut b = Board::new_game(diff);
        b.phase = GamePhase::Playing;
        // (0,0) is the only mine; the three cells touching it have
        // adjacent_mines=1. The other 5 cells stay 0-adjacent so a
        // single reveal at (2,2) cascades through all of them and wins.
        b.cells[0].is_mine = true;
        b.cells[1].adjacent_mines = 1;
        b.cells[3].adjacent_mines = 1;
        b.cells[4].adjacent_mines = 1;
        let mut r = rng();
        b = reveal(b, 2, 2, &mut r);
        // All 8 non-mine cells revealed → Won.
        assert_eq!(b.phase, GamePhase::Won);
        let revealed: usize = b.cells.iter().filter(|c| c.is_revealed).count();
        assert_eq!(revealed, 8);
    }

    #[test]
    fn chord_reveals_neighbors_when_flag_count_matches() {
        // 3x3, single mine at (0,0). Reveal (1,1) so it shows "1", flag
        // (0,0), then chord on (1,1) → should reveal all the rest.
        let diff = Difficulty {
            kind: DifficultyKind::Beginner,
            rows: 3,
            cols: 3,
            mines: 1,
        };
        let mut b = Board::new_game(diff);
        b.phase = GamePhase::Playing;
        b.cells[0].is_mine = true;
        for i in [1, 3, 4] {
            b.cells[i].adjacent_mines = 1;
        }
        b.cells[4].is_revealed = true;
        b.revealed_safe = 1;
        b = toggle_flag(b, 0, 0);
        let mut r = rng();
        b = chord(b, 1, 1, &mut r);
        assert_eq!(b.phase, GamePhase::Won);
    }

    #[test]
    fn chord_with_wrong_flag_loses() {
        // Mine at (0,0). Flag (0,1) instead. Reveal (1,1) so it shows "1"
        // (the flag count matches), then chord — the un-flagged real mine
        // at (0,0) gets revealed and we lose.
        let diff = Difficulty {
            kind: DifficultyKind::Beginner,
            rows: 3,
            cols: 3,
            mines: 1,
        };
        let mut b = Board::new_game(diff);
        b.phase = GamePhase::Playing;
        b.cells[0].is_mine = true;
        for i in [1, 3, 4] {
            b.cells[i].adjacent_mines = 1;
        }
        b.cells[4].is_revealed = true;
        b.revealed_safe = 1;
        b = toggle_flag(b, 0, 1);
        let mut r = rng();
        b = chord(b, 1, 1, &mut r);
        assert_eq!(b.phase, GamePhase::Lost);
    }

    #[test]
    fn chord_is_noop_when_flag_count_does_not_match() {
        let diff = Difficulty {
            kind: DifficultyKind::Beginner,
            rows: 3,
            cols: 3,
            mines: 1,
        };
        let mut b = Board::new_game(diff);
        b.phase = GamePhase::Playing;
        b.cells[0].is_mine = true;
        for i in [1, 3, 4] {
            b.cells[i].adjacent_mines = 1;
        }
        b.cells[4].is_revealed = true;
        b.revealed_safe = 1;
        let before = b.clone();
        let mut r = rng();
        let after = chord(b, 1, 1, &mut r);
        assert_eq!(after, before);
    }

    #[test]
    fn actions_after_loss_are_ignored() {
        let mut b = Board::new_game(Difficulty::BEGINNER);
        b.phase = GamePhase::Lost;
        let before = b.clone();
        let mut r = rng();
        assert_eq!(reveal(b.clone(), 0, 0, &mut r), before);
        assert_eq!(toggle_flag(b.clone(), 0, 0), before);
        assert_eq!(chord(b, 0, 0, &mut r), before);
    }

    #[test]
    fn out_of_bounds_actions_are_ignored() {
        let b = Board::new_game(Difficulty::BEGINNER);
        let mut r = rng();
        let same = reveal(b.clone(), 99, 99, &mut r);
        assert_eq!(same, b);
    }
}

// (no module-level stubs)
