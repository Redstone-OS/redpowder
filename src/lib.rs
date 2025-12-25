//! # Redpowder - SDK para Redstone OS
//!
//! Kit de desenvolvimento oficial para criar aplicações no Redstone OS.
//!
//! ## Uso Rápido
//!
//! ```rust
//! #![no_std]
//! #![no_main]
//!
//! use redpowder::prelude::*;
//!
//! #[no_mangle]
//! pub extern "C" fn _start() -> ! {
//!     println!("Hello from Redstone OS!");
//!     sys_exit(0);
//! }
//! ```
//!
//! ## Módulos
//!
//! - [`syscall`] - Wrappers para syscalls do kernel
//! - [`io`] - Input/Output (console, arquivos)
//! - [`memory`] - Alocação de memória
//! - [`ipc`] - Comunicação entre processos
//! - [`time`] - Funções de tempo
//! - [`prelude`] - Re-exports convenientes

#![no_std]
#![feature(asm_const)]

// Módulos públicos
pub mod io;
pub mod ipc;
pub mod memory;
pub mod prelude;
pub mod syscall;
pub mod time;

// Re-exports principais
pub use syscall::{SysError, SysResult};
