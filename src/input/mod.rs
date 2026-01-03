//! # Input Module
//!
//! Módulo de entrada (mouse, teclado, touch).
//!
//! ## Submódulos
//!
//! | Módulo | Descrição |
//! |--------|-----------|
//! | [`mouse`] | Funções e tipos de mouse |
//! | [`keyboard`] | Funções e tipos de teclado |
//! | [`keycodes`] | Códigos de teclas |
//!
//! ## Re-exports de gfx_types
//!
//! Tipos de input são re-exportados de `gfx_types::input`.

pub mod keyboard;
pub mod keycodes;
pub mod mouse;

// =============================================================================
// RE-EXPORTS DE GFX_TYPES
// =============================================================================

pub use gfx_types::input::{
    CursorHotspot, CursorType, GestureType, SwipeDirection, TouchId, TouchPhase, TouchPoint,
};

// =============================================================================
// EXPORTS DO MÓDULO
// =============================================================================

pub use keyboard::{poll_keyboard, read_key, KeyEvent};
pub use keycodes::KeyCode;
pub use mouse::{poll_mouse, MouseButton, MouseState};
