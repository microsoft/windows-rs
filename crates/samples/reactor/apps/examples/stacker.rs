//! Original puzzle demonstrating Reactor chrome, a Composition settled board, and a Canvas
//! active piece and overlay.

#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use windows_canvas::{
    CanvasCompositionExt, ColorF, GpuDevice, Matrix3x2, ParagraphAlignment, Rect, RoundedRect,
    TextAlignment, TextFormat, WordWrapping,
};
use windows_composition::{
    Color as CompositionColor, CompositionDrawingSurface, CompositionGraphicsDevice, Compositor,
    ContainerVisual, ShapeVisual, SpriteVisual, SurfaceStretch, Vector2 as CompositionVector2,
    Vector3, Visual,
};
use windows_core::Result;
use windows_reactor::*;

const COLS: usize = 8;
const ROWS: usize = 16;
const CELLS: usize = COLS * ROWS;
const ASSET_SIZE: f32 = 64.0;

type Board = [[Option<ColorId>; COLS]; ROWS];

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
    Elbow,
    Step,
    Notch,
}

impl PieceKind {
    const ALL: [Self; 5] = [
        Self::Bar,
        Self::Corner,
        Self::Elbow,
        Self::Step,
        Self::Notch,
    ];
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
        PieceKind::Elbow => [(0, 0), (1, 0), (0, 1)],
        PieceKind::Step => [(0, 0), (1, 0), (1, 1)],
        PieceKind::Notch => [(0, 0), (1, 0), (0, 1)],
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

fn full_rows(board: &Board) -> Vec<usize> {
    board
        .iter()
        .enumerate()
        .filter_map(|(row, cells)| cells.iter().all(Option::is_some).then_some(row))
        .collect()
}

fn clear_full_rows(board: &mut Board) -> u32 {
    let cleared = full_rows(board).len() as u32;
    let mut write = ROWS;
    for read in (0..ROWS).rev() {
        if board[read].iter().all(Option::is_some) {
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct Game {
    board: Board,
    active: Piece,
    score: u32,
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
            score: 0,
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
        let width = cells(kind, 0).into_iter().map(|(x, _)| x).max().unwrap() + 1;
        Piece {
            kind,
            color,
            rot: 0,
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

    fn rotate(&mut self) {
        let mut rotated = self.active;
        rotated.rot = (rotated.rot + 1) % 4;
        for kick in [0, -1, 1] {
            rotated.col = self.active.col + kick;
            if fits(&self.board, rotated) {
                self.active = rotated;
                break;
            }
        }
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

    fn settle(&mut self) -> (Board, Vec<usize>) {
        lock(&mut self.board, self.active);
        let before_clear = self.board;
        let cleared_rows = full_rows(&self.board);
        let cleared = clear_full_rows(&mut self.board);
        self.lines += cleared;
        self.score = self.lines;
        self.active = self.next_piece();
        self.over = !fits(&self.board, self.active);
        (before_clear, cleared_rows)
    }
}

struct Scene {
    compositor: Compositor,
    root: ContainerVisual,
    background: SpriteVisual,
    settled: ContainerVisual,
    overlay: SpriteVisual,
    _graphics: CompositionGraphicsDevice,
    surface: CompositionDrawingSurface,
    visuals: Vec<Option<ShapeVisual>>,
    _device: GpuDevice,
    fading: Vec<(ShapeVisual, std::time::Instant)>,
    width: f32,
    height: f32,
    cell: f32,
    left: f32,
    top: f32,
    scale: f32,
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

        let _ = host.request_set_child_visual(Some(root.as_raw().into()), |result| {
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
            _graphics: graphics,
            surface,
            visuals: std::iter::repeat_with(|| None).take(CELLS).collect(),
            _device: device,
            fading: Vec::new(),
            width: 1.0,
            height: 1.0,
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
        self.width = width.max(1.0);
        self.height = height.max(1.0);
        self.scale = scale.max(0.01);
        self.root.set_size(self.width, self.height);
        self.cell = (self.width / COLS as f32)
            .min(self.height / ROWS as f32)
            .max(1.0);
        let well_width = self.cell * COLS as f32;
        let well_height = self.cell * ROWS as f32;
        self.left = (self.width - well_width) / 2.0;
        self.top = (self.height - well_height) / 2.0;

        self.background.set_size(well_width, well_height);
        self.background.set_offset(self.left, self.top, 0.0);
        self.overlay.set_size(well_width, well_height);
        self.overlay.set_offset(self.left, self.top, 0.0);
        self.surface.resize(
            (well_width * self.scale).ceil() as i32,
            (well_height * self.scale).ceil() as i32,
        )?;

        for (index, visual) in self.visuals.iter().enumerate() {
            if let Some(visual) = visual {
                self.place_visual(visual, index / COLS, index % COLS);
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
        let geometry = self.compositor.create_rounded_rectangle_geometry();
        geometry.set_size(CompositionVector2::new(ASSET_SIZE - 6.0, ASSET_SIZE - 6.0));
        geometry.set_corner_radius(CompositionVector2::new(9.0, 9.0));
        let shape = self.compositor.create_sprite_shape(&geometry);
        shape.set_offset(CompositionVector2::new(3.0, 3.0));
        shape.set_fill_brush(&self.compositor.create_color_brush(color.composition()));
        let visual = self.compositor.create_shape_visual();
        visual.shapes().append(&shape);
        self.place_visual(&visual, row, col);
        self.settled.children().insert_at_top(&visual);
        visual
    }

    fn rebuild(&mut self, board: &Board) {
        self.settled.children().remove_all();
        self.visuals = std::iter::repeat_with(|| None).take(CELLS).collect();
        self.fading.clear();
        for (row, cells) in board.iter().enumerate() {
            for (col, color) in cells.iter().enumerate() {
                if let Some(color) = color {
                    self.visuals[row * COLS + col] = Some(self.add_visual(row, col, *color));
                }
            }
        }
    }

    fn commit(&mut self, board: &Board, cleared: &[usize]) {
        for (row, cells) in board.iter().enumerate() {
            for (col, color) in cells.iter().enumerate() {
                let index = row * COLS + col;
                if self.visuals[index].is_none()
                    && let Some(color) = color
                {
                    self.visuals[index] = Some(self.add_visual(row, col, *color));
                }
            }
        }

        for &row in cleared {
            for col in 0..COLS {
                if let Some(visual) = self.visuals[row * COLS + col].take() {
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
                    self.fading.push((visual, std::time::Instant::now()));
                }
            }
        }

        let mut compacted: Vec<Option<ShapeVisual>> =
            std::iter::repeat_with(|| None).take(CELLS).collect();
        for row in (0..ROWS).rev() {
            if cleared.contains(&row) {
                continue;
            }
            let drop = cleared
                .iter()
                .filter(|cleared_row| **cleared_row > row)
                .count();
            let new_row = row + drop;
            for col in 0..COLS {
                if let Some(visual) = self.visuals[row * COLS + col].take() {
                    visual.set_center_point(Vector3::new(0.0, 0.0, 0.0));
                    self.place_visual(&visual, new_row, col);
                    compacted[new_row * COLS + col] = Some(visual);
                }
            }
        }
        self.visuals = compacted;
    }

    fn remove_finished_fades(&mut self) {
        let mut active = Vec::new();
        for (visual, started) in self.fading.drain(..) {
            if started.elapsed() >= Duration::from_millis(180) {
                self.settled.children().remove(&visual);
            } else {
                active.push((visual, started));
            }
        }
        self.fading = active;
    }

    fn draw_active(&mut self, game: &Game) -> Result<()> {
        if self.try_draw_active(game)? {
            return Ok(());
        }

        let device = GpuDevice::new_or_warp()?;
        device.replace_graphics_device(&self._graphics)?;
        self._device = device;
        if self.try_draw_active(game)? {
            Ok(())
        } else {
            Err(windows_canvas::device_lost_error())
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
            let veil = session.create_solid_brush(ColorF::from_rgba8(15, 20, 27, 190))?;
            let text = session.create_solid_brush(ColorF::from_rgb8(235, 239, 242))?;
            let format = TextFormat::new_bold("Segoe UI Variable", cell * 0.62)?
                .with_alignment(TextAlignment::Center)
                .with_paragraph_alignment(ParagraphAlignment::Center)
                .with_word_wrapping(WordWrapping::NoWrap);
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
                if let Some(label) = label {
                    session.fill_rect(&Rect::from_xywh(0.0, 0.0, width, height), &veil);
                    session.draw_text(
                        label,
                        &format,
                        &Rect::from_xywh(0.0, 0.0, width, height),
                        &text,
                    );
                }
            });
            Ok(())
        })
    }
}

fn cell_rect(col: i32, row: i32, cell: f32) -> RoundedRect {
    let inset = (cell * 0.09).max(1.0);
    RoundedRect::uniform(
        Rect::from_xywh(
            col as f32 * cell + inset,
            row as f32 * cell + inset,
            cell - inset * 2.0,
            cell - inset * 2.0,
        ),
        (cell * 0.15).max(2.0),
    )
}

#[derive(Clone, Copy)]
enum Message {
    Tick(u64),
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
    scene: Rc<RefCell<Option<Scene>>>,
    shared_game: Rc<RefCell<Game>>,
    host: ElementRef<Grid>,
    play_host: ElementRef<Button>,
    timer_generation: u64,
}

impl Stacker {
    fn schedule_tick(context: &ComponentContext<Self>, delay: Duration, generation: u64) {
        context.spawn_background(move |_| {
            std::thread::sleep(delay);
            Message::Tick(generation)
        });
    }

    fn settle(&mut self) {
        let (board, cleared) = self.game.settle();
        if let Some(scene) = self.scene.borrow_mut().as_mut() {
            scene.commit(&board, &cleared);
        }
    }

    fn refresh_scene(&mut self) {
        *self.shared_game.borrow_mut() = self.game.clone();
        if let Some(scene) = self.scene.borrow_mut().as_mut()
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
        let game = Game::new(seed_from_clock());
        Self::schedule_tick(context, game.tick_delay(), 0);
        Self {
            shared_game: Rc::new(RefCell::new(game.clone())),
            game,
            scene: Rc::new(RefCell::new(None)),
            host: ElementRef::new(),
            play_host: ElementRef::new(),
            timer_generation: 0,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        if let Some(scene) = self.scene.borrow_mut().as_mut() {
            scene.remove_finished_fades();
        }
        let can_play = !self.game.paused && !self.game.over;
        match message {
            Message::Tick(generation) if generation == self.timer_generation => {
                if can_play && !self.game.try_move(0, 1) {
                    self.settle();
                }
                if !self.game.paused && !self.game.over {
                    Self::schedule_tick(context, self.game.tick_delay(), self.timer_generation);
                }
            }
            Message::MoveLeft if can_play => {
                self.game.try_move(-1, 0);
            }
            Message::MoveRight if can_play => {
                self.game.try_move(1, 0);
            }
            Message::Rotate if can_play => self.game.rotate(),
            Message::SoftDrop if can_play => {
                if !self.game.try_move(0, 1) {
                    self.settle();
                }
            }
            Message::HardDrop if can_play => {
                self.game.active = self.game.landing_piece();
                self.settle();
            }
            Message::Pause if !self.game.over => {
                self.game.paused = !self.game.paused;
                self.timer_generation = self.timer_generation.wrapping_add(1);
                if !self.game.paused {
                    Self::schedule_tick(context, self.game.tick_delay(), self.timer_generation);
                }
            }
            Message::NewGame => {
                self.game = Game::new(seed_from_clock());
                self.timer_generation = self.timer_generation.wrapping_add(1);
                Self::schedule_tick(context, self.game.tick_delay(), self.timer_generation);
                if let Some(scene) = self.scene.borrow_mut().as_mut() {
                    scene.rebuild(&self.game.board);
                }
            }
            _ => {}
        }
        let _ = self.play_host.request_focus();
        self.refresh_scene();
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
        let scene = Rc::clone(&self.scene);
        let game = Rc::clone(&self.shared_game);
        context.use_effect("stacker-composition", (), move || {
            let event_host = host.clone();
            let observation = host.observe_composition_host(move |event| match event {
                CompositionHostEvent::Ready {
                    compositor,
                    width,
                    height,
                    scale,
                } => {
                    scene.borrow_mut().take();
                    match Scene::build(
                        compositor,
                        &event_host,
                        &game.borrow(),
                        width as f32,
                        height as f32,
                        scale as f32,
                    ) {
                        Ok(built) => *scene.borrow_mut() = Some(built),
                        Err(error) => eprintln!("failed to initialize Stacker scene: {error}"),
                    }
                }
                CompositionHostEvent::Metrics {
                    width,
                    height,
                    scale,
                } => {
                    if let Some(scene) = scene.borrow_mut().as_mut() {
                        if let Err(error) = scene.layout(width as f32, height as f32, scale as f32)
                        {
                            eprintln!("failed to resize Stacker scene: {error}");
                        } else if let Err(error) = scene.draw_active(&game.borrow()) {
                            eprintln!("failed to redraw Stacker canvas: {error}");
                        }
                    }
                }
            });
            Some(Box::new(move || drop(observation)))
        });

        let play_host = self.play_host.clone();
        context.use_effect("stacker-focus", (), move || {
            let _ = play_host.request_focus();
            None
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
            .children((
                TextBlock::new()
                    .text(format!(
                        "Score  {}    Lines  {}    Level  {}",
                        self.game.score,
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

        let playfield = Button::new()
            .element_ref(&self.play_host)
            .key_accelerators(KeyAccelerators::new(accelerators))
            .horizontal_alignment(HorizontalAlignment::Stretch)
            .vertical_alignment(VerticalAlignment::Stretch)
            .horizontal_content_alignment(HorizontalAlignment::Stretch)
            .vertical_content_alignment(VerticalAlignment::Stretch)
            .resource_overrides(
                ResourceOverrides::new()
                    .set("ButtonBackground", Color::rgb(22, 28, 36))
                    .set("ButtonBackgroundPointerOver", Color::rgb(22, 28, 36))
                    .set("ButtonBackgroundPressed", Color::rgb(22, 28, 36)),
            )
            .content(Grid::new().element_ref(&self.host));

        Grid::new()
            .rows([GridLength::Auto, GridLength::Auto, GridLength::STAR])
            .background(Color::rgb(22, 28, 36))
            .children((
                TitleBar::new().title("Stacker").grid_row(0),
                Border::new().grid_row(1).content(header),
                Border::new().grid_row(2).content(playfield),
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
        assert_eq!(clear_full_rows(&mut board), 1);
        assert_eq!(board[ROWS - 1][3], Some(ColorId::Olive));
        assert!(board[0].iter().all(Option::is_none));
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
