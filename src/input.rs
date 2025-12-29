//! # Módulo de Input
//!
//! API de alto nível para mouse e teclado.

use crate::syscall::{check_error, syscall1, syscall2, SysResult};
use crate::syscall::{SYS_KEYBOARD_READ, SYS_MOUSE_READ};

// ============================================================================
// TIPOS - MOUSE
// ============================================================================

/// Estado do mouse
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MouseState {
    /// Posição X absoluta
    pub x: i32,
    /// Posição Y absoluta  
    pub y: i32,
    /// Delta X desde última leitura
    pub delta_x: i32,
    /// Delta Y desde última leitura
    pub delta_y: i32,
    /// Botões (bit 0=esquerdo, 1=direito, 2=meio)
    pub buttons: u8,
    pub _pad: [u8; 3],
}

impl MouseState {
    /// Botão esquerdo pressionado
    pub fn left_button(&self) -> bool {
        (self.buttons & 0x01) != 0
    }

    /// Botão direito pressionado
    pub fn right_button(&self) -> bool {
        (self.buttons & 0x02) != 0
    }

    /// Botão do meio pressionado
    pub fn middle_button(&self) -> bool {
        (self.buttons & 0x04) != 0
    }
}

// ============================================================================
// TIPOS - TECLADO
// ============================================================================

/// Evento de teclado
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KeyEvent {
    /// Scancode da tecla
    pub scancode: u8,
    /// Tecla pressionada (true) ou solta (false)
    pub pressed: bool,
    pub _pad: [u8; 2],
}

// ============================================================================
// FUNÇÕES DE ALTO NÍVEL
// ============================================================================

/// Lê estado atual do mouse
pub fn poll_mouse() -> SysResult<MouseState> {
    let mut state = MouseState::default();
    let ret = syscall1(SYS_MOUSE_READ, &mut state as *mut _ as usize);
    check_error(ret)?;
    Ok(state)
}

/// Lê eventos de teclado pendentes
pub fn poll_keyboard(buffer: &mut [KeyEvent]) -> SysResult<usize> {
    if buffer.is_empty() {
        return Ok(0);
    }
    let ret = syscall2(
        SYS_KEYBOARD_READ,
        buffer.as_mut_ptr() as usize,
        buffer.len(),
    );
    check_error(ret)?;
    Ok(ret as usize)
}

/// Lê um único evento de teclado (se disponível)
pub fn read_key() -> SysResult<Option<KeyEvent>> {
    let mut event = KeyEvent::default();
    let ret = syscall2(SYS_KEYBOARD_READ, &mut event as *mut _ as usize, 1);
    check_error(ret)?;
    if ret > 0 {
        Ok(Some(event))
    } else {
        Ok(None)
    }
}
