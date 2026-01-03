//! # Módulo de Gráficos
//!
//! API de alto nível para acesso ao framebuffer e desenho.
//!
//! Este módulo é dividido em submódulos especializados e usa
//! `gfx_types` para todos os tipos gráficos fundamentais.
//!
//! ## Submódulos
//!
//! | Módulo | Descrição |
//! |--------|-----------|
//! | [`framebuffer`] | Acesso ao framebuffer do kernel |
//! | [`canvas`] | API de desenho sobre buffers |
//! | [`draw`] | Primitivas de desenho (linhas, círculos) |
//!
//! ## Re-exports de gfx_types
//!
//! Todos os tipos de `gfx_types` são re-exportados aqui para conveniência.

pub mod canvas;
pub mod draw;
pub mod framebuffer;

// =============================================================================
// RE-EXPORTS DE GFX_TYPES
// =============================================================================

// Geometry
pub use gfx_types::geometry::{
    Circle, Ellipse, Insets, Line, LineF, Point, PointF, Rect, RectF, RoundedRect, Size, SizeF,
    Transform2D,
};

// Color
pub use gfx_types::color::{
    AlphaMode, BlendMode, Color, ColorF, Palette, PixelFormat, CATPPUCCIN_MOCHA, DRACULA, NORD,
    REDSTONE_DEFAULT,
};

// Buffer
pub use gfx_types::buffer::{BufferDescriptor, BufferHandle, BufferRegion, BufferUsage};

// Render
pub use gfx_types::render::{BlitParams, ClipOp, ClipRect, FillParams, RenderOp};

// Damage
pub use gfx_types::damage::{DamageHint, DamageRegion};

// =============================================================================
// EXPORTS DO MÓDULO
// =============================================================================

pub use canvas::Canvas;
pub use draw::{draw_circle, draw_line, draw_rect};
pub use framebuffer::{clear_screen, get_info, write_pixels, Framebuffer, FramebufferInfo};
