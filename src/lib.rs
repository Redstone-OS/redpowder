//! # Redpowder SDK
//!
//! SDK para desenvolvimento userland no Redstone OS.
//!
//! ## Filosofia
//! - **No-std**: Zero dependências de runtime
//! - **Type-safe**: Handles tipados, erros explícitos
//! - **Capability-based**: Segue modelo do kernel
//!
//! ## Módulos
//!
//! | Módulo | Função |
//! |--------|--------|
//! | `syscall` | Invocação de syscalls (inline asm) |
//! | `console` | print!, println!, reboot, poweroff |
//! | `fs` | Arquivos e diretórios (File, Dir, stat) |
//! | `process` | Processos (exit, spawn, yield) |
//! | `mem` | Memória (alloc, free, map) |
//! | `ipc` | IPC (Port, send, recv) |
//! | `time` | Tempo (sleep, clock) |
//! | `io` | Handle, Rights |
//! | `event` | poll |
//! | `sys` | sysinfo, debug |
//! | `graphics` | Framebuffer, cores, desenho |
//! | `input` | Mouse, teclado |
//! | `window` | Janelas |
//!
//! ## Exemplo Rápido
//!
//! ```rust
//! #![no_std]
//! use redpowder::prelude::*;
//! use redpowder::fs::{File, Dir};
//!
//! fn main() {
//!     // Ler arquivo
//!     if let Ok(file) = File::open("/apps/config.txt") {
//!         let mut buf = [0u8; 256];
//!         if let Ok(bytes) = file.read(&mut buf) {
//!             println!("Lido: {} bytes", bytes);
//!         }
//!     }
//!
//!     // Listar diretório
//!     if let Ok(dir) = Dir::open("/apps") {
//!         for entry in dir.entries() {
//!             println!("{}", entry.name());
//!         }
//!     }
//! }
//! ```

#![no_std]

// === Módulos ===
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

// === Prelude ===
/// Prelude com os tipos e funções mais comuns
pub mod prelude {
    pub use crate::console::{poweroff, reboot};
    pub use crate::fs::{chdir, exists, getcwd, is_dir, is_file, stat};
    pub use crate::fs::{Dir, DirEntry, File, FileStat, OpenFlags};
    pub use crate::graphics::{Color, Framebuffer, FramebufferInfo};
    pub use crate::input::{poll_mouse, KeyEvent, MouseState};
    pub use crate::io::{Handle, HandleRights};
    pub use crate::ipc::Port;
    pub use crate::print;
    pub use crate::println;
    pub use crate::process::{exit, getpid, yield_now};
    pub use crate::syscall::{SysError, SysResult};
    pub use crate::time::sleep;
}

// === Re-exports ===
pub use syscall::{SysError, SysResult};
