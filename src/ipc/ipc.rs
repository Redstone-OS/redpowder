//! # IPC - Inter-Process Communication
//!
//! Comunicação entre processos via portas.

use crate::io::Handle;
use crate::syscall::{check_error, syscall1, syscall4, SysResult};
use crate::syscall::{SYS_CREATE_PORT, SYS_RECV_MSG, SYS_SEND_MSG};

/// Flags de mensagem
pub mod flags {
    pub const NONBLOCK: u32 = 1 << 0;
    pub const URGENT: u32 = 1 << 1;
}

/// Porta de IPC
pub struct Port {
    handle: Handle,
}

impl Port {
    /// Cria nova porta
    ///
    /// # Args
    /// - capacity: capacidade máxima de mensagens
    pub fn create(capacity: usize) -> SysResult<Self> {
        let ret = syscall1(SYS_CREATE_PORT, capacity);
        let handle = Handle::from_raw(check_error(ret)? as u32);
        Ok(Self { handle })
    }

    /// Envia mensagem
    ///
    /// # Args
    /// - data: dados da mensagem
    /// - flags: flags de envio
    ///
    /// # Returns
    /// Bytes enviados
    pub fn send(&self, data: &[u8], flags: u32) -> SysResult<usize> {
        let ret = syscall4(
            SYS_SEND_MSG,
            self.handle.raw() as usize,
            data.as_ptr() as usize,
            data.len(),
            flags as usize,
        );
        check_error(ret)
    }

    /// Recebe mensagem
    ///
    /// # Args
    /// - buf: buffer para receber
    /// - timeout_ms: timeout (0 = infinito)
    ///
    /// # Returns
    /// Bytes recebidos
    pub fn recv(&self, buf: &mut [u8], timeout_ms: u64) -> SysResult<usize> {
        let ret = syscall4(
            SYS_RECV_MSG,
            self.handle.raw() as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
            timeout_ms as usize,
        );
        check_error(ret)
    }

    /// Handle interno
    pub fn handle(&self) -> &Handle {
        &self.handle
    }
}

impl Drop for Port {
    fn drop(&mut self) {
        use crate::syscall::{syscall1, SYS_HANDLE_CLOSE};
        let _ = syscall1(SYS_HANDLE_CLOSE, self.handle.raw() as usize);
    }
}
