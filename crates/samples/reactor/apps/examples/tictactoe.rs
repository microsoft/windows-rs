#![windows_subsystem = "windows"]

use windows_reactor::*;

const SIZE: usize = 3;
const TOTAL: usize = SIZE * SIZE;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Empty,
    X,
    O,
}

impl Cell {
    fn label(&self) -> &'static str {
        match self {
            Cell::Empty => " ",
            Cell::X => "X",
            Cell::O => "O",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Player {
    X,
    O,
}

impl Player {
    fn cell(&self) -> Cell {
        match self {
            Player::X => Cell::X,
            Player::O => Cell::O,
        }
    }

    fn next(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Player::X => "X",
            Player::O => "O",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Status {
    Playing,
    Won(Player),
    Draw,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Game {
    cells: [Cell; TOTAL],
    turn: Player,
    status: Status,
}

impl Game {
    fn new() -> Self {
        Self {
            cells: [Cell::Empty; TOTAL],
            turn: Player::X,
            status: Status::Playing,
        }
    }
}

const LINES: [[usize; 3]; 8] = [
    // rows
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    // columns
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    // diagonals
    [0, 4, 8],
    [2, 4, 6],
];

fn detect_winner(cells: &[Cell; TOTAL]) -> Option<Player> {
    for line in &LINES {
        let a = cells[line[0]];
        if a == Cell::Empty {
            continue;
        }
        if a == cells[line[1]] && a == cells[line[2]] {
            return match a {
                Cell::X => Some(Player::X),
                Cell::O => Some(Player::O),
                // Unreachable: filtered out by the `Cell::Empty` check above.
                Cell::Empty => None,
            };
        }
    }
    None
}

fn apply_move(game: &Game, pos: usize) -> Option<Game> {
    if pos >= TOTAL {
        return None;
    }
    if game.status != Status::Playing {
        return None;
    }
    if game.cells[pos] != Cell::Empty {
        return None;
    }

    let mut cells = game.cells;
    cells[pos] = game.turn.cell();

    let status = if let Some(winner) = detect_winner(&cells) {
        Status::Won(winner)
    } else if cells.iter().all(|c| *c != Cell::Empty) {
        Status::Draw
    } else {
        Status::Playing
    };

    let turn = if status == Status::Playing {
        game.turn.next()
    } else {
        game.turn
    };

    Some(Game {
        cells,
        turn,
        status,
    })
}

fn status_line(game: &Game) -> String {
    match game.status {
        Status::Playing => format!("Turn: {}", game.turn.label()),
        Status::Won(p) => format!("🎉 {} wins!", p.label()),
        Status::Draw => "It's a draw".to_string(),
    }
}

fn app(cx: &mut RenderCx) -> Element {
    let (game, set_game) = cx.use_state(Game::new());

    let reset_handler = make_reset_handler(set_game.clone());
    let click_handler = make_click_handler(set_game, game.clone());

    let header = vstack((
        text_block(status_line(&game))
            .bold()
            .font_size(24.0)
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

    let board = build_board(&game, click_handler);

    let title_bar = TitleBar::new("windows_reactor — tictactoe");

    vstack((title_bar, vstack((header, board)).spacing(12.0))).into()
}

fn build_board(game: &Game, click_handler: impl Fn(usize) + Clone + 'static) -> Element {
    let cells = build_cells(game, click_handler);
    grid(cells)
        .rows([GridLength::STAR; SIZE])
        .columns([GridLength::STAR; SIZE])
        .row_spacing(4.0)
        .column_spacing(4.0)
        .width(360.0)
        .height(360.0)
        .horizontal_alignment(HorizontalAlignment::Center)
        .into()
}

fn build_cells(game: &Game, click_handler: impl Fn(usize) + Clone + 'static) -> Vec<Element> {
    let game_over = game.status != Status::Playing;
    (0..TOTAL)
        .map(|pos| {
            let cell = game.cells[pos];
            let row = pos / SIZE;
            let col = pos % SIZE;
            let cb = click_handler.clone();
            let label = cell.label().to_string();
            let mut btn = button(label)
                .on_click(move || cb(pos))
                .background(ThemeRef::SubtleFill)
                .foreground(ThemeRef::PrimaryText)
                .with_key(format!("cell-{pos}"))
                .horizontal_alignment(HorizontalAlignment::Stretch)
                .vertical_alignment(VerticalAlignment::Stretch);
            if game_over || cell != Cell::Empty {
                btn = btn.enabled(false);
            }
            btn.grid_row(row as i32).grid_column(col as i32).into()
        })
        .collect()
}

fn make_reset_handler(set: SetState<Game>) -> impl Fn() + 'static {
    move || set.call(Game::new())
}

fn make_click_handler(set: SetState<Game>, current: Game) -> impl Fn(usize) + Clone + 'static {
    move |pos| {
        if let Some(next) = apply_move(&current, pos) {
            set.call(next);
        }
    }
}

fn main() -> Result<()> {
    App::new().title("windows_reactor — tictactoe").render(app)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_is_empty_with_x_to_move() {
        let g = Game::new();
        assert!(g.cells.iter().all(|c| *c == Cell::Empty));
        assert_eq!(g.turn, Player::X);
        assert_eq!(g.status, Status::Playing);
    }

    #[test]
    fn move_marks_cell_and_alternates_turn() {
        let g = Game::new();
        let g = apply_move(&g, 0).expect("legal");
        assert_eq!(g.cells[0], Cell::X);
        assert_eq!(g.turn, Player::O);
        assert_eq!(g.status, Status::Playing);

        let g = apply_move(&g, 4).expect("legal");
        assert_eq!(g.cells[4], Cell::O);
        assert_eq!(g.turn, Player::X);
    }

    #[test]
    fn cannot_play_on_occupied_cell() {
        let g = Game::new();
        let g = apply_move(&g, 0).expect("legal");
        assert!(apply_move(&g, 0).is_none());
    }

    #[test]
    fn out_of_range_move_is_rejected() {
        let g = Game::new();
        assert!(apply_move(&g, TOTAL).is_none());
    }

    #[test]
    fn detects_row_win_for_x() {
        // X: 0,1,2 ; O: 3,4
        let mut g = Game::new();
        for pos in [0_usize, 3, 1, 4, 2] {
            g = apply_move(&g, pos).expect("legal");
        }
        assert_eq!(g.status, Status::Won(Player::X));
    }

    #[test]
    fn detects_column_win_for_o() {
        // X: 0,1,4 ; O: 2,5,8 (column 2)
        let mut g = Game::new();
        for pos in [0_usize, 2, 1, 5, 4, 8] {
            g = apply_move(&g, pos).expect("legal");
        }
        assert_eq!(g.status, Status::Won(Player::O));
    }

    #[test]
    fn detects_diagonal_win() {
        // X: 0,4,8 ; O: 1,2
        let mut g = Game::new();
        for pos in [0_usize, 1, 4, 2, 8] {
            g = apply_move(&g, pos).expect("legal");
        }
        assert_eq!(g.status, Status::Won(Player::X));
    }

    #[test]
    fn detects_draw() {
        // A classic full-board draw:
        // X O X
        // X O O
        // O X X
        let mut g = Game::new();
        for pos in [0_usize, 1, 2, 4, 3, 5, 7, 6, 8] {
            g = apply_move(&g, pos).expect("legal");
        }
        assert_eq!(g.status, Status::Draw);
        assert!(g.cells.iter().all(|c| *c != Cell::Empty));
    }

    #[test]
    fn no_moves_after_game_ends() {
        let mut g = Game::new();
        for pos in [0_usize, 3, 1, 4, 2] {
            g = apply_move(&g, pos).expect("legal");
        }
        assert_eq!(g.status, Status::Won(Player::X));
        // any further move must be rejected
        assert!(apply_move(&g, 5).is_none());
        assert!(apply_move(&g, 6).is_none());
    }
}
