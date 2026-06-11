use std::os::windows::ffi::OsStrExt;
use std::path::Path;

use crate::bindings::*;

/// A GPU-resident bitmap.
///
/// - **File-loaded** — via [`crate::SwapChain::load_bitmap`] or
///   [`crate::DrawingSession::load_bitmap`].
/// - **Render target** — via [`crate::DrawingSession::create_bitmap_target`].
///   Can be drawn into with [`crate::DrawingSession::with_target`] and used
///   as input for [`crate::Effect`].
#[derive(Clone)]
pub struct Bitmap(pub(crate) ID2D1Bitmap1);

impl Bitmap {
    /// Load a bitmap from an image file (PNG, JPEG, BMP, etc.).
    pub(crate) fn load_from_file(
        context: &ID2D1DeviceContext,
        path: &Path,
    ) -> windows_core::Result<Self> {
        unsafe {
            let wic_factory = wic_factory()?;

            let wide_path: Vec<u16> = path
                .as_os_str()
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();

            let decoder = wic_factory.CreateDecoderFromFilename(
                windows_core::PCWSTR(wide_path.as_ptr()),
                None,
                GENERIC_READ,
                WICDecodeMetadataCacheOnDemand,
            )?;

            let frame = decoder.GetFrame(0)?;
            let converter = wic_factory.CreateFormatConverter()?;

            converter.Initialize(
                &frame,
                &GUID_WICPixelFormat32bppPBGRA,
                WICBitmapDitherTypeNone,
                None,
                0.0,
                WICBitmapPaletteTypeMedianCut,
            )?;

            let bitmap = context.CreateBitmapFromWicBitmap(&converter, None)?;
            Ok(Self(bitmap))
        }
    }

    /// Width of the bitmap in device-independent pixels.
    pub fn width(&self) -> f32 {
        let size = unsafe { self.0.GetSize() };
        size.width
    }

    /// Height of the bitmap in device-independent pixels.
    pub fn height(&self) -> f32 {
        let size = unsafe { self.0.GetSize() };
        size.height
    }
}

fn wic_factory() -> windows_core::Result<IWICImagingFactory> {
    thread_local! {
        static SHARED: std::cell::OnceCell<IWICImagingFactory> = const { std::cell::OnceCell::new() };
    }

    SHARED.with(|cell| {
        if let Some(factory) = cell.get() {
            return Ok(factory.clone());
        }

        let factory: IWICImagingFactory = unsafe {
            let mut ptr = core::ptr::null_mut();
            CoCreateInstance(
                &CLSID_WICImagingFactory,
                core::ptr::null_mut(),
                CLSCTX_INPROC_SERVER,
                &<IWICImagingFactory as windows_core::Interface>::IID,
                &mut ptr,
            )
            .ok()?;
            windows_core::Type::from_abi(ptr)?
        };
        Ok(cell.get_or_init(|| factory).clone())
    })
}
