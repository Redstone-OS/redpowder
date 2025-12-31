//! # Events
//!
//! Multiplexação de I/O.

use crate::io::Handle;
use crate::syscall::SYS_POLL;
use crate::syscall::{check_error, syscall3, SysResult};

/// Eventos de poll
pub mod events {
    pub const IN: u16 = 1 << 0; // Dados disponíveis
    pub const OUT: u16 = 1 << 1; // Espaço para escrita
    pub const ERR: u16 = 1 << 2; // Erro
    pub const HUP: u16 = 1 << 3; // Hangup
    pub const NVAL: u16 = 1 << 4; // Handle inválido
}

/// Descritor de poll
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PollFd {
    pub handle: u32,
    pub events: u16,
    pub revents: u16,
}

impl PollFd {
    /// Cria novo descritor
    pub fn new(handle: &Handle, events: u16) -> Self {
        Self {
            handle: handle.raw(),
            events,
            revents: 0,
        }
    }

    /// Verifica se evento ocorreu
    pub fn has_event(&self, event: u16) -> bool {
        (self.revents & event) != 0
    }
}

/// Espera eventos em múltiplos handles
///
/// # Args
/// - fds: array de PollFd
/// - timeout_ms: timeout (-1 = infinito, 0 = não bloqueia)
///
/// # Returns
/// Número de handles com eventos
pub fn poll(fds: &mut [PollFd], timeout_ms: i64) -> SysResult<usize> {
    let ret = syscall3(
        SYS_POLL,
        fds.as_mut_ptr() as usize,
        fds.len(),
        timeout_ms as usize,
    );
    check_error(ret)
}

// ============================================================================
// Tipos de Eventos (High Level)
// ============================================================================

/// Tipos de Eventos de Input (Códigos)
pub mod event_type {
    pub const KEY_DOWN: u32 = 1;
    pub const KEY_UP: u32 = 2;
    pub const MOUSE_MOVE: u32 = 3;
    pub const MOUSE_DOWN: u32 = 4;
    pub const MOUSE_UP: u32 = 5;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct InputEvent {
    pub op: u32,         // EVENT_INPUT
    pub event_type: u32, // event_type constants
    pub param1: u32,     // KeyCode ou MouseX
    pub param2: u32,     // Modifiers ou MouseY
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ResizeEvent {
    pub op: u32, // EVENT_RESIZE
    pub width: u32,
    pub height: u32,
}

/// Enum de Eventos de Alto Nível para a API
#[derive(Debug, Clone, Copy)]
pub enum Event {
    Input(InputEvent),
    Resize(ResizeEvent),
    Unknown,
}
