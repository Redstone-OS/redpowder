//! # IPC - Inter-Process Communication
//!
//! Comunicação entre processos via portas e memória compartilhada.

use crate::io::Handle;
use crate::syscall::{
    check_error, syscall1, syscall2, syscall4, SysResult, SYS_CREATE_PORT, SYS_HANDLE_DUP,
    SYS_PORT_CONNECT, SYS_RECV_MSG, SYS_SEND_MSG, SYS_SHM_ATTACH, SYS_SHM_CREATE, SYS_SHM_GET_SIZE,
};

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
    /// Cria nova porta nomeada
    pub fn create(name: &str, capacity: usize) -> SysResult<Self> {
        let ret = syscall4(
            SYS_CREATE_PORT,
            name.as_ptr() as usize,
            name.len(),
            capacity,
            0,
        );
        let handle = Handle::from_raw(check_error(ret)? as u32);
        Ok(Self { handle })
    }

    /// Conecta a uma porta nomeada
    pub fn connect(name: &str) -> SysResult<Self> {
        let ret = syscall2(SYS_PORT_CONNECT, name.as_ptr() as usize, name.len());
        let handle = Handle::from_raw(check_error(ret)? as u32);
        Ok(Self { handle })
    }

    /// Envia mensagem
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
    /// Recebe mensagem
    pub fn recv(&self, buf: &mut [u8], timeout_ms: u64) -> SysResult<usize> {
        let mut waited = 0;
        let poll_interval = 10;

        loop {
            let ret = syscall4(
                SYS_RECV_MSG,
                self.handle.raw() as usize,
                buf.as_mut_ptr() as usize,
                buf.len(),
                0, // Kernel ignora timeout param por enquanto
            );

            match check_error(ret) {
                Ok(len) => {
                    if len > 0 {
                        return Ok(len);
                    }
                    // Se len == 0, fila vazia
                }
                Err(e) => return Err(e),
            }

            if timeout_ms == 0 {
                return Ok(0);
            }

            if waited >= timeout_ms {
                // Timeout expirou, retorna 0 bytes (sem mensagem)
                return Ok(0);
            }

            // Dormir e tentar novamente
            let _ = crate::time::sleep(poll_interval);
            waited += poll_interval;
        }
    }

    /// Handle interno
    pub fn handle(&self) -> &Handle {
        &self.handle
    }
}

impl Clone for Port {
    fn clone(&self) -> Self {
        let ret = syscall2(
            SYS_HANDLE_DUP,
            self.handle.raw() as usize,
            0xFFFFFFFFFFFFFFFF,
        );
        let new_handle = Handle::from_raw(check_error(ret).unwrap_or(0) as u32);
        Self { handle: new_handle }
    }
}

impl Drop for Port {
    fn drop(&mut self) {
        if self.handle.is_valid() {
            use crate::syscall::{syscall1, SYS_HANDLE_CLOSE};
            let _ = syscall1(SYS_HANDLE_CLOSE, self.handle.raw() as usize);
        }
    }
}

// ============================================================================
// SHARED MEMORY
// ============================================================================

/// ID de região de memória compartilhada
#[derive(Debug, Clone, Copy)]
pub struct ShmId(pub u64);

/// Região de memória compartilhada mapeada
pub struct SharedMemory {
    id: ShmId,
    addr: *mut u8,
    size: usize,
}

impl SharedMemory {
    /// Cria nova região de memória compartilhada
    pub fn create(size: usize) -> SysResult<Self> {
        let ret = syscall1(SYS_SHM_CREATE, size);
        let id = ShmId(check_error(ret)? as u64);

        // Mapear automaticamente
        let ret = syscall2(SYS_SHM_ATTACH, id.0 as usize, 0);
        let addr = check_error(ret)? as *mut u8;

        Ok(Self { id, addr, size })
    }

    /// Abre região existente pelo ID
    pub fn open(id: ShmId) -> SysResult<Self> {
        // Primeiro, obter o tamanho real da região SHM
        let size_ret = syscall1(SYS_SHM_GET_SIZE, id.0 as usize);
        let size = check_error(size_ret)?;

        // Agora mapear a região
        let ret = syscall2(SYS_SHM_ATTACH, id.0 as usize, 0);
        let addr = check_error(ret)? as *mut u8;

        Ok(Self { id, addr, size })
    }

    /// ID da região
    pub fn id(&self) -> ShmId {
        self.id
    }

    /// Ponteiro para a memória
    pub fn as_ptr(&self) -> *const u8 {
        self.addr
    }

    /// Ponteiro mutável para a memória
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.addr
    }

    /// Tamanho em bytes
    pub fn size(&self) -> usize {
        self.size
    }

    /// Acesso como slice
    pub fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.addr, self.size) }
    }

    /// Acesso como slice mutável
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.addr, self.size) }
    }
}
