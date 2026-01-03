//! # Protocolo Firefly
//!
//! Definições de mensagens do protocolo de comunicação com o compositor.

use crate::event::{InputEvent, ResizeEvent};

// =============================================================================
// CONSTANTES
// =============================================================================

/// Nome da porta do compositor.
pub const COMPOSITOR_PORT: &str = "firefly.compositor";

/// Tamanho máximo de mensagem.
pub const MAX_MSG_SIZE: usize = 256;

// =============================================================================
// OPCODES
// =============================================================================

/// Identificadores de mensagem (OpCodes).
pub mod opcodes {
    // Client -> Server
    pub const CREATE_WINDOW: u32 = 0x01;
    pub const DESTROY_WINDOW: u32 = 0x02;
    pub const COMMIT_BUFFER: u32 = 0x03;
    pub const INPUT_UPDATE: u32 = 0x04;
    pub const MINIMIZE_WINDOW: u32 = 0x05;
    pub const RESTORE_WINDOW: u32 = 0x06;
    pub const REGISTER_TASKBAR: u32 = 0x07;
    pub const SET_WINDOW_FLAGS: u32 = 0x08;
    pub const MOVE_WINDOW: u32 = 0x09;
    pub const RESIZE_WINDOW: u32 = 0x0A;

    // Server -> Client
    pub const WINDOW_CREATED: u32 = 0x10;
    pub const EVENT_INPUT: u32 = 0x20;
    pub const EVENT_RESIZE: u32 = 0x21;
    pub const EVENT_WINDOW_LIFECYCLE: u32 = 0x22;
    pub const EVENT_FOCUS: u32 = 0x23;
    pub const ERROR: u32 = 0xFF;
}

/// Tipos de eventos de lifecycle.
pub mod lifecycle_events {
    pub const CREATED: u32 = 0;
    pub const DESTROYED: u32 = 1;
    pub const MINIMIZED: u32 = 2;
    pub const RESTORED: u32 = 3;
    pub const FOCUSED: u32 = 4;
    pub const UNFOCUSED: u32 = 5;
}

// =============================================================================
// REQUESTS (Client -> Server)
// =============================================================================

/// Request para criar janela.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CreateWindowRequest {
    pub op: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub flags: u32,
    /// Nome da porta onde o servidor deve responder.
    pub reply_port: [u8; 32],
    /// Título da janela / Nome da aplicação.
    pub title: [u8; 64],
}

/// Request para registrar taskbar.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RegisterTaskbarRequest {
    pub op: u32,
    /// Porta para receber eventos de lifecycle.
    pub listener_port: [u8; 32],
}

/// Request para destruir janela.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DestroyWindowRequest {
    pub op: u32,
    pub window_id: u32,
}

/// Request para commit de buffer.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CommitBufferRequest {
    pub op: u32,
    pub window_id: u32,
    pub x: u32,      // Dirty Rect X
    pub y: u32,      // Dirty Rect Y
    pub width: u32,  // Dirty Rect W
    pub height: u32, // Dirty Rect H
}

/// Request genérico para operações de janela.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WindowOpRequest {
    pub op: u32,
    pub window_id: u32,
}

/// Request para mover janela.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MoveWindowRequest {
    pub op: u32,
    pub window_id: u32,
    pub x: i32,
    pub y: i32,
}

/// Request para redimensionar janela.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ResizeWindowRequest {
    pub op: u32,
    pub window_id: u32,
    pub width: u32,
    pub height: u32,
}

/// Request para alterar flags da janela.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SetWindowFlagsRequest {
    pub op: u32,
    pub window_id: u32,
    pub flags: u32,
}

// =============================================================================
// RESPONSES (Server -> Client)
// =============================================================================

/// Response de janela criada.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WindowCreatedResponse {
    pub op: u32,
    pub window_id: u32,
    pub shm_handle: u64,
    pub buffer_size: u64,
}

/// Response de erro.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ErrorResponse {
    pub op: u32,
    pub code: u32,
}

/// Evento de lifecycle de janela.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WindowLifecycleEvent {
    pub op: u32,
    pub event_type: u32,
    pub window_id: u32,
    pub title: [u8; 64],
}

// =============================================================================
// PROTOCOL MESSAGE UNION
// =============================================================================

/// União de todas as mensagens possíveis (para leitura genérica).
#[repr(C)]
pub union ProtocolMessage {
    pub header: u32,
    pub create_req: CreateWindowRequest,
    pub buf_req: CommitBufferRequest,
    pub destroy_req: DestroyWindowRequest,
    pub op_req: WindowOpRequest,
    pub move_req: MoveWindowRequest,
    pub resize_req: ResizeWindowRequest,
    pub flags_req: SetWindowFlagsRequest,
    pub reg_taskbar_req: RegisterTaskbarRequest,
    pub win_resp: WindowCreatedResponse,
    pub input_evt: InputEvent,
    pub resize_evt: ResizeEvent,
    pub lifecycle_evt: WindowLifecycleEvent,
    pub raw: [u8; MAX_MSG_SIZE],
}
