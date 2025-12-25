//! Time - Funções de tempo
//!
//! Relógios, sleep e monotonic.

use crate::syscall::{syscall0, syscall1, SysError, SysResult, SYS_MONOTONIC, SYS_SLEEP};

/// Dorme por N milissegundos
pub fn sleep(ms: u64) -> SysResult<u64> {
    let ret = syscall1(SYS_SLEEP, ms as usize);

    if ret < 0 {
        Err(SysError::from_code(ret))
    } else {
        Ok(ret as u64)
    }
}

/// Obtém tempo monotônico em ticks desde o boot
pub fn monotonic() -> u64 {
    syscall0(SYS_MONOTONIC) as u64
}

/// Obtém tempo em milissegundos desde o boot (aproximado)
///
/// Assume timer de 100Hz
pub fn uptime_ms() -> u64 {
    monotonic() * 10
}
