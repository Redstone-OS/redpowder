//! Prelude - Importações convenientes
//!
//! Use `use redpowder::prelude::*;` para importar as funções mais comuns.

// Syscalls principais
pub use crate::syscall::{sys_exit, sys_getpid, sys_yield};

// IO
pub use crate::io::{print, println};

// Tempo
pub use crate::time::{monotonic, sleep};

// Erros
pub use crate::syscall::{SysError, SysResult};
