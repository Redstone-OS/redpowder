//! # Keyboard Input
//!
//! Funções e tipos para entrada de teclado.

use crate::syscall::SYS_KEYBOARD_READ;
use crate::syscall::{check_error, syscall2, SysResult};

use super::keycodes::KeyCode;

// =============================================================================
// TIPOS
// =============================================================================

/// Evento de teclado.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct KeyEvent {
    /// Scancode da tecla.
    pub scancode: u8,
    /// Tecla pressionada (true) ou solta (false).
    pub pressed: bool,
    pub _pad: [u8; 6],
}

impl KeyEvent {
    /// Retorna o KeyCode da tecla.
    pub fn keycode(&self) -> KeyCode {
        KeyCode::from_scancode(self.scancode)
    }

    /// Verifica se é evento de tecla pressionada.
    #[inline]
    pub fn is_press(&self) -> bool {
        self.pressed
    }

    /// Verifica se é evento de tecla solta.
    #[inline]
    pub fn is_release(&self) -> bool {
        !self.pressed
    }
}

// =============================================================================
// FUNÇÕES
// =============================================================================

/// Lê eventos de teclado pendentes.
///
/// Retorna o número de eventos lidos.
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

/// Lê um único evento de teclado (se disponível).
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
