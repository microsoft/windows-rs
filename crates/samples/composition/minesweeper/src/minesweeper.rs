//! The minesweeper game logic: board state, mine generation, flood-fill sweep,
//! win detection, and the spiral mine-reveal animation. This module is
//! rendering-agnostic — it drives [`CompUI`] but knows nothing about
//! composition.

use crate::comp_ui::CompUI;
use crate::rng::Rng;
use crate::visual_grid::TileCoordinate;
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};
use windows_composition::{ContainerVisual, Result, Vector2};

/// A board size in tiles.
#[derive(Copy, Clone)]
pub struct GridSize {
    pub width: i32,
    pub height: i32,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum MineState {
    Empty,
    Flag,
    Question,
    Revealed,
}
impl MineState {
    fn cycle(self) -> Self {
        match self {
            Self::Empty => Self::Flag,
            Self::Flag => Self::Question,
            Self::Question => Self::Empty,
            Self::Revealed => unreachable!("We shouldn't be cycling a revealed tile!"),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum MineGenerationState {
    Deferred,
    Generated,
}

/// Maps between `(x, y)` tile coordinates and flat vector indices.
pub struct IndexHelper {
    width: i32,
    height: i32,
}

impl IndexHelper {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn compute_index(&self, x: i32, y: i32) -> usize {
        (x * self.height + y) as usize
    }

    pub fn compute_x_from_index(&self, index: usize) -> i32 {
        index as i32 / self.height
    }

    pub fn compute_y_from_index(&self, index: usize) -> i32 {
        index as i32 % self.height
    }

    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        (x >= 0 && x < self.width) && (y >= 0 && y < self.height)
    }
}

pub struct Minesweeper {
    ui: CompUI,

    game_board_width: i32,
    game_board_height: i32,
    index_helper: IndexHelper,

    mine_states: Vec<MineState>,
    mines: Vec<bool>,
    neighbor_counts: Vec<i32>,
    mine_generation_state: MineGenerationState,
    num_mines: i32,
    last_tile: Option<TileCoordinate>,
    rng: Rng,

    game_over: bool,
}

impl Minesweeper {
    pub fn new(parent_visual: &ContainerVisual, parent_size: &Vector2) -> Result<Self> {
        let game_board_size_in_tiles = GridSize {
            width: 16,
            height: 16,
        };
        let ui = CompUI::new(parent_visual, parent_size, &game_board_size_in_tiles)?;

        let tile_count =
            (game_board_size_in_tiles.width * game_board_size_in_tiles.height) as usize;

        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0x9E37_79B9_7F4A_7C15, |d| d.as_nanos() as u64);

        let mut result = Self {
            ui,

            game_board_width: game_board_size_in_tiles.width,
            game_board_height: game_board_size_in_tiles.height,
            index_helper: IndexHelper::new(
                game_board_size_in_tiles.width,
                game_board_size_in_tiles.height,
            ),

            mine_states: vec![MineState::Empty; tile_count],
            mines: vec![false; tile_count],
            neighbor_counts: Vec::new(),
            mine_generation_state: MineGenerationState::Deferred,
            num_mines: 0,
            last_tile: None,
            rng: Rng::new(seed),

            game_over: false,
        };

        result.new_game(
            game_board_size_in_tiles.width,
            game_board_size_in_tiles.height,
            40,
        )?;
        result.on_parent_size_changed(parent_size)?;

        Ok(result)
    }

    pub fn on_pointer_moved(&mut self, point: &Vector2) -> Result<()> {
        if self.game_over || self.ui.is_animation_playing() {
            return Ok(());
        }

        let selected_tile = if let Some(tile) = self.ui.hit_test(point)? {
            self.last_tile = Some(tile);
            if self.mine_states[self.index_helper.compute_index(tile.x, tile.y)]
                != MineState::Revealed
            {
                Some(tile)
            } else {
                None
            }
        } else {
            self.last_tile = None;
            None
        };
        self.ui.select_tile(selected_tile)?;

        Ok(())
    }

    pub fn on_parent_size_changed(&mut self, new_size: &Vector2) -> Result<()> {
        self.ui.resize(new_size)?;
        Ok(())
    }

    pub fn on_pointer_pressed(&mut self, is_right_button: bool, is_eraser: bool) -> Result<()> {
        // We can't subscribe to the animation-completed event yet, so restart on
        // any press once the game is over.
        if self.game_over {
            self.new_game(
                self.game_board_width,
                self.game_board_height,
                self.num_mines,
            )?;
        }

        let current_selection = self.ui.current_selected_tile();
        if let Some(current_selection) = current_selection {
            let index = self
                .index_helper
                .compute_index(current_selection.x, current_selection.y);

            if self.mine_states[index] != MineState::Revealed {
                if is_right_button || is_eraser {
                    let state = self.mine_states[index].cycle();
                    self.mine_states[index] = state;
                    self.ui.update_tile_with_state(&current_selection, state)?;
                } else if self.mine_states[index] == MineState::Empty
                    && self.sweep(current_selection.x, current_selection.y)?
                {
                    // We hit a mine! Setup and play an animation while locking any input.
                    let hit_x = current_selection.x;
                    let hit_y = current_selection.y;

                    // First, hide the selection visual and reset the selection.
                    self.ui.select_tile(None)?;

                    self.play_animation_on_all_mines(hit_x, hit_y)?;

                    self.game_over = true;
                } else if self.mine_states[index] == MineState::Empty && self.check_if_won() {
                    self.ui.select_tile(None)?;
                    self.game_over = true;
                }
            }
        } else {
            if is_right_button || is_eraser {
                // Do nothing on right click or eraser mode.
                return Ok(());
            }
            self.check_and_clear_satisfied()?;
        }
        Ok(())
    }

    pub fn check_and_clear_satisfied(&mut self) -> Result<()> {
        // We're outside the unrevealed/flagged/etc tiles, but we should be at
        // last_tile.
        if self.last_tile.is_none() {
            return Ok(());
        }

        let cur_tile = self
            .last_tile
            .expect("Somehow last tile became None after test");

        // Does the current tile have a number in it?
        let index = self.index_helper.compute_index(cur_tile.x, cur_tile.y);
        if self.neighbor_counts[index] < 1 || self.mine_states[index] != MineState::Revealed {
            // No neighbors, or not revealed, do nothing!
            return Ok(());
        }

        // Make a vector of coordinates to query based on the current coordinate.
        let base_vec: Vec<(i32, i32)> = vec![
            (cur_tile.x - 1, cur_tile.y - 1),
            (cur_tile.x, cur_tile.y - 1),
            (cur_tile.x + 1, cur_tile.y - 1),
            (cur_tile.x - 1, cur_tile.y),
            (cur_tile.x + 1, cur_tile.y),
            (cur_tile.x - 1, cur_tile.y + 1),
            (cur_tile.x, cur_tile.y + 1),
            (cur_tile.x + 1, cur_tile.y + 1),
        ];

        // Filter out-of-bounds if we're on the edges.
        let width = self.game_board_width;
        let height = self.game_board_height;
        let query_vec: Vec<(i32, i32)> = base_vec
            .into_iter()
            .filter(|cur| cur.0 >= 0 && cur.0 < width && cur.1 >= 0 && cur.1 < height)
            .collect();
        // See if all mines are marked that are in those 8 (or fewer) tiles.
        let mut flag_count = 0;
        for query_coord in &query_vec {
            let query_index = self
                .index_helper
                .compute_index(query_coord.0, query_coord.1);
            if self.mine_states[query_index] == MineState::Flag {
                flag_count += 1;
            }
        }
        if flag_count != self.neighbor_counts[index] {
            // Too many or not enough flags.
            return Ok(());
        }

        // Go through the query_vec and try to reveal all of them with sweep if
        // they're not flagged.
        let mut hit_coordinate: Option<TileCoordinate> = None;
        for query_coord in &query_vec {
            let query_index = self
                .index_helper
                .compute_index(query_coord.0, query_coord.1);
            // Is it unrevealed? Only click on those spaces.
            if self.mine_states[query_index] != MineState::Empty {
                continue;
            }
            if self.sweep(query_coord.0, query_coord.1)? {
                hit_coordinate = Some(TileCoordinate {
                    x: query_coord.0,
                    y: query_coord.1,
                });
                break;
            }
        }

        if let Some(cur_coordinate) = hit_coordinate {
            // We hit a mine! Setup and play an animation while locking any input.
            let hit_x = cur_coordinate.x;
            let hit_y = cur_coordinate.y;

            self.ui.select_tile(None)?;
            self.play_animation_on_all_mines(hit_x, hit_y)?;
            self.game_over = true;
        } else if self.check_if_won() {
            self.ui.select_tile(None)?;
            self.game_over = true;
        }

        Ok(())
    }

    fn new_game(&mut self, board_width: i32, board_height: i32, mines: i32) -> Result<()> {
        self.game_board_width = board_width;
        self.game_board_height = board_height;
        self.index_helper = IndexHelper::new(board_width, board_height);

        self.ui.reset(&GridSize {
            width: board_width,
            height: board_height,
        })?;

        self.mine_states.fill(MineState::Empty);

        self.game_over = false;
        self.mine_generation_state = MineGenerationState::Deferred;
        self.num_mines = mines;
        self.last_tile = None;

        Ok(())
    }

    fn sweep(&mut self, x: i32, y: i32) -> Result<bool> {
        if self.mine_generation_state == MineGenerationState::Deferred {
            // We don't want the first thing the user clicks to be a mine, so
            // generate mines avoiding the clicked tile.
            self.generate_mines(self.num_mines, x, y);
            self.mine_generation_state = MineGenerationState::Generated;
        }

        let mut hit_mine = false;
        let mut sweeps: VecDeque<usize> = VecDeque::new();
        sweeps.push_back(self.index_helper.compute_index(x, y));
        self.reveal(*sweeps.front().unwrap())?;

        while !sweeps.is_empty() {
            let index = *sweeps.front().unwrap();
            let current_x = self.index_helper.compute_x_from_index(index);
            let current_y = self.index_helper.compute_y_from_index(index);

            if self.mines[index] {
                hit_mine = true;
                break;
            }

            if self.neighbor_counts[index] == 0 {
                self.push_if_unmarked(&mut sweeps, current_x - 1, current_y - 1)?;
                self.push_if_unmarked(&mut sweeps, current_x, current_y - 1)?;
                self.push_if_unmarked(&mut sweeps, current_x + 1, current_y - 1)?;
                self.push_if_unmarked(&mut sweeps, current_x + 1, current_y)?;
                self.push_if_unmarked(&mut sweeps, current_x + 1, current_y + 1)?;
                self.push_if_unmarked(&mut sweeps, current_x, current_y + 1)?;
                self.push_if_unmarked(&mut sweeps, current_x - 1, current_y + 1)?;
                self.push_if_unmarked(&mut sweeps, current_x - 1, current_y)?;
            }

            sweeps.pop_front().unwrap();
        }

        Ok(hit_mine)
    }

    fn reveal(&mut self, index: usize) -> Result<()> {
        let tile_coordinate = TileCoordinate {
            x: self.index_helper.compute_x_from_index(index),
            y: self.index_helper.compute_y_from_index(index),
        };

        if self.mines[index] {
            self.ui.update_tile_as_mine(&tile_coordinate)?;
        } else {
            let count = self.neighbor_counts[index];
            self.ui
                .update_tile_with_mine_count(&tile_coordinate, count)?;
        }

        self.mine_states[index] = MineState::Revealed;
        Ok(())
    }

    fn is_in_bounds_and_unmarked(&self, x: i32, y: i32) -> bool {
        let index = self.index_helper.compute_index(x, y);
        self.index_helper.is_in_bounds(x, y) && self.mine_states[index] == MineState::Empty
    }

    fn push_if_unmarked(&mut self, sweeps: &mut VecDeque<usize>, x: i32, y: i32) -> Result<()> {
        if self.is_in_bounds_and_unmarked(x, y) {
            let index = self.index_helper.compute_index(x, y);
            self.reveal(index)?;
            sweeps.push_back(index);
        }

        Ok(())
    }

    fn generate_mines(&mut self, num_mines: i32, exclude_x: i32, exclude_y: i32) {
        self.mines.fill(false);

        let bound = (self.game_board_width * self.game_board_height) as usize;
        let exclude_index = self.index_helper.compute_index(exclude_x, exclude_y);
        for _ in 0..num_mines {
            loop {
                let index = self.rng.next_range(bound);
                if index != exclude_index && !self.mines[index] {
                    self.mines[index] = true;
                    break;
                }
            }
        }

        self.neighbor_counts.clear();
        for i in 0..self.mines.len() {
            let x = self.index_helper.compute_x_from_index(i);
            let y = self.index_helper.compute_y_from_index(i);

            if self.mines[i] {
                // -1 means a mine.
                self.neighbor_counts.push(-1);
            } else {
                let count = self.get_surrounding_mine_count(x, y);
                self.neighbor_counts.push(count);
            }
        }
    }

    fn test_spot(&self, x: i32, y: i32) -> bool {
        self.index_helper.is_in_bounds(x, y) && self.mines[self.index_helper.compute_index(x, y)]
    }

    fn get_surrounding_mine_count(&self, x: i32, y: i32) -> i32 {
        let mut count = 0;

        if self.test_spot(x + 1, y) {
            count += 1;
        }
        if self.test_spot(x - 1, y) {
            count += 1;
        }
        if self.test_spot(x, y + 1) {
            count += 1;
        }
        if self.test_spot(x, y - 1) {
            count += 1;
        }
        if self.test_spot(x + 1, y + 1) {
            count += 1;
        }
        if self.test_spot(x - 1, y - 1) {
            count += 1;
        }
        if self.test_spot(x - 1, y + 1) {
            count += 1;
        }
        if self.test_spot(x + 1, y - 1) {
            count += 1;
        }

        count
    }

    fn check_tile_for_mine_for_animation(
        &self,
        x: i32,
        y: i32,
        mine_indices: &mut VecDeque<usize>,
        visited_tiles: &mut i32,
        mines_in_ring: &mut i32,
    ) {
        if self.index_helper.is_in_bounds(x, y) {
            let tile_index = self.index_helper.compute_index(x, y);
            if self.mines[tile_index] {
                mine_indices.push_back(tile_index);
                *mines_in_ring += 1;
            }
            *visited_tiles += 1;
        }
    }

    fn play_animation_on_all_mines(&mut self, center_x: i32, center_y: i32) -> Result<()> {
        // Build a queue containing the indices of the mines in a spiral starting
        // from the clicked mine.
        let mut mine_indices: VecDeque<usize> = VecDeque::new();
        let mut mines_per_ring: VecDeque<i32> = VecDeque::new();
        let mut visited_tiles: i32 = 0;
        let mut ring_level: i32 = 0;
        while visited_tiles < (self.game_board_width * self.game_board_height) {
            if ring_level == 0 {
                let hit_mine_index = self.index_helper.compute_index(center_x, center_y);
                mine_indices.push_back(hit_mine_index);
                mines_per_ring.push_back(1);
                visited_tiles += 1;
            } else {
                let mut current_mines_in_ring = 0;

                // Top side.
                for x in (center_x - ring_level)..=(center_x + ring_level) {
                    let y = center_y - ring_level;
                    self.check_tile_for_mine_for_animation(
                        x,
                        y,
                        &mut mine_indices,
                        &mut visited_tiles,
                        &mut current_mines_in_ring,
                    );
                }

                // Right side.
                for y in (center_y - ring_level + 1)..=(center_y + ring_level) {
                    let x = center_x + ring_level;
                    self.check_tile_for_mine_for_animation(
                        x,
                        y,
                        &mut mine_indices,
                        &mut visited_tiles,
                        &mut current_mines_in_ring,
                    );
                }

                // Bottom side.
                for x in (center_x - ring_level)..(center_x + ring_level) {
                    let y = center_y + ring_level;
                    self.check_tile_for_mine_for_animation(
                        x,
                        y,
                        &mut mine_indices,
                        &mut visited_tiles,
                        &mut current_mines_in_ring,
                    );
                }

                // Left side.
                for y in (center_y - ring_level + 1)..(center_y + ring_level) {
                    let x = center_x - ring_level;
                    self.check_tile_for_mine_for_animation(
                        x,
                        y,
                        &mut mine_indices,
                        &mut visited_tiles,
                        &mut current_mines_in_ring,
                    );
                }

                if current_mines_in_ring > 0 {
                    mines_per_ring.push_back(current_mines_in_ring);
                }
            }
            ring_level += 1;
        }

        self.ui.play_mine_animations(mine_indices, mines_per_ring)?;

        Ok(())
    }

    fn check_if_won(&self) -> bool {
        self.mine_states
            .iter()
            .filter(|state| **state != MineState::Revealed)
            .count()
            == self.num_mines as usize
    }
}
