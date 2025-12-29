//! # Módulo de Gráficos
//!
//! API de alto nível para acesso ao framebuffer.

use crate::syscall::{check_error, syscall1, syscall3, SysResult};
use crate::syscall::{SYS_FB_CLEAR, SYS_FB_INFO, SYS_FB_WRITE};

// ============================================================================
// TIPOS
// ============================================================================

/// Informações do framebuffer
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FramebufferInfo {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub bpp: u32,
    pub format: u32,
}

/// Cor ARGB
#[derive(Clone, Copy)]
pub struct Color(pub u32);

impl Color {
    pub const BLACK: Color = Color(0x000000);
    pub const WHITE: Color = Color(0xFFFFFF);
    pub const RED: Color = Color(0xFF0000);
    pub const GREEN: Color = Color(0x00FF00);
    pub const BLUE: Color = Color(0x0000FF);
    pub const ORANGE: Color = Color(0xFF8C00); // Cor principal do Redstone!
    pub const DARK_GRAY: Color = Color(0x333333);
    pub const LIGHT_GRAY: Color = Color(0xAAAAAA);

    /// Cria cor a partir de componentes RGB
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    /// Cria cor a partir de componentes ARGB
    pub const fn argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Color(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }
}

// ============================================================================
// FUNÇÕES DE ALTO NÍVEL
// ============================================================================

/// Obtém informações do framebuffer
pub fn get_framebuffer_info() -> SysResult<FramebufferInfo> {
    let mut info = FramebufferInfo::default();
    let ret = syscall1(SYS_FB_INFO, &mut info as *mut _ as usize);
    check_error(ret)?;
    Ok(info)
}

/// Limpa o framebuffer com uma cor
pub fn clear_screen(color: Color) -> SysResult<()> {
    let ret = syscall1(SYS_FB_CLEAR, color.0 as usize);
    check_error(ret)?;
    Ok(())
}

/// Escreve dados diretamente no framebuffer
///
/// # Safety
/// O caller deve garantir que offset + len não excede o tamanho do framebuffer
pub fn write_framebuffer(offset: usize, data: &[u8]) -> SysResult<usize> {
    let ret = syscall3(SYS_FB_WRITE, offset, data.as_ptr() as usize, data.len());
    check_error(ret)?;
    Ok(ret as usize)
}

/// Framebuffer wrapper com buffer local
pub struct Framebuffer {
    pub info: FramebufferInfo,
}

impl Framebuffer {
    /// Cria nova instância obtendo info do kernel
    pub fn new() -> SysResult<Self> {
        let info = get_framebuffer_info()?;
        Ok(Self { info })
    }

    /// Largura em pixels
    pub fn width(&self) -> u32 {
        self.info.width
    }

    /// Altura em pixels
    pub fn height(&self) -> u32 {
        self.info.height
    }

    /// Limpa tela com cor
    pub fn clear(&mut self, color: Color) -> SysResult<()> {
        clear_screen(color)
    }

    /// Desenha um pixel
    pub fn put_pixel(&mut self, x: u32, y: u32, color: Color) -> SysResult<()> {
        if x >= self.info.width || y >= self.info.height {
            return Ok(()); // Fora dos bounds, ignorar silenciosamente
        }

        let offset = (y as usize * self.info.stride as usize) + (x as usize * 4);
        let pixel_data = color.0.to_le_bytes();
        write_framebuffer(offset, &pixel_data)?;
        Ok(())
    }

    /// Preenche um retângulo
    pub fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: Color) -> SysResult<()> {
        for dy in 0..h {
            for dx in 0..w {
                self.put_pixel(x + dx, y + dy, color)?;
            }
        }
        Ok(())
    }
}
