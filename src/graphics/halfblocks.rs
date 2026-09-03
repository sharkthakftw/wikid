use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RgbPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn render_halfblock_lines(
    pixels: &[RgbPixel],
    width: usize,
    height: usize,
) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let rows = height.div_ceil(2);

    for row in 0..rows {
        let top_y = row * 2;
        let bot_y = top_y + 1;

        let mut spans = Vec::with_capacity(width);

        for x in 0..width {
            let top_pixel = if top_y < height {
                pixels.get(top_y * width + x).copied()
            } else {
                None
            };

            let bot_pixel = if bot_y < height {
                pixels.get(bot_y * width + x).copied()
            } else {
                None
            };

            let fg = top_pixel
                .map(|p| Color::Rgb(p.r, p.g, p.b))
                .unwrap_or(Color::Reset);
            let bg = bot_pixel
                .map(|p| Color::Rgb(p.r, p.g, p.b))
                .unwrap_or(Color::Reset);

            let style = Style::default().fg(fg).bg(bg);
            spans.push(Span::styled("▀", style));
        }

        lines.push(Line::from(spans));
    }

    lines
}

pub fn render_halfblock_image_from_bytes(
    image_bytes: &[u8],
    target_cols: usize,
    target_rows: usize,
    filter: crate::config::HalfblockFilter,
) -> Option<Vec<Line<'static>>> {
    let img = image::load_from_memory(image_bytes).ok()?;
    let target_px_height = target_rows * 2;
    let filter_type = match filter {
        crate::config::HalfblockFilter::Nearest => image::imageops::FilterType::Nearest,
        crate::config::HalfblockFilter::Triangle => image::imageops::FilterType::Triangle,
        crate::config::HalfblockFilter::Catmullrom => image::imageops::FilterType::CatmullRom,
        crate::config::HalfblockFilter::Gaussian => image::imageops::FilterType::Gaussian,
        crate::config::HalfblockFilter::Lanczos3 => image::imageops::FilterType::Lanczos3,
    };
    let resized = img.resize_exact(target_cols as u32, target_px_height as u32, filter_type);
    let rgba = resized.to_rgba8();
    let pixels: Vec<RgbPixel> = rgba
        .pixels()
        .map(|p| {
            let a = p[3] as u32;
            let inv_a = 255 - a;
            let r = ((p[0] as u32 * a + 255 * inv_a) / 255) as u8;
            let g = ((p[1] as u32 * a + 255 * inv_a) / 255) as u8;
            let b = ((p[2] as u32 * a + 255 * inv_a) / 255) as u8;
            RgbPixel { r, g, b }
        })
        .collect();
    let mut lines = render_halfblock_lines(&pixels, target_cols, target_px_height);
    for line in &mut lines {
        line.alignment = Some(ratatui::layout::Alignment::Center);
    }
    Some(lines)
}
