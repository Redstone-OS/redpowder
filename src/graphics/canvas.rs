//! # Canvas
//!
//! API de desenho de alto nível.
//!
//! Fornece uma interface simples para operações de desenho
//! sobre um buffer de pixels.

extern crate alloc;

use alloc::vec::Vec;
use gfx_types::{Color, Point, Rect, Size};

/// Canvas - superfície de desenho.
pub struct Canvas<'a> {
    /// Buffer de pixels.
    buffer: &'a mut [u32],
    /// Largura em pixels.
    width: u32,
    /// Altura em pixels.
    height: u32,
    /// Região de clipping (opcional).
    clip: Option<Rect>,
    /// Regiões modificadas.
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

    /// Retorna tamanho do canvas.
    #[inline]
    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    /// Define região de clipping.
    pub fn set_clip(&mut self, rect: Option<Rect>) {
        self.clip = rect;
    }

    /// Limpa todo o canvas com uma cor.
    pub fn clear(&mut self, color: Color) {
        let rect = Rect::new(0, 0, self.width, self.height);
        self.fill_rect(rect, color);
    }

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
        // Top
        self.fill_rect(Rect::new(rect.x, rect.y, rect.width, thickness), color);
        // Bottom
        self.fill_rect(
            Rect::new(
                rect.x,
                rect.y + rect.height as i32 - thickness as i32,
                rect.width,
                thickness,
            ),
            color,
        );
        // Left
        self.fill_rect(Rect::new(rect.x, rect.y, thickness, rect.height), color);
        // Right
        self.fill_rect(
            Rect::new(
                rect.x + rect.width as i32 - thickness as i32,
                rect.y,
                thickness,
                rect.height,
            ),
            color,
        );
    }

    /// Desenha um pixel.
    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }

        // Verificar clipping
        if let Some(clip) = &self.clip {
            if !clip.contains_point(Point::new(x, y)) {
                return;
            }
        }

        let idx = (y as usize * self.width as usize) + x as usize;
        if idx < self.buffer.len() {
            self.buffer[idx] = color.as_u32();
        }
    }

    /// Desenha linha horizontal.
    pub fn hline(&mut self, x: i32, y: i32, width: u32, color: Color) {
        self.fill_rect(Rect::new(x, y, width, 1), color);
    }

    /// Desenha linha vertical.
    pub fn vline(&mut self, x: i32, y: i32, height: u32, color: Color) {
        self.fill_rect(Rect::new(x, y, 1, height), color);
    }

    /// Copia região de outro slice.
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

    /// Aplica clipping a um retângulo.
    fn clip_rect(&self, rect: Rect) -> Rect {
        let mut result = rect;

        // Clip aos limites do canvas
        let canvas_rect = Rect::new(0, 0, self.width, self.height);
        if let Some(clipped) = result.intersection(&canvas_rect) {
            result = clipped;
        } else {
            return Rect::ZERO;
        }

        // Clip à região de clipping definida
        if let Some(clip) = &self.clip {
            if let Some(clipped) = result.intersection(clip) {
                result = clipped;
            } else {
                return Rect::ZERO;
            }
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
