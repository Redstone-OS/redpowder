//! # Process Control
//!
//! Controle de processos.

use crate::syscall::{check_error, syscall0, syscall1, syscall4, SysResult};
use crate::syscall::{SYS_EXIT, SYS_GETPID, SYS_SPAWN, SYS_WAIT, SYS_YIELD};
use core::arch::asm;

/// Encerra o processo atual
///
/// Esta função nunca retorna.
pub fn exit(code: i32) -> ! {
    let _ = syscall1(SYS_EXIT, code as usize);
    // Nunca deveria chegar aqui
    loop {
        unsafe { asm!("hlt") };
    }
}

/// Cede o restante do quantum de tempo
pub fn yield_now() -> SysResult<()> {
    check_error(syscall0(SYS_YIELD))?;
    Ok(())
}

/// Obtém PID do processo atual
pub fn getpid() -> usize {
    syscall0(SYS_GETPID) as usize
}

/// Cria novo processo
///
/// # Args
/// - path: caminho do executável
/// - args: argumentos (pode ser vazio)
///
/// # Returns
/// PID do novo processo
pub fn spawn(path: &str, args: &[&str]) -> SysResult<usize> {
    let args_ptr = if args.is_empty() {
        0
    } else {
        args.as_ptr() as usize
    };

    let ret = syscall4(
        SYS_SPAWN,
        path.as_ptr() as usize,
        path.len(),
        args_ptr,
        args.len(),
    );

    check_error(ret)
}

/// Espera processo terminar
///
/// # Args
/// - pid: PID do processo (0 = qualquer filho)
/// - timeout_ms: timeout em ms (0 = infinito)
///
/// # Returns
/// Exit code do processo
pub fn wait(pid: usize, timeout_ms: u64) -> SysResult<i32> {
    let ret = syscall2(SYS_WAIT, pid, timeout_ms as usize);
    check_error(ret).map(|v| v as i32)
}

// Importar syscall2
use crate::syscall::syscall2;
