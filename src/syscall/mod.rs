//! Syscalls - The Kernel Boundary
//!
//! # Análise Arquitetural Profunda
//!
//! Este módulo é a "fronteira" entre o User-Space (Apps/Services) e o Kernel-Space (Forge).
//! Ele define *como* o processador muda de nível de privilégio (Ring 3 -> Ring 0).
//! Atualmente, implementa a convenção via interrupção de software (`int 0x80`).
//!
//! ## Estrutura e Funcionamento
//!
//! 1.  **ABI (Application Binary Interface)**: Define quais registradores carregam quais dados.
//!     *   `RAX`: Número da Syscall.
//!     *   `RDI`, `RSI`, `RDX`, `R10`, `R8`, `R9`: Argumentos 1 a 6.
//!     *   `RAX` (Retorno): Código de erro (negativo) ou sucesso (positivo).
//! 2.  **Inline Assembly**: Usa `core::arch::asm!` para emitir a instrução exata sem overhead de função.
//! 3.  **Vectored IO (Scatter/Gather)**: `sys_write` usa `IoVec` imediatamente, evitando buffers intermediários.
//!
//! ## Análise Crítica (Kernel Engineer Review)
//!
//! ### ✅ O que está bem feito (Conceitual)
//! *   **Type Safety**: As funções públicas (`sys_yield`, `sys_write`) retornam `SysResult`, forçando tratamento de erro.
//! *   **No-Stack Option**: As diretivas `options(nostack)` otimizam a chamada, assumindo que a syscall não suja a stack userspace.
//!
//! ### ❌ O que está mal feito / Riscos Atuais
//! *   **Instruction Choice (`int 0x80`)**: Em x86-64, a instrução `syscall` é muito mais rápida (menos ciclos de CPU) que uma interrupção de software.
//!     *   *Impacto*: Latência desnecessária em operações frequentes (como `sys_yield` ou `sys_write`).
//! *   **Pointer Validation**: O User-Space passa ponteiros (`*const u8`). Se passar lixo, o Kernel crasha?
//!     *   *Nota*: Isso é responsabilidade do Kernel (copy_from_user), mas o SDK deveria ajudar com abstrações melhores.
//!
//! ### ⚠️ Problemas de Arquitetura
//! *   **Argument Limit**: Só temos wrappers até `syscall4`. Algumas syscalls complexas (ex: `mmap`, `socket`) precisam de 6 argumentos.
//!
//! # Guia de Implementação (TODOs)
//!
//! ## 1. Migração para `syscall` (Urgency: High)
//! // TODO: Substituir `int 0x80` por `syscall` (e `sysret` no kernel).
//! // - Motivo: Performance moderna. `int 0x80` é legado 32-bit.
//!
//! ## 2. Wrapper para 6 Argumentos (Urgency: Medium)
//! // TODO: Implementar `syscall5` e `syscall6`.
//! // - Motivo: Suporte futuro a sockets e io_uring-like interfaces.
//!
//! ## 3. vDSO (Future)
//! // TODO: Mapear uma página de kernel em user-space (vDSO) para `sys_time` e `sys_getpid`.
//! // - Motivo: Ler o relógio não deveria exigir uma troca de contexto (Ring Switch).

use core::arch::asm;

// Sub-módulos
mod error;
mod numbers;

pub use error::{SysError, SysResult};
pub use numbers::*;

// ============================================================================
// INVOCAÇÃO DE SYSCALL (Assembly)
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
// PROCESSO (High Level)
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
// IO INTERNO (Vectored)
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
