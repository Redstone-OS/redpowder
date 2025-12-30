//! # User Heap Allocator
//!
//! Implementação simples de alocador que delega para o Kernel via Syscall.
//! Isso é ineficiente para muitas alocações pequenas, mas funciona sem dependências extras.

use crate::mem::mem::alloc as sys_alloc;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

// ============================================================================
// ALOCADOR GLOBAL (SYSCALL)
// ============================================================================

pub struct SyscallAllocator;

unsafe impl GlobalAlloc for SyscallAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Alocar via syscall
        // O kernel (redstone) alloc retorna memória alocada pelo PMM/VMM.
        // Se o tamanho for pequeno, o kernel pode arredondar para página (4KB).
        // Isso desperdiça memória, mas permite que Vec/String funcionem.

        match sys_alloc(layout.size(), 0) {
            Ok(ptr) => ptr,
            Err(_) => null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // Syscall free
        let _ = crate::mem::mem::free(ptr, layout.size());
    }
}

pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
