//! # Mouse Input
//!
//! Funções e tipos para entrada de mouse.

use crate::syscall::SYS_MOUSE_READ;
use crate::syscall::{check_error, syscall1, SysResult};

use gfx_types::geometry::Point;

// =============================================================================
// TIPOS
// =============================================================================

/// Botões do mouse.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
}

impl MouseButton {
    /// Retorna a máscara de bits para este botão.
    pub fn mask(self) -> u8 {
        match self {
            MouseButton::Left => 0x01,
            MouseButton::Right => 0x02,
            MouseButton::Middle => 0x04,
            MouseButton::Button4 => 0x08,
            MouseButton::Button5 => 0x10,
        }
    }
}

/// Estado do mouse.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct MouseState {
    /// Posição X absoluta.
    pub x: i32,
    /// Posição Y absoluta.
    pub y: i32,
    /// Delta X desde última leitura.
    pub delta_x: i32,
    /// Delta Y desde última leitura.
    pub delta_y: i32,
    /// Botões (bitmask).
    pub buttons: u8,
    pub _pad: [u8; 3],
}

impl MouseState {
    /// Retorna a posição como Point.
    #[inline]
    pub fn position(&self) -> Point {
        Point::new(self.x, self.y)
    }

    /// Retorna o delta como Point.
    #[inline]
    pub fn delta(&self) -> Point {
        Point::new(self.delta_x, self.delta_y)
    }

    /// Verifica se um botão está pressionado.
    #[inline]
    pub fn is_pressed(&self, button: MouseButton) -> bool {
        (self.buttons & button.mask()) != 0
    }

    /// Botão esquerdo pressionado.
    #[inline]
    pub fn left_button(&self) -> bool {
        self.is_pressed(MouseButton::Left)
    }

    /// Botão direito pressionado.
    #[inline]
    pub fn right_button(&self) -> bool {
        self.is_pressed(MouseButton::Right)
    }

    /// Botão do meio pressionado.
    #[inline]
    pub fn middle_button(&self) -> bool {
        self.is_pressed(MouseButton::Middle)
    }

    /// Retorna true se algum botão está pressionado.
    #[inline]
    pub fn any_button(&self) -> bool {
        self.buttons != 0
    }
}

// =============================================================================
// FUNÇÕES
// =============================================================================

/// Lê estado atual do mouse.
pub fn poll_mouse() -> SysResult<MouseState> {
    let mut state = MouseState::default();
    let ret = syscall1(SYS_MOUSE_READ, &mut state as *mut _ as usize);
    check_error(ret)?;
    Ok(state)
}
