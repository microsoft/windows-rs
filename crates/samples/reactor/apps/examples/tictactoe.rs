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
            Self::Empty => " ",
            Self::X => "X",
            Self::O => "O",
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
            Self::X => Cell::X,
            Self::O => Cell::O,
        }
    }

    fn next(&self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::X => "X",
            Self::O => "O",
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
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
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

#[derive(Clone)]
enum Message {
    Play(usize),
    Reset,
}

impl Component for Game {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self::new()
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Play(pos) => {
                if let Some(next) = apply_move(self, pos) {
                    *self = next;
                }
            }
            Message::Reset => *self = Self::new(),
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("windows_reactor — tictactoe");
        let header = StackPanel::new()
            .orientation(Orientation::Vertical)
            .spacing(8.0)
            .margin(Thickness::new(0.0, 12.0, 0.0, 4.0))
            .children((
                TextBlock::new()
                    .text(status_line(self))
                    .font_weight(700)
                    .font_size(24.0)
                    .horizontal_alignment(HorizontalAlignment::Center),
                Button::new()
                    .on_click(context.message(Message::Reset))
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .content(TextBlock::new().text("New Game")),
            ));

        let board = build_board(self, context.callback(Message::Play));

        let title_bar = TitleBar::new().title("windows_reactor — tictactoe");

        StackPanel::new()
            .orientation(Orientation::Vertical)
            .children((
                title_bar,
                StackPanel::new()
                    .orientation(Orientation::Vertical)
                    .spacing(12.0)
                    .children((header, board)),
            ))
    }
}

fn build_board(game: &Game, click_handler: Callback<usize>) -> View {
    let cells = build_cells(game, click_handler);
    Grid::new()
        .rows([GridLength::STAR; SIZE])
        .columns([GridLength::STAR; SIZE])
        .row_spacing(4.0)
        .column_spacing(4.0)
        .width(360.0)
        .height(360.0)
        .horizontal_alignment(HorizontalAlignment::Center)
        .keyed_children(cells)
}

fn build_cells(game: &Game, click_handler: Callback<usize>) -> Vec<KeyedView> {
    let game_over = game.status != Status::Playing;
    (0..TOTAL)
        .map(|pos| {
            let cell = game.cells[pos];
            let row = pos / SIZE;
            let col = pos % SIZE;
            let label = cell.label().to_string();
            let mut btn = Button::new()
                .on_click({
                    let click_handler = click_handler.clone();
                    move || {
                        _ = click_handler.call(pos);
                    }
                })
                .horizontal_alignment(HorizontalAlignment::Stretch)
                .vertical_alignment(VerticalAlignment::Stretch);
            if game_over || cell != Cell::Empty {
                btn = btn.is_enabled(false);
            }
            KeyedView::new(
                pos,
                btn.grid_row(row as i32)
                    .grid_column(col as i32)
                    .content(TextBlock::new().text(label)),
            )
        })
        .collect()
}

fn main() {
    App::run_component::<Game>(()).unwrap();
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
        let mut g = Game::new();
        for pos in [0_usize, 3, 1, 4, 2] {
            g = apply_move(&g, pos).expect("legal");
        }
        assert_eq!(g.status, Status::Won(Player::X));
    }

    #[test]
    fn detects_column_win_for_o() {
        let mut g = Game::new();
        for pos in [0_usize, 2, 1, 5, 4, 8] {
            g = apply_move(&g, pos).expect("legal");
        }
        assert_eq!(g.status, Status::Won(Player::O));
    }

    #[test]
    fn detects_diagonal_win() {
        let mut g = Game::new();
        for pos in [0_usize, 1, 4, 2, 8] {
            g = apply_move(&g, pos).expect("legal");
        }
        assert_eq!(g.status, Status::Won(Player::X));
    }

    #[test]
    fn detects_draw() {
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
        assert!(apply_move(&g, 5).is_none());
        assert!(apply_move(&g, 6).is_none());
    }
}
