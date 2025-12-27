//! # Raw Syscall Invocation
//!
//! Usa instrução `syscall` (não int 0x80!).
//!
//! ## Convenção de Registradores
//! - RAX: número da syscall
//! - RDI: arg1
//! - RSI: arg2
//! - RDX: arg3
//! - R10: arg4 (RCX é clobbered por syscall)
//! - R8: arg5
//! - R9: arg6
//! - RAX: retorno

use core::arch::asm;

/// Syscall com 0 argumentos
#[inline(always)]
pub fn syscall0(num: usize) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") num => ret,
            out("rcx") _,  // clobbered by syscall
            out("r11") _,  // clobbered by syscall
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// Syscall com 1 argumento
#[inline(always)]
pub fn syscall1(num: usize, arg1: usize) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") num => ret,
            in("rdi") arg1,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// Syscall com 2 argumentos
#[inline(always)]
pub fn syscall2(num: usize, arg1: usize, arg2: usize) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") num => ret,
            in("rdi") arg1,
            in("rsi") arg2,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// Syscall com 3 argumentos
#[inline(always)]
pub fn syscall3(num: usize, arg1: usize, arg2: usize, arg3: usize) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") num => ret,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// Syscall com 4 argumentos
#[inline(always)]
pub fn syscall4(num: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") num => ret,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("r10") arg4,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// Syscall com 5 argumentos
#[inline(always)]
pub fn syscall5(
    num: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") num => ret,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("r10") arg4,
            in("r8") arg5,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// Syscall com 6 argumentos
#[inline(always)]
pub fn syscall6(
    num: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") num => ret,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("r10") arg4,
            in("r8") arg5,
            in("r9") arg6,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }
    ret
}
