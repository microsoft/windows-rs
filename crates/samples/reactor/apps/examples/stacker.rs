//! Original puzzle demonstrating Reactor chrome, a Composition settled board, and a Canvas
//! active piece and overlay.

#![windows_subsystem = "windows"]

use std::time::*;
use windows_canvas::*;
use windows_composition::*;
use windows_core::Result;
use windows_reactor::*;

const COLS: usize = 8;
const ROWS: usize = 16;
const ASSET_SIZE: f32 = 64.0;
const CELL_INSET: f32 = 3.0 / ASSET_SIZE;
const CELL_RADIUS: f32 = 9.0 / ASSET_SIZE;

type Board = [[Option<ColorId>; COLS]; ROWS];
type VisualBoard = [[Option<ShapeVisual>; COLS]; ROWS];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
enum ColorId {
    Teal,
    Amber,
    Plum,
    Olive,
    Steel,
}

impl ColorId {
    const ALL: [Self; 5] = [
        Self::Teal,
        Self::Amber,
        Self::Plum,
        Self::Olive,
        Self::Steel,
    ];

    fn rgb(self) -> (u8, u8, u8) {
        match self {
            Self::Teal => (66, 145, 142),
            Self::Amber => (196, 142, 65),
            Self::Plum => (137, 91, 137),
            Self::Olive => (123, 133, 75),
            Self::Steel => (85, 116, 139),
        }
    }

    fn canvas(self) -> ColorF {
        let (r, g, b) = self.rgb();
        ColorF::from_rgb8(r, g, b)
    }

    fn composition(self) -> CompositionColor {
        let (r, g, b) = self.rgb();
        CompositionColor::rgb(r, g, b)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PieceKind {
    Bar,
    Corner,
}

impl PieceKind {
    const ALL: [Self; 2] = [Self::Bar, Self::Corner];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Piece {
    kind: PieceKind,
    color: ColorId,
    rot: u8,
    col: i32,
    row: i32,
}

fn base_cells(kind: PieceKind) -> [(i32, i32); 3] {
    match kind {
        PieceKind::Bar => [(0, 0), (1, 0), (2, 0)],
        PieceKind::Corner => [(0, 0), (0, 1), (1, 1)],
    }
}

fn cells(kind: PieceKind, rot: u8) -> [(i32, i32); 3] {
    let mut result = base_cells(kind);
    for point in &mut result {
        for _ in 0..rot % 4 {
            *point = (-point.1, point.0);
        }
    }
    let min_x = result.iter().map(|point| point.0).min().unwrap();
    let min_y = result.iter().map(|point| point.1).min().unwrap();
    for point in &mut result {
        point.0 -= min_x;
        point.1 -= min_y;
    }
    result
}

fn fits(board: &Board, piece: Piece) -> bool {
    cells(piece.kind, piece.rot).into_iter().all(|(x, y)| {
        let col = piece.col + x;
        let row = piece.row + y;
        col >= 0
            && col < COLS as i32
            && row >= 0
            && row < ROWS as i32
            && board[row as usize][col as usize].is_none()
    })
}

fn lock(board: &mut Board, piece: Piece) {
    for (x, y) in cells(piece.kind, piece.rot) {
        board[(piece.row + y) as usize][(piece.col + x) as usize] = Some(piece.color);
    }
}

fn clear_full_rows(board: &mut Board) -> Vec<usize> {
    let cleared: Vec<_> = board
        .iter()
        .enumerate()
        .filter_map(|(row, cells)| cells.iter().all(Option::is_some).then_some(row))
        .collect();
    let mut write = ROWS;
    for read in (0..ROWS).rev() {
        if cleared.contains(&read) {
            continue;
        }
        write -= 1;
        board[write] = board[read];
    }
    for row in board.iter_mut().take(write) {
        *row = [None; COLS];
    }
    cleared
}

#[derive(Debug)]
struct Game {
    board: Board,
    active: Piece,
    lines: u32,
    paused: bool,
    over: bool,
    rng: u64,
}

impl Game {
    fn new(seed: u64) -> Self {
        let mut game = Self {
            board: [[None; COLS]; ROWS],
            active: Piece {
                kind: PieceKind::Bar,
                color: ColorId::Teal,
                rot: 0,
                col: 2,
                row: 0,
            },
            lines: 0,
            paused: false,
            over: false,
            rng: seed.max(1),
        };
        game.active = game.next_piece();
        game
    }

    fn level(&self) -> u32 {
        self.lines / 10 + 1
    }

    fn tick_delay(&self) -> Duration {
        Duration::from_millis(
            650_u64
                .saturating_sub((self.level() - 1) as u64 * 55)
                .max(120),
        )
    }

    fn next_random(&mut self) -> usize {
        self.rng = self
            .rng
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        (self.rng >> 32) as usize
    }

    fn next_piece(&mut self) -> Piece {
        let kind = PieceKind::ALL[self.next_random() % PieceKind::ALL.len()];
        let color = ColorId::ALL[self.next_random() % ColorId::ALL.len()];
        let rot = (self.next_random() % 4) as u8;
        let width = cells(kind, rot).into_iter().map(|(x, _)| x).max().unwrap() + 1;
        Piece {
            kind,
            color,
            rot,
            col: (COLS as i32 - width) / 2,
            row: 0,
        }
    }

    fn try_move(&mut self, dc: i32, dr: i32) -> bool {
        let mut moved = self.active;
        moved.col += dc;
        moved.row += dr;
        if fits(&self.board, moved) {
            self.active = moved;
            true
        } else {
            false
        }
    }

    fn rotate(&mut self) -> bool {
        let mut rotated = self.active;
        rotated.rot = (rotated.rot + 1) % 4;
        for kick in [0, -1, 1] {
            rotated.col = self.active.col + kick;
            if fits(&self.board, rotated) {
                self.active = rotated;
                return true;
            }
        }
        false
    }

    fn landing_piece(&self) -> Piece {
        let mut piece = self.active;
        while {
            let mut next = piece;
            next.row += 1;
            if fits(&self.board, next) {
                piece = next;
                true
            } else {
                false
            }
        } {}
        piece
    }

    fn settle(&mut self) -> Settlement {
        let piece = self.active;
        lock(&mut self.board, piece);
        let cleared = clear_full_rows(&mut self.board);
        self.lines += cleared.len() as u32;
        self.active = self.next_piece();
        self.over = !fits(&self.board, self.active);
        Settlement { piece, cleared }
    }
}

struct Settlement {
    piece: Piece,
    cleared: Vec<usize>,
}

struct Scene {
    compositor: Compositor,
    root: ContainerVisual,
    background: SpriteVisual,
    settled: ContainerVisual,
    overlay: SpriteVisual,
    graphics: CompositionGraphicsDevice,
    surface: CompositionDrawingSurface,
    cell_geometry: CompositionRoundedRectangleGeometry,
    cell_brushes: [CompositionColorBrush; ColorId::ALL.len()],
    visuals: VisualBoard,
    device: GpuDevice,
    fades: Vec<Fade>,
    next_fade: u64,
    cell: f32,
    left: f32,
    top: f32,
    scale: f32,
}

struct Fade {
    id: u64,
    visuals: Vec<ShapeVisual>,
    _completion: windows_core::EventRevoker,
}

impl Scene {
    fn build(
        compositor: windows_core::IUnknown,
        host: &ElementRef<Grid>,
        game: &Game,
        width: f32,
        height: f32,
        scale: f32,
    ) -> Result<Self> {
        let compositor = Compositor::from_host(compositor)?;
        let root = compositor.create_container_visual();
        let background = compositor.create_sprite_visual();
        background.set_brush(&compositor.create_color_brush(CompositionColor::rgb(30, 38, 48)));
        root.children().insert_at_bottom(&background);

        let settled = compositor.create_container_visual();
        root.children().insert_at_top(&settled);

        let device = GpuDevice::new_or_warp()?;
        let graphics = device.create_graphics_device(&compositor)?;
        let surface = graphics.create_drawing_surface(1.0, 1.0)?;
        let overlay = compositor.create_sprite_visual();
        let overlay_brush = compositor.create_surface_brush(&surface);
        overlay_brush.set_stretch(SurfaceStretch::Fill);
        overlay.set_brush(&overlay_brush);
        root.children().insert_at_top(&overlay);

        let cell_geometry = compositor.create_rounded_rectangle_geometry();
        cell_geometry.set_size(Vector2::new(
            ASSET_SIZE * (1.0 - CELL_INSET * 2.0),
            ASSET_SIZE * (1.0 - CELL_INSET * 2.0),
        ));
        cell_geometry.set_corner_radius(Vector2::new(
            ASSET_SIZE * CELL_RADIUS,
            ASSET_SIZE * CELL_RADIUS,
        ));
        let cell_brushes =
            ColorId::ALL.map(|color| compositor.create_color_brush(color.composition()));

        let _ = host.request_set_child_visual(Some(root.host_visual()), |result| {
            if let Err(error) = result {
                eprintln!("failed to attach Stacker visuals: {error:?}");
            }
        });

        let mut scene = Self {
            compositor,
            root,
            background,
            settled,
            overlay,
            graphics,
            surface,
            cell_geometry,
            cell_brushes,
            visuals: empty_visuals(),
            device,
            fades: Vec::new(),
            next_fade: 0,
            cell: 1.0,
            left: 0.0,
            top: 0.0,
            scale: 0.0,
        };
        scene.layout(width, height, scale)?;
        scene.rebuild(&game.board);
        scene.draw_active(game)?;
        Ok(scene)
    }

    fn layout(&mut self, width: f32, height: f32, scale: f32) -> Result<()> {
        let width = width.max(1.0);
        let height = height.max(1.0);
        self.scale = scale.max(0.01);
        self.root.set_size(width, height);
        self.cell = (width / COLS as f32).min(height / ROWS as f32).max(1.0);
        let well_width = self.cell * COLS as f32;
        let well_height = self.cell * ROWS as f32;
        self.left = (width - well_width) / 2.0;
        self.top = (height - well_height) / 2.0;

        self.background.set_size(well_width, well_height);
        self.background.set_offset(self.left, self.top, 0.0);
        self.overlay.set_size(well_width, well_height);
        self.overlay.set_offset(self.left, self.top, 0.0);
        self.surface.resize(
            (well_width * self.scale).ceil() as i32,
            (well_height * self.scale).ceil() as i32,
        )?;

        for row in 0..ROWS {
            for col in 0..COLS {
                if let Some(visual) = &self.visuals[row][col] {
                    self.place_visual(visual, row, col);
                }
            }
        }
        Ok(())
    }

    fn place_visual(&self, visual: &Visual, row: usize, col: usize) {
        visual.set_size(ASSET_SIZE, ASSET_SIZE);
        let scale = self.cell / ASSET_SIZE;
        visual.set_scale(Vector3::new(scale, scale, 1.0));
        visual.set_offset(
            self.left + col as f32 * self.cell,
            self.top + row as f32 * self.cell,
            0.0,
        );
    }

    fn add_visual(&self, row: usize, col: usize, color: ColorId) -> ShapeVisual {
        let shape = self.compositor.create_sprite_shape(&self.cell_geometry);
        shape.set_offset(Vector2::new(
            ASSET_SIZE * CELL_INSET,
            ASSET_SIZE * CELL_INSET,
        ));
        shape.set_fill_brush(&self.cell_brushes[color as usize]);
        let visual = self.compositor.create_shape_visual();
        visual.shapes().append(&shape);
        self.place_visual(&visual, row, col);
        self.settled.children().insert_at_top(&visual);
        visual
    }

    fn rebuild(&mut self, board: &Board) {
        self.settled.children().remove_all();
        self.visuals = empty_visuals();
        self.fades.clear();
        for (row, cells) in board.iter().enumerate() {
            for (col, color) in cells.iter().enumerate() {
                if let Some(color) = color {
                    self.visuals[row][col] = Some(self.add_visual(row, col, *color));
                }
            }
        }
    }

    fn commit(&mut self, settlement: &Settlement, sender: LocalSender<Message>) -> Result<()> {
        for (x, y) in cells(settlement.piece.kind, settlement.piece.rot) {
            let row = (settlement.piece.row + y) as usize;
            let col = (settlement.piece.col + x) as usize;
            if self.visuals[row][col].is_none() {
                self.visuals[row][col] = Some(self.add_visual(row, col, settlement.piece.color));
            }
        }

        let batch = self.compositor.create_scoped_batch(BatchKind::Animation);
        let mut fading = Vec::new();
        for &row in &settlement.cleared {
            for col in 0..COLS {
                if let Some(visual) = self.visuals[row][col].take() {
                    visual.set_center_point(Vector3::new(ASSET_SIZE / 2.0, ASSET_SIZE / 2.0, 0.0));
                    let fade = self.compositor.create_scalar_key_frame_animation();
                    fade.insert_key_frame(0.0, 1.0);
                    fade.insert_key_frame(1.0, 0.0);
                    fade.set_duration(Duration::from_millis(180));
                    visual.start_animation("Opacity", &fade);

                    let shrink = self.compositor.create_vector3_key_frame_animation();
                    let scale = self.cell / ASSET_SIZE;
                    shrink.insert_key_frame(0.0, Vector3::new(scale, scale, 1.0));
                    shrink.insert_key_frame(1.0, Vector3::new(scale * 0.65, scale * 0.65, 1.0));
                    shrink.set_duration(Duration::from_millis(180));
                    visual.start_animation("Scale", &shrink);
                    fading.push(visual);
                }
            }
        }

        let mut compacted = empty_visuals();
        for row in (0..ROWS).rev() {
            if settlement.cleared.contains(&row) {
                continue;
            }
            let drop = settlement
                .cleared
                .iter()
                .filter(|cleared_row| **cleared_row > row)
                .count();
            let new_row = row + drop;
            for col in 0..COLS {
                if let Some(visual) = self.visuals[row][col].take() {
                    visual.set_center_point(Vector3::new(0.0, 0.0, 0.0));
                    self.place_visual(&visual, new_row, col);
                    compacted[new_row][col] = Some(visual);
                }
            }
        }
        self.visuals = compacted;
        if !fading.is_empty() {
            let id = self.next_fade;
            self.next_fade = self.next_fade.wrapping_add(1);
            let completion = batch.on_completed(move || {
                sender.send(Message::FadeCompleted(id));
            });
            batch.end();
            let completion = completion?;
            self.fades.push(Fade {
                id,
                visuals: fading,
                _completion: completion,
            });
        } else {
            batch.end();
        }
        Ok(())
    }

    fn finish_fade(&mut self, id: u64) {
        let Some(index) = self.fades.iter().position(|fade| fade.id == id) else {
            return;
        };
        let fade = self.fades.swap_remove(index);
        for visual in fade.visuals {
            self.settled.children().remove(&visual);
        }
    }

    fn draw_active(&mut self, game: &Game) -> Result<()> {
        if self.try_draw_active(game)? {
            return Ok(());
        }

        let device = GpuDevice::new_or_warp()?;
        device.replace_graphics_device(&self.graphics)?;
        self.device = device;
        if self.try_draw_active(game)? {
            Ok(())
        } else {
            Err(device_lost_error())
        }
    }

    fn try_draw_active(&self, game: &Game) -> Result<bool> {
        let cell = self.cell;
        let active = game.active;
        let landing = game.landing_piece();
        let width = cell * COLS as f32;
        let height = cell * ROWS as f32;
        let transform = Matrix3x2::scale(self.scale, self.scale);
        self.surface.draw(|session| {
            session.clear(ColorF::TRANSPARENT);

            let label = if game.over {
                Some("GAME OVER\nPress N for a new game")
            } else if game.paused {
                Some("PAUSED\nPress P to continue")
            } else {
                None
            };
            let outline = session.create_solid_brush(ColorF::from_rgba8(220, 226, 232, 110))?;
            let active_brush = session.create_solid_brush(active.color.canvas())?;
            let overlay = if label.is_some() {
                Some((
                    session.create_solid_brush(ColorF::from_rgba8(15, 20, 27, 190))?,
                    session.create_solid_brush(ColorF::from_rgb8(235, 239, 242))?,
                    TextFormat::new_bold("Segoe UI Variable", cell * 0.62)?
                        .with_alignment(TextAlignment::Center)
                        .with_paragraph_alignment(ParagraphAlignment::Center)
                        .with_word_wrapping(WordWrapping::NoWrap),
                ))
            } else {
                None
            };
            session.with_transform(&transform, || {
                if !game.over {
                    for (x, y) in cells(landing.kind, landing.rot) {
                        let rect = cell_rect(landing.col + x, landing.row + y, cell);
                        session.draw_rounded_rect(&rect, &outline, 1.5);
                    }
                    for (x, y) in cells(active.kind, active.rot) {
                        let rect = cell_rect(active.col + x, active.row + y, cell);
                        session.fill_rounded_rect(&rect, &active_brush);
                    }
                }
                if let (Some(label), Some((veil, text, format))) = (label, overlay.as_ref()) {
                    session.fill_rect(&Rect::from_xywh(0.0, 0.0, width, height), veil);
                    session.draw_text(
                        label,
                        format,
                        &Rect::from_xywh(0.0, 0.0, width, height),
                        text,
                    );
                }
            });
            Ok(())
        })
    }
}

fn cell_rect(col: i32, row: i32, cell: f32) -> RoundedRect {
    let inset = cell * CELL_INSET;
    RoundedRect::uniform(
        Rect::from_xywh(
            col as f32 * cell + inset,
            row as f32 * cell + inset,
            cell - inset * 2.0,
            cell - inset * 2.0,
        ),
        cell * CELL_RADIUS,
    )
}

fn empty_visuals() -> VisualBoard {
    std::array::from_fn(|_| std::array::from_fn(|_| None))
}

#[derive(Clone)]
enum Message {
    Host(CompositionHostEvent),
    FadeCompleted(u64),
    Tick,
    MoveLeft,
    MoveRight,
    Rotate,
    SoftDrop,
    HardDrop,
    Pause,
    NewGame,
}

struct Stacker {
    game: Game,
    scene: Option<Scene>,
    host: ElementRef<Grid>,
    timer: Option<ComponentTimer>,
}

impl Stacker {
    fn schedule_tick(
        context: &ComponentContext<Self>,
        delay: Duration,
    ) -> windows_core::Result<ComponentTimer> {
        context.set_timeout(delay, Message::Tick)
    }

    fn replace_timer(&mut self, context: &ComponentContext<Self>) {
        self.cancel_timer();
        match Self::schedule_tick(context, self.game.tick_delay()) {
            Ok(timer) => self.timer = Some(timer),
            Err(error) => {
                self.game.paused = true;
                eprintln!("Stacker gravity paused because its timer could not start: {error}");
            }
        }
    }

    fn cancel_timer(&mut self) {
        if let Some(timer) = self.timer.take() {
            timer.cancel();
        }
    }

    fn settle(&mut self, context: &ComponentContext<Self>) {
        let settlement = self.game.settle();
        if let Some(scene) = &mut self.scene
            && let Err(error) = scene.commit(&settlement, context.sender())
        {
            eprintln!("failed to animate cleared Stacker rows: {error}");
            scene.rebuild(&self.game.board);
        }
    }

    fn refresh_scene(&mut self) {
        if let Some(scene) = &mut self.scene
            && let Err(error) = scene.draw_active(&self.game)
        {
            eprintln!("failed to redraw Stacker canvas: {error}");
        }
    }
}

impl Component for Stacker {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), context: &ComponentContext<Self>) -> Self {
        let mut game = Game::new(seed_from_clock());
        let timer = match Self::schedule_tick(context, game.tick_delay()) {
            Ok(timer) => Some(timer),
            Err(error) => {
                game.paused = true;
                eprintln!("Stacker gravity paused because its timer could not start: {error}");
                None
            }
        };
        Self {
            game,
            scene: None,
            host: ElementRef::new(),
            timer,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        let can_play = !self.game.paused && !self.game.over;
        let changed = match message {
            Message::Host(CompositionHostEvent::Ready {
                compositor,
                width,
                height,
                scale,
            }) => {
                match Scene::build(
                    compositor,
                    &self.host,
                    &self.game,
                    width as f32,
                    height as f32,
                    scale as f32,
                ) {
                    Ok(scene) => self.scene = Some(scene),
                    Err(error) => eprintln!("failed to initialize Stacker scene: {error}"),
                }
                false
            }
            Message::Host(CompositionHostEvent::Metrics {
                width,
                height,
                scale,
            }) => {
                if let Some(scene) = &mut self.scene {
                    if let Err(error) = scene.layout(width as f32, height as f32, scale as f32) {
                        eprintln!("failed to resize Stacker scene: {error}");
                    } else if let Err(error) = scene.draw_active(&self.game) {
                        eprintln!("failed to redraw Stacker canvas: {error}");
                    }
                }
                false
            }
            Message::FadeCompleted(id) => {
                if let Some(scene) = &mut self.scene {
                    scene.finish_fade(id);
                }
                false
            }
            Message::Tick => {
                self.timer = None;
                if can_play && !self.game.try_move(0, 1) {
                    self.settle(context);
                }
                if !self.game.paused && !self.game.over {
                    self.replace_timer(context);
                }
                can_play
            }
            Message::MoveLeft if can_play => self.game.try_move(-1, 0),
            Message::MoveRight if can_play => self.game.try_move(1, 0),
            Message::Rotate if can_play => self.game.rotate(),
            Message::SoftDrop if can_play => {
                if !self.game.try_move(0, 1) {
                    self.settle(context);
                }
                if self.game.over {
                    self.cancel_timer();
                } else {
                    self.replace_timer(context);
                }
                true
            }
            Message::HardDrop if can_play => {
                self.game.active = self.game.landing_piece();
                self.settle(context);
                if self.game.over {
                    self.cancel_timer();
                } else {
                    self.replace_timer(context);
                }
                true
            }
            Message::Pause if !self.game.over => {
                self.game.paused = !self.game.paused;
                self.cancel_timer();
                if !self.game.paused {
                    self.replace_timer(context);
                }
                true
            }
            Message::NewGame => {
                self.game = Game::new(seed_from_clock());
                self.replace_timer(context);
                if let Some(scene) = &mut self.scene {
                    scene.rebuild(&self.game.board);
                }
                true
            }
            _ => false,
        };
        if changed {
            self.refresh_scene();
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Stacker");
        context.window_visuals(WindowVisuals::new().client_size(520.0, 760.0).constraints(
            WindowConstraints {
                min_width: Some(360.0),
                min_height: Some(560.0),
                ..Default::default()
            },
        ));

        let host = self.host.clone();
        let sender = context.sender();
        context.use_effect_guard("stacker-composition", (), move || {
            host.observe_composition_host(move |event| {
                sender.send(Message::Host(event));
            })
        });

        let accelerators = [
            (AcceleratorKey::Left, Message::MoveLeft),
            (AcceleratorKey::Right, Message::MoveRight),
            (AcceleratorKey::Up, Message::Rotate),
            (AcceleratorKey::Down, Message::SoftDrop),
            (AcceleratorKey::Space, Message::HardDrop),
            (AcceleratorKey::P, Message::Pause),
            (AcceleratorKey::N, Message::NewGame),
        ]
        .map(|(key, message)| {
            KeyAccelerator::new(key, AcceleratorModifiers::None, context.message(message))
        });

        let header = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(16.0)
            .margin(Thickness::new(16.0, 10.0, 16.0, 10.0))
            .horizontal_alignment(HorizontalAlignment::Center)
            .grid_row(1)
            .children((
                TextBlock::new()
                    .text(format!(
                        "Score  {}    Lines  {}    Level  {}",
                        self.game.lines,
                        self.game.lines,
                        self.game.level()
                    ))
                    .font_weight(FontWeight::BOLD)
                    .vertical_alignment(VerticalAlignment::Center),
                Button::new()
                    .on_click(context.message(Message::Pause))
                    .content(if self.game.paused { "Resume" } else { "Pause" }),
                Button::new()
                    .on_click(context.message(Message::NewGame))
                    .content("New game"),
            ));

        let playfield = Grid::new().element_ref(&self.host).grid_row(2);

        Grid::new()
            .rows([GridLength::Auto, GridLength::Auto, GridLength::STAR])
            .background(Color::rgb(22, 28, 36))
            .key_accelerators(KeyAccelerators::new(accelerators))
            .children((
                TitleBar::new().title("Stacker").grid_row(0),
                header,
                playfield,
            ))
    }
}

fn seed_from_clock() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0x9E37_79B9_7F4A_7C15, |duration| duration.as_nanos() as u64)
}

fn main() -> Result<()> {
    App::run_component::<Stacker>(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_piece_has_three_cells_in_every_rotation() {
        for kind in PieceKind::ALL {
            for rot in 0..4 {
                let cells = cells(kind, rot);
                assert_eq!(cells.len(), 3);
                assert!(cells.iter().all(|(x, y)| *x >= 0 && *y >= 0));
                assert_ne!(cells[0], cells[1]);
                assert_ne!(cells[0], cells[2]);
                assert_ne!(cells[1], cells[2]);
            }
        }
    }

    #[test]
    fn fits_rejects_walls_floor_and_occupied_cells() {
        let mut board = [[None; COLS]; ROWS];
        let piece = Piece {
            kind: PieceKind::Bar,
            color: ColorId::Teal,
            rot: 0,
            col: 2,
            row: 0,
        };
        assert!(fits(&board, piece));
        assert!(!fits(&board, Piece { col: -1, ..piece }));
        assert!(!fits(
            &board,
            Piece {
                row: ROWS as i32,
                ..piece
            }
        ));
        board[0][2] = Some(ColorId::Steel);
        assert!(!fits(&board, piece));
    }

    #[test]
    fn lock_writes_exactly_three_cells() {
        let mut board = [[None; COLS]; ROWS];
        lock(
            &mut board,
            Piece {
                kind: PieceKind::Corner,
                color: ColorId::Plum,
                rot: 0,
                col: 3,
                row: 4,
            },
        );
        assert_eq!(
            board.iter().flatten().filter(|cell| cell.is_some()).count(),
            3
        );
    }

    #[test]
    fn clear_full_rows_compacts_remaining_cells() {
        let mut board = [[None; COLS]; ROWS];
        board[ROWS - 1].fill(Some(ColorId::Amber));
        board[ROWS - 2][3] = Some(ColorId::Olive);
        assert_eq!(clear_full_rows(&mut board), vec![ROWS - 1]);
        assert_eq!(board[ROWS - 1][3], Some(ColorId::Olive));
        assert!(board[0].iter().all(Option::is_none));
    }

    #[test]
    fn clear_full_rows_handles_multiple_rows() {
        let mut board = [[None; COLS]; ROWS];
        board[ROWS - 1].fill(Some(ColorId::Amber));
        board[ROWS - 2].fill(Some(ColorId::Plum));
        board[ROWS - 3][5] = Some(ColorId::Olive);

        assert_eq!(clear_full_rows(&mut board), vec![ROWS - 2, ROWS - 1]);
        assert_eq!(board[ROWS - 1][5], Some(ColorId::Olive));
        assert!(board[..ROWS - 1].iter().flatten().all(Option::is_none));
    }

    #[test]
    fn settle_updates_lines_score_and_level() {
        let mut game = Game::new(7);
        game.lines = 9;
        game.active = Piece {
            kind: PieceKind::Bar,
            color: ColorId::Teal,
            rot: 0,
            col: 0,
            row: ROWS as i32 - 1,
        };
        game.board[ROWS - 1][3..].fill(Some(ColorId::Steel));

        let settlement = game.settle();

        assert_eq!(settlement.cleared, vec![ROWS - 1]);
        assert_eq!(game.lines, 10);
        assert_eq!(game.level(), 2);
        assert!(game.board.iter().flatten().all(Option::is_none));
        assert!(!game.over);
    }

    #[test]
    fn rotation_uses_a_one_cell_wall_kick() {
        let mut game = Game::new(4);
        game.active = Piece {
            kind: PieceKind::Bar,
            color: ColorId::Teal,
            rot: 1,
            col: COLS as i32 - 2,
            row: 2,
        };
        game.rotate();
        assert_eq!(game.active.rot, 2);
        assert_eq!(game.active.col, COLS as i32 - 3);
        assert!(fits(&game.board, game.active));
    }

    #[test]
    fn landing_piece_stops_on_the_floor() {
        let mut game = Game::new(7);
        game.active = Piece {
            kind: PieceKind::Bar,
            color: ColorId::Steel,
            rot: 0,
            col: 2,
            row: 0,
        };
        let landing = game.landing_piece();
        assert_eq!(landing.row, ROWS as i32 - 1);
        assert!(fits(&game.board, landing));
    }
}
