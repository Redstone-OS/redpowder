//! # Framebuffer
//!
//! Acesso direto ao framebuffer do kernel via syscalls.

use crate::syscall::{check_error, syscall1, syscall3, SysResult};
use crate::syscall::{SYS_FB_CLEAR, SYS_FB_INFO, SYS_FB_WRITE};

use gfx_types::buffer::BufferDescriptor;
use gfx_types::color::{Color, PixelFormat};
use gfx_types::geometry::{Point, Rect, Size};

// =============================================================================
// TIPOS
// =============================================================================

/// Informações do framebuffer (layout compatível com kernel).
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct FramebufferInfo {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub bpp: u32,
    pub format: u32,
}

impl FramebufferInfo {
    /// Converte para Size.
    #[inline]
    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    /// Retorna o formato de pixel.
    #[inline]
    pub fn pixel_format(&self) -> PixelFormat {
        PixelFormat::from_u32(self.format).unwrap_or(PixelFormat::ARGB8888)
    }

    /// Cria um BufferDescriptor equivalente.
    #[inline]
    pub fn to_buffer_descriptor(&self) -> BufferDescriptor {
        BufferDescriptor::with_stride(self.width, self.height, self.stride, self.pixel_format())
    }

    /// Retorna o Rect total do framebuffer.
    #[inline]
    pub fn bounds(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }

    /// Calcula o offset de um pixel.
    #[inline]
    pub fn pixel_offset(&self, x: u32, y: u32) -> usize {
        (y as usize * self.stride as usize) + (x as usize * (self.bpp as usize / 8))
    }

    /// Tamanho total em bytes.
    #[inline]
    pub fn size_bytes(&self) -> usize {
        self.stride as usize * self.height as usize
    }
}

// =============================================================================
// FUNÇÕES DE SYSCALL
// =============================================================================

/// Obtém informações do framebuffer.
pub fn get_info() -> SysResult<FramebufferInfo> {
    let mut info = FramebufferInfo::default();
    let ret = syscall1(SYS_FB_INFO, &mut info as *mut _ as usize);
    check_error(ret)?;
    Ok(info)
}

/// Limpa o framebuffer com uma cor.
pub fn clear_screen(color: Color) -> SysResult<()> {
    let ret = syscall1(SYS_FB_CLEAR, color.0 as usize);
    check_error(ret)?;
    Ok(())
}

/// Escreve dados diretamente no framebuffer.
///
/// # Safety
/// O caller deve garantir que offset + len não excede o tamanho do framebuffer.
pub fn write_pixels(offset: usize, data: &[u8]) -> SysResult<usize> {
    let ret = syscall3(SYS_FB_WRITE, offset, data.as_ptr() as usize, data.len());
    check_error(ret)?;
    Ok(ret as usize)
}

// =============================================================================
// FRAMEBUFFER WRAPPER
// =============================================================================

/// Wrapper do framebuffer com operações de desenho.
pub struct Framebuffer {
    pub info: FramebufferInfo,
}

impl Framebuffer {
    /// Cria nova instância obtendo info do kernel.
    pub fn new() -> SysResult<Self> {
        let info = get_info()?;
        Ok(Self { info })
    }

    /// Largura em pixels.
    #[inline]
    pub fn width(&self) -> u32 {
        self.info.width
    }

    /// Altura em pixels.
    #[inline]
    pub fn height(&self) -> u32 {
        self.info.height
    }

    /// Retorna Size.
    #[inline]
    pub fn size(&self) -> Size {
        self.info.size()
    }

    /// Retorna o retângulo completo do framebuffer.
    #[inline]
    pub fn bounds(&self) -> Rect {
        self.info.bounds()
    }

    /// Retorna o BufferDescriptor.
    #[inline]
    pub fn descriptor(&self) -> BufferDescriptor {
        self.info.to_buffer_descriptor()
    }

    /// Limpa tela com cor.
    pub fn clear(&mut self, color: Color) -> SysResult<()> {
        clear_screen(color)
    }

    /// Desenha um pixel.
    pub fn put_pixel(&mut self, x: u32, y: u32, color: Color) -> SysResult<()> {
        if x >= self.info.width || y >= self.info.height {
            return Ok(());
        }

        let offset = self.info.pixel_offset(x, y);
        let pixel_data = color.0.to_le_bytes();
        write_pixels(offset, &pixel_data)?;
        Ok(())
    }

    /// Desenha um pixel usando Point.
    #[inline]
    pub fn put_pixel_at(&mut self, p: Point, color: Color) -> SysResult<()> {
        if p.x < 0 || p.y < 0 {
            return Ok(());
        }
        self.put_pixel(p.x as u32, p.y as u32, color)
    }

    /// Preenche um Rect.
    pub fn fill(&mut self, rect: Rect, color: Color) -> SysResult<()> {
        let clipped = match rect.intersection(&self.bounds()) {
            Some(r) => r,
            None => return Ok(()),
        };

        self.fill_rect_internal(
            clipped.x as u32,
            clipped.y as u32,
            clipped.width,
            clipped.height,
            color,
        )
    }

    /// Desenha borda de um retângulo.
    pub fn stroke_rect(&mut self, rect: Rect, color: Color, thickness: u32) -> SysResult<()> {
        let t = thickness;
        // Top
        self.fill(Rect::new(rect.x, rect.y, rect.width, t), color)?;
        // Bottom
        self.fill(
            Rect::new(rect.x, rect.bottom() - t as i32, rect.width, t),
            color,
        )?;
        // Left
        self.fill(Rect::new(rect.x, rect.y, t, rect.height), color)?;
        // Right
        self.fill(
            Rect::new(rect.right() - t as i32, rect.y, t, rect.height),
            color,
        )?;
        Ok(())
    }

    /// Desenha uma linha horizontal.
    pub fn hline(&mut self, x: u32, y: u32, w: u32, color: Color) -> SysResult<()> {
        self.fill_rect_internal(x, y, w, 1, color)
    }

    /// Desenha uma linha vertical.
    pub fn vline(&mut self, x: u32, y: u32, h: u32, color: Color) -> SysResult<()> {
        for dy in 0..h {
            self.put_pixel(x, y + dy, color)?;
        }
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Implementação interna otimizada
    // -------------------------------------------------------------------------

    fn fill_rect_internal(
        &mut self,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        color: Color,
    ) -> SysResult<()> {
        const CHUNK_WIDTH: usize = 1024;
        let mut line_buffer = [0u8; CHUNK_WIDTH * 4];

        let pixel = color.0.to_le_bytes();
        for i in 0..CHUNK_WIDTH {
            line_buffer[i * 4] = pixel[0];
            line_buffer[i * 4 + 1] = pixel[1];
            line_buffer[i * 4 + 2] = pixel[2];
            line_buffer[i * 4 + 3] = pixel[3];
        }

        for dy in 0..h {
            let py = y + dy;
            if py >= self.info.height {
                break;
            }

            let mut pixels_remaining = w as usize;
            let mut current_x = x as usize;

            while pixels_remaining > 0 {
                let chunk_size = pixels_remaining.min(CHUNK_WIDTH);
                let offset = self.info.pixel_offset(current_x as u32, py);
                let bytes_to_write = chunk_size * 4;

                write_pixels(offset, &line_buffer[..bytes_to_write])?;

                pixels_remaining -= chunk_size;
                current_x += chunk_size;
            }
        }
        Ok(())
    }
}
