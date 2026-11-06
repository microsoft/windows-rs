#![windows_subsystem = "windows"]

mod bindings {
    #![allow(
        non_snake_case,
        non_camel_case_types,
        non_upper_case_globals,
        dead_code,
        clippy::all
    )]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use bindings::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ptr::{null, null_mut};
use std::rc::Rc;
use std::time::{Duration, Instant};
use windows_core::{HRESULT, Interface, PCSTR, PCWSTR, Result, s};
use windows_text::{
    ColorLayer, Decoration, FontFace, FontWeight, GlyphImage, GlyphRun, PixelGeometry,
    RenderingParams, TextFormat, TextLayout, Vector2,
};
use windows_window::{Window, run_with};

const TEXT: &str = "Glyph atlas\n\
DirectWrite lays out and shapes this text, windows-text rasterizes each glyph once, and \
Direct3D 11 draws every glyph as a rectangle from one atlas texture. Scroll the mouse wheel to \
change the size and press C to switch between ClearType and grayscale.\n\
Scripts: caf\u{e9}, \u{39a}\u{3b1}\u{3bb}\u{3b7}\u{3bc}\u{3ad}\u{3c1}\u{3b1}, \
\u{5e9}\u{5dc}\u{5d5}\u{5dd}, \u{645}\u{631}\u{62d}\u{628}\u{627}, \u{65e5}\u{672c}\u{8a9e}, \
\u{928}\u{92e}\u{938}\u{94d}\u{924}\u{947}\n\
Emoji: \u{1f600} \u{1f3a8} \u{1f680} \u{1f44d}\u{1f3fd}\n\
Decorations: underline and strikethrough.";

const MARGIN: f32 = 24.0;
const BACKGROUND: [f32; 4] = [0.063, 0.078, 0.11, 1.0];
const FOREGROUND: [f32; 4] = [0.92, 0.93, 0.95, 1.0];

// Glyphs are rasterized at this many horizontal positions within a pixel.
const SUBPIXEL: f32 = 4.0;
const ATLAS_SIZE: u32 = 1024;

// The largest texture Direct3D 11 supports.
const MAX_ATLAS_SIZE: u32 = 16384;

// One rectangle to draw: where on the screen, which atlas texels, and the straight color. The
// layout matches `Instance` in the shader.
#[repr(C)]
#[derive(Copy, Clone)]
struct Instance {
    bounds: [f32; 4],
    texels: [f32; 4],
    color: [f32; 4],
}

#[derive(Copy, Clone, PartialEq)]
enum Mode {
    Grayscale,
    ClearType { bgr: bool },
}

#[derive(PartialEq, Eq, Hash)]
struct GlyphKey {
    face: FontFace,
    glyph: u16,
    size: u32,
    subpixel: u8,
    cleartype: bool,
}

// Where a glyph image is in the atlas and where it goes relative to the pen.
#[derive(Copy, Clone)]
struct Slot {
    left: i32,
    top: i32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

// A row of the atlas that glyphs of about the same height are packed into from left to right.
struct Shelf {
    y: u32,
    height: u32,
    x: u32,
}

struct AtlasFull;

// Glyph images packed into one texture. The texel at (0, 0) is white so decorations can be drawn
// as rectangles with full coverage. `Load` reads exact texels, so glyphs need no padding.
struct Atlas {
    texture: ID3D11Texture2D,
    view: ID3D11ShaderResourceView,
    glyphs: HashMap<GlyphKey, Option<Slot>>,
    layers: HashMap<(FontFace, u16), Option<Rc<[ColorLayer]>>>,
    size: u32,
    shelves: Vec<Shelf>,
    bottom: u32,
    uploads: usize,
    flushes: usize,
    raster: Duration,
    upload: Duration,
}

impl Atlas {
    fn new(device: &ID3D11Device, context: &ID3D11DeviceContext) -> Result<Self> {
        let (texture, view) = create_atlas_texture(device, context, ATLAS_SIZE)?;

        Ok(Self {
            texture,
            view,
            glyphs: HashMap::new(),
            layers: HashMap::new(),
            size: ATLAS_SIZE,
            shelves: Vec::new(),
            bottom: 1,
            uploads: 0,
            flushes: 0,
            raster: Duration::ZERO,
            upload: Duration::ZERO,
        })
    }

    // Replaces the texture with one twice the size. The glyphs are uploaded again as they are
    // used.
    fn grow(&mut self, device: &ID3D11Device, context: &ID3D11DeviceContext) -> Result<()> {
        self.size *= 2;
        assert!(self.size <= MAX_ATLAS_SIZE);
        (self.texture, self.view) = create_atlas_texture(device, context, self.size)?;
        self.clear();
        Ok(())
    }

    fn clear(&mut self) {
        self.glyphs.clear();
        self.shelves.clear();
        self.bottom = 1;
        self.flushes += 1;
    }

    fn allocate(&mut self, width: u32, height: u32) -> Option<(u32, u32)> {
        for shelf in &mut self.shelves {
            if height <= shelf.height
                && shelf.height <= height + height / 4 + 2
                && shelf.x + width <= self.size
            {
                let x = shelf.x;
                shelf.x += width;
                return Some((x, shelf.y));
            }
        }

        if width > self.size || self.bottom + height > self.size {
            return None;
        }

        let y = self.bottom;
        self.bottom += height;
        self.shelves.push(Shelf {
            y,
            height,
            x: width,
        });
        Some((0, y))
    }

    // Returns the glyph's slot, rasterizing and uploading it the first time it is used. `None`
    // is a glyph without ink, such as a space.
    fn glyph(
        &mut self,
        context: &ID3D11DeviceContext,
        key: GlyphKey,
        bgr: bool,
    ) -> std::result::Result<Option<Slot>, AtlasFull> {
        if let Some(slot) = self.glyphs.get(&key) {
            return Ok(*slot);
        }

        let size = f32::from_bits(key.size);
        let offset = Vector2::new(f32::from(key.subpixel) / SUBPIXEL, 0.0);

        let start = Instant::now();

        let image = if key.cleartype {
            key.face.rasterize_cleartype(key.glyph, size, offset)
        } else {
            key.face.rasterize(key.glyph, size, offset)
        };

        self.raster += start.elapsed();

        let slot = match image {
            None => None,
            Some(image) => {
                let (x, y) = self.allocate(image.width, image.height).ok_or(AtlasFull)?;
                let start = Instant::now();
                let pixels = expand(&image, bgr);
                let bounds = D3D11_BOX {
                    left: x,
                    top: y,
                    front: 0,
                    right: x + image.width,
                    bottom: y + image.height,
                    back: 1,
                };

                unsafe {
                    context.UpdateSubresource(
                        &self.texture,
                        0,
                        Some(&bounds),
                        pixels.as_ptr().cast(),
                        image.width * 4,
                        0,
                    );
                }

                self.uploads += 1;
                self.upload += start.elapsed();

                Some(Slot {
                    left: image.left,
                    top: image.top,
                    x,
                    y,
                    width: image.width,
                    height: image.height,
                })
            }
        };

        self.glyphs.insert(key, slot);
        Ok(slot)
    }

    // Color glyphs are looked up once per face and glyph, since splitting a glyph into layers
    // costs more than drawing it.
    fn layers(&mut self, face: &FontFace, glyph: u16) -> Option<Rc<[ColorLayer]>> {
        if !face.is_color_font() {
            return None;
        }

        self.layers
            .entry((face.clone(), glyph))
            .or_insert_with(|| face.color_layers(glyph).map(Rc::from))
            .clone()
    }

    // Builds the rectangles for the text with the layout box's top left at `origin` in pixels.
    // Glyphs are placed at a quarter pixel horizontally and on whole pixels vertically.
    fn build(
        &mut self,
        context: &ID3D11DeviceContext,
        runs: &[GlyphRun],
        decorations: &[Decoration],
        origin: f32,
        scale: f32,
        mode: Mode,
    ) -> std::result::Result<Vec<Instance>, AtlasFull> {
        let mut instances = Vec::new();
        let bgr = mode == Mode::ClearType { bgr: true };
        let cleartype = mode != Mode::Grayscale;

        for run in runs {
            let size = (run.font_size * scale).to_bits();

            for glyph in &run.glyphs {
                let x = origin + glyph.position.x * scale;
                let mut pixel = x.floor();
                let mut subpixel = ((x - pixel) * SUBPIXEL).round() as u8;

                if f32::from(subpixel) == SUBPIXEL {
                    pixel += 1.0;
                    subpixel = 0;
                }

                let pen = (pixel, (origin + glyph.position.y * scale).round());

                if let Some(layers) = self.layers(&run.font_face, glyph.id) {
                    for &layer in layers.iter() {
                        let key = GlyphKey {
                            face: run.font_face.clone(),
                            glyph: layer.glyph,
                            size,
                            subpixel,
                            cleartype: false,
                        };

                        if let Some(slot) = self.glyph(context, key, false)? {
                            let color = layer.color.unwrap_or(FOREGROUND);
                            instances.push(glyph_instance(pen, slot, color));
                        }
                    }
                } else {
                    let key = GlyphKey {
                        face: run.font_face.clone(),
                        glyph: glyph.id,
                        size,
                        subpixel,
                        cleartype,
                    };

                    if let Some(slot) = self.glyph(context, key, bgr)? {
                        instances.push(glyph_instance(pen, slot, FOREGROUND));
                    }
                }
            }
        }

        for decoration in decorations {
            let bounds = decoration.bounds;
            let left = (origin + bounds.left * scale).round();
            let top = (origin + bounds.top * scale).round();
            let right = (origin + bounds.right * scale).round();
            let bottom = (origin + bounds.bottom * scale).round().max(top + 1.0);

            instances.push(Instance {
                bounds: [left, top, right - left, bottom - top],
                texels: [0.0; 4],
                color: FOREGROUND,
            });
        }

        Ok(instances)
    }
}

fn create_atlas_texture(
    device: &ID3D11Device,
    context: &ID3D11DeviceContext,
    size: u32,
) -> Result<(ID3D11Texture2D, ID3D11ShaderResourceView)> {
    let desc = D3D11_TEXTURE2D_DESC {
        Width: size,
        Height: size,
        MipLevels: 1,
        ArraySize: 1,
        Format: DXGI_FORMAT_R8G8B8A8_UNORM,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: D3D11_BIND_SHADER_RESOURCE as u32,
        ..Default::default()
    };

    let mut texture = None;
    let mut view = None;

    unsafe {
        device
            .CreateTexture2D(&desc, None, Some(&mut texture))
            .ok()?;
        let texture = texture.unwrap();
        device
            .CreateShaderResourceView(&texture, None, Some(&mut view))
            .ok()?;
        let white = D3D11_BOX {
            right: 1,
            bottom: 1,
            back: 1,
            ..Default::default()
        };
        context.UpdateSubresource(&texture, 0, Some(&white), [255u8; 4].as_ptr().cast(), 4, 0);
        Ok((texture, view.unwrap()))
    }
}

fn glyph_instance(pen: (f32, f32), slot: Slot, color: [f32; 4]) -> Instance {
    Instance {
        bounds: [
            pen.0 + slot.left as f32,
            pen.1 + slot.top as f32,
            slot.width as f32,
            slot.height as f32,
        ],
        texels: [
            slot.x as f32,
            slot.y as f32,
            slot.width as f32,
            slot.height as f32,
        ],
        color,
    }
}

// Converts coverage to the atlas's four bytes per pixel. ClearType coverage is stored in red,
// green, blue order for the monitor's stripes.
fn expand(image: &GlyphImage, bgr: bool) -> Vec<u8> {
    let pixels = (image.width * image.height) as usize;
    let mut output = Vec::with_capacity(pixels * 4);

    if image.data.len() == pixels {
        for &coverage in &image.data {
            output.extend_from_slice(&[coverage; 4]);
        }
    } else {
        for rgb in image.data.as_chunks::<3>().0 {
            let (red, blue) = if bgr {
                (rgb[2], rgb[0])
            } else {
                (rgb[0], rgb[2])
            };
            output.extend_from_slice(&[red, rgb[1], blue, red.max(rgb[1]).max(blue)]);
        }
    }

    output
}

struct Renderer {
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    chain: IDXGISwapChain1,
    target: Option<ID3D11RenderTargetView>,
    size: (u32, u32),
    vertex_shader: ID3D11VertexShader,
    pixel_shader: ID3D11PixelShader,
    blend: ID3D11BlendState,
    constants: ID3D11Buffer,
    instances: Option<(ID3D11Buffer, ID3D11ShaderResourceView)>,
    capacity: usize,
    atlas: Atlas,
}

impl Renderer {
    fn new(hwnd: HWND, width: u32, height: u32) -> Result<Self> {
        let (device, context) = create_device()?;

        unsafe {
            let adapter = device.cast::<IDXGIDevice>()?.GetAdapter()?;
            let factory: IDXGIFactory2 = adapter.GetParent()?;

            let desc = DXGI_SWAP_CHAIN_DESC1 {
                Width: width,
                Height: height,
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                BufferCount: 2,
                SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
                ..Default::default()
            };

            let chain = factory.CreateSwapChainForHwnd(&device, hwnd, &desc, None, None)?;

            let vertex = compile(s!("vs"), s!("vs_5_0"));
            let pixel = compile(s!("ps"), s!("ps_5_0"));
            let mut vertex_shader = None;
            let mut pixel_shader = None;

            device
                .CreateVertexShader(
                    vertex.GetBufferPointer(),
                    vertex.GetBufferSize(),
                    None,
                    Some(&mut vertex_shader),
                )
                .ok()?;

            device
                .CreatePixelShader(
                    pixel.GetBufferPointer(),
                    pixel.GetBufferSize(),
                    None,
                    Some(&mut pixel_shader),
                )
                .ok()?;

            // Dual-source blending applies a separate alpha to each color channel, which
            // ClearType needs. Grayscale coverage has the same alpha in every channel.
            let mut blend_desc = D3D11_BLEND_DESC::default();
            blend_desc.RenderTarget[0] = D3D11_RENDER_TARGET_BLEND_DESC {
                BlendEnable: true.into(),
                SrcBlend: D3D11_BLEND_ONE,
                DestBlend: D3D11_BLEND_INV_SRC1_COLOR,
                BlendOp: D3D11_BLEND_OP_ADD,
                SrcBlendAlpha: D3D11_BLEND_ONE,
                DestBlendAlpha: D3D11_BLEND_INV_SRC1_ALPHA,
                BlendOpAlpha: D3D11_BLEND_OP_ADD,
                RenderTargetWriteMask: D3D11_COLOR_WRITE_ENABLE_ALL as u8,
            };

            let mut blend = None;
            device
                .CreateBlendState(&blend_desc, Some(&mut blend))
                .ok()?;

            let constants = create_buffer(&device, 16, D3D11_BIND_CONSTANT_BUFFER, 0, 0)?;

            let atlas = Atlas::new(&device, &context)?;

            Ok(Self {
                device,
                context,
                chain,
                target: None,
                size: (width, height),
                vertex_shader: vertex_shader.unwrap(),
                pixel_shader: pixel_shader.unwrap(),
                blend: blend.unwrap(),
                constants,
                instances: None,
                capacity: 0,
                atlas,
            })
        }
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        if self.size != (width, height) {
            self.target = None;

            unsafe {
                self.chain
                    .ResizeBuffers(0, width, height, DXGI_FORMAT_B8G8R8A8_UNORM, 0)
                    .ok()?;
            }

            self.size = (width, height);
        }

        if self.target.is_none() {
            unsafe {
                let buffer: ID3D11Texture2D = self.chain.GetBuffer(0)?;
                self.device
                    .CreateRenderTargetView(&buffer, None, Some(&mut self.target))
                    .ok()?;
            }
        }

        Ok(())
    }

    // Grows the instance buffer to a power of two that holds `count` instances.
    fn reserve(&mut self, count: usize) -> Result<()> {
        if count <= self.capacity {
            return Ok(());
        }

        let capacity = count.next_power_of_two().max(256);
        let stride = size_of::<Instance>() as u32;
        let buffer = create_buffer(
            &self.device,
            capacity as u32 * stride,
            D3D11_BIND_SHADER_RESOURCE,
            D3D11_RESOURCE_MISC_BUFFER_STRUCTURED,
            stride,
        )?;

        let mut view = None;
        unsafe {
            self.device
                .CreateShaderResourceView(&buffer, None, Some(&mut view))
                .ok()?;
        }

        self.instances = Some((buffer, view.unwrap()));
        self.capacity = capacity;
        Ok(())
    }

    fn draw(&mut self, instances: &[Instance], constants: [f32; 4]) -> Result<()> {
        self.reserve(instances.len())?;
        let (buffer, view) = self.instances.as_ref().unwrap();
        write(&self.context, buffer, instances)?;
        write(&self.context, &self.constants, &constants)?;

        let viewport = D3D11_VIEWPORT {
            Width: self.size.0 as f32,
            Height: self.size.1 as f32,
            MaxDepth: 1.0,
            ..Default::default()
        };

        unsafe {
            let context = &self.context;
            context.ClearRenderTargetView(self.target.as_ref(), BACKGROUND.as_ptr());
            context.OMSetRenderTargets(1, Some(&self.target), None);
            context.OMSetBlendState(&self.blend, None, u32::MAX);
            context.RSSetViewports(1, Some(&viewport));
            context.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
            context.VSSetShader(&self.vertex_shader, None, 0);
            context.VSSetConstantBuffers(0, 1, Some(&Some(self.constants.clone())));
            context.VSSetShaderResources(0, 1, Some(&Some(view.clone())));
            context.PSSetShader(&self.pixel_shader, None, 0);
            context.PSSetConstantBuffers(0, 1, Some(&Some(self.constants.clone())));
            context.PSSetShaderResources(1, 1, Some(&Some(self.atlas.view.clone())));
            context.DrawInstanced(4, instances.len() as u32, 0, 0);
            self.chain.Present(1, 0).ok()
        }
    }
}

fn create_device() -> Result<(ID3D11Device, ID3D11DeviceContext)> {
    let mut error = HRESULT(0);

    for driver in [D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_WARP] {
        let mut device = null_mut();
        let mut context = null_mut();

        error = unsafe {
            D3D11CreateDevice(
                null_mut(),
                driver,
                null_mut(),
                0,
                null(),
                0,
                D3D11_SDK_VERSION as u32,
                &mut device,
                null_mut(),
                &mut context,
            )
        };

        if error.is_ok() {
            unsafe {
                return Ok((
                    ID3D11Device::from_raw(device),
                    ID3D11DeviceContext::from_raw(context),
                ));
            }
        }
    }

    Err(error.into())
}

fn create_buffer(
    device: &ID3D11Device,
    size: u32,
    bind: D3D11_BIND_FLAG,
    misc: D3D11_RESOURCE_MISC_FLAG,
    stride: u32,
) -> Result<ID3D11Buffer> {
    let desc = D3D11_BUFFER_DESC {
        ByteWidth: size,
        Usage: D3D11_USAGE_DYNAMIC,
        BindFlags: bind as u32,
        CPUAccessFlags: D3D11_CPU_ACCESS_WRITE as u32,
        MiscFlags: misc as u32,
        StructureByteStride: stride,
    };

    let mut buffer = None;
    unsafe { device.CreateBuffer(&desc, None, Some(&mut buffer)).ok()? };
    Ok(buffer.unwrap())
}

fn write<T: Copy>(context: &ID3D11DeviceContext, buffer: &ID3D11Buffer, data: &[T]) -> Result<()> {
    let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();

    unsafe {
        context
            .Map(buffer, 0, D3D11_MAP_WRITE_DISCARD, 0, Some(&mut mapped))
            .ok()?;
        std::ptr::copy_nonoverlapping(data.as_ptr(), mapped.pData.cast(), data.len());
        context.Unmap(buffer, 0);
    }

    Ok(())
}

// Compiles a shader from `shaders.hlsl`. The source is part of the sample, so an error is a bug.
fn compile(entry: PCSTR, target: PCSTR) -> ID3D10Blob {
    let source = include_str!("shaders.hlsl");
    let mut code = null_mut();
    let mut errors = null_mut();

    unsafe {
        let result = D3DCompile(
            source.as_ptr().cast(),
            source.len(),
            s!("shaders.hlsl"),
            null(),
            null_mut(),
            entry,
            target,
            D3DCOMPILE_OPTIMIZATION_LEVEL3 as u32,
            0,
            &mut code,
            &mut errors,
        );

        if result.is_err() {
            let errors = ID3D10Blob::from_raw(errors);
            let message = std::slice::from_raw_parts(
                errors.GetBufferPointer().cast::<u8>(),
                errors.GetBufferSize(),
            );
            panic!("{}", String::from_utf8_lossy(message));
        }

        ID3D10Blob::from_raw(code)
    }
}

fn is_device_lost(code: HRESULT) -> bool {
    code == DXGI_ERROR_DEVICE_REMOVED || code == DXGI_ERROR_DEVICE_RESET
}

// The inputs the layout depends on. The glyph runs are kept until one of them changes.
#[derive(PartialEq)]
struct LayoutKey {
    width: f32,
    font_size: f32,
}

struct Layout {
    key: LayoutKey,
    runs: Vec<GlyphRun>,
    decorations: Vec<Decoration>,
}

struct State {
    hwnd: HWND,
    renderer: Option<Renderer>,
    layout: Option<Layout>,
    params: RenderingParams,
    font_size: f32,
    cleartype: bool,
    dirty: bool,
}

impl State {
    fn mode(&self) -> Mode {
        match self.params.pixel_geometry {
            PixelGeometry::Rgb if self.cleartype => Mode::ClearType { bgr: false },
            PixelGeometry::Bgr if self.cleartype => Mode::ClearType { bgr: true },
            _ => Mode::Grayscale,
        }
    }

    // Lays out the text in device-independent pixels for the window's width.
    fn layout(&mut self, width: f32) -> Result<Duration> {
        let key = LayoutKey {
            width: (width - 2.0 * MARGIN).max(0.0),
            font_size: self.font_size,
        };

        if self.layout.as_ref().is_some_and(|layout| layout.key == key) {
            return Ok(Duration::ZERO);
        }

        let start = Instant::now();
        let format = TextFormat::new("Segoe UI", key.font_size)?;
        let mut layout = TextLayout::new(TEXT, &format, key.width, f32::MAX)?;
        let heading = 0.."Glyph atlas".len();
        layout.set_font_size(heading.clone(), key.font_size * 1.6);
        layout.set_font_weight(heading, FontWeight::SEMI_BOLD);

        for (word, underline) in [("underline", true), ("strikethrough", false)] {
            let start = TEXT.rfind(word).unwrap();
            let range = start..start + word.len();

            if underline {
                layout.set_underline(range, true);
            } else {
                layout.set_strikethrough(range, true);
            }
        }

        self.layout = Some(Layout {
            key,
            runs: layout.glyph_runs(),
            decorations: layout.decorations(),
        });

        Ok(start.elapsed())
    }

    // Draws a frame if anything changed and returns the new window title.
    fn frame(&mut self) -> Result<Option<String>> {
        if !self.dirty || self.hwnd.is_null() {
            return Ok(None);
        }

        let mut client = RECT::default();
        unsafe { GetClientRect(self.hwnd, &mut client).ok()? };
        let (width, height) = (client.right as u32, client.bottom as u32);

        if width == 0 || height == 0 {
            self.dirty = false;
            return Ok(None);
        }

        let scale = unsafe { GetDpiForWindow(self.hwnd) } as f32 / 96.0;
        let layout_time = self.layout(width as f32 / scale)?;
        let mode = self.mode();

        if self.renderer.is_none() {
            self.renderer = Some(Renderer::new(self.hwnd, width, height)?);
        }

        let renderer = self.renderer.as_mut().unwrap();
        let layout = self.layout.as_ref().unwrap();
        let origin = (MARGIN * scale).round();
        let start = Instant::now();
        renderer.atlas.uploads = 0;
        renderer.atlas.raster = Duration::ZERO;
        renderer.atlas.upload = Duration::ZERO;

        // When the atlas fills up, start over with an empty one, which only has to hold the glyphs
        // of this frame. If they do not fit either, double the size of the atlas.
        let build = |atlas: &mut Atlas| {
            atlas.build(
                &renderer.context,
                &layout.runs,
                &layout.decorations,
                origin,
                scale,
                mode,
            )
        };

        let mut flushed = false;

        let instances = loop {
            match build(&mut renderer.atlas) {
                Ok(instances) => break instances,
                Err(AtlasFull) if !flushed => {
                    renderer.atlas.clear();
                    flushed = true;
                }
                Err(AtlasFull) => renderer.atlas.grow(&renderer.device, &renderer.context)?,
            }
        };

        let build_time = start.elapsed();

        let (contrast, level) = match mode {
            Mode::Grayscale => (self.params.grayscale_enhanced_contrast, 1.0),
            Mode::ClearType { .. } => (self.params.enhanced_contrast, self.params.cleartype_level),
        };

        let result = renderer.resize(width, height).and_then(|()| {
            renderer.draw(&instances, [width as f32, height as f32, contrast, level])
        });

        match result {
            Ok(()) => self.dirty = false,
            Err(error) if is_device_lost(error.code()) => {
                self.renderer = None;
                return Ok(None);
            }
            Err(error) => return Err(error),
        }

        let atlas = &renderer.atlas;
        let mode = match mode {
            Mode::Grayscale => "grayscale",
            Mode::ClearType { .. } => "ClearType",
        };

        Ok(Some(format!(
            "Glyph atlas - {mode}, {size} px - {glyphs} cached, {uploads} uploaded, {drawn} drawn, \
             {flushes} flushes, {atlas_size} atlas - layout {layout_time:.2?}, build \
             {build_time:.2?} (raster {raster:.2?}, upload {upload:.2?})",
            size = self.font_size,
            glyphs = atlas.glyphs.len(),
            uploads = atlas.uploads,
            drawn = instances.len(),
            flushes = atlas.flushes,
            atlas_size = atlas.size,
            raster = atlas.raster,
            upload = atlas.upload,
        )))
    }
}

// Draws a frame and shows the statistics in the title bar. Returns `true` if the frame needs to
// be drawn again, such as after the device was lost.
fn render(state: &RefCell<State>) -> Result<bool> {
    let (title, hwnd) = {
        let mut state = state.borrow_mut();
        (state.frame()?, state.hwnd)
    };

    if let Some(title) = title {
        let title: Vec<u16> = title.encode_utf16().chain(Some(0)).collect();
        unsafe { SetWindowTextW(hwnd, PCWSTR(title.as_ptr())).ok()? };
    }

    Ok(state.borrow().dirty)
}

fn main() -> Result<()> {
    let params = RenderingParams::system();

    let state = Rc::new(RefCell::new(State {
        hwnd: null_mut(),
        renderer: None,
        layout: None,
        params,
        font_size: 18.0,
        cleartype: params.cleartype,
        dirty: true,
    }));

    let messages = Rc::clone(&state);
    let resize = Rc::clone(&state);

    let window = Window::new("Glyph atlas")
        .client_size(900, 600)
        .on_message(move |hwnd, message, wparam, lparam| {
            let message = message as i32;

            if message == WM_CHAR && matches!(char::from_u32(wparam as u32), Some('c' | 'C')) {
                let mut state = messages.borrow_mut();
                state.cleartype = !state.cleartype;
            } else if message == WM_MOUSEWHEEL {
                let delta = (wparam >> 16) as i16;
                let mut state = messages.borrow_mut();
                state.font_size = (state.font_size + delta.signum() as f32).clamp(8.0, 72.0);
            } else if message == WM_DPICHANGED {
                let rect = unsafe { &*(lparam as *const RECT) };

                unsafe {
                    SetWindowPos(
                        hwnd.cast(),
                        null_mut(),
                        rect.left,
                        rect.top,
                        rect.right - rect.left,
                        rect.bottom - rect.top,
                        (SWP_NOZORDER | SWP_NOACTIVATE) as u32,
                    )
                    .ok()
                    .unwrap();
                }
            } else {
                return None;
            }

            messages.borrow_mut().dirty = true;
            render(&messages).unwrap();
            Some(0)
        })
        .on_resize(move |_, _| {
            resize.borrow_mut().dirty = true;
            render(&resize).unwrap();
        })
        .create()?;

    state.borrow_mut().hwnd = window.hwnd().cast();
    run_with(|| render(&state))
}
