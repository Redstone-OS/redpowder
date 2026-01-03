//! # Canvas
//!
//! API de desenho de alto nível sobre um buffer de pixels.
//!
//! ## Exemplo
//!
//! ```rust
//! use redpowder::graphics::{Canvas, Color, Rect};
//!
//! let mut buffer = vec![0u32; 800 * 600];
//! let mut canvas = Canvas::new(&mut buffer, 800, 600);
//!
//! canvas.clear(Color::BLACK);
//! canvas.fill_rect(Rect::new(10, 10, 100, 50), Color::RED);
//! canvas.stroke_rect(Rect::new(10, 10, 100, 50), Color::WHITE, 1);
//! ```

extern crate alloc;

use alloc::vec::Vec;

use gfx_types::color::Color;
use gfx_types::geometry::{Circle, Line, Point, Rect, Size};
use gfx_types::render::ClipRect;

use super::draw::{circle_points, draw_circle, draw_line, fill_circle, line_points};

// =============================================================================
// CANVAS
// =============================================================================

/// Canvas - superfície de desenho sobre buffer de pixels.
pub struct Canvas<'a> {
    /// Buffer de pixels (ARGB).
    buffer: &'a mut [u32],
    /// Largura em pixels.
    width: u32,
    /// Altura em pixels.
    height: u32,
    /// Região de clipping.
    clip: Option<ClipRect>,
    /// Regiões modificadas (damage tracking).
    damage: Vec<Rect>,
}

impl<'a> Canvas<'a> {
    /// Cria novo canvas sobre um buffer.
    pub fn new(buffer: &'a mut [u32], width: u32, height: u32) -> Self {
        Self {
            buffer,
            width,
            height,
            clip: None,
            damage: Vec::with_capacity(8),
        }
    }

    // =========================================================================
    // PROPRIEDADES
    // =========================================================================

    /// Retorna tamanho do canvas.
    #[inline]
    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    /// Retorna largura.
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Retorna altura.
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Retorna o retângulo total do canvas.
    #[inline]
    pub fn bounds(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }

    /// Define região de clipping.
    pub fn set_clip(&mut self, rect: Option<Rect>) {
        self.clip = rect.map(|r| ClipRect::new(r));
    }

    /// Retorna referência ao buffer.
    pub fn buffer(&self) -> &[u32] {
        self.buffer
    }

    /// Retorna referência mutável ao buffer.
    pub fn buffer_mut(&mut self) -> &mut [u32] {
        self.buffer
    }

    // =========================================================================
    // DESENHO BÁSICO
    // =========================================================================

    /// Limpa todo o canvas com uma cor.
    pub fn clear(&mut self, color: Color) {
        self.buffer.fill(color.as_u32());
        self.add_damage(self.bounds());
    }

    /// Desenha um pixel.
    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        if !self.is_visible(x, y) {
            return;
        }

        let idx = (y as usize * self.width as usize) + x as usize;
        if idx < self.buffer.len() {
            self.buffer[idx] = color.as_u32();
        }
    }

    /// Desenha um pixel em Point.
    #[inline]
    pub fn put_pixel_at(&mut self, p: Point, color: Color) {
        self.put_pixel(p.x, p.y, color);
    }

    /// Lê cor de um pixel.
    pub fn get_pixel(&self, x: i32, y: i32) -> Option<Color> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }
        let idx = (y as usize * self.width as usize) + x as usize;
        self.buffer.get(idx).map(|&v| Color(v))
    }

    // =========================================================================
    // RETÂNGULOS
    // =========================================================================

    /// Preenche retângulo com cor sólida.
    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        let rect = self.clip_rect(rect);
        if rect.is_empty() {
            return;
        }

        let color_u32 = color.as_u32();

        for y in rect.y.max(0) as u32..((rect.y + rect.height as i32) as u32).min(self.height) {
            let start = (y as usize * self.width as usize) + rect.x.max(0) as usize;
            let width = rect.width as usize;
            let end = (start + width).min(self.buffer.len());

            if start < self.buffer.len() {
                self.buffer[start..end].fill(color_u32);
            }
        }

        self.add_damage(rect);
    }

    /// Desenha borda de retângulo.
    pub fn stroke_rect(&mut self, rect: Rect, color: Color, thickness: u32) {
        let t = thickness;
        // Top
        self.fill_rect(Rect::new(rect.x, rect.y, rect.width, t), color);
        // Bottom
        self.fill_rect(
            Rect::new(
                rect.x,
                rect.y + rect.height as i32 - t as i32,
                rect.width,
                t,
            ),
            color,
        );
        // Left
        self.fill_rect(Rect::new(rect.x, rect.y, t, rect.height), color);
        // Right
        self.fill_rect(
            Rect::new(
                rect.x + rect.width as i32 - t as i32,
                rect.y,
                t,
                rect.height,
            ),
            color,
        );
    }

    // =========================================================================
    // LINHAS
    // =========================================================================

    /// Desenha linha horizontal.
    pub fn hline(&mut self, x: i32, y: i32, width: u32, color: Color) {
        self.fill_rect(Rect::new(x, y, width, 1), color);
    }

    /// Desenha linha vertical.
    pub fn vline(&mut self, x: i32, y: i32, height: u32, color: Color) {
        self.fill_rect(Rect::new(x, y, 1, height), color);
    }

    /// Desenha linha entre dois pontos (Bresenham).
    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        for p in line_points(x0, y0, x1, y1) {
            self.put_pixel(p.x, p.y, color);
        }
    }

    /// Desenha uma Line.
    pub fn draw_line(&mut self, line: Line, color: Color) {
        for p in draw_line(line) {
            self.put_pixel(p.x, p.y, color);
        }
    }

    // =========================================================================
    // CÍRCULOS
    // =========================================================================

    /// Desenha borda de círculo.
    pub fn stroke_circle(&mut self, cx: i32, cy: i32, radius: i32, color: Color) {
        for p in circle_points(cx, cy, radius) {
            self.put_pixel(p.x, p.y, color);
        }
    }

    /// Desenha borda de Circle.
    pub fn draw_circle(&mut self, circle: Circle, color: Color) {
        for p in draw_circle(circle) {
            self.put_pixel(p.x, p.y, color);
        }
    }

    /// Preenche círculo.
    pub fn fill_circle(&mut self, cx: i32, cy: i32, radius: i32, color: Color) {
        let circle = Circle::from_coords(cx as f32, cy as f32, radius as f32);
        for (x, y, w) in fill_circle(circle) {
            self.hline(x, y, w as u32, color);
        }
    }

    // =========================================================================
    // BLIT / COPY
    // =========================================================================

    /// Copia região de outro buffer.
    pub fn blit(&mut self, src: &[u32], src_size: Size, src_rect: Rect, dst_point: Point) {
        let dst_rect = Rect::new(dst_point.x, dst_point.y, src_rect.width, src_rect.height);
        let dst_rect = self.clip_rect(dst_rect);
        if dst_rect.is_empty() {
            return;
        }

        let src_stride = src_size.width as usize;
        let dst_stride = self.width as usize;

        for y in 0..dst_rect.height as usize {
            let src_y = src_rect.y as usize + y;
            let dst_y = dst_rect.y as usize + y;

            if src_y >= src_size.height as usize || dst_y >= self.height as usize {
                continue;
            }

            let src_start = src_y * src_stride + src_rect.x as usize;
            let dst_start = dst_y * dst_stride + dst_rect.x as usize;
            let width = dst_rect.width as usize;

            let src_end = (src_start + width).min(src.len());
            let dst_end = (dst_start + width).min(self.buffer.len());
            let actual_width = (src_end - src_start).min(dst_end - dst_start);

            if actual_width > 0 {
                self.buffer[dst_start..dst_start + actual_width]
                    .copy_from_slice(&src[src_start..src_start + actual_width]);
            }
        }

        self.add_damage(dst_rect);
    }

    /// Copia com alpha blending.
    pub fn blit_blend(&mut self, src: &[u32], src_size: Size, src_rect: Rect, dst_point: Point) {
        let dst_rect = self.clip_rect(Rect::new(
            dst_point.x,
            dst_point.y,
            src_rect.width,
            src_rect.height,
        ));
        if dst_rect.is_empty() {
            return;
        }

        for y in 0..dst_rect.height as usize {
            for x in 0..dst_rect.width as usize {
                let src_x = src_rect.x as usize + x;
                let src_y = src_rect.y as usize + y;
                let dst_x = dst_rect.x as usize + x;
                let dst_y = dst_rect.y as usize + y;

                if src_x >= src_size.width as usize
                    || src_y >= src_size.height as usize
                    || dst_x >= self.width as usize
                    || dst_y >= self.height as usize
                {
                    continue;
                }

                let src_idx = src_y * src_size.width as usize + src_x;
                let dst_idx = dst_y * self.width as usize + dst_x;

                if src_idx < src.len() && dst_idx < self.buffer.len() {
                    let src_color = Color(src[src_idx]);
                    let dst_color = Color(self.buffer[dst_idx]);
                    let blended = blend_over(src_color, dst_color);
                    self.buffer[dst_idx] = blended.as_u32();
                }
            }
        }

        self.add_damage(dst_rect);
    }

    // =========================================================================
    // DAMAGE TRACKING
    // =========================================================================

    /// Retorna regiões danificadas.
    pub fn damage(&self) -> &[Rect] {
        &self.damage
    }

    /// Retorna e limpa regiões danificadas.
    pub fn take_damage(&mut self) -> Vec<Rect> {
        core::mem::take(&mut self.damage)
    }

    /// Limpa lista de damage.
    pub fn clear_damage(&mut self) {
        self.damage.clear();
    }

    // =========================================================================
    // HELPERS INTERNOS
    // =========================================================================

    /// Verifica se ponto é visível (dentro dos bounds e clip).
    fn is_visible(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return false;
        }

        if let Some(clip) = &self.clip {
            return clip.rect.contains_point(Point::new(x, y));
        }

        true
    }

    /// Aplica clipping a um retângulo.
    fn clip_rect(&self, rect: Rect) -> Rect {
        let canvas_rect = self.bounds();
        let mut result = match rect.intersection(&canvas_rect) {
            Some(r) => r,
            None => return Rect::ZERO,
        };

        if let Some(clip) = &self.clip {
            result = match result.intersection(&clip.rect) {
                Some(r) => r,
                None => return Rect::ZERO,
            };
        }

        result
    }

    /// Adiciona região ao damage tracking.
    fn add_damage(&mut self, rect: Rect) {
        if rect.is_empty() {
            return;
        }

        // Tentar merge com rect existente
        for existing in &mut self.damage {
            if existing.intersects(&rect) {
                *existing = existing.union(&rect);
                return;
            }
        }

        self.damage.push(rect);

        // Limitar número de rects
        if self.damage.len() > 8 {
            self.collapse_damage();
        }
    }

    /// Agrupa damage em um único bounding box.
    fn collapse_damage(&mut self) {
        if self.damage.len() <= 1 {
            return;
        }

        let mut bounds = self.damage[0];
        for rect in &self.damage[1..] {
            bounds = bounds.union(rect);
        }

        self.damage.clear();
        self.damage.push(bounds);
    }
}

// =============================================================================
// BLENDING
// =============================================================================

/// Alpha blend (source over).
fn blend_over(src: Color, dst: Color) -> Color {
    let sa = src.alpha() as u32;
    let sr = src.red() as u32;
    let sg = src.green() as u32;
    let sb = src.blue() as u32;

    let da = dst.alpha() as u32;
    let dr = dst.red() as u32;
    let dg = dst.green() as u32;
    let db = dst.blue() as u32;

    if sa == 255 {
        return src;
    }
    if sa == 0 {
        return dst;
    }

    let inv_sa = 255 - sa;

    let out_a = sa + (da * inv_sa / 255);
    let out_r = (sr * sa + dr * inv_sa) / 255;
    let out_g = (sg * sa + dg * inv_sa) / 255;
    let out_b = (sb * sa + db * inv_sa) / 255;

    Color::argb(out_a as u8, out_r as u8, out_g as u8, out_b as u8)
}
