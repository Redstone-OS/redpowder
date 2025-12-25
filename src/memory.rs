//! Memory - Alocação de memória
//!
//! Funções para alocar e liberar memória.

use crate::syscall::{syscall2, SysError, SysResult, SYS_ALLOC, SYS_FREE};

/// Flags de alocação
pub mod flags {
    pub const READ: u32 = 1 << 0;
    pub const WRITE: u32 = 1 << 1;
    pub const EXEC: u32 = 1 << 2;
}

/// Aloca memória virtual
///
/// # Argumentos
/// - `size`: Tamanho em bytes
/// - `flags`: Permissões (READ, WRITE, EXEC)
///
/// # Retorno
/// Endereço da região alocada
pub fn alloc(size: usize, flags: u32) -> SysResult<*mut u8> {
    let ret = syscall2(SYS_ALLOC, size, flags as usize);

    if ret < 0 {
        Err(SysError::from_code(ret))
    } else {
        Ok(ret as *mut u8)
    }
}

/// Libera memória alocada
pub fn free(ptr: *mut u8, size: usize) -> SysResult<()> {
    let ret = syscall2(SYS_FREE, ptr as usize, size);

    if ret < 0 {
        Err(SysError::from_code(ret))
    } else {
        Ok(())
    }
}

/// Aloca memória com permissões padrão (READ + WRITE)
pub fn alloc_rw(size: usize) -> SysResult<*mut u8> {
    alloc(size, flags::READ | flags::WRITE)
}
