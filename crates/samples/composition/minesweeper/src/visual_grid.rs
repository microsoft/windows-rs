//! The visual tree for the tile grid: a container visual holding one sprite
//! visual per tile, plus a hollow nine-grid "selection" visual that highlights
//! the tile under the pointer.

use crate::colors;
use crate::minesweeper::{GridSize, IndexHelper};
use crate::numerics::from_vector2;
use windows_composition::{Compositor, ContainerVisual, Result, SpriteVisual, Vector2};

/// The `(x, y)` coordinate of a tile on the board.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct TileCoordinate {
    pub x: i32,
    pub y: i32,
}

pub struct VisualGrid {
    compositor: Compositor,
    root: ContainerVisual,

    tiles: Vec<SpriteVisual>,
    selection_visual: SpriteVisual,
    index_helper: IndexHelper,

    grid_width_in_tiles: i32,
    grid_height_in_tiles: i32,
    tile_size: Vector2,
    margin: Vector2,

    current_selection: Option<TileCoordinate>,
}

impl VisualGrid {
    pub fn new(
        compositor: &Compositor,
        grid_size_in_tiles: &GridSize,
        tile_size: &Vector2,
        margin: &Vector2,
    ) -> Result<Self> {
        let compositor = compositor.clone();
        let root = compositor.create_container_visual();

        let selection_visual = compositor.create_sprite_visual();
        let color_brush = compositor.create_color_brush(colors::RED);
        let nine_grid_brush = compositor.create_nine_grid_brush();
        nine_grid_brush.set_insets(margin.x, margin.y, margin.x, margin.y);
        nine_grid_brush.set_center_hollow(true);
        nine_grid_brush.set_source(&color_brush);
        selection_visual.set_brush(&nine_grid_brush);
        let offset = *margin * -1.0;
        selection_visual.set_offset(offset.x, offset.y, 0.0);
        selection_visual.set_visible(false);
        let size = *tile_size + *margin * 2.0;
        selection_visual.set_size(size.x, size.y);

        let mut result = Self {
            compositor,
            root,

            tiles: Vec::new(),
            selection_visual,
            index_helper: IndexHelper::new(grid_size_in_tiles.width, grid_size_in_tiles.height),

            grid_width_in_tiles: grid_size_in_tiles.width,
            grid_height_in_tiles: grid_size_in_tiles.height,
            tile_size: *tile_size,
            margin: *margin,

            current_selection: None,
        };

        result.reset(grid_size_in_tiles)?;

        Ok(result)
    }

    pub fn reset(&mut self, grid_size_in_tiles: &GridSize) -> Result<()> {
        let children = self.root.children();
        children.remove_all();
        self.tiles.clear();

        self.index_helper = IndexHelper::new(grid_size_in_tiles.width, grid_size_in_tiles.height);

        self.grid_width_in_tiles = grid_size_in_tiles.width;
        self.grid_height_in_tiles = grid_size_in_tiles.height;
        self.select_tile(None)?;

        let root_size = (self.tile_size + self.margin)
            * Vector2::new(
                self.grid_width_in_tiles as f32,
                self.grid_height_in_tiles as f32,
            );
        self.root.set_size(root_size.x, root_size.y);

        for x in 0..self.grid_width_in_tiles {
            for y in 0..self.grid_height_in_tiles {
                let visual = self.compositor.create_sprite_visual();
                visual.set_size(self.tile_size.x, self.tile_size.y);
                visual.set_center_point(from_vector2(self.tile_size / 2.0, 0.0));
                let offset = from_vector2(
                    (self.margin / 2.0)
                        + ((self.tile_size + self.margin) * Vector2::new(x as f32, y as f32)),
                    0.0,
                );
                visual.set_offset(offset.x, offset.y, offset.z);

                children.insert_at_top(&visual);
                self.tiles.push(visual);
            }
        }

        Ok(())
    }

    pub fn tiles_iter(&self) -> impl Iterator<Item = &SpriteVisual> {
        self.tiles.iter()
    }

    pub fn root(&self) -> &ContainerVisual {
        &self.root
    }

    pub fn selection_visual(&self) -> &SpriteVisual {
        &self.selection_visual
    }

    pub fn size(&self) -> Result<Vector2> {
        Ok(self.root.size())
    }

    pub fn hit_test(&self, point: &Vector2) -> Option<TileCoordinate> {
        let x = (point.x / (self.tile_size.x + self.margin.x)) as i32;
        let y = (point.y / (self.tile_size.y + self.margin.y)) as i32;

        if self.index_helper.is_in_bounds(x, y) {
            Some(TileCoordinate { x, y })
        } else {
            None
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&SpriteVisual> {
        if self.index_helper.is_in_bounds(x, y) {
            Some(&self.tiles[self.index_helper.compute_index(x, y)])
        } else {
            None
        }
    }

    pub fn select_tile(&mut self, tile_coordinate: Option<TileCoordinate>) -> Result<()> {
        self.current_selection = tile_coordinate;
        if let Some(tile_coordinate) = tile_coordinate {
            let visual = &self.tiles[self
                .index_helper
                .compute_index(tile_coordinate.x, tile_coordinate.y)];
            self.selection_visual.set_parent_for_transform(visual);
            self.selection_visual.set_visible(true);
        } else {
            self.selection_visual.set_visible(false);
        }

        Ok(())
    }

    pub fn current_selected_tile(&self) -> Option<TileCoordinate> {
        self.current_selection
    }
}
