#![windows_subsystem = "windows"]

use windows_reactor::{
    AutomationHeadingLevel, Button, Element, FontWeight, Grid, GridChild, GridLength,
    HorizontalAlignment, RenderCx, StackPanel, State, TextBlock, Thickness, VerticalAlignment,
    vstack,
};

const SIZE: usize = 3;
const TOTAL: usize = SIZE * SIZE;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Empty,
    X,
    O,
}

impl Cell {
    fn label(self) -> &'static str {
        match self {
            Self::Empty => " ",
            Self::X => "X",
            Self::O => "O",
        }
    }

    fn accessible_label(self, position: usize) -> String {
        let value = match self {
            Self::Empty => "empty",
            Self::X => "X",
            Self::O => "O",
        };
        format!("Cell {}: {value}", position + 1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Player {
    X,
    O,
}

impl Player {
    fn cell(self) -> Cell {
        match self {
            Self::X => Cell::X,
            Self::O => Cell::O,
        }
    }

    fn next(self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }

    fn label(self) -> &'static str {
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
    for line in LINES {
        let first = cells[line[0]];
        if first != Cell::Empty && first == cells[line[1]] && first == cells[line[2]] {
            return Some(match first {
                Cell::X => Player::X,
                Cell::O => Player::O,
                Cell::Empty => unreachable!(),
            });
        }
    }
    None
}

fn apply_move(game: &Game, position: usize) -> Option<Game> {
    if position >= TOTAL || game.status != Status::Playing || game.cells[position] != Cell::Empty {
        return None;
    }

    let mut cells = game.cells;
    cells[position] = game.turn.cell();
    let status = if let Some(winner) = detect_winner(&cells) {
        Status::Won(winner)
    } else if cells.iter().all(|cell| *cell != Cell::Empty) {
        Status::Draw
    } else {
        Status::Playing
    };

    Some(Game {
        cells,
        turn: if status == Status::Playing {
            game.turn.next()
        } else {
            game.turn
        },
        status,
    })
}

fn status_line(game: &Game) -> String {
    match game.status {
        Status::Playing => format!("Turn: {}", game.turn.label()),
        Status::Won(player) => format!("{} wins!", player.label()),
        Status::Draw => "It's a draw".to_string(),
    }
}

fn header(game: Game, state: State<Game>) -> Element {
    let reset = state;
    vstack(
        8.0,
        [
            TextBlock::new(status_line(&game))
                .font_size(24.0)
                .font_weight(FontWeight::BOLD)
                .horizontal_alignment(HorizontalAlignment::Center)
                .automation_id("game-status")
                .heading_level(AutomationHeadingLevel::Level1)
                .build(),
            Button::new("New Game")
                .on_click(move || {
                    reset.set(Game::new());
                })
                .horizontal_alignment(HorizontalAlignment::Center)
                .automation_id("new-game")
                .build(),
        ],
    )
}

fn board(game: Game, state: State<Game>) -> Element {
    let game_over = game.status != Status::Playing;
    let cells = (0..TOTAL)
        .map(|position| {
            let cell = game.cells[position];
            let update = state.clone();
            let button = Button::new(cell.label())
                .on_click(move || {
                    update.update(|game| {
                        if let Some(next) = apply_move(game, position) {
                            *game = next;
                        }
                    });
                })
                .enabled(!game_over && cell == Cell::Empty)
                .font_size(48.0)
                .font_weight(FontWeight::BOLD)
                .horizontal_alignment(HorizontalAlignment::Stretch)
                .vertical_alignment(VerticalAlignment::Stretch)
                .automation_id(format!("cell-{position}"))
                .automation_name(cell.accessible_label(position))
                .build()
                .key(position as u64);
            GridChild::new(button)
                .row((position / SIZE) as i32)
                .column((position % SIZE) as i32)
        })
        .collect::<Vec<_>>();

    Grid::new(cells)
        .rows([GridLength::STAR; SIZE])
        .columns([GridLength::STAR; SIZE])
        .row_spacing(4.0)
        .column_spacing(4.0)
        .width(360.0)
        .height(360.0)
        .horizontal_alignment(HorizontalAlignment::Center)
        .build()
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let state = cx.use_state(Game::new);
    let game = state.value();

    StackPanel::new([header(game.clone(), state.clone()), board(game, state)])
        .spacing(12.0)
        .margin(Thickness::uniform(16.0))
        .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Tic-tac-toe", app)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_is_empty_with_x_to_move() {
        let game = Game::new();
        assert!(game.cells.iter().all(|cell| *cell == Cell::Empty));
        assert_eq!(game.turn, Player::X);
        assert_eq!(game.status, Status::Playing);
    }

    #[test]
    fn move_marks_cell_and_alternates_turn() {
        let game = apply_move(&Game::new(), 0).unwrap();
        assert_eq!(game.cells[0], Cell::X);
        assert_eq!(game.turn, Player::O);

        let game = apply_move(&game, 4).unwrap();
        assert_eq!(game.cells[4], Cell::O);
        assert_eq!(game.turn, Player::X);
    }

    #[test]
    fn rejects_invalid_moves() {
        let game = apply_move(&Game::new(), 0).unwrap();
        assert!(apply_move(&game, 0).is_none());
        assert!(apply_move(&game, TOTAL).is_none());
    }

    #[test]
    fn detects_row_column_and_diagonal_wins() {
        assert_eq!(play([0, 3, 1, 4, 2]).status, Status::Won(Player::X));
        assert_eq!(play([0, 2, 1, 5, 4, 8]).status, Status::Won(Player::O));
        assert_eq!(play([0, 1, 4, 2, 8]).status, Status::Won(Player::X));
    }

    #[test]
    fn detects_draw_and_rejects_later_moves() {
        let game = play([0, 1, 2, 4, 3, 5, 7, 6, 8]);
        assert_eq!(game.status, Status::Draw);
        assert!(apply_move(&game, 0).is_none());
    }

    fn play<const N: usize>(moves: [usize; N]) -> Game {
        moves.into_iter().fold(Game::new(), |game, position| {
            apply_move(&game, position).unwrap()
        })
    }
}
