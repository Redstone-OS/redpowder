//! # Time
//!
//! Operações de tempo.

use crate::syscall::{check_error, syscall1, syscall2, SysResult};
use crate::syscall::{SYS_CLOCK_GET, SYS_SLEEP};

/// Tipos de clock
#[repr(u32)]
pub enum ClockId {
    Realtime = 0,
    Monotonic = 1,
    ProcessCpu = 2,
    ThreadCpu = 3,
}

/// Estrutura de tempo
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TimeSpec {
    pub seconds: u64,
    pub nanoseconds: u32,
    pub _pad: u32,
}

impl TimeSpec {
    /// Converte para milissegundos
    pub fn to_millis(&self) -> u64 {
        self.seconds * 1000 + (self.nanoseconds / 1_000_000) as u64
    }

    /// Cria de milissegundos
    pub fn from_millis(ms: u64) -> Self {
        Self {
            seconds: ms / 1000,
            nanoseconds: ((ms % 1000) * 1_000_000) as u32,
            _pad: 0,
        }
    }
}

/// Obtém tempo do clock especificado
pub fn clock_get(clock: ClockId) -> SysResult<TimeSpec> {
    let mut ts = TimeSpec::default();
    let ret = syscall2(
        SYS_CLOCK_GET,
        clock as usize,
        &mut ts as *mut TimeSpec as usize,
    );
    check_error(ret)?;
    Ok(ts)
}

/// Dorme por N milissegundos
pub fn sleep(ms: u64) -> SysResult<u64> {
    let ret = syscall1(SYS_SLEEP, ms as usize);
    check_error(ret).map(|v| v as u64)
}

/// Obtém tempo monotônico (desde boot)
pub fn monotonic() -> SysResult<TimeSpec> {
    clock_get(ClockId::Monotonic)
}

/// Obtém tempo desde boot em milissegundos
///
/// Versão simplificada de monotonic() para uso comum.
pub fn clock() -> SysResult<u64> {
    monotonic().map(|ts| ts.to_millis())
}
