#![windows_subsystem = "windows"]

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use windows_reactor::{
    Application, Border, Button, Callback, Color, Element, Grid, GridChild, GridLength,
    HorizontalAlignment, PointerEvent, RenderCx, TextBlock, Thickness, VerticalAlignment, Window,
    WindowConstraints, hstack, vstack,
};

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
    columns: usize,
    mines: usize,
}

impl Difficulty {
    const BEGINNER: Self = Self {
        kind: DifficultyKind::Beginner,
        rows: 9,
        columns: 9,
        mines: 10,
    };
    const INTERMEDIATE: Self = Self {
        kind: DifficultyKind::Intermediate,
        rows: 16,
        columns: 16,
        mines: 40,
    };
    const EXPERT: Self = Self {
        kind: DifficultyKind::Expert,
        rows: 16,
        columns: 30,
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
    const EMPTY_HIDDEN: Self = Self {
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
        Self {
            cells: vec![BoardCell::EMPTY_HIDDEN; difficulty.rows * difficulty.columns],
            difficulty,
            phase: GamePhase::NotStarted,
            exploded_at: None,
            revealed_safe: 0,
            flag_count: 0,
        }
    }

    fn index(&self, row: usize, column: usize) -> usize {
        row * self.difficulty.columns + column
    }

    fn cell(&self, row: usize, column: usize) -> BoardCell {
        self.cells[self.index(row, column)]
    }

    fn in_bounds(&self, row: isize, column: isize) -> bool {
        row >= 0
            && column >= 0
            && (row as usize) < self.difficulty.rows
            && (column as usize) < self.difficulty.columns
    }

    fn neighbors(&self, row: usize, column: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        (-1..=1).flat_map(move |row_offset| {
            (-1..=1).filter_map(move |column_offset| {
                if row_offset == 0 && column_offset == 0 {
                    return None;
                }
                let neighbor_row = row as isize + row_offset;
                let neighbor_column = column as isize + column_offset;
                self.in_bounds(neighbor_row, neighbor_column)
                    .then_some((neighbor_row as usize, neighbor_column as usize))
            })
        })
    }

    fn mines_remaining(&self) -> i32 {
        self.difficulty.mines as i32 - self.flag_count as i32
    }

    fn total_safe_cells(&self) -> usize {
        self.cells.len() - self.difficulty.mines
    }
}

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

    fn range(&mut self, low: usize, high: usize) -> usize {
        low + (self.next_u64() % (high - low + 1) as u64) as usize
    }
}

fn place_mines_avoiding(
    mut board: Board,
    safe_row: usize,
    safe_column: usize,
    rng: &mut Lcg,
) -> Board {
    let mut forbidden = vec![false; board.cells.len()];
    for row_offset in -1..=1 {
        for column_offset in -1..=1 {
            let row = safe_row as isize + row_offset;
            let column = safe_column as isize + column_offset;
            if board.in_bounds(row, column) {
                let index = board.index(row as usize, column as usize);
                forbidden[index] = true;
            }
        }
    }

    let mut candidates = (0..board.cells.len())
        .filter(|index| !forbidden[*index])
        .collect::<Vec<_>>();
    let picks = board.difficulty.mines.min(candidates.len());
    for index in 0..picks {
        let swap = rng.range(index, candidates.len() - 1);
        candidates.swap(index, swap);
    }
    for index in candidates.into_iter().take(picks) {
        board.cells[index].is_mine = true;
    }

    for row in 0..board.difficulty.rows {
        for column in 0..board.difficulty.columns {
            let index = board.index(row, column);
            if board.cells[index].is_mine {
                continue;
            }
            board.cells[index].adjacent_mines = board
                .neighbors(row, column)
                .filter(|(neighbor_row, neighbor_column)| {
                    board.cell(*neighbor_row, *neighbor_column).is_mine
                })
                .count() as u8;
        }
    }
    board.phase = GamePhase::Playing;
    board
}

fn cascade_reveal(mut board: Board, row: usize, column: usize) -> Board {
    let mut queue = std::collections::VecDeque::from([(row, column)]);
    while let Some((row, column)) = queue.pop_front() {
        let index = board.index(row, column);
        let cell = board.cells[index];
        if cell.is_revealed || cell.mark == CellMark::Flag || cell.is_mine {
            continue;
        }
        board.cells[index] = BoardCell {
            is_revealed: true,
            mark: CellMark::None,
            ..cell
        };
        board.revealed_safe += 1;
        if cell.adjacent_mines == 0 {
            queue.extend(board.neighbors(row, column));
        }
    }

    if board.revealed_safe >= board.total_safe_cells() {
        board.phase = GamePhase::Won;
        for cell in &mut board.cells {
            if cell.is_mine && cell.mark != CellMark::Flag {
                cell.mark = CellMark::Flag;
                board.flag_count += 1;
            }
        }
    }
    board
}

fn reveal(mut board: Board, row: usize, column: usize, rng: &mut Lcg) -> Board {
    if matches!(board.phase, GamePhase::Won | GamePhase::Lost)
        || !board.in_bounds(row as isize, column as isize)
    {
        return board;
    }
    let cell = board.cell(row, column);
    if cell.is_revealed || cell.mark == CellMark::Flag {
        return board;
    }
    if board.phase == GamePhase::NotStarted {
        board = place_mines_avoiding(board, row, column, rng);
    }
    if board.cell(row, column).is_mine {
        for cell in &mut board.cells {
            if cell.is_mine && cell.mark != CellMark::Flag {
                cell.is_revealed = true;
            }
        }
        board.phase = GamePhase::Lost;
        board.exploded_at = Some((row, column));
        return board;
    }
    cascade_reveal(board, row, column)
}

fn toggle_flag(mut board: Board, row: usize, column: usize) -> Board {
    if matches!(board.phase, GamePhase::Won | GamePhase::Lost)
        || !board.in_bounds(row as isize, column as isize)
    {
        return board;
    }
    let index = board.index(row, column);
    let cell = board.cells[index];
    if cell.is_revealed {
        return board;
    }
    let mark = match cell.mark {
        CellMark::None => CellMark::Flag,
        CellMark::Flag => CellMark::Question,
        CellMark::Question => CellMark::None,
    };
    let flag_delta = (mark == CellMark::Flag) as i32 - (cell.mark == CellMark::Flag) as i32;
    board.cells[index].mark = mark;
    board.flag_count = (board.flag_count as i32 + flag_delta).max(0) as usize;
    board
}

fn chord(mut board: Board, row: usize, column: usize, rng: &mut Lcg) -> Board {
    if board.phase != GamePhase::Playing || !board.in_bounds(row as isize, column as isize) {
        return board;
    }
    let cell = board.cell(row, column);
    if !cell.is_revealed || cell.is_mine || cell.adjacent_mines == 0 {
        return board;
    }
    let flags = board
        .neighbors(row, column)
        .filter(|(neighbor_row, neighbor_column)| {
            board.cell(*neighbor_row, *neighbor_column).mark == CellMark::Flag
        })
        .count() as u8;
    if flags != cell.adjacent_mines {
        return board;
    }
    let targets = board
        .neighbors(row, column)
        .filter(|(neighbor_row, neighbor_column)| {
            let cell = board.cell(*neighbor_row, *neighbor_column);
            !cell.is_revealed && cell.mark != CellMark::Flag
        })
        .collect::<Vec<_>>();
    for (neighbor_row, neighbor_column) in targets {
        board = reveal(board, neighbor_row, neighbor_column, rng);
        if board.phase == GamePhase::Lost {
            break;
        }
    }
    board
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AppState {
    board: Board,
    elapsed_seconds: u32,
    chord_preview: Option<(usize, usize)>,
    rng_seed: u64,
}

impl AppState {
    fn initial() -> Self {
        Self {
            board: Board::new_game(Difficulty::BEGINNER),
            elapsed_seconds: 0,
            chord_preview: None,
            rng_seed: fresh_seed(),
        }
    }
}

enum Action {
    Reveal(usize, usize),
    Flag(usize, usize),
    Chord(usize, usize),
    BeginChordPreview(usize, usize),
    EndChordPreview(bool),
    NewGame(Difficulty),
    Tick,
}

fn reduce(mut state: AppState, action: Action) -> AppState {
    match action {
        Action::Reveal(row, column) => {
            let mut rng = Lcg::new(state.rng_seed);
            state.board = reveal(state.board, row, column, &mut rng);
            state.rng_seed = rng.state;
            state.chord_preview = None;
        }
        Action::Flag(row, column) => {
            state.board = toggle_flag(state.board, row, column);
        }
        Action::Chord(row, column) => {
            let mut rng = Lcg::new(state.rng_seed);
            state.board = chord(state.board, row, column, &mut rng);
            state.rng_seed = rng.state;
            state.chord_preview = None;
        }
        Action::BeginChordPreview(row, column) => {
            state.chord_preview = Some((row, column));
        }
        Action::EndChordPreview(commit) => {
            if let Some((row, column)) = state.chord_preview.take()
                && commit
            {
                let mut rng = Lcg::new(state.rng_seed);
                state.board = chord(state.board, row, column, &mut rng);
                state.rng_seed = rng.state;
            }
        }
        Action::NewGame(difficulty) => {
            state.board = Board::new_game(difficulty);
            state.elapsed_seconds = 0;
            state.chord_preview = None;
            state.rng_seed = state.rng_seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        }
        Action::Tick => {
            if state.board.phase == GamePhase::Playing {
                state.elapsed_seconds = state.elapsed_seconds.saturating_add(1).min(999);
            }
        }
    }
    state
}

fn fresh_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0xCAFE_BABE_DEAD_BEEF, |duration| duration.as_nanos() as u64)
}

fn cell_size(difficulty: Difficulty) -> f64 {
    match difficulty.kind {
        DifficultyKind::Beginner => 36.0,
        DifficultyKind::Intermediate => 30.0,
        DifficultyKind::Expert => 26.0,
    }
}

fn cell_glyph(cell: BoardCell, exploded: bool, lost: bool) -> String {
    if exploded || (lost && cell.is_mine && cell.is_revealed) {
        return "💣".to_string();
    }
    if lost && !cell.is_mine && cell.mark == CellMark::Flag {
        return "✗".to_string();
    }
    if cell.is_revealed {
        return match cell.adjacent_mines {
            0 => String::new(),
            count => count.to_string(),
        };
    }
    match cell.mark {
        CellMark::Flag => "🚩".to_string(),
        CellMark::Question => "?".to_string(),
        CellMark::None => String::new(),
    }
}

fn number_color(number: u8) -> Color {
    match number {
        1 => Color::rgb(0x42, 0x9B, 0xE6),
        2 => Color::rgb(0x4C, 0xAF, 0x50),
        3 => Color::rgb(0xE5, 0x57, 0x4B),
        4 => Color::rgb(0x7E, 0x57, 0xC2),
        5 => Color::rgb(0xAB, 0x47, 0xBC),
        6 => Color::rgb(0x26, 0xA6, 0xA4),
        _ => Color::rgb(0x9E, 0x9E, 0x9E),
    }
}

fn build_cell(
    state: &AppState,
    row: usize,
    column: usize,
    size: f64,
    dispatch: Callback<Action>,
) -> GridChild {
    let cell = state.board.cell(row, column);
    let lost = state.board.phase == GamePhase::Lost;
    let ended = matches!(state.board.phase, GamePhase::Won | GamePhase::Lost);
    let exploded = state.board.exploded_at == Some((row, column));
    let previewed = state
        .chord_preview
        .is_some_and(|(preview_row, preview_column)| {
            (row as isize - preview_row as isize).abs() <= 1
                && (column as isize - preview_column as isize).abs() <= 1
        });
    let chordable = cell.is_revealed && cell.adjacent_mines > 0;
    let covered = !cell.is_revealed && cell.mark != CellMark::Flag;

    let tapped = dispatch.clone();
    let right_tapped = dispatch.clone();
    let pressed = dispatch.clone();
    let released = dispatch.clone();
    let exited = dispatch;
    let mut button = Button::new(cell_glyph(cell, exploded, lost))
        .on_tapped(move || {
            if !ended {
                if chordable {
                    tapped.call(Action::Chord(row, column));
                } else if !cell.is_revealed {
                    tapped.call(Action::Reveal(row, column));
                }
            }
        })
        .on_right_tapped(move || {
            if !ended && covered {
                right_tapped.call(Action::Flag(row, column));
            }
        })
        .on_pointer_pressed(move |event: PointerEvent| {
            if !ended && event.is_right_button_pressed && chordable {
                pressed.call(Action::BeginChordPreview(row, column));
            }
        })
        .on_pointer_released(move |event: PointerEvent| {
            if !ended && !event.is_right_button_pressed {
                released.call(Action::EndChordPreview(true));
            }
        })
        .on_pointer_exited(move |_event: PointerEvent| {
            exited.call(Action::EndChordPreview(false));
        })
        .width(size)
        .height(size)
        .horizontal_alignment(HorizontalAlignment::Stretch)
        .vertical_alignment(VerticalAlignment::Stretch);

    if cell.is_revealed && cell.adjacent_mines > 0 {
        button = button.foreground(number_color(cell.adjacent_mines));
    } else if exploded {
        button = button.foreground(Color::rgb(0xD3, 0x2F, 0x2F));
    } else if previewed {
        button = button.foreground(Color::rgb(0x40, 0x70, 0xA0));
    }
    GridChild::new(
        button
            .build()
            .key((row * state.board.difficulty.columns + column) as u64),
    )
    .row(row as i32)
    .column(column as i32)
}

fn board_view(state: &AppState, dispatch: Callback<Action>) -> Element {
    let rows = state.board.difficulty.rows;
    let columns = state.board.difficulty.columns;
    let size = cell_size(state.board.difficulty);
    Grid::new((0..rows * columns).map(|position| {
        build_cell(
            state,
            position / columns,
            position % columns,
            size,
            dispatch.clone(),
        )
    }))
    .rows(vec![GridLength::Pixel(size); rows])
    .columns(vec![GridLength::Pixel(size); columns])
    .row_spacing(1.0)
    .column_spacing(1.0)
    .horizontal_alignment(HorizontalAlignment::Center)
    .build()
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let open = cx.use_state(|| true);
    let (state, dispatch) = cx.use_reducer(AppState::initial, reduce);
    let ticking = state.board.phase == GamePhase::Playing;
    let timer_dispatch = dispatch.clone();
    cx.use_interval(ticking, Duration::from_secs(1), move || {
        if ticking {
            timer_dispatch.call(Action::Tick);
        }
    });

    let windows = if open.value() {
        vec![
            Window::new("Dotsweeper", game_view(&state, dispatch), move || {
                open.set(false);
            })
            .client_size(920.0, 620.0)
            .client_constraints(WindowConstraints {
                min_width: Some(480.0),
                min_height: Some(520.0),
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

fn game_view(state: &AppState, dispatch: Callback<Action>) -> Element {
    let reset = dispatch.clone();
    let current_difficulty = state.board.difficulty;
    let status = match state.board.phase {
        GamePhase::Won => format!("You won in {}s!", state.elapsed_seconds),
        GamePhase::Lost => "Boom! Start a new game.".to_string(),
        _ => format!(
            "{} - {} mines - {}s",
            state.board.difficulty.display_name(),
            state.board.mines_remaining(),
            state.elapsed_seconds
        ),
    };
    let status_panel = hstack(
        16.0,
        [
            TextBlock::new(format!("{:03}", state.board.mines_remaining()))
                .font_size(22.0)
                .foreground(Color::rgb(0xFF, 0x3B, 0x30))
                .build(),
            Button::new(match state.board.phase {
                GamePhase::Won => "😎",
                GamePhase::Lost => "😵",
                _ => "🙂",
            })
            .on_click(move || reset.call(Action::NewGame(current_difficulty)))
            .width(56.0)
            .height(40.0)
            .build(),
            TextBlock::new(format!("{:03}", state.elapsed_seconds))
                .font_size(22.0)
                .foreground(Color::rgb(0xFF, 0x3B, 0x30))
                .build(),
        ],
    );

    let difficulty_button =
        |label: &'static str, difficulty: Difficulty, dispatch: Callback<Action>| {
            Button::new(label)
                .on_click(move || dispatch.call(Action::NewGame(difficulty)))
                .build()
        };
    Border::new(vstack(
        8.0,
        [
            TextBlock::new(status)
                .font_size(18.0)
                .horizontal_alignment(HorizontalAlignment::Center)
                .build(),
            status_panel,
            hstack(
                8.0,
                [
                    difficulty_button("Beginner", Difficulty::BEGINNER, dispatch.clone()),
                    difficulty_button("Intermediate", Difficulty::INTERMEDIATE, dispatch.clone()),
                    difficulty_button("Expert", Difficulty::EXPERT, dispatch.clone()),
                ],
            ),
            board_view(state, dispatch),
        ],
    ))
    .margin(Thickness::uniform(12.0))
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run_application(app)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rng() -> Lcg {
        Lcg::new(0xDEAD_BEEF)
    }

    #[test]
    fn new_game_has_no_mines_reveals_or_flags() {
        let board = Board::new_game(Difficulty::BEGINNER);
        assert_eq!(board.phase, GamePhase::NotStarted);
        assert_eq!(board.revealed_safe, 0);
        assert_eq!(board.flag_count, 0);
        assert!(
            board
                .cells
                .iter()
                .all(|cell| !cell.is_mine && !cell.is_revealed)
        );
    }

    #[test]
    fn first_reveal_is_safe_and_clears_a_pocket() {
        for seed in 1..32 {
            let mut rng = Lcg::new(seed);
            let board = reveal(Board::new_game(Difficulty::BEGINNER), 4, 4, &mut rng);
            assert!(board.cell(4, 4).is_revealed);
            assert!(!board.cell(4, 4).is_mine);
            assert!(
                board
                    .neighbors(4, 4)
                    .all(|(row, column)| !board.cell(row, column).is_mine)
            );
        }
    }

    #[test]
    fn mine_count_matches_difficulty() {
        let mut rng = rng();
        let board = reveal(Board::new_game(Difficulty::INTERMEDIATE), 0, 0, &mut rng);
        assert_eq!(
            board.cells.iter().filter(|cell| cell.is_mine).count(),
            Difficulty::INTERMEDIATE.mines
        );
    }

    #[test]
    fn toggle_flag_cycles_all_marks() {
        let mut board = Board::new_game(Difficulty::BEGINNER);
        board = toggle_flag(board, 0, 0);
        assert_eq!(board.cell(0, 0).mark, CellMark::Flag);
        board = toggle_flag(board, 0, 0);
        assert_eq!(board.cell(0, 0).mark, CellMark::Question);
        board = toggle_flag(board, 0, 0);
        assert_eq!(board.cell(0, 0).mark, CellMark::None);
    }

    #[test]
    fn flagged_cells_cannot_be_revealed() {
        let mut rng = rng();
        let board = toggle_flag(Board::new_game(Difficulty::BEGINNER), 0, 0);
        assert_eq!(reveal(board.clone(), 0, 0, &mut rng), board);
    }

    #[test]
    fn revealing_a_mine_loses() {
        let mut board = Board::new_game(Difficulty::BEGINNER);
        board.phase = GamePhase::Playing;
        let index = board.index(8, 8);
        board.cells[index].is_mine = true;
        let mut rng = rng();
        board = reveal(board, 8, 8, &mut rng);
        assert_eq!(board.phase, GamePhase::Lost);
        assert_eq!(board.exploded_at, Some((8, 8)));
    }

    #[test]
    fn chord_requires_matching_flag_count() {
        let mut board = Board::new_game(Difficulty::BEGINNER);
        board.phase = GamePhase::Playing;
        let center = board.index(1, 1);
        board.cells[center].is_revealed = true;
        board.cells[center].adjacent_mines = 1;
        let before = board.clone();
        let mut rng = rng();
        assert_eq!(chord(board, 1, 1, &mut rng), before);
    }

    #[test]
    fn reducer_clears_chord_preview() {
        let state = reduce(AppState::initial(), Action::BeginChordPreview(1, 1));
        assert_eq!(state.chord_preview, Some((1, 1)));
        let state = reduce(state, Action::EndChordPreview(false));
        assert_eq!(state.chord_preview, None);
    }
}
