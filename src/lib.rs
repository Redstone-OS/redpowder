//! # Redpowder SDK v0.3.0
//!
//! SDK para desenvolvimento userland no Redstone OS.
//!
//! ## Filosofia
//! - **No-std**: Zero dependências de runtime
//! - **Type-safe**: Handles tipados, erros explícitos
//! - **Capability-based**: Segue modelo do kernel
//! - **GFX-Powered**: Tipos gráficos completos via `gfx_types`
//!
//! ## Módulos
//!
//! | Módulo | Função |
//! |--------|--------|
//! | [`syscall`] | Invocação de syscalls (inline asm) |
//! | [`console`] | print!, println!, reboot, poweroff |
//! | [`fs`] | Arquivos e diretórios (File, Dir, stat) |
//! | [`process`] | Processos (exit, spawn, yield) |
//! | [`mem`] | Memória (alloc, free, map) |
//! | [`ipc`] | IPC (Port, send, recv) |
//! | [`time`] | Tempo (sleep, clock) |
//! | [`io`] | Handle, Rights |
//! | [`event`] | Eventos e polling |
//! | [`sys`] | sysinfo, debug |
//! | [`graphics`] | Framebuffer, canvas, desenho |
//! | [`input`] | Mouse, teclado, touch |
//! | [`window`] | Janelas (protocolo Firefly) |
//! | [`gfx`] | Re-export completo de `gfx_types` |
//! | [`math`] | Re-export de `rdsmath` |
//!
//! ## Exemplo Rápido
//!
//! ```rust
//! #![no_std]
//! use redpowder::prelude::*;
//!
//! fn main() {
//!     // Criar cor usando gfx_types
//!     let bg = Color::from_hex(0x1e1e2e);
//!     let fg = Color::WHITE;
//!     
//!     // Geometria
//!     let rect = Rect::new(10, 10, 100, 50);
//!     let point = Point::new(50, 30);
//!     
//!     println!("Rect contains point: {}", rect.contains_point(point));
//! }
//! ```

#![no_std]

// =============================================================================
// MÓDULOS INTERNOS
// =============================================================================

pub mod console;
pub mod event;
pub mod fs;
pub mod graphics;
pub mod input;
pub mod io;
pub mod ipc;
pub mod mem;
pub mod process;
pub mod sys;
pub mod syscall;
pub mod time;
pub mod window;

// =============================================================================
// RE-EXPORTS DE LIBS EXTERNAS
// =============================================================================

/// Re-export completo de `gfx_types` para tipos gráficos.
///
/// Inclui: geometry, color, buffer, display, window, damage, render, input, text.
pub mod gfx {
    pub use gfx_types::*;
}

/// Re-export completo de `rdsmath` para funções matemáticas.
///
/// Inclui: consts, trig, exp, round, util.
pub mod math {
    pub use rdsmath::*;
}

// =============================================================================
// PRELUDE
// =============================================================================

/// Prelude com os tipos e funções mais comuns.
///
/// Inclui tipos de gfx_types e funções matemáticas para uso direto.
pub mod prelude {
    // Console
    pub use crate::console::{poweroff, reboot};
    pub use crate::print;
    pub use crate::println;

    // Filesystem
    pub use crate::fs::{chdir, exists, getcwd, is_dir, is_file, stat};
    pub use crate::fs::{Dir, DirEntry, File, FileStat, OpenFlags};

    // IO
    pub use crate::io::{Handle, HandleRights};

    // IPC
    pub use crate::ipc::Port;

    // Process
    pub use crate::process::{exit, getpid, yield_now};

    // Syscall
    pub use crate::syscall::{SysError, SysResult};

    // Time
    pub use crate::time::sleep;

    // Graphics (high-level SDK)
    pub use crate::graphics::{clear_screen, get_info, write_pixels};
    pub use crate::graphics::{draw_circle, draw_line, draw_rect};
    pub use crate::graphics::{Canvas, Framebuffer, FramebufferInfo};

    // Input (SDK)
    pub use crate::input::{poll_keyboard, poll_mouse, read_key};
    pub use crate::input::{KeyCode, KeyEvent, MouseButton, MouseState};

    // Window (SDK)
    pub use crate::window::Window;

    // =========================================================================
    // GFX_TYPES - Tipos gráficos completos
    // =========================================================================

    // Geometry
    pub use gfx_types::geometry::{
        Circle, Ellipse, Insets, Line, LineF, Point, PointF, Rect, RectF, RoundedRect, Size, SizeF,
        Transform2D,
    };

    // Color
    pub use gfx_types::color::{
        AlphaMode, BlendMode, Color, ColorF, PixelFormat, CATPPUCCIN_MOCHA, DRACULA, NORD,
        REDSTONE_DEFAULT,
    };

    // Buffer
    pub use gfx_types::buffer::{BufferDescriptor, BufferHandle};

    // Display
    pub use gfx_types::display::{DisplayInfo, DisplayMode, VsyncMode};

    // Window (gfx_types)
    pub use gfx_types::window::{
        LayerType, SurfaceId, WindowEffects, WindowFlags, WindowState, WindowType,
    };

    // Damage
    pub use gfx_types::damage::DamageRegion;

    // Input (gfx_types)
    pub use gfx_types::input::{CursorType, GestureType, TouchPoint};

    // Render
    pub use gfx_types::render::{BlitParams, ClipRect, FillParams, RenderOp};

    // Text
    pub use gfx_types::text::{FontStyle, FontWeight, TextAlign};

    // =========================================================================
    // RDSMATH - Funções matemáticas
    // =========================================================================

    pub use rdsmath::{
        absf, atan2f, ceilf, clampf, cosf, floorf, lerpf, roundf, saturatef, sinf, smoothstepf,
        sqrtf, tanf, PI, TAU,
    };
}

// =============================================================================
// RE-EXPORTS PRINCIPAIS
// =============================================================================

pub use syscall::{SysError, SysResult};

// Re-export dos tipos mais usados no nível raiz
pub use gfx_types::color::Color;
pub use gfx_types::geometry::{Point, Rect, Size};
