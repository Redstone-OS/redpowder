//! # Firefly Window Protocol
//!
//! Protocolo de comunicação entre clientes (apps) e compositor Firefly.
//!
//! Este módulo é dividido em submódulos especializados.
//!
//! ## Submódulos
//!
//! | Módulo | Descrição |
//! |--------|-----------|
//! | [`protocol`] | Mensagens e opcodes do protocolo |
//! | [`client`] | Cliente de janela (Window) |
//!
//! ## Re-exports de gfx_types
//!
//! Tipos de janela são re-exportados de `gfx_types::window`.

pub mod client;
pub mod protocol;

// =============================================================================
// RE-EXPORTS DE GFX_TYPES
// =============================================================================

pub use gfx_types::window::{
    BufferMode, LayerType, SurfaceConfig, SurfaceId, SurfaceType, WindowEffects, WindowFlags,
    WindowState, WindowType,
};

// =============================================================================
// EXPORTS DO MÓDULO
// =============================================================================

pub use client::Window;
pub use protocol::{
    lifecycle_events, opcodes, CommitBufferRequest, CreateWindowRequest, DestroyWindowRequest,
    ErrorResponse, MoveWindowRequest, ProtocolMessage, RegisterTaskbarRequest, ResizeWindowRequest,
    SetWindowFlagsRequest, WindowCreatedResponse, WindowLifecycleEvent, WindowOpRequest,
    COMPOSITOR_PORT, MAX_MSG_SIZE,
};
