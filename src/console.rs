//! # Console I/O
//!
//! Funções para I/O de console (serial).

use crate::syscall::{check_error, syscall0, syscall2, SysResult};
use crate::syscall::{SYS_CONSOLE_READ, SYS_CONSOLE_WRITE, SYS_POWEROFF, SYS_REBOOT};
use core::fmt::{self, Write};

/// Writer para console
struct ConsoleWriter;

impl Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = write_bytes(s.as_bytes());
        Ok(())
    }
}

/// Escreve bytes na console
pub fn write_bytes(buf: &[u8]) -> SysResult<usize> {
    let ret = syscall2(SYS_CONSOLE_WRITE, buf.as_ptr() as usize, buf.len());
    check_error(ret)
}

/// Escreve string na console
pub fn write_str(s: &str) -> SysResult<usize> {
    write_bytes(s.as_bytes())
}

/// Lê bytes da console (blocking)
pub fn read_bytes(buf: &mut [u8]) -> SysResult<usize> {
    let ret = syscall2(SYS_CONSOLE_READ, buf.as_mut_ptr() as usize, buf.len());
    check_error(ret)
}

/// Reinicia o sistema
pub fn reboot() -> ! {
    let _ = syscall0(SYS_REBOOT);
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

/// Desliga o sistema
pub fn poweroff() -> ! {
    let _ = syscall0(SYS_POWEROFF);
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

/// Função interna para print
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut writer = ConsoleWriter;
    let _ = writer.write_fmt(args);
}

/// Macro print! para console
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::console::_print(core::format_args!($($arg)*));
    }};
}

/// Macro println! para console
#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {{
        $crate::console::_print(core::format_args!($($arg)*));
        $crate::console::_print(core::format_args!("\n"));
    }};
}
