//! # Redpowder SDK - The User-Space Foundation
//!
//! # Análise Arquitetural Profunda
//!
//! **Redpowder** é a "Standard Library" (ou CRT - C Runtime) do Redstone OS.
//! O Kernel (Forge) fala apenas Syscalls e ABI binária. O Redpowder traduz isso para
//! Rust seguro. Todo serviço (Init, VFS) e todo App (Shell, Editor) depende desta crate.
//!
//! ## Estrutura e Funcionamento
//!
//! 1.  **Syscall Wrapper**: Converte chamadas de função Rust (`File::open`) em instruções
//!     de processador (`syscall` / `int 0x80`).
//! 2.  **Runtime Initialization**: O `_start` das aplicações não é a `main()`. O Redpowder fornece
//!     o código de "crt0" que inicializa o Heap, Argumentos (argv) e Variáveis de Ambiente antes do `main`.
//! 3.  **Heap Allocator**: Fornece um `GlobalAllocator` para que `Vec` e `Box` funcionem em user-space.
//!
//! ## Análise Crítica (Kernel Engineer Review)
//!
//! ### ✅ O que está bem feito (Conceitual)
//! *   **No-Std by Design**: Garante que o SDK seja leve e não arraste dependências ocultas.
//! *   **Modularidade**: Separa claramente IPC, Memória e Syscalls.
//!
//! ### ❌ O que está mal feito / Riscos Atuais
//! *   **Sync Syscalls Only**: Todas as syscalls parecem ser bloqueantes.
//!     *   *Risco*: UI congela se o disco for lento. Precisamos de `async/await` no nível do SDK.
//! *   **Lack of Unwinding**: Se um programa panica, ele aborta (`panic=abort`).
//!     *   *Impacto*: Difícil logar stack trace ou limpar recursos (RAII) em caso de crash.
//!
//! ### ⚠️ Problemas de Arquitetura & Segurança
//! *   **Raw Pointers**: A API de `syscall` expõe ponteiros crus inseguros.
//!     O Redpowder deve encapsular isso em referências seguras (`&str`, `&[u8]`).
//!
//! # Guia de Implementação (TODOs)
//!
//! ## 1. Runtime Entry Point (crt0) (Urgency: Critical)
//! // TODO: Implementar `_start` genérico que chama `main`.
//! // - Motivo: O desenvolvedor de app não deveria escrever `extern "C" _start`.
//!
//! ## 2. Async Runtime (Urgency: High)
//! // TODO: Implementar um Executor simples (tipo `smol` ou `embassy`) em user-space.
//! // - Impacto: Permitir IO não bloqueante (essencial para GUI e Networking).
//!
//! ## 3. Dynamic Linker Support (Future)
//! // TODO: Preparar para carregar bibliotecas dinâmicas (`.so` / `.dll`).
//! // - Motivo: Economizar RAM compartilhando código entre processos.

#![no_std]

// Módulos públicos
pub mod io;
pub mod ipc;
pub mod memory;
pub mod prelude;
pub mod syscall;
pub mod time;

// Re-exports principais
pub use syscall::{SysError, SysResult};
