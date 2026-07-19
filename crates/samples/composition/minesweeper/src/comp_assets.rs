//! The reusable composition assets (brushes and dot-pattern shapes) that render
//! tile states and neighbor counts. Assets are created once and cloned onto
//! tiles as needed (a clone is a cheap COM `AddRef`).

use crate::colors;
use crate::minesweeper::MineState;
use std::collections::HashMap;
use windows_composition::{
    CompositionColorBrush, CompositionContainerShape, CompositionEllipseGeometry,
    CompositionSpriteShape, Compositor, Result, Vector2,
};

fn get_dot_shape(
    compositor: &Compositor,
    geometry: &CompositionEllipseGeometry,
    brush: &CompositionColorBrush,
    offset: Vector2,
) -> Result<CompositionSpriteShape> {
    let shape = compositor.create_sprite_shape(geometry)?;
    shape.set_fill_brush(brush)?;
    shape.set_offset(offset)?;
    Ok(shape)
}

pub struct CompAssets {
    mine_brush: CompositionColorBrush,
    mine_state_brushes: HashMap<MineState, CompositionColorBrush>,
    mine_count_background_brushes: HashMap<i32, CompositionColorBrush>,
    mine_count_shapes: HashMap<i32, CompositionContainerShape>,
}

impl CompAssets {
    pub fn new(compositor: &Compositor, tile_size: &Vector2) -> Result<Self> {
        let mine_brush = compositor.create_color_brush(colors::RED)?;

        let mut result = Self {
            mine_brush,
            mine_state_brushes: HashMap::new(),
            mine_count_background_brushes: HashMap::new(),
            mine_count_shapes: HashMap::new(),
        };

        result.generate_assets(compositor, tile_size)?;

        Ok(result)
    }

    pub fn get_mine_brush(&self) -> CompositionColorBrush {
        self.mine_brush.clone()
    }

    pub fn get_shape_from_mine_count(&self, count: i32) -> CompositionContainerShape {
        self.mine_count_shapes.get(&count).unwrap().clone()
    }

    pub fn get_color_brush_from_mine_state(&self, state: MineState) -> CompositionColorBrush {
        self.mine_state_brushes.get(&state).unwrap().clone()
    }

    pub fn get_color_brush_from_mine_count(&self, count: i32) -> CompositionColorBrush {
        self.mine_count_background_brushes
            .get(&count)
            .unwrap()
            .clone()
    }

    fn generate_assets(&mut self, compositor: &Compositor, tile_size: &Vector2) -> Result<()> {
        self.mine_state_brushes.clear();
        self.mine_state_brushes.insert(
            MineState::Empty,
            compositor.create_color_brush(colors::BLUE)?,
        );
        self.mine_state_brushes.insert(
            MineState::Flag,
            compositor.create_color_brush(colors::ORANGE)?,
        );
        self.mine_state_brushes.insert(
            MineState::Question,
            compositor.create_color_brush(colors::LIME_GREEN)?,
        );

        self.mine_count_background_brushes.clear();
        self.mine_count_background_brushes
            .insert(1, compositor.create_color_brush(colors::LIGHT_BLUE)?);
        self.mine_count_background_brushes
            .insert(2, compositor.create_color_brush(colors::LIGHT_GREEN)?);
        self.mine_count_background_brushes
            .insert(3, compositor.create_color_brush(colors::LIGHT_SALMON)?);
        self.mine_count_background_brushes
            .insert(4, compositor.create_color_brush(colors::LIGHT_STEEL_BLUE)?);
        self.mine_count_background_brushes
            .insert(5, compositor.create_color_brush(colors::MEDIUM_PURPLE)?);
        self.mine_count_background_brushes
            .insert(6, compositor.create_color_brush(colors::LIGHT_CYAN)?);
        self.mine_count_background_brushes
            .insert(7, compositor.create_color_brush(colors::MAROON)?);
        self.mine_count_background_brushes
            .insert(8, compositor.create_color_brush(colors::DARK_SEA_GREEN)?);
        self.mine_count_background_brushes
            .insert(0, compositor.create_color_brush(colors::WHITE_SMOKE)?);

        self.mine_count_shapes.clear();
        let circle_geometry = compositor.create_ellipse_geometry()?;
        circle_geometry.set_radius(*tile_size / 12.0)?;
        let dot_brush = compositor.create_color_brush(colors::BLACK)?;

        let append_dot = |shape: &CompositionContainerShape, offset: Vector2| -> Result<()> {
            shape.shapes()?.append(&get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                offset,
            )?)
        };

        // 1
        {
            let container_shape = compositor.create_container_shape()?;
            append_dot(&container_shape, *tile_size / 2.0)?;
            self.mine_count_shapes.insert(1, container_shape);
        }
        // 2
        {
            let container_shape = compositor.create_container_shape()?;
            let third_x = tile_size.x / 3.0;
            let half_y = tile_size.y / 2.0;
            append_dot(&container_shape, Vector2::new(third_x, half_y))?;
            append_dot(&container_shape, Vector2::new(third_x * 2.0, half_y))?;
            self.mine_count_shapes.insert(2, container_shape);
        }
        // 3
        {
            let container_shape = compositor.create_container_shape()?;
            let fourth_x = tile_size.x / 4.0;
            let fourth_y = tile_size.y / 4.0;
            append_dot(&container_shape, *tile_size / 2.0)?;
            append_dot(&container_shape, Vector2::new(fourth_x, fourth_y * 3.0))?;
            append_dot(&container_shape, Vector2::new(fourth_x * 3.0, fourth_y))?;
            self.mine_count_shapes.insert(3, container_shape);
        }
        // 4
        {
            let container_shape = compositor.create_container_shape()?;
            let third_x = tile_size.x / 3.0;
            let third_y = tile_size.y / 3.0;
            append_dot(&container_shape, Vector2::new(third_x, third_y))?;
            append_dot(&container_shape, Vector2::new(third_x * 2.0, third_y))?;
            append_dot(&container_shape, Vector2::new(third_x, third_y * 2.0))?;
            append_dot(&container_shape, Vector2::new(third_x * 2.0, third_y * 2.0))?;
            self.mine_count_shapes.insert(4, container_shape);
        }
        // 5
        {
            let container_shape = compositor.create_container_shape()?;
            let fourth_x = tile_size.x / 4.0;
            let fourth_y = tile_size.y / 4.0;
            append_dot(&container_shape, *tile_size / 2.0)?;
            append_dot(&container_shape, Vector2::new(fourth_x, fourth_y * 3.0))?;
            append_dot(&container_shape, Vector2::new(fourth_x * 3.0, fourth_y))?;
            append_dot(&container_shape, Vector2::new(fourth_x, fourth_y))?;
            append_dot(
                &container_shape,
                Vector2::new(fourth_x * 3.0, fourth_y * 3.0),
            )?;
            self.mine_count_shapes.insert(5, container_shape);
        }
        // 6
        {
            let container_shape = compositor.create_container_shape()?;
            let fourth_x = tile_size.x / 4.0;
            let fourth_y = tile_size.y / 4.0;
            append_dot(&container_shape, Vector2::new(fourth_x, fourth_y * 2.0))?;
            append_dot(&container_shape, Vector2::new(fourth_x, fourth_y * 3.0))?;
            append_dot(&container_shape, Vector2::new(fourth_x * 3.0, fourth_y))?;
            append_dot(&container_shape, Vector2::new(fourth_x, fourth_y))?;
            append_dot(
                &container_shape,
                Vector2::new(fourth_x * 3.0, fourth_y * 3.0),
            )?;
            append_dot(
                &container_shape,
                Vector2::new(fourth_x * 3.0, fourth_y * 2.0),
            )?;
            self.mine_count_shapes.insert(6, container_shape);
        }
        // 7
        {
            let container_shape = compositor.create_container_shape()?;
            let fourth_x = tile_size.x / 4.0;
            let fourth_y = tile_size.y / 4.0;
            append_dot(&container_shape, Vector2::new(fourth_x, fourth_y * 2.0))?;
            append_dot(&container_shape, Vector2::new(fourth_x, fourth_y * 3.0))?;
            append_dot(&container_shape, Vector2::new(fourth_x * 3.0, fourth_y))?;
            append_dot(&container_shape, Vector2::new(fourth_x, fourth_y))?;
            append_dot(
                &container_shape,
                Vector2::new(fourth_x * 3.0, fourth_y * 3.0),
            )?;
            append_dot(
                &container_shape,
                Vector2::new(fourth_x * 3.0, fourth_y * 2.0),
            )?;
            append_dot(&container_shape, *tile_size / 2.0)?;
            self.mine_count_shapes.insert(7, container_shape);
        }
        // 8
        {
            let container_shape = compositor.create_container_shape()?;
            let fourth_x = tile_size.x / 4.0;
            let fourth_y = tile_size.y / 4.0;
            let half_x = tile_size.x / 2.0;
            let third_y = tile_size.y / 3.0;
            append_dot(&container_shape, Vector2::new(fourth_x, fourth_y * 2.0))?;
            append_dot(&container_shape, Vector2::new(fourth_x, fourth_y * 3.0))?;
            append_dot(&container_shape, Vector2::new(fourth_x * 3.0, fourth_y))?;
            append_dot(&container_shape, Vector2::new(fourth_x, fourth_y))?;
            append_dot(
                &container_shape,
                Vector2::new(fourth_x * 3.0, fourth_y * 3.0),
            )?;
            append_dot(
                &container_shape,
                Vector2::new(fourth_x * 3.0, fourth_y * 2.0),
            )?;
            append_dot(&container_shape, Vector2::new(half_x, third_y))?;
            append_dot(&container_shape, Vector2::new(half_x, third_y * 2.0))?;
            self.mine_count_shapes.insert(8, container_shape);
        }

        Ok(())
    }
}
