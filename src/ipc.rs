//! IPC - Inter-Process Communication
//!
//! Comunicação entre processos via portas de mensagens.

use crate::syscall::{syscall1, syscall4, SysError, SysResult};
use crate::syscall::{SYS_CREATE_PORT, SYS_PEEK_MSG, SYS_RECV_MSG, SYS_SEND_MSG};

/// Handle para uma porta de IPC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Port(pub usize);

impl Port {
    /// Porta inválida
    pub const INVALID: Port = Port(usize::MAX);
}

/// Cria uma nova porta de IPC
///
/// # Argumentos
/// - `capacity`: Tamanho máximo da fila de mensagens
pub fn create_port(capacity: usize) -> SysResult<Port> {
    let ret = syscall1(SYS_CREATE_PORT, capacity);

    if ret < 0 {
        Err(SysError::from_code(ret))
    } else {
        Ok(Port(ret as usize))
    }
}

/// Envia mensagem para uma porta
pub fn send(port: Port, data: &[u8]) -> SysResult<usize> {
    let ret = syscall4(SYS_SEND_MSG, port.0, data.as_ptr() as usize, data.len(), 0);

    if ret < 0 {
        Err(SysError::from_code(ret))
    } else {
        Ok(ret as usize)
    }
}

/// Recebe mensagem de uma porta
///
/// # Argumentos
/// - `port`: Porta de origem
/// - `buf`: Buffer de destino
/// - `timeout_ms`: Timeout em milissegundos (0 = não bloquear)
pub fn recv(port: Port, buf: &mut [u8], timeout_ms: u64) -> SysResult<usize> {
    let ret = syscall4(
        SYS_RECV_MSG,
        port.0,
        buf.as_mut_ptr() as usize,
        buf.len(),
        timeout_ms as usize,
    );

    if ret < 0 {
        Err(SysError::from_code(ret))
    } else {
        Ok(ret as usize)
    }
}

/// Verifica se há mensagem na porta sem remover
pub fn peek(port: Port) -> SysResult<usize> {
    let ret = syscall4(SYS_PEEK_MSG, port.0, 0, 0, 0);

    if ret < 0 {
        Err(SysError::from_code(ret))
    } else {
        Ok(ret as usize)
    }
}
