//! # System
//!
//! Informações do sistema e debug.

use crate::syscall::{check_error, syscall2, syscall3, SysResult};
use crate::syscall::{SYS_DEBUG, SYS_SYSINFO};

/// Informações do sistema
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct SysInfo {
    pub kernel_version: u32,
    pub abi_version: u32,
    pub total_memory: u64,
    pub free_memory: u64,
    pub uptime_ms: u64,
    pub num_cpus: u32,
    pub num_processes: u32,
}

/// Obtém informações do sistema
pub fn sysinfo() -> SysResult<SysInfo> {
    let mut info = SysInfo::default();
    let ret = syscall2(
        SYS_SYSINFO,
        &mut info as *mut SysInfo as usize,
        core::mem::size_of::<SysInfo>(),
    );
    check_error(ret)?;
    Ok(info)
}

/// Debug: imprime no log do kernel
pub fn kprint(s: &str) -> SysResult<usize> {
    let ret = syscall3(SYS_DEBUG, 0x01, s.as_ptr() as usize, s.len());
    check_error(ret)
}

/// Debug: breakpoint
pub fn breakpoint() {
    let _ = syscall3(SYS_DEBUG, 0x04, 0, 0);
}
