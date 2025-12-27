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
//! | `fs` | Arquivos (open, read, write, close) |
//! | `process` | Processos (exit, spawn, yield) |
//! | `mem` | Memória (alloc, free, map) |
//! | `ipc` | IPC (Port, send, recv) |
//! | `time` | Tempo (sleep, clock) |
//! | `io` | Handle, Rights |
//! | `event` | poll |
//! | `sys` | sysinfo, debug |

#![no_std]

// === Módulos ===
pub mod event;
pub mod fs;
pub mod io;
pub mod ipc;
pub mod mem;
pub mod process;
pub mod sys;
pub mod syscall;
pub mod time;

// === Prelude ===
pub mod prelude {
    pub use crate::fs::File;
    pub use crate::io::{Handle, HandleRights};
    pub use crate::ipc::Port;
    pub use crate::process::{exit, getpid, yield_now};
    pub use crate::syscall::{SysError, SysResult};
    pub use crate::time::sleep;
}

// === Re-exports ===
pub use syscall::{SysError, SysResult};
