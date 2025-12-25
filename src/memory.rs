//! Memory - Low Level Virtual Memory Management
//!
//! # Análise Arquitetural Profunda
//!
//! Este módulo fornece a interface bruta para o Gerenciador de Memória Virtual (VMM) do Kernel.
//! É análogo ao `mmap` (Linux) ou `VirtualAlloc` (Windows).
//! **Não confunda com `malloc`!** Esta API aloca Páginas inteiras (4KiB), não objetos pequenos.
//!
//! ## Estrutura e Funcionamento
//!
//! 1.  **Syscall Wrapper**: `sys_alloc` pede ao Kernel "X bytes" (arredondado para cima em páginas).
//! 2.  **Permissões**: Permite definir Read/Write/Exec. Chave para JIT Compilers e segurança (NX bit).
//! 3.  **Resource Tracking**: O Kernel rastreia quais páginas pertencem ao processo. Se o processo morrer,
//!     essas páginas são liberadas automaticamente.
//!
//! ## Análise Crítica (Kernel Engineer Review)
//!
//! ### ✅ O que está bem feito (Conceitual)
//! *   **Simplificação**: Esconde a complexidade de Page Tables (PML4). O user-space apenas pede "memória".
//! *   **ASLR-Ready**: A API não aceita um endereço fixo como argumento obrigatório. O Kernel decide *onde* alocar.
//!
//! ### ❌ O que está mal feito / Riscos Atuais
//! *   **Fragmentação**: Se um App pedir 100 bytes, o Kernel aloca 4096 bytes (1 Page).
//!     *   *Impacto*: Desperdício massivo de RAM se usado diretamente para objetos pequenos.
//! *   **Lack of Heap**: O Redpowder ainda não exporta um `GlobalAllocator` compatível com Rust `std`.
//!     *   *Impacto*: Não podemos usar `Vec`, `String` ou `Box` em user-space ainda.
//!
//! ### ⚠️ Problemas de Arquitetura
//! *   **No Guard Pages**: Não há opção para criar páginas de guarda (stack overflow protection) explicitamente.
//!
//! # Guia de Implementação (TODOs)
//!
//! ## 1. Global Allocator (Urgency: Critical)
//! // TODO: Implementar a trait `core::alloc::GlobalAlloc`.
//! // - Estratégia: Usar uma crate como `talc` ou `linked_list_allocator`, usando `sys_alloc` como backend.
//! // - Impacto: Habilita `alloc` crate (Vec, String, Rc, Arc).
//!
//! ## 2. `mprotect` Equivalent (Urgency: Medium)
//! // TODO: Syscall para mudar permissões de páginas já alocadas.
//! // - Motivo: Loaders de executáveis precisam escrever código (RW) e depois torná-lo executável (RX).
//!
//! ## 3. Shared Memory (Urgency: High)
//! // TODO: `sys_shm_open` e `sys_map`.
//! // - Motivo: IPC de alta performance.

use crate::syscall::{syscall2, SysError, SysResult, SYS_ALLOC, SYS_FREE};

/// Flags de alocação
pub mod flags {
    pub const READ: u32 = 1 << 0;
    pub const WRITE: u32 = 1 << 1;
    pub const EXEC: u32 = 1 << 2;
}

/// Aloca memória virtual (Páginas)
///
/// # Argumentos
/// - `size`: Tamanho em bytes (arredondado para múltiplos de 4KiB)
/// - `flags`: Permissões (READ, WRITE, EXEC)
///
/// # Retorno
/// Endereço base da região alocada ou Erro.
pub fn alloc(size: usize, flags: u32) -> SysResult<*mut u8> {
    let ret = syscall2(SYS_ALLOC, size, flags as usize);

    if ret < 0 {
        Err(SysError::from_code(ret))
    } else {
        Ok(ret as *mut u8)
    }
}

/// Libera memória alocada
///
/// # Argumentos
/// - `ptr`: Endereço base (deve ser o mesmo retornado por alloc)
/// - `size`: Tamanho a liberar
pub fn free(ptr: *mut u8, size: usize) -> SysResult<()> {
    let ret = syscall2(SYS_FREE, ptr as usize, size);

    if ret < 0 {
        Err(SysError::from_code(ret))
    } else {
        Ok(())
    }
}

/// Helper: Aloca memória RW para dados genéricos
pub fn alloc_rw(size: usize) -> SysResult<*mut u8> {
    alloc(size, flags::READ | flags::WRITE)
}
