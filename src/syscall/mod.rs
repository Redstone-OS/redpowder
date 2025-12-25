//! Syscalls do Redstone OS
//!
//! Wrappers seguros para invocar syscalls do kernel.
//!
//! # Numeração
//!
//! O Redstone OS usa numeração própria (NÃO compatível com Linux/POSIX).

use core::arch::asm;

// Sub-módulos
mod error;
mod numbers;

pub use error::{SysError, SysResult};
pub use numbers::*;

// ============================================================================
// INVOCAÇÃO DE SYSCALL
// ============================================================================

/// Invoca syscall com 0 argumentos
#[inline(always)]
pub fn syscall0(num: usize) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "int 0x80",
            inlateout("rax") num => ret,
            options(nostack)
        );
    }
    ret
}

/// Invoca syscall com 1 argumento
#[inline(always)]
pub fn syscall1(num: usize, arg1: usize) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "int 0x80",
            inlateout("rax") num => ret,
            in("rdi") arg1,
            options(nostack)
        );
    }
    ret
}

/// Invoca syscall com 2 argumentos
#[inline(always)]
pub fn syscall2(num: usize, arg1: usize, arg2: usize) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "int 0x80",
            inlateout("rax") num => ret,
            in("rdi") arg1,
            in("rsi") arg2,
            options(nostack)
        );
    }
    ret
}

/// Invoca syscall com 3 argumentos
#[inline(always)]
pub fn syscall3(num: usize, arg1: usize, arg2: usize, arg3: usize) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "int 0x80",
            inlateout("rax") num => ret,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            options(nostack)
        );
    }
    ret
}

/// Invoca syscall com 4 argumentos
#[inline(always)]
pub fn syscall4(num: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "int 0x80",
            inlateout("rax") num => ret,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("r10") arg4,
            options(nostack)
        );
    }
    ret
}

// ============================================================================
// PROCESSO
// ============================================================================

/// Encerra o processo atual.
///
/// # Exemplo
/// ```
/// sys_exit(0); // Sucesso
/// sys_exit(1); // Erro
/// ```
pub fn sys_exit(code: i32) -> ! {
    let _ = syscall1(SYS_EXIT, code as usize);
    loop {
        unsafe { asm!("hlt") };
    }
}

/// Cede o restante do quantum de tempo.
pub fn sys_yield() -> SysResult<()> {
    let ret = syscall0(SYS_YIELD);
    if ret < 0 {
        Err(SysError::from_code(ret))
    } else {
        Ok(())
    }
}

/// Obtém o PID do processo atual.
pub fn sys_getpid() -> usize {
    syscall0(SYS_GETPID) as usize
}

// ============================================================================
// IO INTERNO
// ============================================================================

/// Estrutura para IO vetorizado
#[repr(C)]
pub struct IoVec {
    pub base: *const u8,
    pub len: usize,
}

impl IoVec {
    pub fn new(data: &[u8]) -> Self {
        Self {
            base: data.as_ptr(),
            len: data.len(),
        }
    }
}

/// Escreve dados em um handle
pub fn sys_write(handle: usize, data: &[u8]) -> SysResult<usize> {
    let iov = IoVec::new(data);
    let ret = syscall4(SYS_WRITEV, handle, &iov as *const IoVec as usize, 1, 0);

    if ret < 0 {
        Err(SysError::from_code(ret))
    } else {
        Ok(ret as usize)
    }
}

/// Debug: imprime no log do kernel
pub fn sys_kprint(s: &str) -> SysResult<usize> {
    let ret = syscall3(SYS_DEBUG, 0x01, s.as_ptr() as usize, s.len());

    if ret < 0 {
        Err(SysError::from_code(ret))
    } else {
        Ok(ret as usize)
    }
}
