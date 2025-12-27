//! # Syscall Interface
//!
//! Invocação direta de syscalls usando instrução `syscall`.

mod error;
mod numbers;
mod raw;

pub use error::{check_error, SysError, SysResult};
pub use numbers::*;
pub use raw::*;
