//! IPC - Inter-Process Communication Primitives
//!
//! # Análise Arquitetural Profunda
//!
//! O módulo de IPC é a "rede neural" do Redstone OS. Em uma arquitetura microkernel,
//! a velocidade e segurança do IPC ditam o desempenho global.
//! Este módulo fornece a abstração básica de **Portas** (Ports).
//!
//! ## Estrutura e Funcionamento
//!
//! 1.  **Portas (Ports)**: São identificadores opacos (Handles) para filas de mensagens no Kernel.
//! 2.  **Semântica de Cópia**: `send` copia dados do buffer do remetente para o Kernel; `recv` copia do Kernel para o buffer do destinatário.
//! 3.  **Sincronia**: `recv` pode bloquear a thread até que uma mensagem chegue.
//!
//! ## Análise Crítica (Kernel Engineer Review)
//!
//! ### ✅ O que está bem feito (Conceitual)
//! *   **Simplicidade**: API fácil de entender (`create`, `send`, `recv`).
//! *   **Type Safety**: O Wrapper `Port(pub usize)` evita confundir um File Descriptor com um Port Handle.
//!
//! ### ❌ O que está mal feito / Riscos Atuais
//! *   **Performance (Memcpy)**: Para mensagens grandes (ex: Texture Upload), copiar dados duas vezes (User->Kernel->User) é proibitivo.
//!     *   *Impacto*: Jogos e Apps de Mídia ficarão lentos.
//! *   **Discovery**: Como eu descubro a porta do serviço "Window Manager"? Nomes de porta hardcoded são frágeis.
//!
//! ### ⚠️ Problemas de Arquitetura
//! *   **Falta de Tipagem**: O payload é `&[u8]`. O receptor precisa "adivinhar" ou fazer cast manual do struct (inseguro).
//!
//! # Guia de Implementação (TODOs)
//!
//! ## 1. Shared Memory Channels (Urgency: Critical)
//! // TODO: Implementar canais baseados em memória compartilhada (Ring Buffer em User Space).
//! // - Motivo: Zero-copy IPC. Permite passar buffers de vídeo sem clonar bytes.
//!
//! ## 2. Typed Wrappers (Serde) (Urgency: High)
//! // TODO: Criar `NativeChannel<T>` que serializa structs automaticamente.
//! // - Motivo: Segurança. Evita corrupção de memória ao interpretar bytes brutos.
//!
//! ## 3. Async IPC (Urgency: High)
//! // TODO: Adicionar `async fn recv_async` integrada ao futuro Executor.
//! // - Motivo: Não bloquear a UI Thread esperando resposta do disco.

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

/// Envia mensagem para uma porta (System V style - Copy based)
pub fn send(port: Port, data: &[u8]) -> SysResult<usize> {
    let ret = syscall4(SYS_SEND_MSG, port.0, data.as_ptr() as usize, data.len(), 0);

    if ret < 0 {
        Err(SysError::from_code(ret))
    } else {
        Ok(ret as usize)
    }
}

/// Recebe mensagem de uma porta (Blocking or Timeout)
///
/// # Argumentos
/// - `port`: Porta de origem
/// - `buf`: Buffer de destino
/// - `timeout_ms`: Timeout em milissegundos (0 = não bloquear - poll)
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

/// Verifica se há mensagem na porta sem remover (Peek)
pub fn peek(port: Port) -> SysResult<usize> {
    let ret = syscall4(SYS_PEEK_MSG, port.0, 0, 0, 0);

    if ret < 0 {
        Err(SysError::from_code(ret))
    } else {
        Ok(ret as usize)
    }
}
