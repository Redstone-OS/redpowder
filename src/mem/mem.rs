//! # Memory Management
//!
//! Alocação e mapeamento de memória.

use crate::syscall::{check_error, syscall2, syscall4, SysResult};
use crate::syscall::{SYS_ALLOC, SYS_FREE, SYS_MAP, SYS_UNMAP};

/// Flags de alocação
pub mod flags {
    pub const ZEROED: u32 = 1 << 0;
    pub const GUARD: u32 = 1 << 1;
}

/// Flags de mapeamento
pub mod map_flags {
    pub const READ: u32 = 1 << 0;
    pub const WRITE: u32 = 1 << 1;
    pub const EXEC: u32 = 1 << 2;
    pub const SHARED: u32 = 1 << 3;
    pub const PRIVATE: u32 = 1 << 4;
    pub const FIXED: u32 = 1 << 5;
}

/// Aloca memória virtual
///
/// # Args
/// - size: tamanho em bytes
/// - flags: flags de alocação
///
/// # Returns
/// Ponteiro para memória alocada
pub fn alloc(size: usize, flags: u32) -> SysResult<*mut u8> {
    let ret = syscall2(SYS_ALLOC, size, flags as usize);
    check_error(ret).map(|v| v as *mut u8)
}

/// Libera memória alocada
pub fn free(ptr: *mut u8, size: usize) -> SysResult<()> {
    check_error(syscall2(SYS_FREE, ptr as usize, size))?;
    Ok(())
}

/// Mapeia memória
///
/// # Args
/// - addr: endereço desejado (0 = kernel escolhe)
/// - size: tamanho
/// - flags: permissões
/// - handle: handle do objeto (0 = anônimo)
pub fn map(addr: usize, size: usize, flags: u32, handle: u32) -> SysResult<*mut u8> {
    let ret = syscall4(SYS_MAP, addr, size, flags as usize, handle as usize);
    check_error(ret).map(|v| v as *mut u8)
}

/// Remove mapeamento
pub fn unmap(addr: *mut u8, size: usize) -> SysResult<()> {
    check_error(syscall2(SYS_UNMAP, addr as usize, size))?;
    Ok(())
}
